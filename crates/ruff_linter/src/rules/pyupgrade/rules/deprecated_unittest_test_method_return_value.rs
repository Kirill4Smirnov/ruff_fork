use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, PythonVersion, Stmt};
use ruff_python_semantic::{
    ScopeKind,
    analyze::{class::any_qualified_base_class, visibility::is_test},
};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for non-`None` return values in `unittest` test methods.
///
/// ## Why is this bad?
/// In Python 3.11 and later, returning a value other than `None` from a
/// `unittest` test method is deprecated.
///
/// Such return values are ignored by `unittest`, so returning one is usually a
/// mistake or leftover debugging code.
///
/// ## Example
/// ```python
/// import unittest
///
///
/// class SomeTest(unittest.TestCase):
///     def test_something(self):
///         return 1
/// ```
///
/// Use instead:
/// ```python
/// import unittest
///
///
/// class SomeTest(unittest.TestCase):
///     def test_something(self):
///         self.assertEqual(1, 1)
/// ```
///
/// ## References
/// - [Python documentation: `unittest` basic example](https://docs.python.org/3/library/unittest.html#basic-example)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "v0.0.285")]
pub(crate) struct DeprecatedUnittestTestMethodReturnValue;

impl Violation for DeprecatedUnittestTestMethodReturnValue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Returning a value from a `unittest` test method is deprecated".to_string()
    }
}

fn is_unittest_test_case(class_def: &ast::StmtClassDef, checker: &Checker) -> bool {
    any_qualified_base_class(class_def, checker.semantic(), |qualified_name| {
        matches!(
            qualified_name.segments(),
            ["unittest", "TestCase" | "IsolatedAsyncioTestCase"]
                | ["unittest", "case", "TestCase"]
                | ["unittest", "async_case", "IsolatedAsyncioTestCase"]
        )
    })
}

fn in_unittest_test_method(checker: &Checker) -> bool {
    let mut function_def = None;
    let mut class_def = None;

    for scope in checker.semantic().current_scopes() {
        match scope.kind {
            ScopeKind::Function(scope_function_def) if function_def.is_none() => {
                function_def = Some(scope_function_def);
            }
            ScopeKind::Class(scope_class_def) if class_def.is_none() => {
                class_def = Some(scope_class_def);
                if function_def.is_some() {
                    break;
                }
            }
            _ => {}
        }
    }

    let Some(function_def) = function_def else {
        return false;
    };
    let Some(class_def) = class_def else {
        return false;
    };

    is_test(&function_def.name) && is_unittest_test_case(class_def, checker)
}

/// UP065
pub(crate) fn deprecated_unittest_test_method_return_value(checker: &Checker, stmt: &Stmt) {
    if checker.target_version() < PythonVersion::PY311 {
        return;
    }

    let Stmt::Return(ast::StmtReturn { value, .. }) = stmt else {
        return;
    };

    let Some(expr) = value else {
        return;
    };

    if expr.is_none_literal_expr() {
        return;
    }

    if !in_unittest_test_method(checker) {
        return;
    }

    let mut diagnostic =
        checker.report_diagnostic(DeprecatedUnittestTestMethodReturnValue, stmt.range());
    diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
}
