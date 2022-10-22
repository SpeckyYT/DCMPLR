use std::fmt::{Display, Formatter, Result};

use ahash::HashMap;
use itertools::Itertools;

use crate::object::GDObj;

pub struct Body(Vec<Statement>);

pub enum Statement {
    // a = b;
    Definition {
        variable_name: String,
        expression: Expression,
    },
    // if a { b } (else {})
    If {
        condition: Expression,
        if_body: Body,
        // else_body: Option<Body>,
    },
    // a!
    Spawn(Expression),
}

pub enum Expression {
    // obj { }
    Object(GDObj),
    // trigger { }
    Trigger(GDObj),
    // !{ }
    TriggerFunction(Vec<Statement>),
    // ?g / ?c / ?b / ?mogus
    Id(Id),
    // 69.420
    Number(Number),
    // "helo"
    String(String),
    // { a: mogus, b: ussy }
    Dictionary(HashMap<String, Expression>),
    // [ "69", "420" ]
    Array(Vec<Expression>),
}

pub enum Number {
    Int(i128),
    Float(f64),
}

pub struct Id {
    number: u16,
    unspecified: bool,
    class_name: IdClass,
}

pub enum IdClass {
    Group,
    Color,
    Item,
    Block,
}


impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for stmnt in &self.0 {
            write!(f, "{}\n", stmnt)?
        }
        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Statement::*;

        match self {
            Definition {
                variable_name,
                expression,
            } => write!(f, "let {} = {}", variable_name, expression),

            If {
                condition,
                if_body,
            } => write!(f, "if {} {{\n{}\n}}", condition, if_body),

            Spawn(expr) => write!(f, "{}!", expr),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Expression::*;

        match self {
            // Expression::Id()
            _ => todo!(),
        }
    }
}
