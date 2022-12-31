use ahash::{AHashMap, AHashSet};
use slotmap::{new_key_type, SlotMap};

use crate::object::{Float, Int, ObjectParams, ObjectType, TriggerType};

new_key_type! {
    pub struct ObjectKey;
    pub struct ContextKey;
}

#[derive(Default, Debug)]
pub struct InnerContext {
    pub delay: Option<Float>,

    pub group: Int,
    pub parent_contexts: Vec<Int>,

    pub triggers: Vec<ObjectKey>,
}

#[derive(Default, Debug)]
pub struct Context {
    pub contexts: AHashMap<Int, InnerContext>,

    pub current_context: Int,
}

impl Context {
    fn current_context(&mut self) -> &mut InnerContext {
        self.contexts.get_mut(&self.current_context).unwrap()
    }

    pub fn push_trigger(&mut self, trigger: ObjectKey) {
        self.current_context().triggers.push(trigger);
    }

    pub fn add_delay(&mut self, delay: Float) {
        self.current_context().delay = Some(delay);
    }

    pub fn add_parents(&mut self, parents: Vec<Int>) {
        self.current_context().parent_contexts = parents;
    }

    pub fn exists(&mut self, group: Int) -> bool {
        self.contexts.contains_key(&group)
    }

    pub fn push(&mut self, group: Int) {
        if !self.contexts.contains_key(&group) {
            self.contexts.insert(
                group,
                InnerContext {
                    group,
                    triggers: vec![],
                    ..Default::default()
                },
            );

            self.current_context = group;
        }
    }

    pub fn pop(&mut self, group: Int) -> InnerContext {
        self.contexts.remove(&group).unwrap()
    }
}

#[derive(Default, Debug)]
pub struct Tree<'ob> {
    pub objects: SlotMap<ObjectKey, &'ob ObjectType>,

    pub targets: AHashSet<Int>,

    trigs_unk_context: Vec<ObjectKey>,
    possibly_parent_contexts: Vec<Int>,

    pub static_objects: Vec<ObjectKey>,

    pub context: Context,
}

impl<'ob> Tree<'ob> {
    pub fn build<'b>(&mut self, objects: &'b Vec<ObjectType>)
    where
        'b: 'ob, // reference to objects vec outlives the references held in `Tree`
    {
        self.context.push(0); // context 0 always exists

        for object in objects {
            let obj_key = self.objects.insert(object);

            match object {
                ObjectType::Block(..) => self.static_objects.push(obj_key),

                ObjectType::Trigger(trig) => {
                    let target = trig.target_group().expect("target group not an integer");

                    // TODO: other contexts such as collision triggers
                    if let TriggerType::Spawn = trig.form {
                        self.context.push(target);

                        if let Some(ObjectParams::Params::SPAWN_DURATION(d)) =
                            trig.params.get(&ObjectParams::Id::SPAWN_DURATION)
                        {
                            self.context.add_delay(*d);
                        }

                        if let Some(groups) = object.groups() {
                            self.possibly_parent_contexts.extend(groups.iter())
                        }

                        let mut to_remove = vec![];

                        // opened a new context so now we need to see if any of the objects previously with an unknown context
                        // are now in the new known context
                        for (i, unk) in self.trigs_unk_context.iter().enumerate() {
                            let known = self.objects[*unk];

                            let known_target = if let ObjectType::Trigger(t) = known {
                                t.target_group().expect("target group not an integer")
                            } else {
                                panic!()
                            };

                            if let Some(groups) = known.groups() {
                                if groups.contains(&target) {
                                    self.context.push_trigger(*unk);
                                    self.targets.insert(known_target);

                                    to_remove.push(i);
                                }
                            }
                        }

                        // requires a second loop cause the first loop is immutable borrow
                        for r in to_remove {
                            self.trigs_unk_context.remove(r);
                        }

                        // unlikely but potentially context splitting triggers could have groups that arent from other context splitting triggers
                        // like giving a move group to a spawn trigger, so we store all parent context groups and then verify later
                        let mut parents = vec![];

                        for cg in self.context.contexts.keys() {
                            if self.possibly_parent_contexts.contains(cg) {
                                parents.push(*cg);
                            }
                        }

                        self.context.add_parents(parents);
                    } else {
                        let groups = object.groups();

                        if (self.context.current_context == 0 && groups.is_none())
                            || groups.is_some()
                            && groups.unwrap().iter().any(|g| self.context.exists(*g))
                        {
                            self.context.push_trigger(obj_key);

                            // triggers in context 0 are added using $.add so we dont need to worry about a target
                            if self.context.current_context != 0 {
                                self.targets.insert(target);
                            }
                        } else {
                            self.trigs_unk_context.push(obj_key);
                        }
                    }
                }
            };
        }
    }
}
