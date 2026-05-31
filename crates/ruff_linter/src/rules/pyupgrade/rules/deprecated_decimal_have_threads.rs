use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{Expr, PythonVersion};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use crate::{AlwaysFixableViolation, Edit, Fix};

/// ## What it does
/// Checks for uses of the deprecated `decimal.HAVE_THREADS` constant.
///
/// ## Why is this bad?
/// `decimal.HAVE_THREADS` is always `True`, since Python always has thread
/// support. As of Python 3.9, the constant is deprecated.
///
/// ## Example
/// ```python
/// import decimal
///
/// decimal.HAVE_THREADS
/// ```
///
/// Use instead:
/// ```python
/// import decimal
///
/// True
/// ```
///
/// ## References
/// - [Python documentation: `decimal.HAVE_THREADS`](https://docs.python.org/3/library/decimal.html#decimal.HAVE_THREADS)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedDecimalHaveThreads;

impl AlwaysFixableViolation for DeprecatedDecimalHaveThreads {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`decimal.HAVE_THREADS` is deprecated; it is always `True`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `True`".to_string()
    }
}

/// UP056
pub(crate) fn deprecated_decimal_have_threads(checker: &Checker, expr: &Expr) {
    if checker.target_version() < PythonVersion::PY39 {
        return;
    }

    if checker
        .semantic()
        .resolve_qualified_name(expr)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["decimal", "HAVE_THREADS"]))
    {
        let mut diagnostic = checker.report_diagnostic(DeprecatedDecimalHaveThreads, expr.range());
        diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
        diagnostic.set_fix(Fix::safe_edit(Edit::range_replacement(
            "True".to_string(),
            expr.range(),
        )));
    }
}
