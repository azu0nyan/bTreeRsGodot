//! The [`PauseAnimation`] action.

use crate::caps::HasAnimationPlayer;
use btree::{BehaviorNode, DebugNode, Status, record};

/// Pauses the context's `AnimationPlayer` and succeeds. Mirrors
/// `BTPauseAnimation`.
#[derive(Default)]
pub struct PauseAnimation;

impl PauseAnimation {
    /// Build a `PauseAnimation`.
    pub fn new() -> Self {
        Self
    }
}

impl<D: HasAnimationPlayer> BehaviorNode<D> for PauseAnimation {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        data.animation_player().pause();
        record(dbg, Status::Success, || "PauseAnimation".to_string());
        Status::Success
    }
}
