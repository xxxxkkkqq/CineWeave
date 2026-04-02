import test from "node:test";
import assert from "node:assert/strict";

import { compileTaskGraph } from "../packages/agent/src/task-compiler.js";
import { createDefaultProject } from "../packages/project/src/project-factory.js";

test("compileTaskGraph builds a production-oriented execution graph", () => {
  const taskGraph = compileTaskGraph({
    prompt: "把采访视频剪得更利落，自动字幕，青橙风格，9:16 输出，加一点 zoom 和小心思",
    project: createDefaultProject(),
  });

  const taskIds = taskGraph.tasks.map((task) => task.id);

  assert.deepEqual(taskIds, [
    "ingest-source",
    "analyze-source",
    "build-cut-plan",
    "build-subtitles",
    "apply-filter-style",
    "apply-effects",
    "polish-edit",
    "quality-check",
    "render-master",
  ]);

  const renderTask = taskGraph.tasks.find((task) => task.id === "render-master");
  assert.deepEqual(renderTask.dependsOn, ["quality-check"]);
  assert.equal(renderTask.payload.exportProfile, "social_master");

  const subtitleTask = taskGraph.tasks.find((task) => task.id === "build-subtitles");
  assert.equal(subtitleTask.payload.styleId, "expressive_social");
});
