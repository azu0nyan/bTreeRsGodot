# btree_godot — design notes

## Key design considerations

- Mirrors the engine-coupled LimboAI tasks; pure-logic nodes (sequences,
  decorators, timers, randomness) are *not* duplicated here — they live in
  `btree` and are re-exported through this crate's prelude.
- Plain library, not a `cdylib`: registers no Godot classes, produces no
  `.gdextension`. It only uses `godot` crate types and links into whatever
  crate is the actual GDExtension entry point.
- Depends on `btree` by path (`../bTreeRs`), expecting the two crates to sit
  side by side as sibling submodules of this stack.

## Gotchas

- Builds and type-checks standalone (`cargo build`/`clippy`/`doc`) without a
  running engine, but runtime behavior can only be exercised by loading the
  host extension inside Godot.

## TODO

- Nothing tracked yet — add items here as they come up.
