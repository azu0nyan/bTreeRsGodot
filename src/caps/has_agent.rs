//! The [`HasAgent`] capability trait.

use godot::classes::Node;
use godot::obj::Gd;

/// A context that exposes its agent [`Node`] — the target for method-call and
/// property nodes.
///
/// Required by [`CallMethod`](crate::CallMethod),
/// [`CheckAgentProperty`](crate::CheckAgentProperty),
/// [`SetAgentProperty`](crate::SetAgentProperty) and
/// [`EvaluateExpression`](crate::EvaluateExpression). The returned handle is a
/// reference-counted `Gd`, so returning `self.agent.clone()` is cheap.
pub trait HasAgent {
    /// The node these nodes call methods on, read/write properties of, or
    /// evaluate expressions against.
    fn agent(&self) -> Gd<Node>;
}
