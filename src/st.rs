use crate::data::{Data, Typed};

pub enum AST {
    Symbol(String),
    Data(Data),
    Let {
        variable:   String,
        expression: Box<AST>,
    },
    Lambda {
        argument: String,
        expression: Box<AST>,
    }
}

pub enum TST {
    Symbol(Typed<String>),
    Data(Typed<Data>),
    Let {
        variable:   Typed<String>,
        expression: Box<Typed<TST>>,
    },
    Lambda {
        argument:   Typed<String>,
        epxression: Box<Typed<TST>>,
    }
}
