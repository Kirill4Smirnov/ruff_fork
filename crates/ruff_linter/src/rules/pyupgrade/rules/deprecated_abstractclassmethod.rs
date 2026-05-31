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
/// Checks for uses of the deprecated `abc.abstractclassmethod` decorator.
///
/// ## Why is this bad?
/// Since Python 3.3, `@classmethod` can be stacked with `@abstractmethod`,
/// making `abc.abstractclassmethod` redundant.
///
/// ## Example
/// ```python
/// from abc import abstractclassmethod
///
///
/// class Base:
///     @abstractclassmethod
///     def create(cls):
///         ...
/// ```
///
/// Use instead:
/// ```python
/// from abc import abstractmethod
///
///
/// class Base:
///     @classmethod
///     @abstractmethod
///     def create(cls):
///         ...
/// ```
///
/// ## Fix safety
/// This fix is marked as unsafe if comments are attached to the deprecated
/// decorator line, since those comments are replaced.
///
/// ## References
/// - [Python documentation: `abc.abstractclassmethod`](https://docs.python.org/3/library/abc.html#abc.abstractclassmethod)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedAbstractClassmethod;

impl Violation for DeprecatedAbstractClassmethod {
    const FIX_AVAILABILITY: crate::FixAvailability = crate::FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "`abc.abstractclassmethod` is deprecated, use `@{}` with `@abstractmethod` instead",
            AbcReplacementDecorator::ClassMethod
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some(deprecated_abc_fix_title(
            AbcReplacementDecorator::ClassMethod,
        ))
    }
}

/// UP057
pub(crate) fn deprecated_abstractclassmethod(checker: &Checker, decorator_list: &[Decorator]) {
    for decorator in decorator_list {
        if is_deprecated_abc_decorator(checker, decorator, "abstractclassmethod") {
            let mut diagnostic =
                checker.report_diagnostic(DeprecatedAbstractClassmethod, decorator.range());
            diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
            diagnostic.try_set_fix(|| {
                replacement_fix(checker, decorator, AbcReplacementDecorator::ClassMethod)
            });
        }
    }
}
