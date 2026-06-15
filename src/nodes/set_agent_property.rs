//! The [`SetAgentProperty`] action.

use crate::caps::HasAgent;
use btree::{record, BehaviorNode, DebugNode, Status};
use godot::builtin::{StringName, Variant};
use godot::meta::ToGodot;

/// Sets a property on the agent `Node` and succeeds. Mirrors
/// `BTSetAgentProperty`.
pub struct SetAgentProperty {
    property: StringName,
    value: Variant,
}

impl SetAgentProperty {
    /// Set `property` to `value`.
    pub fn new(property: impl Into<StringName>, value: impl ToGodot) -> Self {
        Self {
            property: property.into(),
            value: value.to_variant(),
        }
    }
}

impl<D: HasAgent> BehaviorNode<D> for SetAgentProperty {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        let mut agent = data.agent();
        agent.set(&self.property, &self.value);
        let name = self.property.clone();
        record(dbg, Status::Success, || format!("SetAgentProperty: {name}"));
        Status::Success
    }
}
