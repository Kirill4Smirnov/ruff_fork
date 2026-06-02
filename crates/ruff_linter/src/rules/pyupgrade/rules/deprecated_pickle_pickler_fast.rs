use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, Expr};
use ruff_python_semantic::{
    BindingKind,
    SemanticModel,
    analyze::{class::any_qualified_base_class, typing::find_binding_value},
};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for uses of the deprecated `pickle.Pickler.fast` attribute.
///
/// ## Why is this bad?
/// `pickle.Pickler.fast` is deprecated.
///
/// Enabling fast mode disables the memo used by the pickler, which can lead
/// to infinite recursion for self-referential objects. If you need more compact
/// pickles, use `pickletools.optimize()` instead.
///
/// ## Example
/// ```python
/// import io
/// import pickle
///
///
/// pickler = pickle.Pickler(io.BytesIO())
/// pickler.fast = True
/// ```
///
/// ## References
/// - [Python documentation: `pickle.Pickler.fast`](https://docs.python.org/3/library/pickle.html#pickle.Pickler.fast)
/// - [Python documentation: `pickletools.optimize`](https://docs.python.org/3/library/pickletools.html#pickletools.optimize)
#[derive(ViolationMetadata)]
#[violation_metadata(stable_since = "v0.0.285")]
pub(crate) struct DeprecatedPicklePicklerFast;

impl Violation for DeprecatedPicklePicklerFast {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pickle.Pickler.fast` is deprecated".to_string()
    }
}

fn is_pickler_qualified_name(qualified_name: &[&str]) -> bool {
    matches!(qualified_name, ["pickle", "Pickler" | "_Pickler"])
}

fn is_pickler_constructor(expr: &Expr, semantic: &SemanticModel) -> bool {
    semantic
        .resolve_qualified_name(expr)
        .is_some_and(|qualified_name| is_pickler_qualified_name(qualified_name.segments()))
}

fn is_pickler_class_definition(class_def: &ast::StmtClassDef, semantic: &SemanticModel) -> bool {
    any_qualified_base_class(class_def, semantic, |qualified_name| {
        is_pickler_qualified_name(qualified_name.segments())
    })
}

fn is_pickler_like(expr: &Expr, semantic: &SemanticModel) -> bool {
    if is_pickler_constructor(expr, semantic) {
        return true;
    }

    match expr {
        Expr::Call(ast::ExprCall { func, .. }) => is_pickler_constructor(func, semantic),
        Expr::Name(name) => {
            let Some(binding_id) = semantic.resolve_name(name) else {
                return false;
            };
            let binding = semantic.binding(binding_id);

            match binding.kind {
                BindingKind::ClassDefinition(_) => binding.statement(semantic).is_some_and(|stmt| {
                    let ast::Stmt::ClassDef(class_def) = stmt else {
                        return false;
                    };
                    is_pickler_class_definition(class_def, semantic)
                }),
                _ => find_binding_value(binding, semantic).is_some_and(|value| {
                    matches!(value, Expr::Call(ast::ExprCall { func, .. }) if is_pickler_constructor(func, semantic))
                }),
            }
        }
        _ => false,
    }
}

/// UP066
pub(crate) fn deprecated_pickle_pickler_fast(checker: &Checker, expr: &Expr) {
    let Expr::Attribute(attribute) = expr else {
        return;
    };

    if attribute.attr.as_str() != "fast" {
        return;
    }

    if checker
        .semantic()
        .resolve_qualified_name(expr)
        .is_some_and(|qualified_name| {
            matches!(
                qualified_name.segments(),
                ["pickle", "Pickler" | "_Pickler", "fast"]
            )
        })
        || is_pickler_like(&attribute.value, checker.semantic())
    {
        let mut diagnostic = checker.report_diagnostic(DeprecatedPicklePicklerFast, expr.range());
        diagnostic.add_primary_tag(ruff_db::diagnostic::DiagnosticTag::Deprecated);
    }
}
