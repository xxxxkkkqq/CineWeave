import { ProjectStore } from "/packages/frontend-store/src/index.js";

import { BrowserMediaCoreAdapter } from "./browser-adapter.js";

const RECENT_PROJECTS_KEY = "cineweave.desktop-shell.recent-projects";
const MAX_RECENT_PROJECTS = 6;

const adapter = new BrowserMediaCoreAdapter();
const store = new ProjectStore({
  adapter,
  snapshotPath: "target/demo-ui/snapshot.json",
  eventLogPath: "target/demo-ui/event-log.json",
});

const appState = {
  availableProjects: [],
  activeProjectKey: null,
  recentProjects: loadRecentProjects(),
};

const elements = {
  createForm: document.querySelector("#create-project-form"),
  loadButton: document.querySelector("#load-project"),
  saveToggle: document.querySelector("#save-toggle"),
  refreshProjects: document.querySelector("#refresh-projects"),
  projectId: document.querySelector("#project-id"),
  projectName: document.querySelector("#project-name"),
  aspectRatio: document.querySelector("#aspect-ratio"),
  snapshotPath: document.querySelector("#snapshot-path"),
  eventLogPath: document.querySelector("#event-log-path"),
  projectFiles: document.querySelector("#project-files"),
  recentProjects: document.querySelector("#recent-projects"),
  selectionForm: document.querySelector("#selection-form"),
  selectionClipIds: document.querySelector("#selection-clip-ids"),
  selectionTrackId: document.querySelector("#selection-track-id"),
  clearSelection: document.querySelector("#clear-selection"),
  playheadForm: document.querySelector("#playhead-form"),
  playheadMs: document.querySelector("#playhead-ms"),
  viewportForm: document.querySelector("#viewport-form"),
  viewportScrollX: document.querySelector("#viewport-scroll-x"),
  viewportScrollY: document.querySelector("#viewport-scroll-y"),
  viewportZoom: document.querySelector("#viewport-zoom"),
  applyConsole: document.querySelector("#apply-console"),
  commandTextarea: document.querySelector("#command-textarea"),
  undoButton: document.querySelector("#undo-button"),
  redoButton: document.querySelector("#redo-button"),
  stateJson: document.querySelector("#state-json"),
  responseJson: document.querySelector("#response-json"),
  timelineViewer: document.querySelector("#timeline-viewer"),
  projectSummary: document.querySelector("#project-summary"),
  statusLine: document.querySelector("#status-line"),
};

