//! The [`ConsolePrint`] action.

use btree::{BehaviorNode, DebugNode, Status, record};
use godot::global::godot_print;

/// Prints a message to the Godot output and succeeds. Mirrors `BTConsolePrint`.
///
/// Needs nothing from the context, so it works over any `D`.
pub struct ConsolePrint {
    message: String,
}

impl ConsolePrint {
    /// Print `message` each time the node is ticked.
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl<D> BehaviorNode<D> for ConsolePrint {
    fn tick(&mut self, _data: &mut D, dbg: Option<&mut DebugNode>) -> Status {
        godot_print!("{}", self.message);
        let message = self.message.clone();
        record(dbg, Status::Success, || format!("ConsolePrint: {message}"));
        Status::Success
    }
}
