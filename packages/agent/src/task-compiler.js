import { CapabilityError } from "../../core/src/errors.js";
import { createTask, createTaskGraph, TASK_KIND } from "../../domain/src/schema.js";
import { parseIntent } from "./intent-parser.js";

function ensureCapability(project, capability, details) {
  if (!project.capabilities[capability]) {
    throw new CapabilityError(`Project does not support capability: ${capability}`, details);
  }
}

function ensureSupportedValue(project, fieldName, value, supportedValues) {
  if (!supportedValues.includes(value)) {
    throw new CapabilityError(`${fieldName} is not supported: ${value}`, {
      fieldName,
      value,
      supportedValues,
    });
  }
}

function resolveSubtitleStyleId(intent, project) {
  if (intent.polishNotes.includes("micro-polish")) {
    return project.presets.polishedSubtitleStyleId ?? project.presets.defaultSubtitleStyleId ?? null;
  }

  return project.presets.defaultSubtitleStyleId ?? null;
}

function createBaseTasks(intent, project) {
  const subtitleStyleId = resolveSubtitleStyleId(intent, project);
  const exportProfile = project.presets.exportProfile ?? "social_master";
  const tasks = [
    createTask({
      id: "ingest-source",
      kind: TASK_KIND.INGEST,
      title: "Ingest source assets",
      payload: { assetCountHint: 1 },
    }),
    createTask({
      id: "analyze-source",
      kind: TASK_KIND.ANALYZE,
      title: "Analyze speech, pacing, and shot structure",
      dependsOn: ["ingest-source"],
      payload: { precisionMode: intent.precisionMode },
    }),
  ];

  if (intent.requestedCuts) {
    tasks.push(
      createTask({
        id: "build-cut-plan",
        kind: TASK_KIND.CUT,
        title: "Generate rough cut and pacing decisions",
        dependsOn: ["analyze-source"],
        payload: { precisionMode: intent.precisionMode },
      }),
    );
  }

  if (intent.requestedSubtitles) {
    tasks.push(
      createTask({
        id: "build-subtitles",
        kind: TASK_KIND.SUBTITLE,
        title: "Generate and style subtitles",
        dependsOn: ["analyze-source"],
        payload: {
          style: intent.polishNotes.includes("micro-polish") ? "expressive" : "clean",
          styleId: subtitleStyleId,
        },
      }),
    );
  }

  if (intent.styleProfiles.length > 0) {
    tasks.push(
      createTask({
        id: "apply-filter-style",
        kind: TASK_KIND.FILTER,
        title: "Compile visual style profile",
        dependsOn: [intent.requestedCuts ? "build-cut-plan" : "analyze-source"],
        payload: { styles: intent.styleProfiles },
      }),
    );
  }

  if (intent.effects.length > 0) {
    tasks.push(
      createTask({
        id: "apply-effects",
        kind: TASK_KIND.EFFECT,
        title: "Apply effect directives",
        dependsOn: [intent.requestedCuts ? "build-cut-plan" : "analyze-source"],
        payload: { effects: intent.effects },
      }),
    );
  }

  if (intent.polishNotes.length > 0 || intent.precisionMode === "precise") {
    tasks.push(
      createTask({
        id: "polish-edit",
        kind: TASK_KIND.POLISH,
        title: "Perform rhythm polish and attention to detail pass",
        dependsOn: [
          ...(intent.requestedCuts ? ["build-cut-plan"] : ["analyze-source"]),
          ...(intent.requestedSubtitles ? ["build-subtitles"] : []),
        ],
        payload: { notes: intent.polishNotes, precisionMode: intent.precisionMode },
      }),
    );
  }

  const qualityDependencies = tasks
    .filter((task) => ![TASK_KIND.INGEST, TASK_KIND.ANALYZE].includes(task.kind))
    .map((task) => task.id);

  tasks.push(
    createTask({
      id: "quality-check",
      kind: TASK_KIND.QC,
      title: "Run pre-render validation",
      dependsOn: qualityDependencies.length > 0 ? qualityDependencies : ["analyze-source"],
      payload: { aspectRatio: intent.outputAspectRatio },
    }),
    createTask({
      id: "render-master",
      kind: TASK_KIND.RENDER,
      title: "Render output master",
      dependsOn: ["quality-check"],
      payload: { aspectRatio: intent.outputAspectRatio, exportProfile },
    }),
  );

  return tasks;
}

export function compileTaskGraph({ prompt, project }) {
  const intent = parseIntent(prompt);

  ensureCapability(project, "timelineEditing", { projectId: project.id });
  ensureCapability(project, "localRender", { projectId: project.id });
  ensureSupportedValue(
    project,
    "outputAspectRatio",
    intent.outputAspectRatio,
    project.capabilities.supportedAspectRatios,
  );

  if (intent.requestedSubtitles) {
    ensureCapability(project, "subtitles", { projectId: project.id });
  }

  if (intent.styleProfiles.length > 0) {
    ensureCapability(project, "filters", { projectId: project.id });
    for (const style of intent.styleProfiles) {
      ensureSupportedValue(project, "styleProfile", style, project.capabilities.supportedFilters);
    }
  }

  if (intent.effects.length > 0) {
    ensureCapability(project, "effects", { projectId: project.id });
    for (const effect of intent.effects) {
      ensureSupportedValue(project, "effect", effect, project.capabilities.supportedEffects);
    }
  }

  return createTaskGraph({
    intent,
    tasks: createBaseTasks(intent, project),
    warnings: [],
  });
}
