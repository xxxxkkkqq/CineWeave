import test from "node:test";
import assert from "node:assert/strict";

import { createProjectDocument } from "../packages/domain/src/schema.js";
import {
  createDefaultProject,
  createDefaultProjectDocument,
} from "../packages/project/src/project-factory.js";

test("createProjectDocument builds a canonical project model with cross-linked assets and timeline entities", () => {
  const project = createProjectDocument({
    id: "project-structured-001",
    name: "Structured Project",
    aspectRatio: "16:9",
    capabilities: createDefaultProject().capabilities,
    assets: [
      {
        id: "asset-1",
        type: "video",
        label: "Interview A Cam",
        source: { path: "media/interview-a.mp4" },
        media: {
          durationMs: 24000,
          width: 3840,
          height: 2160,
          frameRate: 25,
          audioChannels: 2,
          sampleRate: 48000,
        },
      },
    ],
    subtitleStyles: [
      {
        id: "clean_documentary",
        label: "Clean Documentary",
        fontFamily: "Aptos",
        fontSizePx: 44,
      },
      {
        id: "expressive_social",
        label: "Expressive Social",
        fontFamily: "Trebuchet MS",
        fontSizePx: 52,
        animationPreset: "subtitle_pop",
      },
    ],
    filters: [
      {
        id: "clip-grade-1",
        kind: "cinematic_grade",
        label: "Cinematic Grade on Intro",
        target: { type: "clip", id: "clip-1" },
        parameters: { intensity: 0.72 },
        keyframes: [
          {
            id: "kf-filter-1",
            property: "intensity",
            offsetMs: 0,
            value: 0.45,
          },
          {
            id: "kf-filter-2",
            property: "intensity",
            offsetMs: 1200,
            value: 0.72,
            easing: "ease_in_out",
          },
        ],
      },
      {
        id: "cinematic_grade",
        kind: "color_grade",
        label: "Cinematic Grade Preset",
        parameters: { contrast: 1.08 },
      },
      {
        id: "retro_film",
        kind: "film_emulation",
        label: "Retro Film Preset",
      },
      {
        id: "clean_bright",
        kind: "clarity_boost",
        label: "Clean Bright Preset",
      },
      {
        id: "teal_orange",
        kind: "split_tone",
        label: "Teal Orange Preset",
      },
    ],
    effects: [
      {
        id: "clip-glitch-1",
        kind: "glitch",
        label: "Glitch Accent",
        target: { type: "clip", id: "clip-1" },
        parameters: { intensity: 0.4 },
        keyframes: [
          {
            id: "kf-effect-1",
            property: "intensity",
            offsetMs: 0,
            value: 0.1,
          },
          {
            id: "kf-effect-2",
            property: "intensity",
            offsetMs: 200,
            value: 0.4,
            easing: "hold",
          },
        ],
      },
      {
        id: "zoom_punch",
        kind: "scale_pulse",
        label: "Zoom Punch Preset",
      },
      {
        id: "glitch",
        kind: "signal_break",
        label: "Glitch Preset",
      },
      {
        id: "speed_ramp",
        kind: "time_remap",
        label: "Speed Ramp Preset",
      },
      {
        id: "soft_flash",
        kind: "luma_flash",
        label: "Soft Flash Preset",
      },
      {
        id: "subtitle_pop",
        kind: "caption_motion",
        label: "Subtitle Pop Preset",
      },
    ],
    exportPresets: createDefaultProject().exportPresets,
    presets: createDefaultProject().presets,
    tracks: [
      {
        id: "v1",
        name: "Primary Video",
        kind: "video",
        clips: [
          {
            id: "clip-1",
            assetId: "asset-1",
            label: "Opening Answer",
            timelineRange: { startMs: 0, endMs: 2400 },
            sourceRange: { startMs: 6000, endMs: 8400 },
            filterIds: ["clip-grade-1"],
            effectIds: ["clip-glitch-1"],
            subtitleIds: ["sub-1"],
          },
        ],
      },
    ],
    subtitles: [
      {
        id: "sub-1",
        text: "We finally shipped the first cut.",
        startMs: 0,
        endMs: 2200,
        styleId: "clean_documentary",
        assetId: "asset-1",
        trackId: "v1",
        clipId: "clip-1",
        words: [
          { text: "We", startMs: 0, endMs: 200 },
          { text: "finally", startMs: 200, endMs: 700 },
        ],
      },
    ],
  });

  assert.equal(project.tracks[0].clips[0].filterIds[0], "clip-grade-1");
  assert.equal(project.subtitles[0].words[1].text, "finally");
  assert.equal(project.filters[0].keyframes[1].easing, "ease_in_out");
  assert.equal(project.effects[0].target.id, "clip-1");
  assert.equal(Object.isFrozen(project.filters[0].keyframes[0]), true);
});

test("createProjectDocument rejects broken cross references", () => {
  assert.throws(
    () =>
      createProjectDocument({
        id: "project-invalid-001",
        name: "Invalid Project",
        aspectRatio: "16:9",
        capabilities: createDefaultProject().capabilities,
        assets: [
          {
            id: "asset-1",
            type: "video",
            label: "Source",
            source: { path: "media/source.mp4" },
          },
        ],
        subtitleStyles: [],
        filters: [],
        effects: [],
        exportPresets: createDefaultProject().exportPresets,
        presets: createDefaultProject().presets,
        tracks: [
          {
            id: "v1",
            name: "Primary",
            kind: "video",
            clips: [
              {
                id: "clip-1",
                assetId: "asset-1",
                label: "Clip",
                timelineRange: { startMs: 0, endMs: 1000 },
                sourceRange: { startMs: 0, endMs: 1000 },
                subtitleIds: ["sub-missing"],
              },
            ],
          },
        ],
        subtitles: [],
      }),
    /unknown subtitle/,
  );
});

test("default project factory now exposes canonical registries for subtitles filters effects and export presets", () => {
  const project = createDefaultProject();
  const document = createDefaultProjectDocument();

  assert.equal(project.subtitleStyles.length >= 2, true);
  assert.equal(project.filters.length >= 4, true);
  assert.equal(project.effects.length >= 5, true);
  assert.equal(project.exportPresets.length >= 2, true);
  assert.equal(project.presets.defaultSubtitleStyleId, "clean_documentary");
  assert.equal(document.tracks.length, 0);
  assert.equal(document.aspectRatio, "16:9");
});
