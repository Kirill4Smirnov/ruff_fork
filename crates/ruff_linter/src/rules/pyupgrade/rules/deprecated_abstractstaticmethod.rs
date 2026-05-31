use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::Decorator;
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

use super::deprecated_abc_decorator::{
    AbcReplacementDecorator, deprecated_abc_fix_title, is_deprecated_abc_decorator,
    replacement_fix,
};

/// ## What it does
/// Checks for uses of the deprecated `abc.abstractstaticmethod` decorator.
///
/// ## Why is this bad?
/// Since Python 3.3, `@staticmethod` can be stacked with `@abstractmethod`,
/// making `abc.abstractstaticmethod` redundant.
///
/// ## Example
/// ```python
/// from abc import abstractstaticmethod
///
///
/// class Base:
///     @abstractstaticmethod
///     def create():
///         ...
/// ```
///
/// Use instead:
/// ```python
/// from abc import abstractmethod
///
///
/// class Base:
///     @staticmethod
///     @abstractmethod
///     def create():
///         ...
/// ```
///
/// ## Fix safety
/// This fix is marked as unsafe if comments are attached to the deprecated
/// decorator line, since those comments are replaced.
///
/// ## References
/// - [Python documentation: `abc.abstractstaticmethod`](https://docs.python.org/3/library/abc.html#abc.abstractstaticmethod)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedAbstractStaticmethod;

impl Violation for DeprecatedAbstractStaticmethod {
    const FIX_AVAILABILITY: crate::FixAvailability = crate::FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "`abc.abstractstaticmethod` is deprecated, use `@{}` with `@abstractmethod` instead",
            AbcReplacementDecorator::StaticMethod
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some(deprecated_abc_fix_title(
            AbcReplacementDecorator::StaticMethod,
        ))
    }
}

/// UP058
pub(crate) fn deprecated_abstractstaticmethod(checker: &Checker, decorator_list: &[Decorator]) {
    for decorator in decorator_list {
        if is_deprecated_abc_decorator(checker, decorator, "abstractstaticmethod") {
            let mut diagnostic =
                checker.report_diagnostic(DeprecatedAbstractStaticmethod, decorator.range());
            diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
            diagnostic.try_set_fix(|| {
                replacement_fix(checker, decorator, AbcReplacementDecorator::StaticMethod)
            });
        }
    }
}
