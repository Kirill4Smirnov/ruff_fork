use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{Expr, PythonVersion};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for uses of the deprecated `sys.monitoring.events.BRANCH` constant.
///
/// ## Why is this bad?
/// In Python 3.14, `sys.monitoring.events.BRANCH` was deprecated in favor of
/// the more specific `BRANCH_LEFT` and `BRANCH_RIGHT` constants.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.monitoring.events.BRANCH
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// sys.monitoring.events.BRANCH_LEFT
/// sys.monitoring.events.BRANCH_RIGHT
/// ```
///
/// ## References
/// - [Python documentation: `sys.monitoring`](https://docs.python.org/3/library/sys.monitoring.html)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedSysMonitoringBranch;

impl Violation for DeprecatedSysMonitoringBranch {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.monitoring.events.BRANCH` is deprecated, use `BRANCH_LEFT` or `BRANCH_RIGHT` instead"
            .to_string()
    }
}

/// UP054
pub(crate) fn deprecated_sys_monitoring_branch(checker: &Checker, expr: &Expr) {
    if checker.target_version() < PythonVersion::PY314 {
        return;
    }

    if checker
        .semantic()
        .resolve_qualified_name(expr)
        .is_some_and(|qualified_name| {
            matches!(qualified_name.segments(), ["sys", "monitoring", "events", "BRANCH"])
        })
    {
        let mut diagnostic = checker.report_diagnostic(DeprecatedSysMonitoringBranch, expr.range());
        diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    }
}
