use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, Expr};
use ruff_python_semantic::{SemanticModel, analyze::typing::find_binding_value};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for `gzip.GzipFile` calls that open for writing without an explicit
/// `mode` argument.
///
/// ## Why is this bad?
/// Since Python 3.9, opening `gzip.GzipFile` for writing without explicitly
/// specifying `mode` is deprecated.
///
/// Passing the mode explicitly makes the code clearer and avoids relying on
/// implicit mode inference from the underlying file object.
///
/// ## Example
/// ```python
/// import gzip
///
/// with open("archive.gz", "wb") as raw:
///     with gzip.GzipFile(fileobj=raw) as gz:
///         gz.write(b"payload")
/// ```
///
/// Use instead:
/// ```python
/// import gzip
///
/// with open("archive.gz", "wb") as raw:
///     with gzip.GzipFile(fileobj=raw, mode="wb") as gz:
///         gz.write(b"payload")
/// ```
///
/// ## References
/// - [Python documentation: `gzip.GzipFile`](https://docs.python.org/3/library/gzip.html#gzip.GzipFile)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "v0.0.285")]
pub(crate) struct DeprecatedGzipFileWithoutMode;

impl Violation for DeprecatedGzipFileWithoutMode {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Opening `gzip.GzipFile` for writing without explicit `mode` is deprecated".to_string()
    }
}

fn is_gzip_gzipfile(func: &Expr, semantic: &SemanticModel) -> bool {
    semantic
        .resolve_qualified_name(func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["gzip", "GzipFile"]))
}

fn is_pathlib_open_call(func: &Expr, semantic: &SemanticModel) -> bool {
    let Expr::Attribute(ast::ExprAttribute { attr, value, .. }) = func else {
        return false;
    };

    if attr.as_str() != "open" {
        return false;
    }

    if let Expr::Call(call) = value.as_ref() {
        return semantic
            .resolve_qualified_name(call.func.as_ref())
            .is_some_and(|qualified_name| {
                matches!(
                    qualified_name.segments(),
                    ["pathlib", "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath"]
                )
            });
    }

    let Expr::Name(name) = value.as_ref() else {
        return false;
    };
    let Some(binding_id) = semantic.resolve_name(name) else {
        return false;
    };
    let binding = semantic.binding(binding_id);
    let Some(Expr::Call(call)) = find_binding_value(binding, semantic) else {
        return false;
    };

    semantic
        .resolve_qualified_name(call.func.as_ref())
        .is_some_and(|qualified_name| {
            matches!(
                qualified_name.segments(),
                ["pathlib", "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath"]
            )
        })
}

fn mode_enables_writing(expr: &Expr) -> bool {
    let Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) = expr else {
        return false;
    };

    let mode = value.to_str();
    mode.contains('w') || mode.contains('a') || mode.contains('x') || mode.contains('+')
}

fn call_opens_writable_file(call: &ast::ExprCall, semantic: &SemanticModel) -> bool {
    if semantic.match_builtin_expr(&call.func, "open")
        || semantic
            .resolve_qualified_name(&call.func)
            .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["io" | "_io", "open"]))
    {
        return call
            .arguments
            .find_argument_value("mode", 1)
            .is_some_and(mode_enables_writing);
    }

    if is_pathlib_open_call(&call.func, semantic) {
        return call
            .arguments
            .find_argument_value("mode", 0)
            .is_some_and(mode_enables_writing);
    }

    if let Some(qualified_name) = semantic.resolve_qualified_name(&call.func) {
        return match qualified_name.segments() {
            ["tempfile", "TemporaryFile" | "NamedTemporaryFile"] => call
                .arguments
                .find_argument_value("mode", 0)
                .map_or(true, mode_enables_writing),
            ["tempfile", "SpooledTemporaryFile"] => call
                .arguments
                .find_argument_value("mode", 1)
                .map_or(true, mode_enables_writing),
            _ => false,
        };
    }

    false
}

fn fileobj_is_writable(expr: &Expr, semantic: &SemanticModel) -> bool {
    match expr {
        Expr::Call(call) => call_opens_writable_file(call, semantic),
        Expr::Name(name) => {
            let Some(binding_id) = semantic.resolve_name(name) else {
                return false;
            };
            let binding = semantic.binding(binding_id);
            let Some(Expr::Call(call)) = find_binding_value(binding, semantic) else {
                return false;
            };
            call_opens_writable_file(call, semantic)
        }
        _ => false,
    }
}

/// UP068
pub(crate) fn deprecated_gzipfile_without_mode(checker: &Checker, call: &ast::ExprCall) {
    if !is_gzip_gzipfile(&call.func, checker.semantic()) {
        return;
    }

    if call.arguments.args.iter().any(Expr::is_starred_expr)
        || call
            .arguments
            .keywords
            .iter()
            .any(|keyword| keyword.arg.is_none())
    {
        return;
    }

    if call.arguments.find_argument_value("mode", 1).is_some() {
        return;
    }

    let Some(fileobj) = call.arguments.find_argument_value("fileobj", 3) else {
        return;
    };

    if !fileobj_is_writable(fileobj, checker.semantic()) {
        return;
    }

    let mut diagnostic = checker.report_diagnostic(DeprecatedGzipFileWithoutMode, call.range());
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
}
