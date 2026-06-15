# btree_godot — Godot behavior-tree nodes for `btree`

`btree_godot` is the [Godot](https://godotengine.org) ([gdext](https://godot-rust.github.io))
adapter for the engine-agnostic [`btree`](https://github.com/azu0nyan/bTreeRs)
behavior-tree library. `btree` itself has no engine dependency; everything that
touches Godot — playing an animation, calling a method on a scene node,
evaluating a Godot `Expression` — lives here.

The node set mirrors the **engine-coupled** tasks of the
[LimboAI](https://github.com/limbonaut/limboai) addon. The pure-logic tasks
(sequences, fallbacks, decorators, timers, randomness) are *not* duplicated
here — they are in `btree` and re-exported through this crate's
[`prelude`](src/lib.rs), so you build a whole tree from a single import.

## Capability traits

Like `btree` (which uses `HasDelta` / `HasRng`), each Godot node is generic over
the context type `D` and bounded on a small trait the context implements,
handing the node the engine object it drives:

| Trait | Provides | Nodes |
|-------|----------|-------|
| `HasAnimationPlayer` | `Gd<AnimationPlayer>` | `PlayAnimation`, `AwaitAnimation`, `PauseAnimation`, `StopAnimation` |
| `HasAgent` | `Gd<Node>` | `CallMethod`, `CheckAgentProperty`, `SetAgentProperty`, `EvaluateExpression` |
| *(none)* | — | `ConsolePrint` |

## Node catalogue

| Node | Mirrors | Behavior |
|------|---------|----------|
| `PlayAnimation` | `BTPlayAnimation` | play a named animation; `Running` until it finishes |
| `AwaitAnimation` | `BTAwaitAnimation` | wait for the current animation to finish |
| `PauseAnimation` | `BTPauseAnimation` | pause the player; succeed |
| `StopAnimation` | `BTStopAnimation` | stop the player; succeed |
| `CallMethod` | `BTCallMethod` | call a method on the agent; `false` ⇒ failure, else success |
| `CheckAgentProperty` | `BTCheckAgentProperty` | succeed iff an agent property equals an expected value |
| `SetAgentProperty` | `BTSetAgentProperty` | set an agent property; succeed |
| `EvaluateExpression` | `BTEvaluateExpression` | parse & run a Godot `Expression` against the agent |
| `ConsolePrint` | `BTConsolePrint` | print a message; succeed |

Each node lives in its own file under [`src/nodes`](src/nodes); the capability
traits are in [`src/caps`](src/caps). Both are re-exported at the crate root and
from the `prelude`.

## Usage

Make your blackboard / agent type implement the capability traits, then mix
nodes from both crates:

```rust
use btree_godot::prelude::*; // brings btree's prelude too
use godot::prelude::*;
use godot::classes::AnimationPlayer;

struct Blackboard {
    player: Gd<AnimationPlayer>,
    agent: Gd<Node>,
    delta: f64,
}

impl HasAnimationPlayer for Blackboard {
    fn animation_player(&self) -> Gd<AnimationPlayer> { self.player.clone() }
}
impl HasAgent for Blackboard {
    fn agent(&self) -> Gd<Node> { self.agent.clone() }
}
impl HasDelta for Blackboard {           // HasDelta is from `btree`
    fn delta_seconds(&self) -> f64 { self.delta }
}

// `Sequence` and `Wait` come from `btree`; `PlayAnimation` from here.
let mut tree = Sequence::new(nodes![
    PlayAnimation::new("attack"),
    Wait::new(0.5),
    ConsolePrint::new("done"),
]);

// each frame, from _process(delta):
//   bb.delta = delta;
//   tree.tick(&mut bb, None);
```

This crate is a plain library, **not** a `cdylib`: it registers no Godot classes
and produces no `.gdextension`. It only uses the `godot` crate's types, so it
links into whichever crate is your actual GDExtension entry point. It builds and
type-checks with `cargo build` / `cargo clippy` / `cargo doc` without a running
engine; runtime behavior is exercised by loading the host extension in Godot.

## Building it standalone

The crate depends on `btree` by path (`../bTreeRs`), expecting the two
repositories to sit side by side (as sibling git submodules). Check out
[`bTreeRs`](https://github.com/azu0nyan/bTreeRs) next to this crate, or point the
dependency at the git URL.

## License

Licensed under either of MIT or Apache-2.0 at your option.
