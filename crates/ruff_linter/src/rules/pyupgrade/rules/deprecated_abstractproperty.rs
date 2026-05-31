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
/// Checks for uses of the deprecated `abc.abstractproperty` decorator.
///
/// ## Why is this bad?
/// Since Python 3.3, `@property` can be stacked with `@abstractmethod`,
/// making `abc.abstractproperty` redundant.
///
/// ## Example
/// ```python
/// from abc import abstractproperty
///
///
/// class Base:
///     @abstractproperty
///     def name(self):
///         ...
/// ```
///
/// Use instead:
/// ```python
/// from abc import abstractmethod
///
///
/// class Base:
///     @property
///     @abstractmethod
///     def name(self):
///         ...
/// ```
///
/// ## Fix safety
/// This fix is marked as unsafe if comments are attached to the deprecated
/// decorator line, since those comments are replaced.
///
/// ## References
/// - [Python documentation: `abc.abstractproperty`](https://docs.python.org/3/library/abc.html#abc.abstractproperty)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "NEXT_RUFF_VERSION")]
pub(crate) struct DeprecatedAbstractProperty;

impl Violation for DeprecatedAbstractProperty {
    const FIX_AVAILABILITY: crate::FixAvailability = crate::FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "`abc.abstractproperty` is deprecated, use `@{}` with `@abstractmethod` instead",
            AbcReplacementDecorator::Property
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some(deprecated_abc_fix_title(AbcReplacementDecorator::Property))
    }
}

/// UP059
pub(crate) fn deprecated_abstractproperty(checker: &Checker, decorator_list: &[Decorator]) {
    for decorator in decorator_list {
        if is_deprecated_abc_decorator(checker, decorator, "abstractproperty") {
            let mut diagnostic =
                checker.report_diagnostic(DeprecatedAbstractProperty, decorator.range());
            diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
            diagnostic.try_set_fix(|| {
                replacement_fix(checker, decorator, AbcReplacementDecorator::Property)
            });
        }
    }
}
