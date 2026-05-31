use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{Expr, PythonVersion};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for uses of the deprecated `sys.last_type`, `sys.last_value`, and
/// `sys.last_traceback` variables.
///
/// ## Why is this bad?
/// These variables are the deprecated legacy representation of
/// `sys.last_exc`. For projects targeting Python 3.12 or later, prefer
/// `sys.last_exc` instead.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.last_value
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// sys.last_exc
/// ```
///
/// ## References
/// - [Python documentation: `sys.last_exc`](https://docs.python.org/3/library/sys.html#sys.last_exc)
/// - [Python documentation: `sys.last_type`](https://docs.python.org/3/library/sys.html#sys.last_type)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedSysLastVars {
    existing: String,
}

impl Violation for DeprecatedSysLastVars {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DeprecatedSysLastVars { existing } = self;
        format!("`sys.{existing}` is deprecated, use `sys.last_exc` instead")
    }
}

/// UP055
pub(crate) fn deprecated_sys_last_vars(checker: &Checker, expr: &Expr) {
    if checker.target_version() < PythonVersion::PY312 {
        return;
    }

    let Some(existing) = checker
        .semantic()
        .resolve_qualified_name(expr)
        .and_then(|qualified_name| match qualified_name.segments() {
            ["sys", "last_type"] => Some("last_type"),
            ["sys", "last_value"] => Some("last_value"),
            ["sys", "last_traceback"] => Some("last_traceback"),
            _ => None,
        })
    else {
        return;
    };

    let mut diagnostic = checker.report_diagnostic(
        DeprecatedSysLastVars {
            existing: existing.to_string(),
        },
        expr.range(),
    );
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
}
