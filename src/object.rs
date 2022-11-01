use ahash::AHashMap;
use phf_macros::phf_map;


type Int = isize;
type Float = f64;

const SPWN_GROUP: Int = 1001;

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum ObjParam {
    Bool(bool),
    Int(Int),
    Ints(Vec<Int>),
    Float(Float),
    Text(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct GDObj {
    pub params: AHashMap<Int, ObjParam>,
    pub by_spwn: bool,
}

impl GDObj {
    pub fn new() -> GDObj {
        GDObj {
            params: Default::default(),
            by_spwn: false,
        }
    }
    pub fn update_by_spwn(&mut self) {
        if self.is_by_spwn() {
            if let Some(ObjParam::Ints(groups)) = self.params.get_mut(&57) {
                *groups = groups
                    .iter()
                    .filter(|&i| i != &SPWN_GROUP)
                    .copied()
                    .collect();
            }
            self.by_spwn = true
        }
    }
    pub fn is_by_spwn(&self) -> bool {
        self.by_spwn || self.has_group(SPWN_GROUP as u16)
    }
    pub fn has_group(&self, group_id: u16) -> bool {
        if let Some(ObjParam::Ints(v)) = self.params.get(&57) {
            v.contains(&(group_id as isize))
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParameterType {
    Bool,
    Text,
    IntegerArray,
}

use ParameterType::*;

pub static PROPERTIES: phf::Map<&'static str, ParameterType> = phf_map! {
    // the rest are just ints or floats, so not a huge issue
    "4" => Bool,
    "5" => Bool,
    "11" => Bool,
    "13" => Bool,
    "14" => Bool,
    "15" => Bool,
    "16" => Bool,
    "17" => Bool,
    "31" => Text,
    "34" => Bool,
    "41" => Bool,
    "42" => Bool,
    "43" => Text,
    "44" => Text,
    "49" => Text,
    "55" => Bool,
    "56" => Bool,
    "57" => IntegerArray,
    "58" => Bool,
    "59" => Bool,
    "60" => Bool,
    "62" => Bool,
    "64" => Bool,
    "65" => Bool,
    "66" => Bool,
    "67" => Bool,
    "70" => Bool,
    "78" => Bool,
    "81" => Bool,
    "86" => Bool,
    "87" => Bool,
    "89" => Bool,
    "93" => Bool,
    "94" => Bool,
    "96" => Bool,
    "98" => Bool,
    "99" => Bool,
    "100" => Bool,
    "102" => Bool,
    "103" => Bool,
    "104" => Bool,
    "106" => Bool,
};
