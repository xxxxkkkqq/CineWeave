import { EFFECT_LIBRARY, STYLE_LIBRARY } from "./style-library.js";

function unique(values) {
  return [...new Set(values)];
}

export function compileVisualChain(intent) {
  const chain = [];
  const warnings = [];

  for (const style of intent.styleProfiles) {
    const filters = STYLE_LIBRARY[style];
    if (!filters) {
      warnings.push(`Unsupported style profile: ${style}`);
      continue;
    }
    chain.push(...filters);
  }

  for (const effect of intent.effects) {
    const effectDefinition = EFFECT_LIBRARY[effect];
    if (!effectDefinition) {
      warnings.push(`Unsupported effect: ${effect}`);
      continue;
    }
    if (!effectDefinition.ffmpeg) {
      warnings.push(`Effect requires non-FFmpeg compositor or custom renderer: ${effect}`);
      continue;
    }
    chain.push(effectDefinition.ffmpeg);
  }

  return {
    filterChain: unique(chain),
    warnings,
  };
}

export function compileRenderPlan(taskGraph) {
  const visual = compileVisualChain(taskGraph.intent);

  return {
    aspectRatio: taskGraph.intent.outputAspectRatio,
    precisionMode: taskGraph.intent.precisionMode,
    subtitles: taskGraph.intent.requestedSubtitles
      ? {
          enabled: true,
          mode: taskGraph.intent.polishNotes.includes("micro-polish") ? "expressive" : "clean",
        }
      : { enabled: false, mode: "off" },
    filterChain: visual.filterChain,
    warnings: [...taskGraph.warnings, ...visual.warnings],
  };
}

export function buildFFmpegCommand({ inputPath, outputPath, renderPlan }) {
  const args = ["ffmpeg", "-i", inputPath];

  if (renderPlan.filterChain.length > 0) {
    args.push("-vf", renderPlan.filterChain.join(","));
  }

  args.push("-y", outputPath);
  return args;
}
