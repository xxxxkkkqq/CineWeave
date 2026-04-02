import { assert, assertArray, assertNonEmptyString } from "../../core/src/assert.js";

export const TASK_KIND = Object.freeze({
  INGEST: "ingest",
  ANALYZE: "analyze",
  CUT: "cut",
  SUBTITLE: "subtitle",
  FILTER: "filter",
  EFFECT: "effect",
  POLISH: "polish",
  QC: "quality_check",
  RENDER: "render",
});

export const ASSET_TYPE = Object.freeze({
  VIDEO: "video",
  AUDIO: "audio",
  IMAGE: "image",
  TEXT: "text",
});

export const TRACK_KIND = Object.freeze({
  VIDEO: "video",
  AUDIO: "audio",
  SUBTITLE: "subtitle",
  EFFECT: "effect",
});

export const TARGET_TYPE = Object.freeze({
  PROJECT: "project",
  TRACK: "track",
  CLIP: "clip",
});

export const KEYFRAME_EASING = Object.freeze({
  LINEAR: "linear",
  EASE_IN: "ease_in",
  EASE_OUT: "ease_out",
  EASE_IN_OUT: "ease_in_out",
  HOLD: "hold",
});

function assertObject(value, fieldName) {
  assert(value && typeof value === "object" && !Array.isArray(value), `${fieldName} must be an object`, {
    fieldName,
    value,
  });
}

function assertInteger(value, fieldName, { min = 0, required = true } = {}) {
  if (value == null) {
    assert(!required, `${fieldName} is required`, { fieldName, value });
    return null;
  }

  assert(Number.isInteger(value), `${fieldName} must be an integer`, { fieldName, value });
  assert(value >= min, `${fieldName} must be greater than or equal to ${min}`, {
    fieldName,
    value,
    min,
  });
  return value;
}

function assertNumber(value, fieldName, { min = 0, required = true } = {}) {
  if (value == null) {
    assert(!required, `${fieldName} is required`, { fieldName, value });
    return null;
  }

  assert(typeof value === "number" && Number.isFinite(value), `${fieldName} must be a finite number`, {
    fieldName,
    value,
  });
  assert(value >= min, `${fieldName} must be greater than or equal to ${min}`, {
    fieldName,
    value,
    min,
  });
  return value;
}

function assertOptionalString(value, fieldName) {
  if (value == null) {
    return null;
  }
  assertNonEmptyString(value, fieldName);
  return value.trim();
}

function assertEnum(value, fieldName, enumValues) {
  assertNonEmptyString(value, fieldName);
  assert(Object.values(enumValues).includes(value), `${fieldName} is not supported: ${value}`, {
    fieldName,
    value,
    supportedValues: Object.values(enumValues),
  });
  return value;
}

function cloneValue(value, fieldName = "value") {
  if (value === null || ["string", "number", "boolean"].includes(typeof value)) {
    return value;
  }

  if (Array.isArray(value)) {
    return value.map((entry, index) => cloneValue(entry, `${fieldName}[${index}]`));
  }

  assertObject(value, fieldName);
  return Object.fromEntries(
    Object.entries(value).map(([key, entry]) => [key, cloneValue(entry, `${fieldName}.${key}`)]),
  );
}

function deepFreeze(value) {
  if (value && typeof value === "object" && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const child of Object.values(value)) {
      deepFreeze(child);
    }
  }
  return value;
}

function mapEntities(values, fieldName, createEntity) {
  assertArray(values ?? [], fieldName);
  return (values ?? []).map((entry, index) => createEntity(entry, `${fieldName}[${index}]`));
}

function uniqueStrings(values, fieldName) {
  assertArray(values ?? [], fieldName);
  const seen = new Set();
  const normalized = [];

  for (const [index, value] of (values ?? []).entries()) {
    const normalizedValue = assertOptionalString(value, `${fieldName}[${index}]`);
    if (normalizedValue && !seen.has(normalizedValue)) {
      seen.add(normalizedValue);
      normalized.push(normalizedValue);
    }
  }

  return normalized;
}

function ensureUniqueIds(items, fieldName) {
  const seen = new Set();
  for (const item of items) {
    assertNonEmptyString(item.id, `${fieldName}.id`);
    assert(!seen.has(item.id), `duplicate ${fieldName} id: ${item.id}`, {
      fieldName,
      id: item.id,
    });
    seen.add(item.id);
  }
}