function escapeHtml(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function basename(filePath) {
  return String(filePath ?? "").split(/[\\/]/).pop() ?? "";
}

function projectKey(project) {
  return `${project.snapshotPath}::${project.eventLogPath}`;
}

function setPathsFromInputs() {
  store.snapshotPath = elements.snapshotPath.value.trim();
  store.eventLogPath = elements.eventLogPath.value.trim();
  appState.activeProjectKey = projectKey(store.paths());
}

function setInputsFromPaths(project) {
  elements.snapshotPath.value = project.snapshotPath;
  elements.eventLogPath.value = project.eventLogPath;
}

function parseClipIds(value) {
  return value
    .split(",")
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function readStoredRecentProjects() {
  try {
    return JSON.parse(window.localStorage.getItem(RECENT_PROJECTS_KEY) ?? "[]");
  } catch {
    return [];
  }
}

function loadRecentProjects() {
  return readStoredRecentProjects().filter(
    (entry) => entry && typeof entry.snapshotPath === "string" && typeof entry.eventLogPath === "string",
  );
}

function saveRecentProjects() {
  window.localStorage.setItem(RECENT_PROJECTS_KEY, JSON.stringify(appState.recentProjects));
}

function pushRecentProject(project, state = null) {
  const item = {
    snapshotPath: project.snapshotPath,
    eventLogPath: project.eventLogPath,
    label: state?.name ?? project.name ?? project.label ?? basename(project.snapshotPath),
    projectId: state?.id ?? project.projectId ?? null,
    name: state?.name ?? project.name ?? null,
    aspectRatio: state?.aspect_ratio ?? project.aspectRatio ?? null,
    openedAt: new Date().toISOString(),
  };

  appState.recentProjects = [
    item,
    ...appState.recentProjects.filter((entry) => projectKey(entry) !== projectKey(item)),
  ].slice(0, MAX_RECENT_PROJECTS);

  saveRecentProjects();
  renderProjectPanels();
}

function formatDate(dateValue) {
  if (!dateValue) {
    return "Unknown time";
  }

  const date = new Date(dateValue);
  if (Number.isNaN(date.getTime())) {
    return "Unknown time";
  }

  return date.toLocaleString([], {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function formatDuration(ms) {
  return `${(Number(ms) / 1000).toFixed(1)}s`;
}

function buildProjectMarkup(project, meta = {}) {
  const isActive = projectKey(project) === appState.activeProjectKey;
  const title = project.name ?? project.label ?? basename(project.snapshotPath);
  const metaLine = [project.projectId, project.aspectRatio].filter(Boolean).join(" · ");

  return `
    <button
      class="project-entry${isActive ? " is-active" : ""}"
      type="button"
      data-action="open-project"
      data-snapshot-path="${escapeHtml(project.snapshotPath)}"
      data-event-log-path="${escapeHtml(project.eventLogPath)}"
      data-project-name="${escapeHtml(project.name ?? project.label ?? "")}"
      data-project-id="${escapeHtml(project.projectId ?? "")}"
      data-aspect-ratio="${escapeHtml(project.aspectRatio ?? "")}"
    >
      <span class="project-entry-title">${escapeHtml(title)}</span>
      <span class="project-entry-subtitle">${escapeHtml(metaLine || basename(project.eventLogPath))}</span>
      <span class="project-entry-path">${escapeHtml(project.snapshotPath)}</span>
      <span class="project-entry-meta">${escapeHtml(meta.timestampLabel)} ${escapeHtml(meta.timestampValue)}</span>
    </button>
  `;
}

function renderProjectPanels() {
  elements.projectFiles.innerHTML = appState.availableProjects.length
    ? appState.availableProjects
        .map((project) =>
          buildProjectMarkup(project, {
            timestampLabel: "Updated",
            timestampValue: formatDate(project.updatedAt),
          }),
        )
        .join("")
    : `<div class="empty-state compact-empty">No snapshot/event-log pairs found yet.</div>`;

  elements.recentProjects.innerHTML = appState.recentProjects.length
    ? appState.recentProjects
        .map((project) =>
          buildProjectMarkup(project, {
            timestampLabel: "Opened",
            timestampValue: formatDate(project.openedAt),
          }),
        )
        .join("")
    : `<div class="empty-state compact-empty">Recent project documents will appear here.</div>`;
}

function calculateTimelineMetrics(state) {
  const clipEndMs = state.tracks.reduce(
    (trackMax, track) =>
      Math.max(
        trackMax,
        ...track.clips.map((clip) => Number(clip.timeline_range?.end_ms ?? clip.timeline_range?.start_ms ?? 0)),
      ),
    0,
  );
  const totalMs = Math.max(4000, clipEndMs, Number(state.editor.playhead_ms ?? 0) + 1000);
  const playheadPercent = Math.max(
    0,
    Math.min(100, (Number(state.editor.playhead_ms ?? 0) / totalMs) * 100),
  );

  return {
    totalMs,
    playheadPercent,
  };
}

function renderTimelineClip(trackId, clip, metrics, selection) {
  const startMs = Number(clip.timeline_range?.start_ms ?? 0);
  const endMs = Number(clip.timeline_range?.end_ms ?? startMs);
  const rawWidth = ((Math.max(endMs - startMs, 1) / metrics.totalMs) * 100) || 1;
  const left = Math.max(0, Math.min(100, (startMs / metrics.totalMs) * 100));
  const width = Math.max(1, Math.min(100 - left, rawWidth < 8 ? Math.min(8, 100 - left) : rawWidth));
  const isSelected = selection.clip_ids.includes(clip.id);

  return `
    <button
      class="timeline-clip${isSelected ? " is-selected" : ""}"
      type="button"
      style="left: ${left}%; width: ${width}%;"
      data-action="select-clip"
      data-track-id="${escapeHtml(trackId)}"
      data-clip-id="${escapeHtml(clip.id)}"
      data-start-ms="${startMs}"
    >
      <span class="timeline-clip-label">${escapeHtml(clip.label)}</span>
      <span class="timeline-clip-range">${formatDuration(startMs)} - ${formatDuration(endMs)}</span>
    </button>
  `;
}

function renderTrack(track, metrics, selection) {
  const trackSelected = selection.track_id === track.id;
  const clips = track.clips.length
    ? track.clips.map((clip) => renderTimelineClip(track.id, clip, metrics, selection)).join("")
    : `<div class="timeline-lane-empty">No clips on this track yet. Click the lane to select it.</div>`;

  return `
    <section class="timeline-track-card${trackSelected ? " is-selected" : ""}">
      <header class="timeline-track-head">
        <div>
          <div class="track-name">${escapeHtml(track.name)}</div>
          <div class="track-kind">${escapeHtml(track.kind)}</div>
        </div>
        <div class="timeline-track-id">${escapeHtml(track.id)}</div>
      </header>
      <div class="timeline-lane" data-action="select-track" data-track-id="${escapeHtml(track.id)}">
        <div class="timeline-playhead" style="left: ${metrics.playheadPercent}%;"></div>
        ${clips}
      </div>
    </section>
  `;
}

function renderTimeline(state) {
  const metrics = calculateTimelineMetrics(state);
  const markers = Array.from({ length: 6 }, (_, index) => {
    const ms = Math.round((metrics.totalMs / 5) * index);
    const position = (index / 5) * 100;

    return `
      <button
        class="timeline-marker"
        type="button"
        style="left: ${position}%;"
        data-action="set-playhead"
        data-playhead-ms="${ms}"
      >
        <span>${formatDuration(ms)}</span>
      </button>
    `;
  }).join("");

  return `
    <div class="timeline-board">
      <div class="timeline-ruler">${markers}</div>
      <div class="timeline-track-list">${state.tracks.map((track) => renderTrack(track, metrics, state.editor.selection)).join("")}</div>
    </div>
  `;
}

function syncInputsFromState(state) {
  if (!state) {
    return;
  }

  elements.projectId.value = state.id ?? elements.projectId.value;
  elements.projectName.value = state.name ?? elements.projectName.value;
  elements.aspectRatio.value = state.aspect_ratio ?? elements.aspectRatio.value;
  elements.selectionClipIds.value = state.editor.selection.clip_ids.join(", ");
  elements.selectionTrackId.value = state.editor.selection.track_id ?? "";
  elements.playheadMs.value = String(state.editor.playhead_ms ?? 0);
  elements.viewportScrollX.value = String(state.editor.viewport.scroll_x_px ?? 0);
  elements.viewportScrollY.value = String(state.editor.viewport.scroll_y_px ?? 0);
  elements.viewportZoom.value = String(state.editor.viewport.zoom_percent ?? 100);
}

function renderState(state, response) {
  elements.stateJson.textContent = state ? JSON.stringify(state, null, 2) : "No state loaded";
  elements.responseJson.textContent = response ? JSON.stringify(response, null, 2) : "No response yet";

  if (!state) {
    elements.projectSummary.innerHTML = `<div class="empty-state">Create or open a project document to begin.</div>`;
    elements.timelineViewer.innerHTML = `<div class="empty-state">Timeline will appear here.</div>`;
    return;
  }

  syncInputsFromState(state);

  elements.projectSummary.innerHTML = `
    <div class="summary-grid">
      <div><span>Project</span><strong>${escapeHtml(state.name)}</strong></div>
      <div><span>Aspect</span><strong>${escapeHtml(state.aspect_ratio)}</strong></div>
      <div><span>Tracks</span><strong>${state.tracks.length}</strong></div>
      <div><span>Playhead</span><strong>${state.editor.playhead_ms} ms</strong></div>
      <div><span>Selection</span><strong>${escapeHtml(state.editor.selection.clip_ids.join(", ") || state.editor.selection.track_id || "None")}</strong></div>
      <div><span>Viewport</span><strong>${state.editor.viewport.zoom_percent}%</strong></div>
    </div>
  `;

  elements.timelineViewer.innerHTML = state.tracks.length
    ? renderTimeline(state)
    : `<div class="empty-state">No tracks yet. Use the command console to add one.</div>`;
}

function setStatus(message, tone = "neutral") {
  elements.statusLine.dataset.tone = tone;
  elements.statusLine.textContent = message;
}

async function refreshProjectLibrary() {
  const response = await fetch("/api/projects");
  const payload = await response.json();

  if (!response.ok || payload.ok === false) {
    throw new Error(payload.message ?? `project lookup failed with status ${response.status}`);
  }

  appState.availableProjects = payload.projects ?? [];
  renderProjectPanels();
  return appState.availableProjects;
}

function projectFromAction(target) {
  return {
    snapshotPath: target.dataset.snapshotPath,
    eventLogPath: target.dataset.eventLogPath,
    name: target.dataset.projectName || null,
    projectId: target.dataset.projectId || null,
    aspectRatio: target.dataset.aspectRatio || null,
    label: target.dataset.projectName || basename(target.dataset.snapshotPath),
  };
}

function currentProjectReference(state = null) {
  const fallbackName = elements.projectName.value.trim();
  const fallbackProjectId = elements.projectId.value.trim();
  const fallbackAspectRatio = elements.aspectRatio.value.trim();
  const fallbackLabel = fallbackName || basename(elements.snapshotPath.value.trim());

  return {
    snapshotPath: elements.snapshotPath.value.trim(),
    eventLogPath: elements.eventLogPath.value.trim(),
    name: state?.name ?? (fallbackName || null),
    projectId: state?.id ?? (fallbackProjectId || null),
    aspectRatio: state?.aspect_ratio ?? (fallbackAspectRatio || null),
    label: state?.name ?? fallbackLabel,
  };
}

async function openProject(project, successMessage = "Project document loaded") {
  setInputsFromPaths(project);
  setPathsFromInputs();
  renderProjectPanels();
  await store.load();
  pushRecentProject(project, store.getState());
  setStatus(successMessage, "success");
}

async function dispatchEditorCommands(commands, successMessage) {
  await store.dispatch(commands, {
    save: elements.saveToggle.checked,
  });
  setStatus(successMessage, "success");
}

store.subscribe((state, response) => {
  renderState(state, response);
});

elements.createForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    setPathsFromInputs();
    await store.createProject({
      projectId: elements.projectId.value.trim(),
      name: elements.projectName.value.trim(),
      aspectRatio: elements.aspectRatio.value.trim(),
    });
    pushRecentProject(currentProjectReference(store.getState()), store.getState());
    await refreshProjectLibrary();
    setStatus("Project document created", "success");
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.loadButton.addEventListener("click", async () => {
  try {
    await openProject(currentProjectReference());
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.refreshProjects.addEventListener("click", async () => {
  try {
    await refreshProjectLibrary();
    setStatus("Project file list refreshed", "success");
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.projectFiles.addEventListener("click", async (event) => {
  const target = event.target.closest("[data-action='open-project']");
  if (!target) {
    return;
  }

  try {
    await openProject(projectFromAction(target));
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.recentProjects.addEventListener("click", async (event) => {
  const target = event.target.closest("[data-action='open-project']");
  if (!target) {
    return;
  }

  try {
    await openProject(projectFromAction(target), "Recent project opened");
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.selectionForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    await dispatchEditorCommands(
      [
        {
          type: "set_selection",
          clip_ids: parseClipIds(elements.selectionClipIds.value),
          track_id: elements.selectionTrackId.value.trim() || null,
        },
      ],
      "Selection updated",
    );
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.clearSelection.addEventListener("click", async () => {
  try {
    await dispatchEditorCommands([{ type: "clear_selection" }], "Selection cleared");
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.playheadForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    await dispatchEditorCommands(
      [{ type: "set_playhead", playhead_ms: Number(elements.playheadMs.value) }],
      "Playhead updated",
    );
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.viewportForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    await dispatchEditorCommands(
      [
        {
          type: "set_viewport",
          viewport: {
            scroll_x_px: Number(elements.viewportScrollX.value),
            scroll_y_px: Number(elements.viewportScrollY.value),
            zoom_percent: Number(elements.viewportZoom.value),
          },
        },
      ],
      "Viewport updated",
    );
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.applyConsole.addEventListener("submit", async (event) => {
  event.preventDefault();
  try {
    const commands = JSON.parse(elements.commandTextarea.value);
    await dispatchEditorCommands(commands, `Applied ${commands.length} command(s)`);
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.undoButton.addEventListener("click", async () => {
  try {
    await dispatchEditorCommands([{ type: "undo" }], "Undo applied");
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.redoButton.addEventListener("click", async () => {
  try {
    await dispatchEditorCommands([{ type: "redo" }], "Redo applied");
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.timelineViewer.addEventListener("click", async (event) => {
  const target = event.target.closest("[data-action]");
  if (!target || !elements.timelineViewer.contains(target)) {
    return;
  }

  try {
    if (target.dataset.action === "select-clip") {
      await dispatchEditorCommands(
        [
          {
            type: "set_selection",
            clip_ids: [target.dataset.clipId],
            track_id: target.dataset.trackId,
          },
        ],
        `Selected clip ${target.dataset.clipId}`,
      );
      return;
    }

    if (target.dataset.action === "select-track") {
      await dispatchEditorCommands(
        [
          {
            type: "set_selection",
            clip_ids: [],
            track_id: target.dataset.trackId,
          },
        ],
        `Selected track ${target.dataset.trackId}`,
      );
      return;
    }

    if (target.dataset.action === "set-playhead") {
      await dispatchEditorCommands(
        [
          {
            type: "set_playhead",
            playhead_ms: Number(target.dataset.playheadMs),
          },
        ],
        `Playhead moved to ${target.dataset.playheadMs} ms`,
      );
    }
  } catch (error) {
    setStatus(error.message, "error");
  }
});

elements.commandTextarea.value = JSON.stringify(
  [
    {
      type: "add_track",
      track_id: "v1",
      name: "Primary Video",
      kind: "Video",
      index: 0,
    },
    {
      type: "set_playhead",
      playhead_ms: 1200,
    },
  ],
  null,
  2,
);

renderProjectPanels();
renderState(null, null);

try {
  const projects = await refreshProjectLibrary();
  const preferredProject =
    appState.recentProjects
      .map((entry) => projects.find((project) => projectKey(project) === projectKey(entry)))
      .find(Boolean) ??
    projects[0];

  if (preferredProject) {
    await openProject(preferredProject);
  } else {
    setStatus("No project documents found. Create one to begin.", "neutral");
  }
} catch (error) {
  setStatus(error.message, "error");
}
