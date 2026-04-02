import { compileTaskGraph } from "../../../packages/agent/src/task-compiler.js";
import { createDefaultProject } from "../../../packages/project/src/project-factory.js";
import { buildFFmpegCommand, compileRenderPlan } from "../../../packages/render/src/ffmpeg/compiler.js";

function printJson(value) {
  console.log(JSON.stringify(value, null, 2));
}

function runDoctor() {
  const project = createDefaultProject();
  printJson({
    name: project.name,
    capabilities: project.capabilities,
    assets: project.assets,
    note: "Rust toolchain is not installed in the current environment, so only the control-plane foundation is active.",
  });
}

function runPlan(prompt) {
  const project = createDefaultProject();
  const taskGraph = compileTaskGraph({ prompt, project });
  const renderPlan = compileRenderPlan(taskGraph);
  const ffmpegCommand = buildFFmpegCommand({
    inputPath: "input.mp4",
    outputPath: "output.mp4",
    renderPlan,
  });

  printJson({
    taskGraph,
    renderPlan,
    ffmpegCommand,
  });
}

const [, , command = "doctor", ...rest] = process.argv;

switch (command) {
  case "doctor":
    runDoctor();
    break;
  case "plan":
    runPlan(rest.join(" ").trim() || "把采访视频剪成更快节奏的 9:16 版本，自动字幕，电影感加一点 glitch 小心思");
    break;
  default:
    console.error(`Unknown command: ${command}`);
    process.exitCode = 1;
}
