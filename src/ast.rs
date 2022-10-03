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
    Int(isize),
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


impl Body {
    fn to_code(&self, indent: usize) -> String {
        self.0
            .iter()
            .map(|stmnt| format!("{};", stmnt.to_code(indent)))
            .join("\n")
    }
}

impl Statement {
    fn to_code(&self, indent: usize) -> String {
        use Statement::*;

        match self {
            Definition {
                variable_name,
                expression,
            } => format!("let {} = {}", variable_name, expression.to_code(indent)),

            If {
                condition,
                if_body,
            } => format!("if {} {{\n{}\n}}", condition.to_code(0), if_body.to_code(indent + 1)),

            Spawn(expr) => format!("{}!", expr.to_code(indent)),
        }
    }
}

impl Expression {
    fn to_code(&self, indent: usize) -> String {
        use Expression::*;

        match self {
            // Expression::Id()
            _ => todo!(),
        }
    }
}
