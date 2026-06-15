//! The [`HasAnimationPlayer`] capability trait.

use godot::classes::AnimationPlayer;
use godot::obj::Gd;

/// A context that can hand out the [`AnimationPlayer`] its animation nodes drive.
///
/// Required by [`PlayAnimation`](crate::PlayAnimation),
/// [`AwaitAnimation`](crate::AwaitAnimation),
/// [`PauseAnimation`](crate::PauseAnimation) and
/// [`StopAnimation`](crate::StopAnimation). The returned handle is a
/// reference-counted `Gd`, so returning `self.player.clone()` is cheap.
pub trait HasAnimationPlayer {
    /// The animation player to play / pause / stop animations on.
    fn animation_player(&self) -> Gd<AnimationPlayer>;
}
