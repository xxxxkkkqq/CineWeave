import { createProjectDocument, createProjectDraft } from "../../domain/src/schema.js";

function createDefaultProjectConfig() {
  return {
    id: "cineweave-demo",
    name: "CineWeave Demo Project",
    capabilities: {
      subtitles: true,
      filters: true,
      effects: true,
      timelineEditing: true,
      localRender: true,
      rhythmPolish: true,
      supportedAspectRatios: ["16:9", "9:16", "1:1"],
      supportedFilters: ["cinematic_grade", "retro_film", "clean_bright", "teal_orange"],
      supportedEffects: ["zoom_punch", "glitch", "speed_ramp", "soft_flash", "subtitle_pop"],
    },
    assets: [
      {
        id: "asset-interview-001",
        type: "video",
        label: "Interview Source",
        source: {
          path: "media/interview-source.mp4",
        },
        media: {
          durationMs: 184000,
          width: 3840,
          height: 2160,
          frameRate: 29.97,
          audioChannels: 2,
          sampleRate: 48000,
        },
        tags: ["interview", "source"],
      },
    ],
    subtitleStyles: [
      {
        id: "clean_documentary",
        label: "Clean Documentary",
        fontFamily: "Aptos",
        fontSizePx: 46,
        placement: "bottom",
        fillColor: "#FFFFFF",
        strokeColor: "#141414",
        maxLines: 2,
      },
      {
        id: "expressive_social",
        label: "Expressive Social",
        fontFamily: "Trebuchet MS",
        fontSizePx: 54,
        placement: "bottom",
        fillColor: "#FFF7E8",
        strokeColor: "#1D1D1B",
        backgroundColor: "rgba(29,29,27,0.58)",
        maxLines: 2,
        animationPreset: "subtitle_pop",
      },
    ],
    filters: [
      {
        id: "cinematic_grade",
        kind: "color_grade",
        label: "Cinematic Grade",
        category: "look",
        parameters: {
          contrast: 1.08,
          saturation: 1.12,
          brightness: 0.01,
        },
        renderBackend: "ffmpeg",
      },
      {
        id: "retro_film",
        kind: "film_emulation",
        label: "Retro Film",
        category: "look",
        parameters: {
          grain: 0.4,
          curve: "vintage",
        },
        renderBackend: "ffmpeg",
      },
      {
        id: "clean_bright",
        kind: "clarity_boost",
        label: "Clean Bright",
        category: "utility",
        parameters: {
          brightness: 0.03,
          saturation: 1.04,
        },
        renderBackend: "ffmpeg",
      },
      {
        id: "teal_orange",
        kind: "split_tone",
        label: "Teal Orange",
        category: "look",
        parameters: {
          teal: 0.05,
          orange: 0.04,
        },
        renderBackend: "ffmpeg",
      },
    ],
    effects: [
      {
        id: "zoom_punch",
        kind: "scale_pulse",
        label: "Zoom Punch",
        category: "motion",
        parameters: {
          maxZoom: 1.2,
        },
        renderBackend: "ffmpeg",
      },
      {
        id: "glitch",
        kind: "signal_break",
        label: "Glitch",
        category: "stylized",
        parameters: {
          intensity: 0.65,
        },
        renderBackend: "compositor",
      },
      {
        id: "speed_ramp",
        kind: "time_remap",
        label: "Speed Ramp",
        category: "timing",
        parameters: {
          speed: 0.92,
        },
        renderBackend: "ffmpeg",
      },
      {
        id: "soft_flash",
        kind: "luma_flash",
        label: "Soft Flash",
        category: "light",
        parameters: {
          brightness: 0.06,
        },
        renderBackend: "ffmpeg",
      },
      {
        id: "subtitle_pop",
        kind: "caption_motion",
        label: "Subtitle Pop",
        category: "text",
        parameters: {
          scale: 1.08,
        },
        renderBackend: "compositor",
      },
    ],
    exportPresets: [
      {
        id: "social_master",
        label: "Social Master",
        container: "mp4",
        aspectRatio: "9:16",
        video: {
          codec: "h264",
          width: 1080,
          height: 1920,
          frameRate: 30,
          bitrateKbps: 18000,
          pixelFormat: "yuv420p",
        },
        audio: {
          codec: "aac",
          sampleRate: 48000,
          channels: 2,
          bitrateKbps: 320,
        },
      },
      {
        id: "editorial_review",
        label: "Editorial Review",
        container: "mov",
        aspectRatio: "16:9",
        video: {
          codec: "prores",
          width: 1920,
          height: 1080,
          frameRate: 25,
          bitrateKbps: 50000,
          pixelFormat: "yuv422p10le",
        },
        audio: {
          codec: "pcm_s16le",
          sampleRate: 48000,
          channels: 2,
        },
      },
    ],
    presets: {
      defaultAspectRatio: "16:9",
      defaultSubtitleStyleId: "clean_documentary",
      polishedSubtitleStyleId: "expressive_social",
      exportProfile: "social_master",
    },
    metadata: {
      version: 1,
      tags: ["demo", "local-first"],
    },
  };
}

export function createDefaultProject() {
  return createProjectDraft(createDefaultProjectConfig());
}

export function createDefaultProjectDocument() {
  return createProjectDocument({
    ...createDefaultProjectConfig(),
    aspectRatio: "16:9",
    tracks: [],
    subtitles: [],
  });
}
