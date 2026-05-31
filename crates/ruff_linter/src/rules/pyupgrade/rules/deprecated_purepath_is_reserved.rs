use ruff_diagnostics::Applicability;
use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, Expr};
use ruff_python_semantic::analyze::typing::{self, PathlibPathChecker, TypeChecker};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;
use crate::importer::ImportRequest;
use crate::{Edit, Fix, FixAvailability};

/// ## What it does
/// Checks for uses of the deprecated `PurePath.is_reserved()` method.
///
/// ## Why is this bad?
/// As of Python 3.13, `PurePath.is_reserved()` is deprecated in favor of
/// `os.path.isreserved()`.
///
/// ## Example
/// ```python
/// from pathlib import PurePath
///
/// PurePath("NUL").is_reserved()
/// ```
///
/// Use instead:
/// ```python
/// from os.path import isreserved
/// from pathlib import PurePath
///
/// isreserved(PurePath("NUL"))
/// ```
///
/// ## Known problems
/// This rule is likely to have false negatives, as Ruff can only emit the
/// lint if it can say with confidence that the receiver is a `pathlib`
/// path object. In practice, this is limited to bindings that originate from
/// a direct `Path()`/`PurePath()` constructor call or from an annotated
/// function parameter or variable.
///
/// ## Fix safety
/// This rule's fix is marked as safe unless the call expression contains
/// comments, in which case it is marked as unsafe.
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `PurePath.is_reserved()`](https://docs.python.org/3/library/pathlib.html#pathlib.PurePath.is_reserved)
/// - [Python documentation: `os.path.isreserved()`](https://docs.python.org/3/library/os.path.html#os.path.isreserved)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedPurePathIsReserved;

impl Violation for DeprecatedPurePathIsReserved {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`PurePath.is_reserved()` is deprecated, use `os.path.isreserved()` instead"
            .to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `os.path.isreserved()`".to_string())
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

/// UP060
pub(crate) fn deprecated_purepath_is_reserved(checker: &Checker, call: &ast::ExprCall) {
    if call.arguments.len() != 0 {
        return;
    }

    let Expr::Attribute(ast::ExprAttribute { value, attr, .. }) = &*call.func else {
        return;
    };

    if attr != "is_reserved" || !is_pathlib_path_expr(value, checker) {
        return;
    }

    let mut diagnostic =
        checker.report_diagnostic(DeprecatedPurePathIsReserved, call.func.range());
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    diagnostic.try_set_fix(|| {
        let path_expr = checker.locator().slice(value.range());
        let (import_edit, binding) = checker.importer().get_or_import_symbol(
            &ImportRequest::import_from("os.path", "isreserved"),
            call.start(),
            checker.semantic(),
        )?;
        let applicability = if checker.comment_ranges().intersects(call.range()) {
            Applicability::Unsafe
        } else {
            Applicability::Safe
        };

        Ok(Fix::applicable_edits(
            Edit::range_replacement(format!("{binding}({path_expr})"), call.range()),
            [import_edit],
            applicability,
        ))
    });
}
