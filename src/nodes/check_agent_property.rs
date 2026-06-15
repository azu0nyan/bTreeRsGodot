//! The [`CheckAgentProperty`] condition.

use crate::caps::HasAgent;
use btree::{record, BehaviorNode, DebugNode, Status};
use godot::builtin::{StringName, Variant};
use godot::meta::ToGodot;

/// Succeeds if a property of the agent `Node` equals an expected value, else
/// fails. Mirrors `BTCheckAgentProperty`.
pub struct CheckAgentProperty {
    property: StringName,
    expected: Variant,
}

impl CheckAgentProperty {
    /// Check that `property` equals `expected`.
    pub fn new(property: impl Into<StringName>, expected: impl ToGodot) -> Self {
        Self {
            property: property.into(),
            expected: expected.to_variant(),
        }
    }
}

impl<D: HasAgent> BehaviorNode<D> for CheckAgentProperty {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        let agent = data.agent();
        let actual = agent.get(&self.property);
        let status = if actual == self.expected {
            Status::Success
        } else {
            Status::Failure
        };
        let name = self.property.clone();
        record(dbg, status, || format!("CheckAgentProperty: {name}"));
        status
    }
}
