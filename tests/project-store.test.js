import test from "node:test";
import assert from "node:assert/strict";

import { ProjectStore } from "../packages/frontend-store/src/project-store.js";

test("ProjectStore uses adapter responses as the single source of truth", async () => {
  const calls = [];
  const adapter = {
    async createProjectDocument(payload) {
      calls.push({ type: "create", payload });
      return {
        ok: true,
        message: "created",
        state: {
          id: payload.projectId,
          name: payload.name,
          aspect_ratio: payload.aspectRatio,
          tracks: [],
          editor: {
            selection: { clip_ids: [], track_id: null },
            playhead_ms: 0,
            viewport: { scroll_x_px: 0, scroll_y_px: 0, zoom_percent: 100 },
          },
        },
      };
    },
    async getDocumentState(paths) {
      calls.push({ type: "load", paths });
      return {
        ok: true,
        message: "loaded",
        state: {
          id: "demo",
          name: "Demo",
          aspect_ratio: "16:9",
          tracks: [{ id: "v1", name: "Video 1", kind: "Video", clips: [] }],
          editor: {
            selection: { clip_ids: [], track_id: null },
            playhead_ms: 100,
            viewport: { scroll_x_px: 0, scroll_y_px: 0, zoom_percent: 100 },
          },
        },
      };
    },
    async applyCommands(commands, options) {
      calls.push({ type: "dispatch", commands, options });
      return {
        ok: true,
        message: "updated",
        state: {
          id: "demo",
          name: "Demo",
          aspect_ratio: "16:9",
          tracks: [{ id: "v1", name: "Video 1", kind: "Video", clips: [] }],
          editor: {
            selection: { clip_ids: ["clip-1"], track_id: "v1" },
            playhead_ms: 1200,
            viewport: { scroll_x_px: 20, scroll_y_px: 30, zoom_percent: 150 },
          },
        },
      };
    },
  };

  const store = new ProjectStore({
    adapter,
    snapshotPath: "snap.json",
    eventLogPath: "events.json",
  });

  let notifications = 0;
  const unsubscribe = store.subscribe((state) => {
    notifications += 1;
    assert.ok(state);
  });

  await store.createProject({ projectId: "demo", name: "Demo", aspectRatio: "16:9" });
  assert.equal(store.getState().id, "demo");

  await store.load();
  assert.equal(store.getState().editor.playhead_ms, 100);

  await store.selectClips({ clipIds: ["clip-1"], trackId: "v1" });
  assert.equal(store.getState().editor.selection.clip_ids[0], "clip-1");

  await store.setPlayhead(1200);
  assert.equal(store.getState().editor.playhead_ms, 1200);

  await store.setViewport({ scroll_x_px: 20, scroll_y_px: 30, zoom_percent: 150 });
  assert.equal(store.getState().editor.viewport.zoom_percent, 150);

  unsubscribe();
  assert.ok(notifications >= 5);
  assert.equal(calls[0].type, "create");
  assert.equal(calls[1].type, "load");
  assert.equal(calls[2].type, "dispatch");
});