function createTargetReference(input, fieldName = "target") {
  if (input == null) {
    return null;
  }

  assertObject(input, fieldName);
  const type = assertEnum(input.type, `${fieldName}.type`, TARGET_TYPE);
  const id = type === TARGET_TYPE.PROJECT ? null : assertOptionalString(input.id, `${fieldName}.id`);
  if (type !== TARGET_TYPE.PROJECT) {
    assert(id, `${fieldName}.id is required for ${type} targets`, { fieldName, type, id });
  }

  return deepFreeze({
    type,
    id,
  });
}

function createWordTiming(input, fieldName = "subtitle.word") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.text, `${fieldName}.text`);
  const range = createTimeRange(
    {
      startMs: input.startMs,
      endMs: input.endMs,
    },
    `${fieldName}.range`,
  );

  return deepFreeze({
    text: input.text.trim(),
    startMs: range.startMs,
    endMs: range.endMs,
  });
}

function normalizeCapabilities(input, fieldName = "project.capabilities") {
  assertObject(input, fieldName);

  return deepFreeze({
    subtitles: !!input.subtitles,
    filters: !!input.filters,
    effects: !!input.effects,
    timelineEditing: !!input.timelineEditing,
    localRender: !!input.localRender,
    rhythmPolish: !!input.rhythmPolish,
    supportedAspectRatios: uniqueStrings(
      input.supportedAspectRatios ?? [],
      `${fieldName}.supportedAspectRatios`,
    ),
    supportedFilters: uniqueStrings(input.supportedFilters ?? [], `${fieldName}.supportedFilters`),
    supportedEffects: uniqueStrings(input.supportedEffects ?? [], `${fieldName}.supportedEffects`),
  });
}

function normalizeProjectPresets(input = {}, fieldName = "project.presets") {
  assertObject(input, fieldName);

  return deepFreeze({
    defaultAspectRatio: assertOptionalString(input.defaultAspectRatio, `${fieldName}.defaultAspectRatio`),
    defaultSubtitleStyleId: assertOptionalString(
      input.defaultSubtitleStyleId,
      `${fieldName}.defaultSubtitleStyleId`,
    ),
    polishedSubtitleStyleId: assertOptionalString(
      input.polishedSubtitleStyleId,
      `${fieldName}.polishedSubtitleStyleId`,
    ),
    exportProfile: assertOptionalString(input.exportProfile, `${fieldName}.exportProfile`),
  });
}

function normalizeProjectMetadata(input = {}, fieldName = "project.metadata") {
  assertObject(input, fieldName);

  return deepFreeze({
    version: assertInteger(input.version ?? 1, `${fieldName}.version`, { min: 1 }),
    createdAt: assertOptionalString(input.createdAt, `${fieldName}.createdAt`),
    updatedAt: assertOptionalString(input.updatedAt, `${fieldName}.updatedAt`),
    tags: uniqueStrings(input.tags ?? [], `${fieldName}.tags`),
  });
}

function validateRegistrySupport(projectLike) {
  const filterIds = new Set(projectLike.filters.map((entry) => entry.id));
  const effectIds = new Set(projectLike.effects.map((entry) => entry.id));
  const exportPresetIds = new Set(projectLike.exportPresets.map((entry) => entry.id));
  const subtitleStyleIds = new Set(projectLike.subtitleStyles.map((entry) => entry.id));

  for (const filterId of projectLike.capabilities.supportedFilters) {
    assert(filterIds.has(filterId), `supported filter is missing from project registry: ${filterId}`, {
      filterId,
    });
  }

  for (const effectId of projectLike.capabilities.supportedEffects) {
    assert(effectIds.has(effectId), `supported effect is missing from project registry: ${effectId}`, {
      effectId,
    });
  }

  if (projectLike.presets.exportProfile) {
    assert(exportPresetIds.has(projectLike.presets.exportProfile), "project preset exportProfile is unknown", {
      exportProfile: projectLike.presets.exportProfile,
      knownExportPresets: [...exportPresetIds],
    });
  }

  if (projectLike.presets.defaultSubtitleStyleId) {
    assert(
      subtitleStyleIds.has(projectLike.presets.defaultSubtitleStyleId),
      "project preset defaultSubtitleStyleId is unknown",
      {
        defaultSubtitleStyleId: projectLike.presets.defaultSubtitleStyleId,
        knownSubtitleStyles: [...subtitleStyleIds],
      },
    );
  }

  if (projectLike.presets.polishedSubtitleStyleId) {
    assert(
      subtitleStyleIds.has(projectLike.presets.polishedSubtitleStyleId),
      "project preset polishedSubtitleStyleId is unknown",
      {
        polishedSubtitleStyleId: projectLike.presets.polishedSubtitleStyleId,
        knownSubtitleStyles: [...subtitleStyleIds],
      },
    );
  }
}

