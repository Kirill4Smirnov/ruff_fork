use anyhow::Result;

use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, Expr};
use ruff_python_semantic::SemanticModel;
use ruff_python_semantic::analyze::typing::{self, PathlibPathChecker, TypeChecker, check_type};
use ruff_text_size::Ranged;

use crate::FixAvailability;
use crate::Violation;
use crate::checkers::ast::Checker;
use crate::importer::ImportRequest;
use crate::{Edit, Fix};

#[derive(Debug, Clone, Copy)]
enum GuessTypeKind {
    ModuleFunction,
    MimeTypesMethod,
}

struct MimeTypesChecker;

impl MimeTypesChecker {
    fn is_mimetypes_constructor(semantic: &SemanticModel, expr: &Expr) -> bool {
        semantic
            .resolve_qualified_name(expr)
            .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["mimetypes", "MimeTypes"]))
    }
}

impl TypeChecker for MimeTypesChecker {
    fn match_annotation(annotation: &Expr, semantic: &SemanticModel) -> bool {
        Self::is_mimetypes_constructor(semantic, annotation)
    }

    fn match_initializer(initializer: &Expr, semantic: &SemanticModel) -> bool {
        let Expr::Call(ast::ExprCall { func, .. }) = initializer else {
            return false;
        };

        Self::is_mimetypes_constructor(semantic, func)
    }
}

#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedMimetypesGuessTypePath;

impl Violation for DeprecatedMimetypesGuessTypePath {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Passing file paths to `mimetypes.guess_type()` is soft deprecated; use `guess_file_type()` instead"
            .to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `guess_file_type()`".to_string())
    }
}

fn is_path_argument(expr: &Expr, checker: &Checker) -> bool {
    match expr {
        Expr::Name(name) => checker
            .semantic()
            .only_binding(name)
            .map(|id| checker.semantic().binding(id))
            .is_some_and(|binding| typing::is_pathlib_path(binding, checker.semantic())),
        expr => PathlibPathChecker::match_initializer(expr, checker.semantic()),
    }
}

fn classify_call(func: &Expr, checker: &Checker) -> Option<GuessTypeKind> {
    if checker
        .semantic()
        .resolve_qualified_name(func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["mimetypes", "guess_type"]))
    {
        return Some(GuessTypeKind::ModuleFunction);
    }

    let Expr::Attribute(ast::ExprAttribute { value, attr, .. }) = func else {
        return None;
    };

    if attr != "guess_type" {
        return None;
    }

    match &**value {
        Expr::Name(name) => checker
            .semantic()
            .only_binding(name)
            .map(|id| checker.semantic().binding(id))
            .is_some_and(|binding| check_type::<MimeTypesChecker>(binding, checker.semantic())),
        expr => MimeTypesChecker::match_initializer(expr, checker.semantic()),
    }
    .then_some(GuessTypeKind::MimeTypesMethod)
}

fn module_fix(checker: &Checker, call: &ast::ExprCall) -> Result<Fix> {
    let (import_edit, binding) = checker.importer().get_or_import_symbol(
        &ImportRequest::import_from("mimetypes", "guess_file_type"),
        call.func.start(),
        checker.semantic(),
    )?;

    Ok(Fix::safe_edits(
        Edit::range_replacement(binding, call.func.range()),
        [import_edit],
    ))
}

fn method_fix(call: &ast::ExprCall) -> Result<Fix> {
    let Expr::Attribute(ast::ExprAttribute { attr, .. }) = &*call.func else {
        anyhow::bail!("expected attribute call")
    };

    Ok(Fix::safe_edit(Edit::range_replacement(
        "guess_file_type".to_string(),
        attr.range(),
    )))
}

/// ## What it does
/// Checks for passing file paths to `mimetypes.guess_type()`.
///
/// ## Why is this bad?
/// As of Python 3.13, passing a file path to `mimetypes.guess_type()` is soft
/// deprecated. Use `mimetypes.guess_file_type()` instead.
///
/// ## Example
/// ```python
/// import mimetypes
/// from pathlib import Path
///
/// mimetypes.guess_type(Path("image.png"))
/// ```
///
/// Use instead:
/// ```python
/// import mimetypes
/// from pathlib import Path
///
/// mimetypes.guess_file_type(Path("image.png"))
/// ```
///
/// ## Known problems
/// Ruff only emits this diagnostic when it can confidently infer that the
/// argument is a `pathlib` path object. String arguments are intentionally not
/// linted, since they may represent either file paths or URLs.
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `mimetypes.guess_type`](https://docs.python.org/3/library/mimetypes.html#mimetypes.guess_type)
/// - [Python documentation: `mimetypes.guess_file_type`](https://docs.python.org/3/library/mimetypes.html#mimetypes.guess_file_type)
/// UP064
pub(crate) fn deprecated_mimetypes_guess_type_path(checker: &Checker, call: &ast::ExprCall) {
    let Some(kind) = classify_call(&call.func, checker) else {
        return;
    };

    let Some(path_arg) = call.arguments.find_argument_value("url", 0) else {
        return;
    };

    if !is_path_argument(path_arg, checker) {
        return;
    }

    let mut diagnostic = checker.report_diagnostic(DeprecatedMimetypesGuessTypePath, call.func.range());
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    match kind {
        GuessTypeKind::ModuleFunction => {
            diagnostic.try_set_fix(|| module_fix(checker, call));
        }
        GuessTypeKind::MimeTypesMethod => {
            diagnostic.try_set_fix(|| method_fix(call));
        }
    }
}
