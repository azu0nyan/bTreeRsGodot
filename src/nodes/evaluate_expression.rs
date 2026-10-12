//! The [`EvaluateExpression`] action.

use crate::caps::HasAgent;
use btree::{BehaviorNode, DebugNode, Status, record};
use godot::classes::Expression;
use godot::global::Error;
use godot::obj::NewGd;

/// Parses and evaluates a Godot `Expression` against the agent `Node`. Mirrors
/// `BTEvaluateExpression`.
///
/// The expression runs with the agent as its base instance, so it can reference
/// the agent's properties and methods. Returns [`Status::Failure`] if parsing or
/// execution fails, or if the result is the boolean `false`; otherwise
/// [`Status::Success`].
pub struct EvaluateExpression {
    expression: String,
}

impl EvaluateExpression {
    /// Evaluate the GDScript `expression` string.
    pub fn new(expression: impl Into<String>) -> Self {
        Self { expression: expression.into() }
    }
}

impl<D: HasAgent> BehaviorNode<D> for EvaluateExpression {
    fn tick(&mut self, data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        let mut expr = Expression::new_gd();
        if expr.parse(self.expression.as_str()) != Error::OK {
            record(dbg, Status::Failure, || "EvaluateExpression (parse error)".to_string());
            return Status::Failure;
        }
        let base = data.agent();
        let value = expr.execute_ex().base_instance(&base).done();
        let status = if expr.has_execute_failed() {
            Status::Failure
        } else {
            match value.try_to::<bool>() {
                Ok(false) => Status::Failure,
                _ => Status::Success,
            }
        };
        record(dbg, status, || "EvaluateExpression".to_string());
        status
    }
}
