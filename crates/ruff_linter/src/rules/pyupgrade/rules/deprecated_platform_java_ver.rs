use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;
use crate::FixAvailability;

/// ## What it does
/// Checks for uses of the deprecated `platform.java_ver()` function.
///
/// ## Why is this bad?
/// As of Python 3.13, `platform.java_ver()` is deprecated and scheduled for
/// removal in Python 3.15.
///
/// The function was primarily useful for Jython support and has a confusing,
/// largely untested API.
///
/// ## Example
/// ```python
/// import platform
///
/// platform.java_ver()
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `platform.java_ver`](https://docs.python.org/3/library/platform.html#platform.java_ver)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedPlatformJavaVer;

impl Violation for DeprecatedPlatformJavaVer {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::None;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`platform.java_ver()` is deprecated and will be removed in Python 3.15".to_string()
    }
}

/// UP061
pub(crate) fn deprecated_platform_java_ver(checker: &Checker, call: &ast::ExprCall) {
    if checker
        .semantic()
        .resolve_qualified_name(&call.func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["platform", "java_ver"]))
    {
        let mut diagnostic = checker.report_diagnostic(DeprecatedPlatformJavaVer, call.func.range());
        diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    }
}
