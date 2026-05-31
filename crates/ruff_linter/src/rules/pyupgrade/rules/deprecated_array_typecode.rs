use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, Expr, PythonVersion};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use crate::{AlwaysFixableViolation, Edit, Fix};

/// ## What it does
/// Checks for uses of the deprecated `"u"` typecode in `array.array(...)` calls.
///
/// ## Why is this bad?
/// The `"u"` typecode is deprecated and will be removed in Python 3.16.
/// For projects targeting Python 3.13 or later, use the `"w"` typecode instead.
///
/// ## Example
/// ```python
/// import array
///
/// array.array("u", "abc")
/// ```
///
/// Use instead:
/// ```python
/// import array
///
/// array.array("w", "abc")
/// ```
///
/// ## References
/// - [Python documentation: `array`](https://docs.python.org/3/library/array.html)
///
/// ## Fix safety
/// Replacing `"u"` with `"w"` is unsafe, as the two typecodes use different
/// underlying C representations and item sizes.
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedArrayTypecode;

impl AlwaysFixableViolation for DeprecatedArrayTypecode {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`array` typecode `u` is deprecated, use `w` instead".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `w` typecode".to_string()
    }
}

/// UP053
pub(crate) fn deprecated_array_typecode(checker: &Checker, call: &ast::ExprCall) {
    if checker.target_version() < PythonVersion::PY313 {
        return;
    }

    if !checker
        .semantic()
        .resolve_qualified_name(&call.func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["array", "array"]))
    {
        return;
    }

    let Some(typecode_arg) = call.arguments.find_argument_value("typecode", 0) else {
        return;
    };

    let Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) = typecode_arg else {
        return;
    };

    if value.to_str() != "u" {
        return;
    }

    let mut diagnostic = checker.report_diagnostic(DeprecatedArrayTypecode, typecode_arg.range());
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    diagnostic.set_fix(Fix::unsafe_edit(Edit::range_replacement(
        format!("{}w{}", checker.stylist().quote(), checker.stylist().quote()),
        typecode_arg.range(),
    )));
}