export function createTimeRange(input, fieldName = "timeRange") {
  assertObject(input, fieldName);
  const startMs = assertInteger(input.startMs, `${fieldName}.startMs`, { min: 0 });
  const endMs = assertInteger(input.endMs, `${fieldName}.endMs`, { min: 1 });
  assert(endMs > startMs, `${fieldName}.endMs must be greater than startMs`, {
    fieldName,
    startMs,
    endMs,
  });

  return deepFreeze({ startMs, endMs });
}

export function createResolution(input, fieldName = "resolution") {
  assertObject(input, fieldName);
  return deepFreeze({
    width: assertInteger(input.width, `${fieldName}.width`, { min: 1 }),
    height: assertInteger(input.height, `${fieldName}.height`, { min: 1 }),
  });
}

export function createAsset(input, fieldName = "asset") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  const type = assertEnum(input.type, `${fieldName}.type`, ASSET_TYPE);
  assertNonEmptyString(input.label, `${fieldName}.label`);
  assertObject(input.source, `${fieldName}.source`);
  assertNonEmptyString(input.source.path, `${fieldName}.source.path`);

  const resolution =
    input.media?.resolution || (input.media?.width && input.media?.height)
      ? createResolution(
          input.media?.resolution ?? {
            width: input.media.width,
            height: input.media.height,
          },
          `${fieldName}.media.resolution`,
        )
      : null;

  const media = deepFreeze({
    durationMs: assertInteger(input.media?.durationMs ?? null, `${fieldName}.media.durationMs`, {
      min: 0,
      required: false,
    }),
    resolution,
    frameRate: assertNumber(input.media?.frameRate ?? null, `${fieldName}.media.frameRate`, {
      min: 0.01,
      required: false,
    }),
    audioChannels: assertInteger(
      input.media?.audioChannels ?? null,
      `${fieldName}.media.audioChannels`,
      {
        min: 1,
        required: false,
      },
    ),
    sampleRate: assertInteger(input.media?.sampleRate ?? null, `${fieldName}.media.sampleRate`, {
      min: 1,
      required: false,
    }),
  });

  return deepFreeze({
    id: input.id.trim(),
    type,
    label: input.label.trim(),
    source: deepFreeze({
      path: input.source.path.trim(),
      checksum: assertOptionalString(input.source.checksum, `${fieldName}.source.checksum`),
    }),
    media,
    tags: uniqueStrings(input.tags ?? [], `${fieldName}.tags`),
    metadata: deepFreeze(cloneValue(input.metadata ?? {}, `${fieldName}.metadata`)),
  });
}

export function createKeyframe(input, fieldName = "keyframe") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.property, `${fieldName}.property`);
  const offsetMs = assertInteger(input.offsetMs, `${fieldName}.offsetMs`, { min: 0 });
  assert(input.value !== undefined, `${fieldName}.value is required`, { fieldName, value: input.value });

  return deepFreeze({
    id: input.id.trim(),
    property: input.property.trim(),
    offsetMs,
    value: cloneValue(input.value, `${fieldName}.value`),
    easing: assertEnum(
      input.easing ?? KEYFRAME_EASING.LINEAR,
      `${fieldName}.easing`,
      KEYFRAME_EASING,
    ),
  });
}

export function createFilter(input, fieldName = "filter") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.kind, `${fieldName}.kind`);
  assertNonEmptyString(input.label, `${fieldName}.label`);

  return deepFreeze({
    id: input.id.trim(),
    kind: input.kind.trim(),
    label: input.label.trim(),
    category: assertOptionalString(input.category, `${fieldName}.category`),
    target: createTargetReference(input.target, `${fieldName}.target`),
    enabled: input.enabled !== false,
    parameters: deepFreeze(cloneValue(input.parameters ?? {}, `${fieldName}.parameters`)),
    keyframes: mapEntities(input.keyframes ?? [], `${fieldName}.keyframes`, createKeyframe),
    renderBackend: assertOptionalString(input.renderBackend, `${fieldName}.renderBackend`),
  });
}

