use anyhow::Result;
use ruff_diagnostics::Applicability;
use ruff_python_ast::Decorator;
use ruff_python_ast::whitespace::indentation;
use ruff_source_file::LineRanges;
use std::fmt;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use crate::importer::ImportRequest;
use crate::{Edit, Fix};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AbcReplacementDecorator {
    ClassMethod,
    StaticMethod,
    Property,
}

impl fmt::Display for AbcReplacementDecorator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ClassMethod => "classmethod",
            Self::StaticMethod => "staticmethod",
            Self::Property => "property",
        })
    }
}

pub(super) fn is_deprecated_abc_decorator(
    checker: &Checker,
    decorator: &Decorator,
    name: &str,
) -> bool {
    checker
        .semantic()
        .resolve_qualified_name(&decorator.expression)
        .is_some_and(|qualified_name| {
            matches!(qualified_name.segments(), ["abc", segment] if *segment == name)
        })
}

pub(super) fn replacement_fix(
    checker: &Checker,
    decorator: &Decorator,
    replacement: AbcReplacementDecorator,
) -> Result<Fix> {
    let (import_edit, binding) = checker.importer().get_or_import_symbol(
        &ImportRequest::import_from("abc", "abstractmethod"),
        decorator.start(),
        checker.semantic(),
    )?;

    let indent = indentation(checker.locator().contents(), decorator).unwrap_or_default();
    let line_ending = checker.stylist().line_ending().as_str();
    let content = format!(
        "{indent}@{replacement}{line_ending}{indent}@{binding}{line_ending}"
    );
    let range = checker.locator().full_lines_range(decorator.range());

    let applicability = if checker.comment_ranges().intersects(range) {
        Applicability::Unsafe
    } else {
        Applicability::Safe
    };

    Ok(Fix::applicable_edits(
        import_edit,
        [Edit::range_replacement(content, range)],
        applicability,
    ))
}
pub(super) fn deprecated_abc_fix_title(replacement: AbcReplacementDecorator) -> String {
    format!("Replace with `@{replacement}` and `@abstractmethod`")
}
