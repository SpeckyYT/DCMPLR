use itertools::Itertools;

use crate::object::{
    GDObj,
    ObjParam,
    ParameterType,
    PROPERTIES,
};

#[derive(Debug, Clone)]
pub struct Level {
    pub objects: Vec<GDObj>,
}

impl Level {
    /// # SPWN SPLIT
    /// `(spwn, non_spwn)`
    pub fn split_spwn(self) -> (Vec<GDObj>, Vec<GDObj>) {
        self.objects
            .into_iter()
            .partition(|obj| obj.by_spwn)
    }
}

pub fn parse_level(level_string: &str) -> Level {
    let mut level = Level {
        objects: vec![],
    };

    let objects_strings: Vec<&str> = level_string.split(";").collect();

    for object_string in objects_strings {
        let mut object = GDObj::new();

        object_string
            .split(",")
            .tuples()
            .map(|(key, value)| {
                let key_int = key.parse().unwrap();

                let typ = PROPERTIES.get(key);
                if let Some(typ) = typ {
                    use ParameterType::*;

                    match typ {
                        Bool => {
                            object.params.insert(
                                key_int,
                                ObjParam::Bool(match value {
                                    "0" => false,
                                    "1" => true,
                                    _ => true, // ????
                                }),
                            );
                        },
                        IntegerArray => {
                            object.params.insert(
                                key_int,
                                ObjParam::Ints(value.split(".").map(|v| v.parse().unwrap()).collect()),
                            );
                        },
                        Text => {
                            object.params.insert(
                                key_int,
                                ObjParam::Text(value.to_string()),
                            );
                        },
                    };
                } else {
                    object.params.insert(
                        key_int,
                        if value.contains(".") {
                            ObjParam::Float(value.parse().unwrap())
                        } else {
                            ObjParam::Int(value.parse().unwrap())
                        }
                    );
                }
            })
            .for_each(drop);

        if object.params.is_empty() { continue }

        object.update_by_spwn();
        level.objects.push(object);
    }

    level
}
