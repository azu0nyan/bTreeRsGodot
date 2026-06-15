//! The [`CallMethod`] action.

use crate::caps::HasAgent;
use btree::{record, BehaviorNode, DebugNode, Status};
use godot::builtin::{StringName, Variant};

/// Calls a method on the context's agent `Node`. Mirrors `BTCallMethod`.
///
/// Returns [`Status::Failure`] only if the method returns the boolean `false`;
/// any other return value (including no meaningful return) yields
/// [`Status::Success`]. This makes it usable both as a fire-and-forget action
/// and as a boolean check.
pub struct CallMethod {
    method: StringName,
    args: Vec<Variant>,
}

impl CallMethod {
    /// Call `method` with no arguments.
    pub fn new(method: impl Into<StringName>) -> Self {
        Self {
            method: method.into(),
            args: Vec::new(),
        }
    }

    /// Call `method` with the given arguments.
    pub fn with_args(method: impl Into<StringName>, args: Vec<Variant>) -> Self {
        Self {
            method: method.into(),
            args,
        }
    }
}

impl<D: HasAgent> BehaviorNode<D> for CallMethod {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        let mut agent = data.agent();
        let result = agent.call(&self.method, &self.args);
        let status = match result.try_to::<bool>() {
            Ok(false) => Status::Failure,
            _ => Status::Success,
        };
        let name = self.method.clone();
        record(dbg, status, || format!("CallMethod: {name}"));
        status
    }
}
