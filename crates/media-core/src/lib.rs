mod adapter;
mod persistence;
mod timeline;

pub use adapter::{execute_adapter_request, AdapterRequest, AdapterResponse, DocumentCommand};
pub use persistence::{
    event_log_from_events, history_log_from_history, load_event_log_from_path,
    load_history_from_paths, load_snapshot_from_path, load_state_from_snapshot_and_log,
    save_event_log_to_path, save_history_to_paths, save_snapshot_to_path, snapshot_from_state,
    ProjectEventLog, ProjectHistoryLog, ProjectSnapshot, PROJECT_DOCUMENT_VERSION,
};
pub use timeline::{
    timeline_demo_history, Asset, AssetMedia, AssetSource, AssetType, Clip, EditorSelection,
    EditorState, EditorViewport, Effect, ExportAudioSettings, ExportPreset, ExportVideoSettings,
    Filter, Keyframe, KeyframeEasing, PersistedHistoryEntry, ProjectEvent, ProjectHistory,
    ProjectMetadata, ProjectPresets, ProjectState, Resolution, Subtitle, SubtitleStyle,
    SubtitleWord, TargetReference, TargetType, TimeRange, Track, TrackKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrecisionMode {
    Standard,
    Precise,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskKind {
    Ingest,
    Analyze,
    Cut,
    Subtitle,
    Filter,
    Effect,
    Polish,
    QualityCheck,
    Render,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditIntent {
    pub prompt: String,
    pub requested_cuts: bool,
    pub requested_subtitles: bool,
    pub output_aspect_ratio: String,
    pub precision_mode: PrecisionMode,
    pub style_profiles: Vec<String>,
    pub effects: Vec<String>,
    pub polish_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub kind: TaskKind,
    pub title: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskGraph {
    pub intent: EditIntent,
    pub tasks: Vec<Task>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderPlan {
    pub aspect_ratio: String,
    pub precision_mode: PrecisionMode,
    pub subtitles_enabled: bool,
    pub subtitle_mode: String,
    pub filter_chain: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDraft {
    pub id: String,
    pub name: String,
    pub supported_aspect_ratios: Vec<String>,
    pub supported_filters: Vec<String>,
    pub supported_effects: Vec<String>,
}

pub fn default_project() -> ProjectDraft {
    ProjectDraft {
        id: "cineweave-demo".into(),
        name: "CineWeave Demo Project".into(),
        supported_aspect_ratios: vec!["16:9".into(), "9:16".into(), "1:1".into()],
        supported_filters: vec![
            "cinematic_grade".into(),
            "retro_film".into(),
            "clean_bright".into(),
            "teal_orange".into(),
        ],
        supported_effects: vec![
            "zoom_punch".into(),
            "glitch".into(),
            "speed_ramp".into(),
            "soft_flash".into(),
            "subtitle_pop".into(),
        ],
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(&needle.to_lowercase()))
}

fn push_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|existing| existing == value) {
        target.push(value.to_string());
    }
}

fn detect_aspect_ratio(prompt: &str) -> String {
    if prompt.contains("9:16")
        || prompt.contains("9：16")
        || prompt.contains("竖屏")
        || prompt.contains("shorts")
    {
        return "9:16".into();
    }

    if prompt.contains("1:1") || prompt.contains("1：1") || prompt.contains("square") {
        return "1:1".into();
    }

    "16:9".into()
}

pub fn parse_intent(prompt: &str) -> EditIntent {
    let normalized = prompt.to_lowercase();
    let mut style_profiles = Vec::new();
    let mut effects = Vec::new();
    let mut polish_notes = Vec::new();

    for (tokens, profile) in [
        (&["电影", "cinematic", "cinema"][..], "cinematic_grade"),
        (&["胶片", "film", "vintage", "复古"][..], "retro_film"),
        (&["清透", "bright", "clean"][..], "clean_bright"),
        (&["teal", "orange", "青橙"][..], "teal_orange"),
    ] {
        if contains_any(&normalized, tokens) {
            push_unique(&mut style_profiles, profile);
        }
    }

    for (tokens, effect) in [
        (&["glitch", "故障"][..], "glitch"),
        (&["zoom", "推近", "punch"][..], "zoom_punch"),
        (&["speed ramp", "变速", "卡点"][..], "speed_ramp"),
        (&["flash", "闪白", "soft flash"][..], "soft_flash"),
        (&["字幕动效", "subtitle pop"][..], "subtitle_pop"),
    ] {
        if contains_any(&normalized, tokens) {
            push_unique(&mut effects, effect);
        }
    }

    if contains_any(&normalized, &["小心思", "polish", "高级感"]) {
        push_unique(&mut polish_notes, "micro-polish");
    }
    if contains_any(&normalized, &["干净", "利落"]) {
        push_unique(&mut polish_notes, "clean-pacing");
    }
    if contains_any(&normalized, &["氛围", "mood"]) {
        push_unique(&mut polish_notes, "atmosphere-driven");
    }

    EditIntent {
        prompt: prompt.into(),
        requested_cuts: contains_any(&normalized, &["剪", "cut", "trim", "节奏", "粗剪", "精剪"]),
        requested_subtitles: contains_any(&normalized, &["字幕", "caption", "subtitle"]),
        output_aspect_ratio: detect_aspect_ratio(prompt),
        precision_mode: if contains_any(&normalized, &["精准", "精确", "卡点", "beat", "对齐"]) {
            PrecisionMode::Precise
        } else {
            PrecisionMode::Standard
        },
        style_profiles,
        effects,
        polish_notes,
    }
}

fn task(id: &str, kind: TaskKind, title: &str, depends_on: &[&str]) -> Task {
    Task {
        id: id.into(),
        kind,
        title: title.into(),
        depends_on: depends_on.iter().map(|value| value.to_string()).collect(),
    }
}

pub fn compile_task_graph(prompt: &str, project: &ProjectDraft) -> Result<TaskGraph, String> {
    let intent = parse_intent(prompt);

    if !project
        .supported_aspect_ratios
        .iter()
        .any(|ratio| ratio == &intent.output_aspect_ratio)
    {
        return Err(format!(
            "unsupported aspect ratio: {}",
            intent.output_aspect_ratio
        ));
    }

    for style in &intent.style_profiles {
        if !project.supported_filters.iter().any(|candidate| candidate == style) {
            return Err(format!("unsupported style profile: {style}"));
        }
    }

    for effect in &intent.effects {
        if !project.supported_effects.iter().any(|candidate| candidate == effect) {
            return Err(format!("unsupported effect: {effect}"));
        }
    }

    let mut tasks = vec![
        task(
            "ingest-source",
            TaskKind::Ingest,
            "Ingest source assets",
            &[],
        ),
        task(
            "analyze-source",
            TaskKind::Analyze,
            "Analyze speech, pacing, and shot structure",
            &["ingest-source"],
        ),
    ];

    if intent.requested_cuts {
        tasks.push(task(
            "build-cut-plan",
            TaskKind::Cut,
            "Generate rough cut and pacing decisions",
            &["analyze-source"],
        ));
    }

    if intent.requested_subtitles {
        tasks.push(task(
            "build-subtitles",
            TaskKind::Subtitle,
            "Generate and style subtitles",
            &["analyze-source"],
        ));
    }

    if !intent.style_profiles.is_empty() {
        tasks.push(task(
            "apply-filter-style",
            TaskKind::Filter,
            "Compile visual style profile",
            &[if intent.requested_cuts {
                "build-cut-plan"
            } else {
                "analyze-source"
            }],
        ));
    }

    if !intent.effects.is_empty() {
        tasks.push(task(
            "apply-effects",
            TaskKind::Effect,
            "Apply effect directives",
            &[if intent.requested_cuts {
                "build-cut-plan"
            } else {
                "analyze-source"
            }],
        ));
    }

    if !intent.polish_notes.is_empty() || intent.precision_mode == PrecisionMode::Precise {
        let mut depends_on = vec![if intent.requested_cuts {
            "build-cut-plan"
        } else {
            "analyze-source"
        }];
        if intent.requested_subtitles {
            depends_on.push("build-subtitles");
        }
        tasks.push(task(
            "polish-edit",
            TaskKind::Polish,
            "Perform rhythm polish and attention to detail pass",
            &depends_on,
        ));
    }

    let quality_dependencies: Vec<&str> = tasks
        .iter()
        .filter(|task| !matches!(task.kind, TaskKind::Ingest | TaskKind::Analyze))
        .map(|task| task.id.as_str())
        .collect();

    let quality_depends = if quality_dependencies.is_empty() {
        vec!["analyze-source"]
    } else {
        quality_dependencies
    };

    tasks.push(task(
        "quality-check",
        TaskKind::QualityCheck,
        "Run pre-render validation",
        &quality_depends,
    ));
    tasks.push(task(
        "render-master",
        TaskKind::Render,
        "Render output master",
        &["quality-check"],
    ));

    Ok(TaskGraph {
        intent,
        tasks,
        warnings: Vec::new(),
    })
}

pub fn compile_render_plan(task_graph: &TaskGraph) -> RenderPlan {
    let mut filter_chain = Vec::new();
    let mut warnings = task_graph.warnings.clone();

    for style in &task_graph.intent.style_profiles {
        match style.as_str() {
            "cinematic_grade" => {
                filter_chain.push("eq=contrast=1.08:saturation=1.12:brightness=0.01".into());
                filter_chain.push("colorbalance=rs=0.02:bs=-0.01".into());
                filter_chain.push("vignette=PI/6".into());
            }
            "retro_film" => {
                filter_chain.push("curves=vintage".into());
                filter_chain.push("noise=alls=10:allf=t".into());
                filter_chain.push("eq=saturation=0.92:contrast=1.03".into());
            }
            "clean_bright" => {
                filter_chain.push("eq=brightness=0.03:saturation=1.04".into());
                filter_chain.push("unsharp=3:3:0.35".into());
            }
            "teal_orange" => {
                filter_chain.push("colorbalance=rs=0.05:gs=0.01:bs=-0.04".into());
                filter_chain.push("eq=saturation=1.07:contrast=1.04".into());
            }
            other => warnings.push(format!("Unsupported style profile: {other}")),
        }
    }

    for effect in &task_graph.intent.effects {
        match effect.as_str() {
            "zoom_punch" => {
                filter_chain.push("zoompan=z='min(zoom+0.0015,1.2)':d=1:s=1080x1920".into())
            }
            "speed_ramp" => filter_chain.push("setpts=0.92*PTS".into()),
            "soft_flash" => filter_chain.push("eq=brightness=0.06:contrast=1.03".into()),
            "subtitle_pop" | "glitch" => warnings.push(format!(
                "Effect requires non-FFmpeg compositor or custom renderer: {effect}"
            )),
            other => warnings.push(format!("Unsupported effect: {other}")),
        }
    }

    filter_chain.dedup();

    RenderPlan {
        aspect_ratio: task_graph.intent.output_aspect_ratio.clone(),
        precision_mode: task_graph.intent.precision_mode.clone(),
        subtitles_enabled: task_graph.intent.requested_subtitles,
        subtitle_mode: if task_graph.intent.polish_notes.iter().any(|note| note == "micro-polish") {
            "expressive".into()
        } else if task_graph.intent.requested_subtitles {
            "clean".into()
        } else {
            "off".into()
        },
        filter_chain,
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_intent_with_styles_and_effects() {
        let intent = parse_intent("帮我剪成 9:16，自动字幕，做成电影胶片感，加一点 glitch 卡点和小心思");
        assert!(intent.requested_subtitles);
        assert!(intent.requested_cuts);
        assert_eq!(intent.output_aspect_ratio, "9:16");
        assert_eq!(intent.precision_mode, PrecisionMode::Precise);
        assert_eq!(
            intent.style_profiles,
            vec!["cinematic_grade".to_string(), "retro_film".to_string()]
        );
        assert_eq!(
            intent.effects,
            vec!["glitch".to_string(), "speed_ramp".to_string()]
        );
    }

    #[test]
    fn compiles_task_graph() {
        let graph = compile_task_graph(
            "把采访视频剪得更利落，自动字幕，青橙风格，9:16 输出，加一点 zoom 和小心思",
            &default_project(),
        )
        .expect("graph should compile");

        let ids: Vec<&str> = graph.tasks.iter().map(|task| task.id.as_str()).collect();
        assert_eq!(
            ids,
            vec![
                "ingest-source",
                "analyze-source",
                "build-cut-plan",
                "build-subtitles",
                "apply-filter-style",
                "apply-effects",
                "polish-edit",
                "quality-check",
                "render-master"
            ]
        );
    }

    #[test]
    fn compiles_render_plan_with_warning_for_glitch() {
        let graph = compile_task_graph("自动字幕，电影胶片风，加一点 glitch", &default_project())
            .expect("graph should compile");
        let render_plan = compile_render_plan(&graph);

        assert!(render_plan.subtitles_enabled);
        assert!(render_plan
            .filter_chain
            .iter()
            .any(|entry| entry == "curves=vintage"));
        assert_eq!(
            render_plan.warnings,
            vec!["Effect requires non-FFmpeg compositor or custom renderer: glitch".to_string()]
        );
    }
}
