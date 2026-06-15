//! Capability traits a Godot context `D` implements to unlock engine nodes.
//!
//! Each trait hands the nodes a `Gd<...>` handle to the engine object they
//! drive. A node bounds its impl on the trait it needs — for example
//! `impl<D: HasAnimationPlayer> BehaviorNode<D> for PlayAnimation` — so it is
//! only usable on contexts that can provide that object. This mirrors the
//! [`btree::caps`] pattern (`HasDelta`, `HasRng`) for the Godot layer.

mod has_agent;
mod has_animation_player;

pub use has_agent::HasAgent;
pub use has_animation_player::HasAnimationPlayer;
