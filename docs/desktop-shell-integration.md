# Desktop Shell Integration

Use these two JavaScript modules as the frontend-facing bridge:

- `packages/desktop-shell/src`
- `packages/frontend-store/src`

## Layer 1: Adapter Client

`MediaCoreAdapterClient` turns frontend operations into `apply_commands` requests for Rust.

```js
import { MediaCoreAdapterClient } from "../packages/desktop-shell/src/index.js";

const adapter = new MediaCoreAdapterClient({
  cwd: "C:/Users/xkq/Desktop/CineWeave",
});
```

## Layer 2: Project Store

`ProjectStore` treats Rust `state` as the only source of truth.

```js
import { ProjectStore } from "../packages/frontend-store/src/index.js";

const store = new ProjectStore({
  adapter,
  snapshotPath: "target/demo-ui/snapshot.json",
  eventLogPath: "target/demo-ui/event-log.json",
});

await store.createProject({
  projectId: "ui-demo",
  name: "UI Demo",
  aspectRatio: "16:9",
});

await store.dispatch([
  {
    type: "add_track",
    track_id: "v1",
    name: "Video 1",
    kind: "Video",
    index: 0,
  },
]);
```

## UI Rendering Rule

Do not maintain an independent frontend timeline state tree.

Instead:

1. Send commands to Rust
2. Receive `response.state`
3. Replace the current frontend store state with that payload
4. Re-render from that state

That keeps undo/redo, persistence, and multi-command edits coherent.
