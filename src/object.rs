use ahash::AHashMap;
use phf_macros::phf_map;
use phf;

type Int = u16;
type Float = f64;

const SPWN_GROUP: Int = 1001;

#[derive(Clone, PartialEq, Debug)]
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
        let groups = self.params.get(&57);

        self.by_spwn = if let Some(groups) = groups {
            match groups {
                ObjParam::Ints(groups) => groups.contains(&SPWN_GROUP),
                _ => unreachable!(),
            }
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
