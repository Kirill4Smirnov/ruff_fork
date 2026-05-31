use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast};
use ruff_text_size::Ranged;

use crate::FixAvailability;
use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for deprecated uses of the `flags` parameter to `select.epoll()`.
///
/// ## Why is this bad?
/// The `flags` parameter to `select.epoll()` has been deprecated since Python
/// 3.4. `select.EPOLL_CLOEXEC` is now used by default.
///
/// If you need to make the file descriptor inheritable, use
/// `os.set_inheritable()` instead.
///
/// ## Example
/// ```python
/// import select
///
/// select.epoll(flags=select.EPOLL_CLOEXEC)
/// ```
///
/// ## References
/// - [Python documentation: `select.epoll`](https://docs.python.org/3/library/select.html#select.epoll)
/// - [Python documentation: `os.set_inheritable`](https://docs.python.org/3/library/os.html#os.set_inheritable)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedSelectEpollFlags;

impl Violation for DeprecatedSelectEpollFlags {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::None;

    #[derive_message_formats]
    fn message(&self) -> String {
        "The `flags` parameter to `select.epoll()` is deprecated; use `os.set_inheritable()` if needed"
            .to_string()
    }
}

/// UP062
pub(crate) fn deprecated_select_epoll_flags(checker: &Checker, call: &ast::ExprCall) {
    if !checker
        .semantic()
        .resolve_qualified_name(&call.func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["select", "epoll"]))
    {
        return;
    }

    let range = if let Some(keyword) = call.arguments.find_keyword("flags") {
        keyword.range()
    } else if let Some(arg) = call.arguments.args.get(1) {
        arg.range()
    } else {
        return;
    };

    let mut diagnostic = checker.report_diagnostic(DeprecatedSelectEpollFlags, range);
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
}
