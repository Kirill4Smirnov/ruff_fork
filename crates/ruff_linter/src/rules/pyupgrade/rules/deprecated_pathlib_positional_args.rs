use ruff_diagnostics::Applicability;
use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, Expr};
use ruff_python_semantic::analyze::typing::{self, PathlibPathChecker, TypeChecker};
use ruff_text_size::{Ranged, TextRange};

use crate::FixAvailability;
use crate::Violation;
use crate::checkers::ast::Checker;
use crate::{Edit, Fix};

#[derive(Debug, Clone, Copy)]
enum DeprecatedPathlibMethod {
    IsRelativeTo,
    RelativeTo,
}

impl DeprecatedPathlibMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "is_relative_to" => Some(Self::IsRelativeTo),
            "relative_to" => Some(Self::RelativeTo),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::IsRelativeTo => "is_relative_to",
            Self::RelativeTo => "relative_to",
        }
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedPathlibPositionalArgs {
    method: DeprecatedPathlibMethod,
}

impl Violation for DeprecatedPathlibPositionalArgs {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { method } = self;
        format!(
            "Passing additional positional arguments to `PurePath.{}` is deprecated",
            method.as_str()
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Join additional path arguments explicitly".to_string())
    }
}

fn is_pathlib_path_expr(expr: &Expr, checker: &Checker) -> bool {
    match expr {
        Expr::Name(name) => checker
            .semantic()
            .only_binding(name)
            .map(|id| checker.semantic().binding(id))
            .is_some_and(|binding| typing::is_pathlib_path(binding, checker.semantic())),
        expr => PathlibPathChecker::match_initializer(expr, checker.semantic()),
    }
}

fn joined_argument_expr(checker: &Checker, value: &Expr, args: &[Expr]) -> Option<String> {
    if args.iter().any(Expr::is_starred_expr) {
        return None;
    }

    let joined_args = args
        .iter()
        .map(|arg| checker.locator().slice(arg.range()))
        .collect::<Vec<_>>()
        .join(", ");

    match value {
        Expr::Name(_) => {
            let receiver = checker.locator().slice(value.range());
            Some(format!("{receiver}.with_segments({joined_args})"))
        }
        Expr::Call(ast::ExprCall { func, .. }) if PathlibPathChecker::match_initializer(value, checker.semantic()) => {
            let constructor = checker.locator().slice(func.range());
            Some(format!("{constructor}({joined_args})"))
        }
        _ => None,
    }
}

fn replacement_call(
    checker: &Checker,
    value: &Expr,
    method: DeprecatedPathlibMethod,
    call: &ast::ExprCall,
) -> Option<String> {
    let joined = joined_argument_expr(checker, value, &call.arguments.args)?;
    let mut arguments = vec![joined];
    arguments.extend(
        call.arguments
            .keywords
            .iter()
            .map(|keyword| checker.locator().slice(keyword.range()).to_string()),
    );
    let receiver = checker.locator().slice(value.range());
    Some(format!(
        "{receiver}.{}({})",
        method.as_str(),
        arguments.join(", ")
    ))
}

/// ## What it does
/// Checks for deprecated uses of additional positional arguments to
/// `PurePath.is_relative_to()` and `PurePath.relative_to()`.
///
/// ## Why is this bad?
/// As of Python 3.12, passing more than one positional path argument to these
/// methods is deprecated and was removed in Python 3.14.
///
/// Additional path segments should be joined explicitly before calling the
/// method.
///
/// ## Example
/// ```python
/// from pathlib import PurePath
///
/// path = PurePath("a/b/c")
/// path.is_relative_to("a", "b")
/// path.relative_to("a", "b")
/// ```
///
/// Use instead:
/// ```python
/// from pathlib import PurePath
///
/// path = PurePath("a/b/c")
/// path.is_relative_to(path.with_segments("a", "b"))
/// path.relative_to(path.with_segments("a", "b"))
/// ```
///
/// ## Known problems
/// Ruff can only detect these calls when it can infer that the receiver is a
/// `pathlib` path object. In practice, this is limited to direct
/// `Path()`/`PurePath()` constructor calls and bindings with sufficiently clear
/// type information.
///
/// ## Fix safety
/// The fix is marked as unsafe if comments appear inside the call range, since
/// the rewritten argument list is normalized and comments may be dropped.
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `PurePath.is_relative_to`](https://docs.python.org/3/library/pathlib.html#pathlib.PurePath.is_relative_to)
/// - [Python documentation: `PurePath.relative_to`](https://docs.python.org/3/library/pathlib.html#pathlib.PurePath.relative_to)
/// UP063
pub(crate) fn deprecated_pathlib_positional_args(checker: &Checker, call: &ast::ExprCall) {
    if call.arguments.args.len() <= 1 {
        return;
    }

    let Expr::Attribute(ast::ExprAttribute { value, attr, .. }) = &*call.func else {
        return;
    };

    let Some(method) = DeprecatedPathlibMethod::from_name(attr.as_str()) else {
        return;
    };

    if !is_pathlib_path_expr(value, checker) {
        return;
    }

    let last = call.arguments.args.last().expect("len checked above");
    let deprecated_range = TextRange::new(call.arguments.args[1].start(), last.end());

    let mut diagnostic = checker.report_diagnostic(
        DeprecatedPathlibPositionalArgs { method },
        deprecated_range,
    );
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    if let Some(replacement) = replacement_call(checker, value, method, call) {
        diagnostic.try_set_fix(|| {
            let applicability = if checker.comment_ranges().intersects(call.range()) {
                Applicability::Unsafe
            } else {
                Applicability::Safe
            };
            Ok(Fix::applicable_edit(
                Edit::range_replacement(replacement, call.range()),
                applicability,
            ))
        });
    }
}
