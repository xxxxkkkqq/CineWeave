import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";

import { createDesktopShellServer } from "../apps/desktop-shell/server.js";

function listen(server) {
  return new Promise((resolve) => {
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      resolve(`http://127.0.0.1:${address.port}`);
    });
  });
}

function close(server) {
  return new Promise((resolve, reject) => {
    server.close((error) => {
      if (error) reject(error);
      else resolve();
    });
  });
}

test("desktop shell server serves UI and proxies adapter requests", async () => {
  const adapter = {
    async send(request) {
      if (request.type === "get_document_state") {
        return {
          ok: true,
          message: "loaded",
          emitted_events: [],
          state: {
            id: "demo",
            name: "Demo",
            aspect_ratio: "16:9",
            tracks: [],
            editor: {
              selection: { clip_ids: [], track_id: null },
              playhead_ms: 0,
              viewport: { scroll_x_px: 0, scroll_y_px: 0, zoom_percent: 100 },
            },
          },
        };
      }

      return {
        ok: true,
        message: "proxied",
        emitted_events: [],
        state: {
          id: "demo",
          name: "Demo",
          aspect_ratio: "16:9",
          tracks: [],
          editor: {
            selection: { clip_ids: [], track_id: null },
            playhead_ms: 0,
            viewport: { scroll_x_px: 0, scroll_y_px: 0, zoom_percent: 100 },
          },
        },
      };
    },
  };

  const server = createDesktopShellServer({ adapter });
  const baseUrl = await listen(server);

  const html = await fetch(`${baseUrl}/`).then((response) => response.text());
  assert.match(html, /Project Control Room/);

  const health = await fetch(`${baseUrl}/api/health`).then((response) => response.json());
  assert.equal(health.ok, true);

  const projects = await fetch(`${baseUrl}/api/projects`).then((response) => response.json());
  assert.equal(projects.ok, true);
  assert.ok(Array.isArray(projects.projects));

  const adapterResponse = await fetch(`${baseUrl}/api/adapter`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      type: "get_document_state",
      snapshot_path: "snap.json",
      event_log_path: "events.json",
    }),
  }).then((response) => response.json());

  assert.equal(adapterResponse.ok, true);
  assert.equal(adapterResponse.state.id, "demo");

  await close(server);
});

test("desktop shell server lists paired project documents", async () => {
  const targetRoot = path.join(process.cwd(), "target");
  await mkdir(targetRoot, { recursive: true });
  const projectsRoot = await mkdtemp(path.join(targetRoot, "desktop-shell-test-"));

  const snapshotPath = path.join(projectsRoot, "review-snapshot.json");
  const eventLogPath = path.join(projectsRoot, "review-event-log.json");

  await writeFile(
    snapshotPath,
    JSON.stringify(
      {
        version: 1,
        state: {
          id: "review-demo",
          name: "Review Demo",
          aspect_ratio: "16:9",
          tracks: [],
          editor: {
            selection: { clip_ids: [], track_id: null },
            playhead_ms: 0,
            viewport: { scroll_x_px: 0, scroll_y_px: 0, zoom_percent: 100 },
          },
        },
      },
      null,
      2,
    ),
  );
  await writeFile(eventLogPath, JSON.stringify({ version: 1, events: [] }, null, 2));
  await writeFile(path.join(projectsRoot, "orphan-snapshot.json"), JSON.stringify({ version: 1 }, null, 2));

  const server = createDesktopShellServer({
    adapter: {
      async send() {
        return { ok: true, emitted_events: [], state: null };
      },
    },
    projectsRoot,
  });
  const baseUrl = await listen(server);

  try {
    const response = await fetch(`${baseUrl}/api/projects`);
    const payload = await response.json();

    assert.equal(response.status, 200);
    assert.equal(payload.ok, true);
    assert.equal(payload.projects.length, 1);
    assert.equal(payload.projects[0].projectId, "review-demo");
    assert.equal(payload.projects[0].name, "Review Demo");
    assert.equal(payload.projects[0].aspectRatio, "16:9");
    assert.match(payload.projects[0].snapshotPath, /target\/desktop-shell-test-/);
    assert.match(payload.projects[0].eventLogPath, /review-event-log\.json$/);
  } finally {
    await close(server);
    await rm(projectsRoot, { recursive: true, force: true });
  }
});