export function createEffect(input, fieldName = "effect") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.kind, `${fieldName}.kind`);
  assertNonEmptyString(input.label, `${fieldName}.label`);

  return deepFreeze({
    id: input.id.trim(),
    kind: input.kind.trim(),
    label: input.label.trim(),
    category: assertOptionalString(input.category, `${fieldName}.category`),
    target: createTargetReference(input.target, `${fieldName}.target`),
    enabled: input.enabled !== false,
    parameters: deepFreeze(cloneValue(input.parameters ?? {}, `${fieldName}.parameters`)),
    keyframes: mapEntities(input.keyframes ?? [], `${fieldName}.keyframes`, createKeyframe),
    renderBackend: assertOptionalString(input.renderBackend, `${fieldName}.renderBackend`),
  });
}

export function createSubtitleStyle(input, fieldName = "subtitleStyle") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.label, `${fieldName}.label`);
  assertNonEmptyString(input.fontFamily, `${fieldName}.fontFamily`);

  return deepFreeze({
    id: input.id.trim(),
    label: input.label.trim(),
    fontFamily: input.fontFamily.trim(),
    fontSizePx: assertInteger(input.fontSizePx ?? 48, `${fieldName}.fontSizePx`, { min: 1 }),
    placement: assertOptionalString(input.placement, `${fieldName}.placement`) ?? "bottom",
    fillColor: assertOptionalString(input.fillColor, `${fieldName}.fillColor`) ?? "#FFFFFF",
    strokeColor: assertOptionalString(input.strokeColor, `${fieldName}.strokeColor`) ?? "#000000",
    backgroundColor: assertOptionalString(
      input.backgroundColor,
      `${fieldName}.backgroundColor`,
    ),
    maxLines: assertInteger(input.maxLines ?? 2, `${fieldName}.maxLines`, { min: 1 }),
    animationPreset: assertOptionalString(input.animationPreset, `${fieldName}.animationPreset`),
  });
}

export function createSubtitle(input, fieldName = "subtitle") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.text, `${fieldName}.text`);

  const range = createTimeRange(
    input.range ?? {
      startMs: input.startMs,
      endMs: input.endMs,
    },
    `${fieldName}.range`,
  );

  return deepFreeze({
    id: input.id.trim(),
    text: input.text.trim(),
    startMs: range.startMs,
    endMs: range.endMs,
    styleId: assertOptionalString(input.styleId, `${fieldName}.styleId`),
    language: assertOptionalString(input.language, `${fieldName}.language`) ?? "und",
    speaker: assertOptionalString(input.speaker, `${fieldName}.speaker`),
    assetId: assertOptionalString(input.assetId, `${fieldName}.assetId`),
    trackId: assertOptionalString(input.trackId, `${fieldName}.trackId`),
    clipId: assertOptionalString(input.clipId, `${fieldName}.clipId`),
    words: mapEntities(input.words ?? [], `${fieldName}.words`, createWordTiming),
  });
}

export function createExportPreset(input, fieldName = "exportPreset") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.label, `${fieldName}.label`);
  assertNonEmptyString(input.container, `${fieldName}.container`);
  assertNonEmptyString(input.aspectRatio, `${fieldName}.aspectRatio`);
  assertObject(input.video, `${fieldName}.video`);
  assertObject(input.audio, `${fieldName}.audio`);

  return deepFreeze({
    id: input.id.trim(),
    label: input.label.trim(),
    container: input.container.trim(),
    aspectRatio: input.aspectRatio.trim(),
    video: deepFreeze({
      codec: assertOptionalString(input.video.codec, `${fieldName}.video.codec`),
      resolution:
        input.video.width || input.video.height || input.video.resolution
          ? createResolution(
              input.video.resolution ?? {
                width: input.video.width,
                height: input.video.height,
              },
              `${fieldName}.video.resolution`,
            )
          : null,
      frameRate: assertNumber(input.video.frameRate, `${fieldName}.video.frameRate`, {
        min: 0.01,
      }),
      bitrateKbps: assertInteger(input.video.bitrateKbps ?? null, `${fieldName}.video.bitrateKbps`, {
        min: 1,
        required: false,
      }),
      pixelFormat: assertOptionalString(input.video.pixelFormat, `${fieldName}.video.pixelFormat`),
    }),
    audio: deepFreeze({
      codec: assertOptionalString(input.audio.codec, `${fieldName}.audio.codec`),
      sampleRate: assertInteger(input.audio.sampleRate, `${fieldName}.audio.sampleRate`, {
        min: 1,
      }),
      channels: assertInteger(input.audio.channels, `${fieldName}.audio.channels`, { min: 1 }),
      bitrateKbps: assertInteger(input.audio.bitrateKbps ?? null, `${fieldName}.audio.bitrateKbps`, {
        min: 1,
        required: false,
      }),
    }),
    destination: deepFreeze(cloneValue(input.destination ?? {}, `${fieldName}.destination`)),
  });
}

