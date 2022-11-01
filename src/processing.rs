use ahash::HashMap;

use crate::object::{GDObj, ObjParam};

// I wasn't sure on how to name it
pub struct FeetAss {
    pub groups: HashMap<Vec<usize>, Vec<GDObj>>,
    pub static_objects: Vec<GDObj>,
}

impl FeetAss {
    pub fn new(objects: Vec<GDObj>) -> FeetAss {
        let (dynamic_objects, static_objects) = split_static_objects(objects);

        FeetAss {
            groups: HashMap::default(),
            static_objects,
        }
    }
}

/// # STATIC OBJECTS SPLIT
/// `(dynamic, static)`
fn split_static_objects(objects: Vec<GDObj>) -> (Vec<GDObj>, Vec<GDObj>) {
    objects.into_iter().partition(|obj|
        obj.params.get(&62)
            .map(|param| matches!(param, &ObjParam::Bool(true)))
            .unwrap_or(false)
    )
}
