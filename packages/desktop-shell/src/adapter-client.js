import { spawn } from "node:child_process";

function withDefaultPaths(paths = {}) {
  return {
    snapshotPath: paths.snapshotPath ?? "target/demo-ui/snapshot.json",
    eventLogPath: paths.eventLogPath ?? "target/demo-ui/event-log.json",
  };
}

function defaultSpawnAdapter({ cwd, cargoCommand, stdin }) {
  return new Promise((resolve, reject) => {
    const child = spawn(
      cargoCommand.command,
      cargoCommand.args,
      {
        cwd,
        stdio: ["pipe", "pipe", "pipe"],
        shell: false,
        env: {
          ...process.env,
          PATH: `${process.env.USERPROFILE}\\.cargo\\bin;${process.env.PATH ?? ""}`,
        },
      },
    );

    let stdout = "";
    let stderr = "";

    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");

    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", reject);
    child.on("close", (code) => {
      if (code !== 0) {
        reject(new Error(`adapter process failed with code ${code}: ${stderr || stdout}`));
        return;
      }
      resolve(stdout.trim());
    });

    child.stdin.write(stdin);
    child.stdin.end();
  });
}

export class MediaCoreAdapterClient {
  constructor(options = {}) {
    this.cwd = options.cwd;
    this.spawnAdapter = options.spawnAdapter ?? defaultSpawnAdapter;
    this.cargoCommand = options.cargoCommand ?? {
      command: "cargo",
      args: ["+stable-x86_64-pc-windows-gnu", "run", "-p", "media-core", "--", "adapter"],
    };
  }

  async send(request) {
    const payload = JSON.stringify(request, null, 2);
    const raw = await this.spawnAdapter({
      cwd: this.cwd,
      cargoCommand: this.cargoCommand,
      stdin: payload,
    });
    return JSON.parse(raw);
  }

  async createProjectDocument({ projectId, name, aspectRatio, paths }) {
    const resolved = withDefaultPaths(paths);
    return this.send({
      type: "create_project_document",
      project_id: projectId,
      name,
      aspect_ratio: aspectRatio,
      snapshot_path: resolved.snapshotPath,
      event_log_path: resolved.eventLogPath,
    });
  }

  async getDocumentState(paths) {
    const resolved = withDefaultPaths(paths);
    return this.send({
      type: "get_document_state",
      snapshot_path: resolved.snapshotPath,
      event_log_path: resolved.eventLogPath,
    });
  }

  async applyCommands(commands, options = {}) {
    const resolved = withDefaultPaths(options.paths);
    return this.send({
      type: "apply_commands",
      snapshot_path: resolved.snapshotPath,
      event_log_path: resolved.eventLogPath,
      save: options.save ?? true,
      commands,
    });
  }
}

export function createTrackCommand({ trackId, name, kind = "Video", index = 0 }) {
  return {
    type: "add_track",
    track_id: trackId,
    name,
    kind,
    index,
  };
}

export function createRenameTrackCommand(trackId, newName) {
  return {
    type: "rename_track",
    track_id: trackId,
    new_name: newName,
  };
}

export function createRemoveTrackCommand(trackId) {
  return {
    type: "remove_track",
    track_id: trackId,
  };
}

export function createSelectionCommand({ clipIds = [], trackId = null }) {
  return {
    type: "set_selection",
    clip_ids: clipIds,
    track_id: trackId,
  };
}

export function createClearSelectionCommand() {
  return {
    type: "clear_selection",
  };
}

export function createPlayheadCommand(playheadMs) {
  return {
    type: "set_playhead",
    playhead_ms: playheadMs,
  };
}

export function createViewportCommand(viewport) {
  return {
    type: "set_viewport",
    viewport,
  };
}

export function createAssetCommand(asset) {
  return {
    type: "upsert_asset",
    asset,
  };
}

export function createRemoveAssetCommand(assetId) {
  return {
    type: "remove_asset",
    asset_id: assetId,
  };
}

export function createSubtitleStyleCommand(style) {
  return {
    type: "upsert_subtitle_style",
    style,
  };
}

export function createRemoveSubtitleStyleCommand(styleId) {
  return {
    type: "remove_subtitle_style",
    style_id: styleId,
  };
}

export function createSubtitleCommand(subtitle) {
  return {
    type: "upsert_subtitle",
    subtitle,
  };
}

export function createRemoveSubtitleCommand(subtitleId) {
  return {
    type: "remove_subtitle",
    subtitle_id: subtitleId,
  };
}

export function createFilterCommand(filter) {
  return {
    type: "upsert_filter",
    filter,
  };
}

export function createRemoveFilterCommand(filterId) {
  return {
    type: "remove_filter",
    filter_id: filterId,
  };
}

export function createEffectCommand(effect) {
  return {
    type: "upsert_effect",
    effect,
  };
}

export function createRemoveEffectCommand(effectId) {
  return {
    type: "remove_effect",
    effect_id: effectId,
  };
}

export function createExportPresetCommand(preset) {
  return {
    type: "upsert_export_preset",
    preset,
  };
}

export function createRemoveExportPresetCommand(presetId) {
  return {
    type: "remove_export_preset",
    preset_id: presetId,
  };
}

export function createProjectPresetsCommand(presets) {
  return {
    type: "set_project_presets",
    presets,
  };
}

export function createInsertClipCommand(trackId, clip) {
  return {
    type: "insert_clip",
    track_id: trackId,
    clip,
  };
}

export function createRemoveClipCommand(trackId, clipId) {
  return {
    type: "remove_clip",
    track_id: trackId,
    clip_id: clipId,
  };
}

export function createMoveClipCommand({ clipId, toTrackId, newTimelineRange }) {
  return {
    type: "move_clip",
    clip_id: clipId,
    to_track_id: toTrackId,
    new_timeline_range: newTimelineRange,
  };
}

export function createTrimClipCommand({ clipId, newSourceRange, newTimelineRange }) {
  return {
    type: "trim_clip",
    clip_id: clipId,
    new_source_range: newSourceRange,
    new_timeline_range: newTimelineRange,
  };
}

export function createSplitClipCommand({ trackId, clipId, splitAtMs, rightClipId }) {
  return {
    type: "split_clip",
    track_id: trackId,
    clip_id: clipId,
    split_at_ms: splitAtMs,
    right_clip_id: rightClipId,
  };
}

export function createRippleMoveClipCommand(clipId, deltaMs) {
  return {
    type: "ripple_move_clip",
    clip_id: clipId,
    delta_ms: deltaMs,
  };
}

export function createCloseGapBeforeClipCommand(clipId) {
  return {
    type: "close_gap_before_clip",
    clip_id: clipId,
  };
}

export function createUndoCommand() {
  return {
    type: "undo",
  };
}

export function createRedoCommand() {
  return {
    type: "redo",
  };
}
