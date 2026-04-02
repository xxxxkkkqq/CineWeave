import test from "node:test";
import assert from "node:assert/strict";

import { parseIntent } from "../packages/agent/src/intent-parser.js";

test("parseIntent extracts subtitle, style, effect, aspect ratio, and precision", () => {
  const intent = parseIntent("帮我剪成 9:16，自动字幕，做成电影胶片感，加一点 glitch 卡点和小心思");

  assert.equal(intent.requestedSubtitles, true);
  assert.equal(intent.requestedCuts, true);
  assert.equal(intent.outputAspectRatio, "9:16");
  assert.equal(intent.precisionMode, "precise");
  assert.deepEqual(intent.styleProfiles, ["cinematic_grade", "retro_film"]);
  assert.deepEqual(intent.effects, ["glitch", "speed_ramp"]);
  assert.deepEqual(intent.polishNotes, ["micro-polish"]);
});
