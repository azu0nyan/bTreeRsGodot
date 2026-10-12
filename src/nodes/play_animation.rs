//! The [`PlayAnimation`] action.

use crate::caps::HasAnimationPlayer;
use btree::{BehaviorNode, DebugNode, Status, record};
use godot::builtin::StringName;

/// Plays a named animation on the context's `AnimationPlayer`, reporting
/// [`Status::Running`] until it finishes. Mirrors `BTPlayAnimation`.
///
/// On the first tick the animation is started; while the player is playing the
/// node returns [`Status::Running`]; once playback stops it returns
/// [`Status::Success`] and re-arms, so a later re-entry plays it again.
pub struct PlayAnimation {
    animation: StringName,
    started: bool,
}

impl PlayAnimation {
    /// Play the animation named `animation`.
    pub fn new(animation: impl Into<StringName>) -> Self {
        Self {
            animation: animation.into(),
            started: false,
        }
    }
}

impl<D: HasAnimationPlayer> BehaviorNode<D> for PlayAnimation {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        let mut player = data.animation_player();
        if !self.started {
            player.play_ex().name(&self.animation).done();
            self.started = true;
        }
        let status = if player.is_playing() {
            Status::Running
        } else {
            self.started = false;
            Status::Success
        };
        let name = self.animation.clone();
        record(dbg, status, || format!("PlayAnimation: {name}"));
        status
    }

    fn halt(&mut self) {
        self.started = false;
    }
}
