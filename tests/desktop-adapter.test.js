import test from "node:test";
import assert from "node:assert/strict";

import {
  createAssetCommand,
  createClearSelectionCommand,
  createCloseGapBeforeClipCommand,
  createEffectCommand,
  createExportPresetCommand,
  createFilterCommand,
  createInsertClipCommand,
  MediaCoreAdapterClient,
  createMoveClipCommand,
  createPlayheadCommand,
  createProjectPresetsCommand,
  createRedoCommand,
  createRemoveAssetCommand,
  createRemoveClipCommand,
  createRemoveEffectCommand,
  createRemoveExportPresetCommand,
  createRemoveFilterCommand,
  createRemoveSubtitleCommand,
  createRemoveSubtitleStyleCommand,
  createRemoveTrackCommand,
  createRenameTrackCommand,
  createRippleMoveClipCommand,
  createSelectionCommand,
  createSplitClipCommand,
  createSubtitleCommand,
  createSubtitleStyleCommand,
  createTrackCommand,
  createTrimClipCommand,
  createUndoCommand,
  createViewportCommand,
} from "../packages/desktop-shell/src/adapter-client.js";

test("MediaCoreAdapterClient builds frontend-facing adapter requests", async () => {
  const seen = [];
  const client = new MediaCoreAdapterClient({
    cwd: "C:\\repo",
    spawnAdapter: async (input) => {
      seen.push(input);
      return JSON.stringify({
        ok: true,
        message: "ok",
        emitted_events: [],
        state: { id: "demo" },
      });
    },
  });

  await client.createProjectDocument({
    projectId: "demo",
    name: "Demo",
    aspectRatio: "16:9",
    paths: {
      snapshotPath: "snap.json",
      eventLogPath: "events.json",
    },
  });

  await client.applyCommands(
    [
      createTrackCommand({ trackId: "v1", name: "Video 1" }),
      createAssetCommand({
        id: "asset-1",
        type: "video",
        label: "Source",
        source: { path: "media/source.mp4" },
      }),
      createSubtitleStyleCommand({
        id: "style-1",
        label: "Clean",
        fontFamily: "Aptos",
        fontSizePx: 42,
      }),
      createFilterCommand({
        id: "filter-1",
        kind: "cinematic_grade",
        label: "Grade",
        target: { type: "clip", id: "clip-1" },
      }),
      createEffectCommand({
        id: "effect-1",
        kind: "glitch",
        label: "Glitch",
        target: { type: "clip", id: "clip-1" },
      }),
      createSubtitleCommand({
        id: "sub-1",
        text: "Hello",
        startMs: 0,
        endMs: 1000,
        styleId: "style-1",
      }),
      createExportPresetCommand({
        id: "preset-1",
        label: "Preset",
        container: "mp4",
        aspectRatio: "16:9",
        video: { frameRate: 30 },
        audio: { sampleRate: 48000, channels: 2 },
      }),
      createProjectPresetsCommand({
        exportProfile: "preset-1",
      }),
      createRemoveAssetCommand("asset-legacy"),
      createSelectionCommand({ clipIds: ["clip-1"], trackId: "v1" }),
      createPlayheadCommand(1200),
      createViewportCommand({ scroll_x_px: 10, scroll_y_px: 20, zoom_percent: 130 }),
    ],
    {
      paths: {
        snapshotPath: "snap.json",
        eventLogPath: "events.json",
      },
    },
  );

  assert.equal(seen.length, 2);
  assert.equal(seen[0].cwd, "C:\\repo");
  assert.equal(JSON.parse(seen[0].stdin).type, "create_project_document");
  const applyRequest = JSON.parse(seen[1].stdin);
  assert.equal(applyRequest.type, "apply_commands");
  assert.equal(applyRequest.commands[0].type, "add_track");
  assert.equal(applyRequest.commands[1].type, "upsert_asset");
  assert.equal(applyRequest.commands[2].type, "upsert_subtitle_style");
  assert.equal(applyRequest.commands[3].type, "upsert_filter");
  assert.equal(applyRequest.commands[4].type, "upsert_effect");
  assert.equal(applyRequest.commands[5].type, "upsert_subtitle");
  assert.equal(applyRequest.commands[6].type, "upsert_export_preset");
  assert.equal(applyRequest.commands[7].type, "set_project_presets");
  assert.equal(applyRequest.commands[8].type, "remove_asset");
  assert.equal(applyRequest.commands[9].type, "set_selection");
  assert.equal(applyRequest.commands[10].playhead_ms, 1200);
  assert.equal(applyRequest.commands[11].viewport.zoom_percent, 130);
});

