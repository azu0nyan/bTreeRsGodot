//! The [`StopAnimation`] action.

use crate::caps::HasAnimationPlayer;
use btree::{BehaviorNode, DebugNode, Status, record};

/// Stops playback on the context's `AnimationPlayer` and succeeds. Mirrors
/// `BTStopAnimation`.
#[derive(Default)]
pub struct StopAnimation;

impl StopAnimation {
    /// Build a `StopAnimation`.
    pub fn new() -> Self {
        Self
    }
}

impl<D: HasAnimationPlayer> BehaviorNode<D> for StopAnimation {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        data.animation_player().stop();
        record(dbg, Status::Success, || "StopAnimation".to_string());
        Status::Success
    }
}
