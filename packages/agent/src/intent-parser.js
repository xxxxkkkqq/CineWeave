import { createEditIntent } from "../../domain/src/schema.js";

const STYLE_RULES = [
  { tokens: ["电影", "cinematic", "cinema"], profile: "cinematic_grade" },
  { tokens: ["胶片", "film", "vintage", "复古"], profile: "retro_film" },
  { tokens: ["清透", "bright", "clean"], profile: "clean_bright" },
  { tokens: ["teal", "orange", "青橙"], profile: "teal_orange" },
];

const EFFECT_RULES = [
  { tokens: ["glitch", "故障"], effect: "glitch" },
  { tokens: ["zoom", "推近", "punch"], effect: "zoom_punch" },
  { tokens: ["speed ramp", "变速", "卡点"], effect: "speed_ramp" },
  { tokens: ["flash", "闪白", "soft flash"], effect: "soft_flash" },
  { tokens: ["字幕动效", "subtitle pop"], effect: "subtitle_pop" },
];

function normalizePrompt(prompt) {
  return prompt.toLowerCase();
}

function includesAny(text, tokens) {
  return tokens.some((token) => text.includes(token.toLowerCase()));
}

function unique(values) {
  return [...new Set(values)];
}

function detectAspectRatio(prompt) {
  if (/9\s*[:：]\s*16/.test(prompt) || prompt.includes("竖屏") || prompt.includes("shorts")) {
    return "9:16";
  }
  if (/1\s*[:：]\s*1/.test(prompt) || prompt.includes("square")) {
    return "1:1";
  }
  return "16:9";
}

export function parseIntent(prompt) {
  const normalized = normalizePrompt(prompt);

  const styleProfiles = unique(
    STYLE_RULES.filter((rule) => includesAny(normalized, rule.tokens)).map((rule) => rule.profile),
  );

  const effects = unique(
    EFFECT_RULES.filter((rule) => includesAny(normalized, rule.tokens)).map((rule) => rule.effect),
  );

  const requestedSubtitles = includesAny(normalized, ["字幕", "caption", "subtitle"]);
  const requestedCuts = includesAny(normalized, ["剪", "cut", "trim", "节奏", "粗剪", "精剪"]);
  const precisionMode = includesAny(normalized, ["精准", "精确", "卡点", "beat", "对齐"]) ? "precise" : "standard";
  const polishNotes = unique(
    [
      includesAny(normalized, ["小心思", "polish", "高级感"]) ? "micro-polish" : null,
      includesAny(normalized, ["干净", "利落"]) ? "clean-pacing" : null,
      includesAny(normalized, ["氛围", "mood"]) ? "atmosphere-driven" : null,
    ].filter(Boolean),
  );

  return createEditIntent({
    prompt,
    requestedCuts,
    requestedSubtitles,
    outputAspectRatio: detectAspectRatio(prompt),
    precisionMode,
    styleProfiles,
    effects,
    polishNotes,
  });
}
