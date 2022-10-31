use itertools::Itertools;

use crate::object::*;

use std::fmt::{Display, Formatter, Result};

/*
let name = obj_props! {
    1 OBJ_ID;
    71 FOLLOW # 1 => 1347;
    71 CENTER # 1 => 1346;
    71 TARGET_POS # 1 => 901;
};
*/

macro_rules! obj_props {
    ($key:ident; $params:expr; $($id:literal $(=> ($($m:literal)*))? : $x:ident,)*) => {
        match $key {
            $(
                $id $(
                    if [$($m,)*].contains(
                        match $params.get(&1).unwrap_or(&ObjParam::Int(0)) {
                            ObjParam::Int(v) => v,
                            _ => unreachable!()
                        }
                    )
                )? => stringify!($x).to_string(),
            )*
            _ => format!("{}", $key),
        }
    };
}

impl Display for GDObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut string = String::new();

        string.push_str("$.add(obj {\n");

        use ObjParam::*;

        let definition = self.params.keys().sorted().map(|key| {
            let param = &self.params[key];

            let key_string = obj_props!{
                key; self.params;
                1: OBJ_ID,
                2: X,
                3: Y,
                4: HORIZONTAL_FLIP,
                5: VERTICAL_FLIP,
                6: ROTATION,
                7: TRIGGER_RED,
                8: TRIGGER_GREEN,
                9: TRIGGER_BLUE,
                10: DURATION,
                11: TOUCH_TRIGGERED,
                13: PORTAL_CHECKED,
                15: PLAYER_COLOR_1,
                16: PLAYER_COLOR_2,
                17: BLENDING,
                20: EDITOR_LAYER_1,
                21: COLOR,
                22: COLOR_2,
                23: TARGET_COLOR,
                24: Z_LAYER,
                25: Z_ORDER,
                28: MOVE_X,
                29: MOVE_Y,
                30: EASING,
                31: TEXT,
                32: SCALING,
                34: GROUP_PARENT,
                35: OPACITY,
                36: ACTIVE_TRIGGER,
                41: HVS_ENABLED,
                42: COLOR_2_HVS_ENABLED,
                43: HVS,
                44: COLOR_2_HVS,
                45: FADE_IN,
                46: HOLD,
                47: FADE_OUT,
                48: PULSE_HSV,
                49: COPIED_COLOR_HVS,
                50: COPIED_COLOR_ID,
                51: TARGET,
                52: TARGET_TYPE,
                54: YELLOW_TELEPORTATION_PORTAL_DISTANCE,
                56: ACTIVATE_GROUP,
                57: GROUPS,
                58: LOCK_TO_PLAYER_X,
                59: LOCK_TO_PLAYER_Y,
                60: COPY_OPACITY,
                61: EDITOR_LAYER_2,
                62: SPAWN_TRIGGERED,
                63: SPAWN_DURATION,
                64: DONT_FADE,
                65: MAIN_ONLY,
                66: DETAIL_ONLY,
                67: DONT_ENTER,
                68: ROTATE_DEGREES,
                69: TIMES_360,
                70: LOCK_OBJECT_ROTATION,
                71 => (901): TARGET_POS,
                71 => (1346): CENTER,
                71 => (1347): FOLLOW,
                71: TARGET_POS,
                72: X_MOD,
                73: Y_MOD,
                75: STRENGTH,
                76: ANIMATION_ID,
                77: COUNT,
                78: SUBTRACT_COUNT,
                79: PICKUP_MODE,
                80 => (1816): BLOCK_A,
                80: BLOCK_A,
                80: ITEM,
                81: HOLD_MODE,
                82: TOGGLE_MODE,
                84: INTERVAL,
                85: EASING_RATE,
                86: EXCLUSIVE,
                87: MULTI_TRIGGER,
                88: COMPARISON,
                89: DUAL_MODE,
                90: SPEED,
                91: DELAY,
                92: Y_OFFSET,
                93: ACTIVATE_ON_EXIT,
                94: DYNAMIC_BLOCK,
                95: BLOCK_B,
                96: GLOW_DISABLED,
                97: ROTATION_SPEED,
                98: DISABLE_ROTATION,
                104: COUNT_MULTI_ACTIVATE,
                100: USE_TARGET,
                101: TARGET_POS_AXES,
                102: EDITOR_DISABLE,
                103: HIGH_DETAIL,
                105: MAX_SPEED,
                106: RANDOMIZE_START,
                107: ANIMATION_SPEED,
                108: LINKED_GROUP,
            };

            format!(
                "  {}: {},",
                key_string,
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
