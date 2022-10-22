use itertools::Itertools;

use crate::object::*;

use std::fmt::{Display, Formatter, Result};

macro_rules! has_props {
    ($p:ident; $($key:expr => $($value:expr)+;)*) => {
        {
            let mut matches = vec![];
            $(
                matches.push(
                    match $p.get_key_value(&$key) {
                        Some((_, value)) => {
                            match value {
                                $(
                                    $value => true,
                                )*
                                _ => false,
                            }
                        },
                        None => false,
                    }
                )
            )*
            matches.contains(&true)
        }
    };
}

/*
let name = obj_props! {
    1 OBJ_ID;
    71 FOLLOW # 1 => 1347;
    71 CENTER # 1 => 1346;
    71 TARGET_POS # 1 => 901;
};
*/

macro_rules! obj_props {
    ($($id:expr => $x:ident $(# $o_id:expr => $(v:expr $(,)?)+)?);*) => {
        println!("h")
    };
}

impl Display for GDObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut string = String::new();

        string.push_str("$.add(obj{\n");

        use ObjParam::*;

        let definition = self.params.keys().sorted().map(|key| {
            let param = &self.params[key];

            /*
            let a = obj_props!(
                1 => OBJ_ID;
                71 => FOLLOW # 1 => 1347;
                71 => CENTER # 1 => 1346;
                71 => TARGET_POS # 1 => 901;
            );
            */

            format!(
                "  {}: {},",
                key,
                match param {
                    Bool(val) => val.to_string(),
                    Float(val) => val.to_string(),
                    Int(val) => val.to_string(),
                    Ints(val) => format!(
                        "[{}]",
                        val
                            .iter()
                            .map(|v| v.to_string() + "g")
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                    Text(val) => {
                        let b64 = base64::decode(val).map(String::from_utf8);

                        if b64.is_ok() && b64.as_ref().unwrap().is_ok() && !val.is_empty() {
                            format!("$.b64encode({})", quote_string(&b64.unwrap().unwrap()))
                        } else {
                            // technically this is unreachable, but I'll let it in anyways
                            // (we could easily .unwrap() above without needing to check if it's valid or not)
                            quote_string(val)
                        }
                    },
                }
            )
        }).collect::<Vec<String>>().join("\n");

        string.push_str(&definition);

        string.push_str("\n})\n");

        write!(f, "{}", string)
    }
}

pub fn compile(objects: Vec<GDObj>) -> String {
    objects
        .iter()
        .map(|object|{
            format!("{}\n", object)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn quote_string(string: &str) -> String {
    // this makes sense, I swear
    if string.contains('\'') {
        format!("\"{}\"", string.replace('\"', "\\\""))
    } else {
        format!("\'{}\'", string.replace('\'', "\\\'"))
    }
}
