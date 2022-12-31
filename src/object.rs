use std::{convert::Infallible, str::FromStr};

pub use crate::trigger::*;
use ahash::AHashMap;
//use phf_macros::phf_map;

macro_rules! obj_params {
    (
        [$name:ident]
        $(
            $param_id:literal
            $(#
                (
                    $($obj_id:literal)*
                )
            )?
            : $param_name:ident($param_type:ty)
            $((
                $($to_string:tt)*
            ))?,
        )*
    ) => {
        pub mod $name {
            use super::*;

            #[allow(non_camel_case_types)]
            #[derive(Clone, Debug, PartialEq)]
            pub enum Params {
                $(
                    $param_name($param_type)
                ),*
            }

            impl ToString for Params {
                fn to_string(&self) -> String {
                    match self {
                        $(
                            Self::$param_name(v) => {
                                (obj_params!(@ $param_type, ( $($($to_string)*)? )))(v)
                            },
                        )*
                    }
                }
            }

            #[allow(non_snake_case)]
            pub mod Id {
                use super::*;

                $(
                    pub const $param_name: Int = $param_id;
                )*
            }

            pub fn from_key(key: Int, val: &str) -> self::Params {
                match key {
                    $(
                        $param_id => self::Params::$param_name(<$param_type as std::str::FromStr>::from_str(val).unwrap()),
                    )*
                    k => panic!("unknown key {k}"),
                }
            }

            pub fn param_name(id: Int, obj_id: Option<Int>) -> String {
                match id {
                    $(
                        $param_id $(
                            if [$(Some($obj_id),)*].contains(&obj_id)
                        )? => stringify!($param_name).to_string(),
                    )*
                    _ => format!("{}", id),
                }
            }
        }
    };

    (@ $typ:ty, ($($to_string:tt)+)) => {{
        $($to_string)+
    }};

    (@ $typ:ty, ()) => {{
        |_v: &$typ| _v.to_string()
    }};
}

pub type Int = isize;
pub type Float = f64;

// const SPWN_GROUP: Int = 1001;

obj_params! {
    [ObjectParams]

    1: OBJ_ID(Int),
    2: X(Float),
    3: Y(Float),
    4: HORIZONTAL_FLIP(Bool),
    5: VERTICAL_FLIP(Bool),
    6: ROTATION(Float),
    7: TRIGGER_RED(Int),
    8: TRIGGER_GREEN(Int),
    9: TRIGGER_BLUE(Int),
    10: DURATION(Float),
    11: TOUCH_TRIGGERED(Bool),
    13: PORTAL_CHECKED(Bool),
    15: PLAYER_COLOR_1(Bool),
    16: PLAYER_COLOR_2(Bool),
    17: BLENDING(Bool),
    20: EDITOR_LAYER_1(Int),

    21: COLOR(Int) (|col: &Int| -> String { format!("{}c", col) }),
    22: COLOR_2(Int) (|col2: &Int|-> String { format!("{}c", col2) }),
    23: TARGET_COLOR(Int) (|target_col: &Int| -> String { format!("{}c", target_col) }),

    24: Z_LAYER(Int),
    25: Z_ORDER(Int),
    28: MOVE_X(Float),
    29: MOVE_Y(Float),
    30: EASING(Int),
    31: TEXT(String),
    32: SCALING(Float),
    34: GROUP_PARENT(Bool),
    35: OPACITY(Float),
    36: ACTIVE_TRIGGER(Bool),
    41: HVS_ENABLED(Bool),
    42: COLOR_2_HVS_ENABLED(Bool),
    43: HVS(String),
    44: COLOR_2_HVS(String),
    45: FADE_IN(Float),
    46: HOLD(Float),
    47: FADE_OUT(Float),
    48: PULSE_HSV(Bool),
    49: COPIED_COLOR_HVS(String),
    50: COPIED_COLOR_ID(Int),
    51: TARGET(Int),
    52: TARGET_TYPE(Int),
    54: YELLOW_TELEPORTATION_PORTAL_DISTANCE(Float),
    56: ACTIVATE_GROUP(Bool),

    57: GROUPS(FromStrVec<Int>)
    (|groups: &FromStrVec<Int>| -> String {
        format!("[{}]", groups.iter().map(|g| format!("{}g", g)).collect::<Vec<_>>().join(","))
    }),

    58: LOCK_TO_PLAYER_X(Bool),
    59: LOCK_TO_PLAYER_Y(Bool),
    60: COPY_OPACITY(Bool),
    61: EDITOR_LAYER_2(Int),
    62: SPAWN_TRIGGERED(Bool),
    63: SPAWN_DURATION(Float),
    64: DONT_FADE(Bool),
    65: MAIN_ONLY(Bool),
    66: DETAIL_ONLY(Bool),
    67: DONT_ENTER(Bool),
    68: ROTATE_DEGREES(Float),
    69: TIMES_360(Float),
    70: LOCK_OBJECT_ROTATION(Bool),
    // TODO: these?
    //71 # (901): TARGET_POS(Int),
    71 # (1346): CENTER(Int),
    71 # (1347): FOLLOW(Int),
    71: TARGET_POS(Int),
    72: X_MOD(Float),
    73: Y_MOD(Float),
    75: STRENGTH(Float),
    76: ANIMATION_ID(Int),
    77: COUNT(Int),
    78: SUBTRACT_COUNT(Int),
    79: PICKUP_MODE(Int),
    80 # (1816): BLOCK_A(Int),
    // TODO: these?
    //80: BLOCK_A(Int),
    80: ITEM(Int),
    81: HOLD_MODE(Bool),
    82: TOGGLE_MODE(Int),
    84: INTERVAL(Float),
    85: EASING_RATE(Float),
    86: EXCLUSIVE(Bool),
    87: MULTI_TRIGGER(Bool),
    88: COMPARISON(Int),
    89: DUAL_MODE(Bool),
    90: SPEED(Float),
    91: DELAY(Float),
    92: Y_OFFSET(Float),
    93: ACTIVATE_ON_EXIT(Bool),
    94: DYNAMIC_BLOCK(Bool),
    95: BLOCK_B(Int),
    96: GLOW_DISABLED(Bool),
    97: ROTATION_SPEED(Float),
    98: DISABLE_ROTATION(Bool),
    104: COUNT_MULTI_ACTIVATE(Bool),
    100: USE_TARGET(Bool),
    101: TARGET_POS_AXES(Int),
    102: EDITOR_DISABLE(Bool),
    103: HIGH_DETAIL(Bool),
    105: MAX_SPEED(Float),
    106: RANDOMIZE_START(Bool),
    107: ANIMATION_SPEED(Float),
    108: LINKED_GROUP(Int),
}

