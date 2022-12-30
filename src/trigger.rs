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
                                "{}( {})",
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
                format!("{}, ", val.to_string())
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

    // `?` and everything following
    // ex: the `?, Y` in `X?, Y`
    (@genparams [$params:ident] ? $($rest:tt)*) => {
        trigger_fns!(@genparams [$params] $($rest)*)
    };

    // `,` and everything following
    // ex: the `, Y` in `X, Y`
    (@genparams [$params:ident] , $($rest:tt)*) => {
        trigger_fns!(@genparams [$params] $($rest)*)
    };

    // no params left
    (@genparams [$params:ident] ) => {
        ""
    };

    // default values
    // parameter is not optional (no `?`)
    (@defaultval , $($rest:tt)*) => {
        panic!("trigger missing param")
    };

    // parameter is optional (has `?`)
    (@defaultval ?, $($rest:tt)*) => {
        "".to_string()
    };
}

trigger_fns! {
    [TriggerFunctions]

    Move => move(
        MOVE_X,
        MOVE_Y,
        DURATION?,
        EASING?,
        EASING_RATE?,
    )
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum TriggerType {
    Spawn = 1268,
    Move = 901,
    // TODO: rest of triggers
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
