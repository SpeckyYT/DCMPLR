use crate::object::*;


impl GDObj {
    fn to_obj(&self) -> String {
        let mut string = String::new();

        string.push_str("$.add(obj{\n");

        use ObjParam::*;

        let definition = self.params.iter().map(|(key, param)|
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
                    Text(val) => val.to_string(),
                }
            )
        ).collect::<Vec<String>>().join("\n");

        string.push_str(&definition);

        string.push_str("\n})\n");

        string
    }
}

pub fn compile(objects: Vec<GDObj>) -> String {
    objects
        .iter()
        .map(|object|{
            object.to_obj()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
