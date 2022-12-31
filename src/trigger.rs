use num_derive::FromPrimitive;

use crate::object::{ObjectParams, Params};

macro_rules! trigger_fns {
    (
        [$name:ident]

        $(
            $trig_name:ident => $fn_name:ident(
                $($params:tt)*
            )
        )*
    ) => {
        pub struct $name {
            form: TriggerType,
        }

        impl $name {
            pub fn with_params(&self, params: &Params) -> String {
                match self.form {
                    $(
                        TriggerType::$trig_name => {
                            format!(
                                "{}({})",
                                stringify!($fn_name),
                                trigger_fns!(@genparams [params] $($params)*)
                            )
                        },
                    )*

                    _ => todo!()
                }
            }
        }

        impl From<TriggerType> for $name {
            fn from(t: TriggerType) -> $name {
                $name {
                    form: t,
                }
            }
        }
    };

    (@genparams [$params:ident] $param_name:ident $($rest:tt)*) => {{
        let mut args = match $params.get(&ObjectParams::Id:: $param_name ) {
            Some(ObjectParams::Params:: $param_name (val)) => {
                format!(trigger_fns!(@formatarg $($rest)*), val.to_string())
            },
            _ => {
                trigger_fns!(@defaultval $($rest)*)
            }
        };

        args.push_str(
            &trigger_fns!(@genparams [$params] $($rest)*)
        );

        args
    }};

    // still more arguments to insert after the current one ($rest)
    (@formatarg $( $__:ident )?$( ? )?$( = $_:literal )? , $($rest:tt)+) => {
        "{}, "
    };

    // no more arguments to insert after the current one (no $rest)
    (@formatarg $( $__:ident )?$( ? )?$( = $_:literal )? , ) => {
        "{}"
    };

    // `?` and everything following
    // ex: the `?, Y` in `X?, Y`
    (@genparams [$params:ident] ?, $($rest:tt)*) => {
        trigger_fns!(@genparams [$params] $($rest)*)
    };

    // `,` and everything following
    // ex: the `, Y` in `X, Y`
    (@genparams [$params:ident] = $_:literal, $($rest:tt)*) => {
        trigger_fns!(@genparams [$params] $($rest)*)
    };

    // no params left
    (@genparams [$params:ident] ) => {
        ""
    };

    // default values
    // parameter is not optional (no `?`)
    (@defaultval = $default:literal, $($rest:tt)*) => {
        format!(trigger_fns!(@formatarg $($rest)*), stringify!($default))
    };

    // parameter is optional (has `?`)
    (@defaultval ?, $($rest:tt)*) => {
        "".to_string()
    };
}

trigger_fns! {
    [TriggerFunctions]

    Move => move(
        MOVE_X = 0,
        MOVE_Y = 0,

        DURATION?,
        EASING?,
        EASING_RATE?,
    )

    Alpha => alpha(
        OPACITY = 1,
        DURATION?,
    )

    Stop => stop()

    Show => show()
    Hide => hide()

    Shake => shake(
        STRENGTH = 1,
        INTERVAL = 0,
        DURATION?,
    )
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum TriggerType {
    Spawn = 1268,
    Move = 901,
    Rotate = 1346,
    Animate = 1585,
    Pulse = 1006,
    Count = 1611,
    InstantCount = 1811,
    Alpha = 1007,
    Toggle = 1049,
    Follow = 1347,
    Stop = 1616,
    Touch = 1595,
    Shake = 1520,
    Color = 899, // this should be the correct one
    Hide = 1612,
    Show = 1613,
    OnDeath = 1812,
    FollowPlayerY = 1814,
    Collision = 1815,
    Pickup = 1817,
    // TODO: add some more minor triggers
}

#[derive(Clone, Debug)]
pub struct Trigger {
    pub params: Params,
    pub form: TriggerType,
}

impl Trigger {
    // TODO: target different in different triggers (pulse trigger, etc)
    pub fn target_group(&self) -> Option<isize> {
        match self.params.get(&ObjectParams::Id::TARGET) {
            Some(ObjectParams::Params::TARGET(t)) => Some(*t),
            _ => None,
        }
    }

    pub fn to_function(&self) -> String {
        TriggerFunctions::from(self.form).with_params(&self.params)
    }
}
