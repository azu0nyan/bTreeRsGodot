//! # btree_godot — Godot behavior-tree nodes for [`btree`]
//!
//! This crate is the Godot ([gdext](https://godot-rust.github.io)) adapter for
//! the engine-agnostic [`btree`] library. `btree` itself never
//! references Godot; everything that does — playing an animation, calling a
//! method on a scene node, evaluating a Godot `Expression` — lives here.
//!
//! The node set mirrors the engine-coupled tasks of the
//! [LimboAI](https://github.com/limbonaut/limboai) addon. Pure-logic tasks
//! (sequences, fallbacks, decorators, timers, randomness) are *not* duplicated
//! here — they are in [`btree`] and reachable through the
//! [`prelude`].
//!
//! ## Capability traits
//!
//! Following `btree`'s pattern of bounding a node on what it needs from the
//! context `D`, the Godot nodes are gated on small traits the agent implements:
//!
//! * [`HasAnimationPlayer`] — hands out a `Gd<AnimationPlayer>`; required by the
//!   animation nodes ([`PlayAnimation`], [`AwaitAnimation`], [`PauseAnimation`],
//!   [`StopAnimation`]).
//! * [`HasAgent`] — hands out a `Gd<Node>`; required by the nodes that act on a
//!   scene node ([`CallMethod`], [`CheckAgentProperty`], [`SetAgentProperty`],
//!   [`EvaluateExpression`]).
//!
//! [`ConsolePrint`] needs nothing and works over any `D`.
//!
//! ## Wiring
//!
//! Make your blackboard / agent type implement the capability traits, then
//! build trees from both crates and tick them from `_process`:
//!
//! ```ignore
//! use btree_godot::prelude::*;
//! use godot::prelude::*;
//! use godot::classes::AnimationPlayer;
//!
//! struct Blackboard {
//!     player: Gd<AnimationPlayer>,
//!     agent: Gd<Node>,
//!     delta: f64,
//! }
//!
//! impl HasAnimationPlayer for Blackboard {
//!     fn animation_player(&self) -> Gd<AnimationPlayer> { self.player.clone() }
//! }
//! impl HasAgent for Blackboard {
//!     fn agent(&self) -> Gd<Node> { self.agent.clone() }
//! }
//! impl HasDelta for Blackboard {
//!     fn delta_seconds(&self) -> f64 { self.delta } // HasDelta comes from `btree`
//! }
//!
//! // `Sequence`, `Wait` are from `btree`; `PlayAnimation` is from here.
//! let mut tree = Sequence::new(nodes![
//!     PlayAnimation::new("attack"),
//!     Wait::new(0.5),
//!     ConsolePrint::new("done"),
//! ]);
//! // each frame: bb.delta = delta; tree.tick(&mut bb, None);
//! ```

#![warn(missing_docs)]

pub mod caps;
pub mod nodes;

pub use caps::{HasAgent, HasAnimationPlayer};
pub use nodes::{AwaitAnimation, CallMethod, CheckAgentProperty, ConsolePrint, EvaluateExpression, PauseAnimation, PlayAnimation, SetAgentProperty, StopAnimation};

/// Re-exports of everything you typically need to build a Godot behavior tree:
/// this crate's nodes and capability traits, plus all of [`btree`]'s prelude
/// (the [`BehaviorNode`](btree::BehaviorNode) trait, [`Status`](btree::Status),
/// the `nodes!` macro, and every engine-free node).
pub mod prelude {
    pub use crate::caps::{HasAgent, HasAnimationPlayer};
    pub use crate::nodes::*;
    pub use btree::prelude::*;
}
