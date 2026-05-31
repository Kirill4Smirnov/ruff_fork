use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{Expr, PythonVersion};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use crate::Violation;

/// ## What it does
/// Checks for uses of `codecs.open` on Python 3.14+.
///
/// ## Why is this bad?
/// `codecs.open` is deprecated as of Python 3.14. Prefer the builtin `open`
/// instead.
///
/// ## Example
/// ```python
/// import codecs
///
/// with codecs.open("file.txt", encoding="utf-8") as f:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// with open("file.txt", encoding="utf-8") as f:
///     ...
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `codecs.open`](https://docs.python.org/3/library/codecs.html#codecs.open)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "0.15.15")]
pub(crate) struct DeprecatedCodecsOpen;

impl Violation for DeprecatedCodecsOpen {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`codecs.open` is deprecated, use builtin `open`".to_string()
    }
}

/// UP052
pub(crate) fn deprecated_codecs_open(checker: &Checker, func: &Expr) {
    if checker.target_version() < PythonVersion::PY314 {
        return;
    }

    if checker
        .semantic()
        .resolve_qualified_name(func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["codecs", "open"]))
    {
        let mut diagnostic = checker.report_diagnostic(DeprecatedCodecsOpen, func.range());
        diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    }
}
