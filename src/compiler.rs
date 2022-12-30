use ahash::AHashMap;
use itertools::Itertools;

use std::fmt::Write;

use crate::{
    object::{ObjectParams, ObjectType},
    processing::{ContextKey, Tree},
};

// #[derive(Default)]
// struct TriggerInfo {
//     pub targets: Vec<isize>,
// }

#[derive(Default)]
pub struct Compiler<'c> {
    tree: Tree<'c>,
}

impl<'c> Compiler<'c> {
    pub fn compile(&mut self, tree: Tree<'c>) -> String {
        self.tree = tree;

        let mut out = String::new();

        // TODO: output comments?
        self.compile_static_objects(&mut out);

        self.compile_context_0(&mut out);
        self.compile_other_contexts(&mut out);

        out
    }

    // context 0 is the base context and triggers
    // can have delays based on player speed so any triggers in context 0
    // are added as raw objects
    fn compile_context_0(&mut self, out: &mut String) {
        // TODO:
        // `obj_ids.triggers`?
        // general trigger functions?,
        for trigger in &self.tree.context.pop(0).triggers {
            out.push_str("$.add(obj {\n");

            let object = self.tree.objects[*trigger];

            write!(out, "{}\n}})\n\n", compile_object(object)).unwrap();
        }
    }

    fn compile_other_contexts(&self, out: &mut String) {
        let mut target_vars = AHashMap::new();

        for target in self.tree.targets.iter().sorted() {
            let target_var = format!("target_{0}", target);
            let target_str = format!("{} = {}g\n", &target_var, target);

            target_vars.insert(target, format!("target_{0}", target));
            out.push_str(&target_str);
        }

        // TODO: parent contexts - separate structs?
        // context 0 is skipped as it was popped earlier
        for (_, context) in &self.tree.context.contexts {
            writeln!(out, "\ncontext_{} = !{{", context.group).unwrap();

            if let Some(delay) = context.delay {
                writeln!(out, "    wait({})", delay).unwrap();
            }

            for trig in &context.triggers {
                // it will always be a trigger
                let trigger = self.tree.objects[*trig].get_trigger();

                writeln!(
                    out,
                    "    {}.{}",
                    target_vars.get(&trigger.target_group().unwrap()).unwrap(),
                    trigger.to_function()
                )
                .unwrap();
            }

            write!(out, "{}}}\ncontext_{}!\n", "", context.group).unwrap();
        }
    }

    fn compile_static_objects(&self, out: &mut String) {
        // TODO: use `target_x` vars (if available) inside GROUPS property?
        for object in self.tree.static_objects.iter() {
            out.push_str("$.add(obj {\n");

            let object = self.tree.objects[*object];

            write!(out, "{}\n}})\n\n", compile_object(object)).unwrap();
        }
    }
}

fn compile_object(object: &ObjectType) -> String {
    let params = match object {
        ObjectType::Block(b) => &b.params,
        ObjectType::Trigger(t) => &t.params,
    };

    let definition = params
        .keys()
        .sorted()
        .map(|key| {
            let param = &params[key];

            // TODO: decode base64 string params?
            let param_name = ObjectParams::param_name(*key, object.id());

            format!("    {}: {}", param_name, param.to_string())
        })
        .collect::<Vec<String>>()
        .join("\n");

    definition
}

// fn quote_string(string: &str) -> String {
//     // this makes sense, I swear
//     if string.contains('\'') {
//         format!("\"{}\"", string.replace('\"', "\\\""))
//     } else {
//         format!("\'{}\'", string.replace('\'', "\\\'"))
//     }
// }
