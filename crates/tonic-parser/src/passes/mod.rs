use crate::Statement;
use std::cmp::Ordering;

pub fn pass(ast: &mut Vec<Statement>) {
    hoist_functions(ast);
    hoist_imports(ast);
}

fn hoist_functions(ast: &mut Vec<Statement>) {
    ast.sort_unstable_by(|a, _| if matches!(a, Statement::Function { .. }) {
        Ordering::Less
    } else {
        Ordering::Equal
    });
}

fn hoist_imports(ast: &mut Vec<Statement>) {
    ast.sort_unstable_by(|a, _| if matches!(a, Statement::Pub { .. }) {
        Ordering::Less
    } else {
        Ordering::Equal
    });
}