// wrapper vec to implement FromStr on
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStrVec<T: FromStr>(pub Vec<T>)
where
    <T as FromStr>::Err: std::fmt::Debug;

impl<T: FromStr> FromStr for FromStrVec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            value.split('.').map(|v| v.parse::<T>().unwrap()).collect(),
        ))
    }
}

impl<T: FromStr> std::ops::Deref for FromStrVec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// (cant parse "0" or "1" into a bool with FromStr)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bool(pub bool);

impl FromStr for Bool {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(match value {
            "0" | "false" => false,
            "1" | "true" => true,
            _ => unreachable!(),
        }))
    }
}

impl std::ops::Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Params = AHashMap<Int, ObjectParams::Params>;

#[derive(Clone, Debug)]
pub struct Block {
    pub params: Params,
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Block(Block),
    Trigger(Trigger),
}

impl ObjectType {
    pub fn groups(&self) -> Option<&Vec<isize>> {
        let params = match self {
            Self::Block(b) => &b.params,
            Self::Trigger(t) => &t.params,
        };

        match params.get(&ObjectParams::Id::GROUPS) {
            Some(ObjectParams::Params::GROUPS(g)) => Some(g),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<isize> {
        let params = match self {
            Self::Block(b) => &b.params,
            Self::Trigger(t) => &t.params,
        };

        match params.get(&ObjectParams::Id::OBJ_ID) {
            Some(ObjectParams::Params::OBJ_ID(i)) => Some(*i),
            _ => None,
        }
    }

    pub fn get_trigger(&self) -> &Trigger {
        match self {
            Self::Trigger(t) => t,
            Self::Block(..) => unreachable!(),
        }
    }
}

// #[derive(Clone, PartialEq, Debug)]
// pub struct GDObj {
//     pub params: AHashMap<Int, ObjectParams::Params>,
//     pub by_spwn: bool,
// }

// impl GDObj {
//     pub fn new() -> GDObj {
//         GDObj {
//             params: Default::default(),
//             by_spwn: false,
//         }
//     }
//     pub fn update_by_spwn(&mut self) {
//         if self.is_by_spwn() {
//             if let Some(ObjectParamVal::Ints(groups)) = self.params.get_mut(&ObjectParams::Id::GROUPS) {
//                 *groups = groups
//                     .iter()
//                     .filter(|&i| i != &SPWN_GROUP)
//                     .copied()
//                     .collect();
//             }
//             self.by_spwn = true
//         }
//     }
//     pub fn is_by_spwn(&self) -> bool {
//         self.by_spwn || self.has_group(SPWN_GROUP as u16)
//     }
//     pub fn has_group(&self, group_id: u16) -> bool {
//         if let Some(ObjectParamVal::Ints(v)) = self.params.get(&57) {
//             v.contains(&(group_id as isize))
//         } else {
//             false
//         }
//     }
// }

// #[derive(Copy, Clone, PartialEq, Eq, Debug)]
// pub enum ParameterType {
//     Bool,
//     Text,
//     IntegerArray,
// }

// use ParameterType::*;

// pub static PROPERTIES: phf::Map<&'static str, ParameterType> = phf_map! {
//     // the rest are just ints or floats, so not a huge issue
//     "1" => IntegerArray,
//     "4" => Bool,
//     "5" => Bool,
//     "11" => Bool,
//     "13" => Bool,
//     "14" => Bool,
//     "15" => Bool,
//     "16" => Bool,
//     "17" => Bool,
//     "31" => Text,
//     "34" => Bool,
//     "41" => Bool,
//     "42" => Bool,
//     "43" => Text,
//     "44" => Text,
//     "49" => Text,
//     "55" => Bool,
//     "56" => Bool,
//     "57" => IntegerArray,
//     "58" => Bool,
//     "59" => Bool,
//     "60" => Bool,
//     "62" => Bool,
//     "64" => Bool,
//     "65" => Bool,
//     "66" => Bool,
//     "67" => Bool,
//     "70" => Bool,
//     "78" => Bool,
//     "81" => Bool,
//     "86" => Bool,
//     "87" => Bool,
//     "89" => Bool,
//     "93" => Bool,
//     "94" => Bool,
//     "96" => Bool,
//     "98" => Bool,
//     "99" => Bool,
//     "100" => Bool,
//     "102" => Bool,
//     "103" => Bool,
//     "104" => Bool,
//     "106" => Bool,
// };