export function createTimelineClip(input, fieldName = "clip") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.assetId, `${fieldName}.assetId`);
  assertNonEmptyString(input.label, `${fieldName}.label`);

  return deepFreeze({
    id: input.id.trim(),
    assetId: input.assetId.trim(),
    label: input.label.trim(),
    timelineRange: createTimeRange(input.timelineRange, `${fieldName}.timelineRange`),
    sourceRange: createTimeRange(input.sourceRange, `${fieldName}.sourceRange`),
    filterIds: uniqueStrings(input.filterIds ?? [], `${fieldName}.filterIds`),
    effectIds: uniqueStrings(input.effectIds ?? [], `${fieldName}.effectIds`),
    subtitleIds: uniqueStrings(input.subtitleIds ?? [], `${fieldName}.subtitleIds`),
    metadata: deepFreeze(cloneValue(input.metadata ?? {}, `${fieldName}.metadata`)),
  });
}

export function createTimelineTrack(input, fieldName = "track") {
  assertObject(input, fieldName);
  assertNonEmptyString(input.id, `${fieldName}.id`);
  assertNonEmptyString(input.name, `${fieldName}.name`);

  const clips = mapEntities(input.clips ?? [], `${fieldName}.clips`, createTimelineClip);
  ensureUniqueIds(clips, `${fieldName}.clips`);

  return deepFreeze({
    id: input.id.trim(),
    name: input.name.trim(),
    kind: assertEnum(input.kind, `${fieldName}.kind`, TRACK_KIND),
    clipIds: clips.map((clip) => clip.id),
    clips,
    filterIds: uniqueStrings(input.filterIds ?? [], `${fieldName}.filterIds`),
    effectIds: uniqueStrings(input.effectIds ?? [], `${fieldName}.effectIds`),
  });
}

