//! Godot-coupled behavior-tree nodes — one type per submodule.
//!
//! Each type implements [`btree::BehaviorNode`] over a context `D` and is
//! bounded on the capability trait it needs (see [`crate::caps`]):
//!
//! * **Animation** (`D: HasAnimationPlayer`): [`PlayAnimation`],
//!   [`AwaitAnimation`], [`PauseAnimation`], [`StopAnimation`].
//! * **Agent** (`D: HasAgent`): [`CallMethod`], [`CheckAgentProperty`],
//!   [`SetAgentProperty`], [`EvaluateExpression`].
//! * **Console** (any `D`): [`ConsolePrint`].
//!
//! They are re-exported at the crate root and from the
//! [`prelude`](crate::prelude).

mod await_animation;
mod call_method;
mod check_agent_property;
mod console_print;
mod evaluate_expression;
mod pause_animation;
mod play_animation;
mod set_agent_property;
mod stop_animation;

pub use await_animation::AwaitAnimation;
pub use call_method::CallMethod;
pub use check_agent_property::CheckAgentProperty;
pub use console_print::ConsolePrint;
pub use evaluate_expression::EvaluateExpression;
pub use pause_animation::PauseAnimation;
pub use play_animation::PlayAnimation;
pub use set_agent_property::SetAgentProperty;
pub use stop_animation::StopAnimation;