test("adapter command helpers match Rust document command payloads", () => {
  assert.deepEqual(createClearSelectionCommand(), { type: "clear_selection" });
  assert.deepEqual(createRenameTrackCommand("v1", "Primary"), {
    type: "rename_track",
    track_id: "v1",
    new_name: "Primary",
  });
  assert.deepEqual(createRemoveTrackCommand("v1"), {
    type: "remove_track",
    track_id: "v1",
  });
  assert.deepEqual(createInsertClipCommand("v1", { id: "clip-1" }), {
    type: "insert_clip",
    track_id: "v1",
    clip: { id: "clip-1" },
  });
  assert.deepEqual(createRemoveClipCommand("v1", "clip-1"), {
    type: "remove_clip",
    track_id: "v1",
    clip_id: "clip-1",
  });
  assert.deepEqual(
    createMoveClipCommand({
      clipId: "clip-1",
      toTrackId: "v2",
      newTimelineRange: { start_ms: 1000, end_ms: 3000 },
    }),
    {
      type: "move_clip",
      clip_id: "clip-1",
      to_track_id: "v2",
      new_timeline_range: { start_ms: 1000, end_ms: 3000 },
    },
  );
  assert.deepEqual(
    createTrimClipCommand({
      clipId: "clip-1",
      newSourceRange: { start_ms: 200, end_ms: 2200 },
      newTimelineRange: { start_ms: 1000, end_ms: 3000 },
    }),
    {
      type: "trim_clip",
      clip_id: "clip-1",
      new_source_range: { start_ms: 200, end_ms: 2200 },
      new_timeline_range: { start_ms: 1000, end_ms: 3000 },
    },
  );
  assert.deepEqual(
    createSplitClipCommand({
      trackId: "v1",
      clipId: "clip-1",
      splitAtMs: 1500,
      rightClipId: "clip-2",
    }),
    {
      type: "split_clip",
      track_id: "v1",
      clip_id: "clip-1",
      split_at_ms: 1500,
      right_clip_id: "clip-2",
    },
  );
  assert.deepEqual(createRippleMoveClipCommand("clip-1", 250), {
    type: "ripple_move_clip",
    clip_id: "clip-1",
    delta_ms: 250,
  });
  assert.deepEqual(createCloseGapBeforeClipCommand("clip-1"), {
    type: "close_gap_before_clip",
    clip_id: "clip-1",
  });
  assert.deepEqual(createRemoveSubtitleStyleCommand("style-1"), {
    type: "remove_subtitle_style",
    style_id: "style-1",
  });
  assert.deepEqual(createRemoveSubtitleCommand("sub-1"), {
    type: "remove_subtitle",
    subtitle_id: "sub-1",
  });
  assert.deepEqual(createRemoveFilterCommand("filter-1"), {
    type: "remove_filter",
    filter_id: "filter-1",
  });
  assert.deepEqual(createRemoveEffectCommand("effect-1"), {
    type: "remove_effect",
    effect_id: "effect-1",
  });
  assert.deepEqual(createRemoveExportPresetCommand("preset-1"), {
    type: "remove_export_preset",
    preset_id: "preset-1",
  });
  assert.deepEqual(createUndoCommand(), { type: "undo" });
  assert.deepEqual(createRedoCommand(), { type: "redo" });
});