export function createProjectDocument(input) {
  assertObject(input, "project");
  assertNonEmptyString(input.id, "project.id");
  assertNonEmptyString(input.name, "project.name");
  assertNonEmptyString(input.aspectRatio ?? "16:9", "project.aspectRatio");

  const assets = mapEntities(input.assets ?? [], "project.assets", createAsset);
  const subtitleStyles = mapEntities(
    input.subtitleStyles ?? [],
    "project.subtitleStyles",
    createSubtitleStyle,
  );
  const filters = mapEntities(input.filters ?? [], "project.filters", createFilter);
  const effects = mapEntities(input.effects ?? [], "project.effects", createEffect);
  const exportPresets = mapEntities(
    input.exportPresets ?? [],
    "project.exportPresets",
    createExportPreset,
  );
  const tracks = mapEntities(input.tracks ?? [], "project.tracks", createTimelineTrack);
  const subtitles = mapEntities(input.subtitles ?? [], "project.subtitles", createSubtitle);
  const capabilities = normalizeCapabilities(input.capabilities ?? {}, "project.capabilities");
  const presets = normalizeProjectPresets(input.presets ?? {}, "project.presets");
  const metadata = normalizeProjectMetadata(input.metadata ?? {}, "project.metadata");

  ensureUniqueIds(assets, "project.assets");
  ensureUniqueIds(subtitleStyles, "project.subtitleStyles");
  ensureUniqueIds(filters, "project.filters");
  ensureUniqueIds(effects, "project.effects");
  ensureUniqueIds(exportPresets, "project.exportPresets");
  ensureUniqueIds(tracks, "project.tracks");
  ensureUniqueIds(subtitles, "project.subtitles");

  const assetIds = new Set(assets.map((entry) => entry.id));
  const trackIds = new Set(tracks.map((entry) => entry.id));
  const clipMap = new Map();
  for (const track of tracks) {
    for (const clip of track.clips) {
      assert(!clipMap.has(clip.id), `duplicate clip id in project timeline: ${clip.id}`, {
        clipId: clip.id,
      });
      clipMap.set(clip.id, clip);
      assert(assetIds.has(clip.assetId), `clip references unknown asset: ${clip.assetId}`, {
        clipId: clip.id,
        assetId: clip.assetId,
      });
    }
  }

  const subtitleStyleIds = new Set(subtitleStyles.map((entry) => entry.id));
  const filterMap = new Map(filters.map((entry) => [entry.id, entry]));
  const effectMap = new Map(effects.map((entry) => [entry.id, entry]));
  const subtitleMap = new Map(subtitles.map((entry) => [entry.id, entry]));

  for (const track of tracks) {
    for (const filterId of track.filterIds) {
      const filter = filterMap.get(filterId);
      assert(filter, `track references unknown filter: ${filterId}`, { trackId: track.id, filterId });
      assert(
        filter.target?.type === TARGET_TYPE.TRACK && filter.target.id === track.id,
        `track filter target mismatch: ${filterId}`,
        { trackId: track.id, filterId, target: filter?.target },
      );
    }

    for (const effectId of track.effectIds) {
      const effect = effectMap.get(effectId);
      assert(effect, `track references unknown effect: ${effectId}`, { trackId: track.id, effectId });
      assert(
        effect.target?.type === TARGET_TYPE.TRACK && effect.target.id === track.id,
        `track effect target mismatch: ${effectId}`,
        { trackId: track.id, effectId, target: effect?.target },
      );
    }

    for (const clip of track.clips) {
      for (const filterId of clip.filterIds) {
        const filter = filterMap.get(filterId);
        assert(filter, `clip references unknown filter: ${filterId}`, { clipId: clip.id, filterId });
        assert(
          filter.target?.type === TARGET_TYPE.CLIP && filter.target.id === clip.id,
          `clip filter target mismatch: ${filterId}`,
          { clipId: clip.id, filterId, target: filter?.target },
        );
      }

      for (const effectId of clip.effectIds) {
        const effect = effectMap.get(effectId);
        assert(effect, `clip references unknown effect: ${effectId}`, { clipId: clip.id, effectId });
        assert(
          effect.target?.type === TARGET_TYPE.CLIP && effect.target.id === clip.id,
          `clip effect target mismatch: ${effectId}`,
          { clipId: clip.id, effectId, target: effect?.target },
        );
      }

      for (const subtitleId of clip.subtitleIds) {
        const subtitle = subtitleMap.get(subtitleId);
        assert(subtitle, `clip references unknown subtitle: ${subtitleId}`, {
          clipId: clip.id,
          subtitleId,
        });
        assert(
          subtitle.clipId === clip.id,
          `clip subtitle target mismatch: ${subtitleId}`,
          { clipId: clip.id, subtitleId, subtitleClipId: subtitle?.clipId },
        );
      }
    }
  }

  for (const subtitle of subtitles) {
    if (subtitle.styleId) {
      assert(subtitleStyleIds.has(subtitle.styleId), "subtitle references unknown style", {
        subtitleId: subtitle.id,
        styleId: subtitle.styleId,
      });
    }

    if (subtitle.trackId) {
      assert(trackIds.has(subtitle.trackId), "subtitle references unknown track", {
        subtitleId: subtitle.id,
        trackId: subtitle.trackId,
      });
    }

    if (subtitle.clipId) {
      assert(clipMap.has(subtitle.clipId), "subtitle references unknown clip", {
        subtitleId: subtitle.id,
        clipId: subtitle.clipId,
      });
    }

    if (subtitle.assetId) {
      assert(assetIds.has(subtitle.assetId), "subtitle references unknown asset", {
        subtitleId: subtitle.id,
        assetId: subtitle.assetId,
      });
    }
  }

  for (const filter of filters) {
    if (!filter.target) {
      continue;
    }
    if (filter.target.type === TARGET_TYPE.TRACK) {
      assert(trackIds.has(filter.target.id), "filter target track does not exist", {
        filterId: filter.id,
        target: filter.target,
      });
    }
    if (filter.target.type === TARGET_TYPE.CLIP) {
      assert(clipMap.has(filter.target.id), "filter target clip does not exist", {
        filterId: filter.id,
        target: filter.target,
      });
    }
  }

  for (const effect of effects) {
    if (!effect.target) {
      continue;
    }
    if (effect.target.type === TARGET_TYPE.TRACK) {
      assert(trackIds.has(effect.target.id), "effect target track does not exist", {
        effectId: effect.id,
        target: effect.target,
      });
    }
    if (effect.target.type === TARGET_TYPE.CLIP) {
      assert(clipMap.has(effect.target.id), "effect target clip does not exist", {
        effectId: effect.id,
        target: effect.target,
      });
    }
  }

  const project = {
    id: input.id.trim(),
    name: input.name.trim(),
    aspectRatio: (input.aspectRatio ?? "16:9").trim(),
    capabilities,
    assets,
    tracks,
    subtitles,
    subtitleStyles,
    filters,
    effects,
    exportPresets,
    presets,
    metadata,
  };

  validateRegistrySupport(project);
  return deepFreeze(project);
}

