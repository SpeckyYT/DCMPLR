use ahash::AHashMap;
use itertools::Itertools;
use num_traits::FromPrimitive;

use crate::object::{Block, Int, ObjectParams, ObjectType, Trigger, TriggerType};

#[derive(Clone)]
pub struct Level {
    pub objects: Vec<ObjectType>,
}

// intermediary object
#[derive(Default)]
struct UnknownObject {
    pub params: AHashMap<Int, ObjectParams::Params>,
}

pub fn parse_level(level_string: &str) -> Level {
    let mut level = Level { objects: vec![] };

    let objects_strings: Vec<&str> = level_string.split(';').collect();

    for object_string in objects_strings {
        let mut temp_object = UnknownObject::default();

        object_string.split(',').tuples().for_each(|(key, value)| {
            let (key, value) = (key.trim(), value.trim());

            let key_int: isize = key.parse().unwrap();

            temp_object
                .params
                .insert(key_int, ObjectParams::from_key(key_int, value));
        });

        if let Some(ObjectParams::Params::OBJ_ID(id)) =
            temp_object.params.get(&ObjectParams::Id::OBJ_ID)
        {
            let maybe_typ: Option<TriggerType> = FromPrimitive::from_isize(*id);

            if let Some(typ) = maybe_typ {
                level.objects.push(ObjectType::Trigger(Trigger {
                    params: temp_object.params,
                    form: typ,
                }));
            } else {
                level.objects.push(ObjectType::Block(Block {
                    params: temp_object.params,
                }));
            }
        } else {
            continue;
        }
    }

    level
}
