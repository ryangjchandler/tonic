use crate::Statement;
use std::cmp::Ordering;

pub fn pass(ast: &mut Vec<Statement>) {
    hoist_functions(ast);
}

fn hoist_functions(ast: &mut Vec<Statement>) {
    ast.sort_unstable_by(|a, _| if matches!(a, Statement::Function { .. }) {
        Ordering::Less
    } else {
        Ordering::Equal
    });
}