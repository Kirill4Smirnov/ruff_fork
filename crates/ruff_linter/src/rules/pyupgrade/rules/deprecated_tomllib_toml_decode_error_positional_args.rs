use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{Arguments, ExprCall, Keyword};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for deprecated constructor calls to `tomllib.TOMLDecodeError`.
///
/// ## Why is this bad?
/// In Python 3.14, `tomllib.TOMLDecodeError` deprecated the old
/// `ValueError`-style constructor behavior.
///
/// Call `tomllib.TOMLDecodeError` with only the `msg`, `doc`, and `pos`
/// arguments instead.
///
/// ## Example
/// ```python
/// import tomllib
///
/// raise tomllib.TOMLDecodeError("message")
/// ```
///
/// Use instead:
/// ```python
/// import tomllib
///
/// raise tomllib.TOMLDecodeError("message", document, position)
/// ```
///
/// ## References
/// - [Python documentation: `tomllib.TOMLDecodeError`](https://docs.python.org/3/library/tomllib.html#tomllib.TOMLDecodeError)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "v0.0.285")]
pub(crate) struct DeprecatedTomllibTomlDecodeErrorPositionalArgs;

impl Violation for DeprecatedTomllibTomlDecodeErrorPositionalArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Passing anything other than `msg`, `doc`, and `pos` to `tomllib.TOMLDecodeError` is deprecated".to_string()
    }
}

fn record_keyword(keyword: &Keyword, seen: &mut [bool; 3]) -> bool {
    let Some(arg) = keyword.arg.as_ref() else {
        return false;
    };

    let slot = match arg.as_str() {
        "msg" => 0,
        "doc" => 1,
        "pos" => 2,
        _ => return false,
    };

    if seen[slot] {
        return false;
    }

    seen[slot] = true;
    true
}

fn uses_deprecated_signature(arguments: &Arguments) -> bool {
    // Avoid flagging dynamic calls that we can't reason about confidently.
    if arguments.args.iter().any(|arg| arg.is_starred_expr())
        || arguments.keywords.iter().any(|keyword| keyword.arg.is_none())
    {
        return false;
    }

    if arguments.args.len() > 3 {
        return true;
    }

    let mut seen = [false; 3];
    for slot in seen.iter_mut().take(arguments.args.len()) {
        *slot = true;
    }

    for keyword in &arguments.keywords {
        if !record_keyword(keyword, &mut seen) {
            return false;
        }
    }

    !seen.into_iter().all(std::convert::identity)
}

/// UP067
pub(crate) fn deprecated_tomllib_toml_decode_error_positional_args(
    checker: &Checker,
    call: &ExprCall,
) {
    if !checker
        .semantic()
        .resolve_qualified_name(&call.func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["tomllib", "TOMLDecodeError"]))
    {
        return;
    }

    if !uses_deprecated_signature(&call.arguments) {
        return;
    }

    let mut diagnostic = checker.report_diagnostic(
        DeprecatedTomllibTomlDecodeErrorPositionalArgs,
        call.range(),
    );
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
}
