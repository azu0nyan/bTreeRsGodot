//! The [`AwaitAnimation`] action.

use crate::caps::HasAnimationPlayer;
use btree::{record, BehaviorNode, DebugNode, Status};

/// Waits for the context's `AnimationPlayer` to finish whatever it is playing,
/// then succeeds. Mirrors `BTAwaitAnimation`.
///
/// Unlike [`PlayAnimation`](crate::PlayAnimation) it starts nothing — it assumes
/// an animation is already playing and reports [`Status::Running`] until the
/// player stops, then [`Status::Success`].
#[derive(Default)]
pub struct AwaitAnimation;

impl AwaitAnimation {
    /// Build an `AwaitAnimation`.
    pub fn new() -> Self {
        Self
    }
}

impl<D: HasAnimationPlayer> BehaviorNode<D> for AwaitAnimation {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        let player = data.animation_player();
        let status = if player.is_playing() {
            Status::Running
        } else {
            Status::Success
        };
        record(dbg, status, || "AwaitAnimation".to_string());
        status
    }
}
