use crate::data::{Data, Type, Typed};
use crate::st::{AST, TST};

pub fn hm(ast: AST) -> Result<TST, String> {
    let pst: TST = preliminary(ast);
    let system = constraints(&pst);
    solve(pst, system)
}

pub struct Constraint {
    left: Type,
    right: Type,
}

fn 

// assign a unique preliminary type to every expression
pub fn preliminary(ast: AST) -> TST {
    match ast {
        AST::Symbol(s) => TST::Symbol(Typed::new(s, Type::Pre())),
        AST::Data(_) => {}
        AST::Let { variable, expression } => {}
        AST::Lambda { argument, expression } => {}
    }
}

// using the preliminary TST, generates a set of constraints that must be solved
pub fn constraints(pst: &TST) -> Vec<Constraint> {
    todo!()
}

pub fn solve(pst: TST, constraints: Vec<Constraint>) -> Result<TST, String> {
    todo!()
}
