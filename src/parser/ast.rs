// Abstract Syntax Tree definitions

#[derive(Debug)]
pub enum Ast {
    Module { body: Vec<Statement> }
}

#[derive(Debug)]
pub enum Statement {
    Global { names: Vec<String> },
    Nonlocal { names: Vec<String> },
    Pass,
    Break,
    Continue,
    Return { value: Option<Expression> }
}

#[derive(Debug)]
pub enum Expression {
    Num(usize)
}