# Frontend Adapter

`media-core` now exposes a JSON adapter entry point for desktop shells:

```bash
cargo +stable-x86_64-pc-windows-gnu run -p media-core -- adapter
```

The process reads a JSON request from `stdin` and writes a JSON response to `stdout`.

## Request Types

### `create_project_document`

Creates `snapshot.json` and `event-log.json`.

```json
{
  "type": "create_project_document",
  "project_id": "ui-demo",
  "name": "UI Demo",
  "aspect_ratio": "16:9",
  "snapshot_path": "target/demo-ui/snapshot.json",
  "event_log_path": "target/demo-ui/event-log.json"
}
```

### `get_document_state`

Loads the current state by replaying snapshot + event log.

```json
{
  "type": "get_document_state",
  "snapshot_path": "target/demo-ui/snapshot.json",
  "event_log_path": "target/demo-ui/event-log.json"
}
```

### `apply_commands`

Applies one or more frontend-facing commands and optionally persists.

```json
{
  "type": "apply_commands",
  "snapshot_path": "target/demo-ui/snapshot.json",
  "event_log_path": "target/demo-ui/event-log.json",
  "save": true,
  "commands": [
    {
      "type": "add_track",
      "track_id": "v1",
      "name": "Video 1",
      "kind": "Video",
      "index": 0
    },
    {
      "type": "set_playhead",
      "playhead_ms": 1200
    }
  ]
}
```

## Supported Commands

- `set_selection`
- `clear_selection`
- `set_playhead`
- `set_viewport`
- `add_track`
- `rename_track`
- `remove_track`
- `insert_clip`
- `remove_clip`
- `move_clip`
- `trim_clip`
- `split_clip`
- `ripple_move_clip`
- `close_gap_before_clip`
- `undo`
- `redo`

## Response Shape

```json
{
  "ok": true,
  "message": "applied 2 command(s)",
  "snapshot_path": "target/demo-ui/snapshot.json",
  "event_log_path": "target/demo-ui/event-log.json",
  "emitted_events": [],
  "state": {
    "id": "ui-demo",
    "name": "UI Demo",
    "aspect_ratio": "16:9",
    "tracks": [],
    "editor": {
      "selection": {
        "clip_ids": [],
        "track_id": null
      },
      "playhead_ms": 1200,
      "viewport": {
        "scroll_x_px": 0,
        "scroll_y_px": 0,
        "zoom_percent": 100
      }
    }
  }
}
```

## Desktop Shell Model

Recommended frontend flow:

1. On project open, call `get_document_state`
2. Render `state`
3. On user interaction, send `apply_commands`
4. Replace local UI state with returned `state`
5. Use `emitted_events` for analytics, inspector panels, or debug tracing