export function createProjectDraft(input) {
  assertObject(input, "project");
  assertNonEmptyString(input.id, "project.id");
  assertNonEmptyString(input.name, "project.name");

  const draft = {
    id: input.id.trim(),
    name: input.name.trim(),
    capabilities: normalizeCapabilities(input.capabilities, "project.capabilities"),
    assets: mapEntities(input.assets ?? [], "project.assets", createAsset),
    subtitleStyles: mapEntities(
      input.subtitleStyles ?? [],
      "project.subtitleStyles",
      createSubtitleStyle,
    ),
    filters: mapEntities(input.filters ?? [], "project.filters", createFilter),
    effects: mapEntities(input.effects ?? [], "project.effects", createEffect),
    exportPresets: mapEntities(input.exportPresets ?? [], "project.exportPresets", createExportPreset),
    presets: normalizeProjectPresets(input.presets ?? {}, "project.presets"),
    metadata: normalizeProjectMetadata(input.metadata ?? {}, "project.metadata"),
  };

  ensureUniqueIds(draft.assets, "project.assets");
  ensureUniqueIds(draft.subtitleStyles, "project.subtitleStyles");
  ensureUniqueIds(draft.filters, "project.filters");
  ensureUniqueIds(draft.effects, "project.effects");
  ensureUniqueIds(draft.exportPresets, "project.exportPresets");
  validateRegistrySupport(draft);

  return deepFreeze(draft);
}

export function createEditIntent(input) {
  assert(input && typeof input === "object", "edit intent input is required");
  assertNonEmptyString(input.prompt, "intent.prompt");

  return deepFreeze({
    prompt: input.prompt,
    requestedCuts: !!input.requestedCuts,
    requestedSubtitles: !!input.requestedSubtitles,
    outputAspectRatio: input.outputAspectRatio ?? "16:9",
    precisionMode: input.precisionMode ?? "standard",
    styleProfiles: uniqueStrings(input.styleProfiles ?? [], "intent.styleProfiles"),
    effects: uniqueStrings(input.effects ?? [], "intent.effects"),
    polishNotes: uniqueStrings(input.polishNotes ?? [], "intent.polishNotes"),
  });
}

export function createTask(input) {
  assert(input && typeof input === "object", "task input is required");
  assertNonEmptyString(input.id, "task.id");
  assertNonEmptyString(input.kind, "task.kind");
  assertNonEmptyString(input.title, "task.title");
  assertArray(input.dependsOn ?? [], "task.dependsOn");

  return deepFreeze({
    id: input.id,
    kind: input.kind,
    title: input.title,
    dependsOn: uniqueStrings(input.dependsOn ?? [], "task.dependsOn"),
    payload: deepFreeze(cloneValue(input.payload ?? {}, "task.payload")),
  });
}

export function createTaskGraph(input) {
  assert(input && typeof input === "object", "task graph input is required");
  const intent = createEditIntent(input.intent);
  const tasks = deepFreeze((input.tasks ?? []).map((task, index) => createTask(task, `task[${index}]`)));
  const warnings = deepFreeze(uniqueStrings(input.warnings ?? [], "taskGraph.warnings"));

  return deepFreeze({
    intent,
    tasks,
    warnings,
  });
}
