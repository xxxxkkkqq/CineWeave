import test from "node:test";
import assert from "node:assert/strict";

import { compileTaskGraph } from "../packages/agent/src/task-compiler.js";
import { createDefaultProject } from "../packages/project/src/project-factory.js";
import { buildFFmpegCommand, compileRenderPlan } from "../packages/render/src/ffmpeg/compiler.js";

test("compileRenderPlan produces FFmpeg filter chains and warnings for non-native effects", () => {
  const taskGraph = compileTaskGraph({
    prompt: "自动字幕，电影胶片风，加一点 glitch",
    project: createDefaultProject(),
  });

  const renderPlan = compileRenderPlan(taskGraph);
  const command = buildFFmpegCommand({
    inputPath: "input.mp4",
    outputPath: "output.mp4",
    renderPlan,
  });

  assert.equal(renderPlan.subtitles.enabled, true);
  assert.equal(renderPlan.filterChain.includes("curves=vintage"), true);
  assert.equal(renderPlan.filterChain.includes("eq=contrast=1.08:saturation=1.12:brightness=0.01"), true);
  assert.equal(renderPlan.warnings.length, 1);
  assert.equal(renderPlan.warnings[0], "Effect requires non-FFmpeg compositor or custom renderer: glitch");
  assert.deepEqual(command, [
    "ffmpeg",
    "-i",
    "input.mp4",
    "-vf",
    renderPlan.filterChain.join(","),
    "-y",
    "output.mp4",
  ]);
});
