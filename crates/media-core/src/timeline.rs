use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(alias = "startMs")]
    pub start_ms: u64,
    #[serde(alias = "endMs")]
    pub end_ms: u64,
}

impl TimeRange {
    pub fn new(start_ms: u64, end_ms: u64) -> Result<Self, String> {
        if end_ms <= start_ms {
            return Err(format!(
                "invalid time range: end_ms ({end_ms}) must be greater than start_ms ({start_ms})"
            ));
        }
        Ok(Self { start_ms, end_ms })
    }

    pub fn duration_ms(&self) -> u64 {
        self.end_ms - self.start_ms
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    #[serde(alias = "Video")]
    Video,
    #[serde(alias = "Audio")]
    Audio,
    #[serde(alias = "Subtitle")]
    Subtitle,
    #[serde(alias = "Effect")]
    Effect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    #[serde(alias = "Video")]
    Video,
    #[serde(alias = "Audio")]
    Audio,
    #[serde(alias = "Image")]
    Image,
    #[serde(alias = "Text")]
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    #[serde(alias = "Project")]
    Project,
    #[serde(alias = "Track")]
    Track,
    #[serde(alias = "Clip")]
    Clip,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyframeEasing {
    #[serde(alias = "Linear")]
    Linear,
    #[serde(alias = "EaseIn")]
    EaseIn,
    #[serde(alias = "EaseOut")]
    EaseOut,
    #[serde(alias = "EaseInOut")]
    EaseInOut,
    #[serde(alias = "Hold")]
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetSource {
    pub path: String,
    #[serde(default, alias = "checksum")]
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetMedia {
    #[serde(default, alias = "durationMs")]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub resolution: Option<Resolution>,
    #[serde(default, alias = "frameRate")]
    pub frame_rate: Option<f64>,
    #[serde(default, alias = "audioChannels")]
    pub audio_channels: Option<u32>,
    #[serde(default, alias = "sampleRate")]
    pub sample_rate: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    #[serde(rename = "type", alias = "type")]
    pub asset_type: AssetType,
    pub label: String,
    pub source: AssetSource,
    #[serde(default)]
    pub media: AssetMedia,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetReference {
    #[serde(rename = "type", alias = "type")]
    pub target_type: TargetType,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: String,
    pub property: String,
    #[serde(alias = "offsetMs")]
    pub offset_ms: u64,
    pub value: Value,
    pub easing: KeyframeEasing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub target: Option<TargetReference>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub parameters: BTreeMap<String, Value>,
    #[serde(default)]
    pub keyframes: Vec<Keyframe>,
    #[serde(default, alias = "renderBackend")]
    pub render_backend: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub target: Option<TargetReference>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub parameters: BTreeMap<String, Value>,
    #[serde(default)]
    pub keyframes: Vec<Keyframe>,
    #[serde(default, alias = "renderBackend")]
    pub render_backend: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubtitleStyle {
    pub id: String,
    pub label: String,
    #[serde(alias = "fontFamily")]
    pub font_family: String,
    #[serde(alias = "fontSizePx")]
    pub font_size_px: u32,
    pub placement: String,
    #[serde(alias = "fillColor")]
    pub fill_color: String,
    #[serde(alias = "strokeColor")]
    pub stroke_color: String,
    #[serde(default, alias = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(alias = "maxLines")]
    pub max_lines: u32,
    #[serde(default, alias = "animationPreset")]
    pub animation_preset: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubtitleWord {
    pub text: String,
    #[serde(alias = "startMs")]
    pub start_ms: u64,
    #[serde(alias = "endMs")]
    pub end_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subtitle {
    pub id: String,
    pub text: String,
    #[serde(alias = "startMs")]
    pub start_ms: u64,
    #[serde(alias = "endMs")]
    pub end_ms: u64,
    #[serde(default, alias = "styleId")]
    pub style_id: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub speaker: Option<String>,
    #[serde(default, alias = "assetId")]
    pub asset_id: Option<String>,
    #[serde(default, alias = "trackId")]
    pub track_id: Option<String>,
    #[serde(default, alias = "clipId")]
    pub clip_id: Option<String>,
    #[serde(default)]
    pub words: Vec<SubtitleWord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportVideoSettings {
    #[serde(default)]
    pub codec: Option<String>,
    #[serde(default)]
    pub resolution: Option<Resolution>,
    #[serde(alias = "frameRate")]
    pub frame_rate: f64,
    #[serde(default, alias = "bitrateKbps")]
    pub bitrate_kbps: Option<u32>,
    #[serde(default, alias = "pixelFormat")]
    pub pixel_format: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportAudioSettings {
    #[serde(default)]
    pub codec: Option<String>,
    #[serde(alias = "sampleRate")]
    pub sample_rate: u32,
    pub channels: u32,
    #[serde(default, alias = "bitrateKbps")]
    pub bitrate_kbps: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportPreset {
    pub id: String,
    pub label: String,
    pub container: String,
    #[serde(alias = "aspectRatio")]
    pub aspect_ratio: String,
    pub video: ExportVideoSettings,
    pub audio: ExportAudioSettings,
    #[serde(default)]
    pub destination: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectPresets {
    #[serde(default, alias = "defaultAspectRatio")]
    pub default_aspect_ratio: Option<String>,
    #[serde(default, alias = "defaultSubtitleStyleId")]
    pub default_subtitle_style_id: Option<String>,
    #[serde(default, alias = "polishedSubtitleStyleId")]
    pub polished_subtitle_style_id: Option<String>,
    #[serde(default, alias = "exportProfile")]
    pub export_profile: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub version: u32,
    #[serde(default, alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(default, alias = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    #[serde(alias = "assetId")]
    pub asset_id: String,
    pub label: String,
    #[serde(alias = "timelineRange")]
    pub timeline_range: TimeRange,
    #[serde(alias = "sourceRange")]
    pub source_range: TimeRange,
    #[serde(default, alias = "filterIds")]
    pub filter_ids: Vec<String>,
    #[serde(default, alias = "effectIds")]
    pub effect_ids: Vec<String>,
    #[serde(default, alias = "subtitleIds")]
    pub subtitle_ids: Vec<String>,
}

impl Clip {
    pub fn new(
        id: impl Into<String>,
        asset_id: impl Into<String>,
        label: impl Into<String>,
        timeline_range: TimeRange,
        source_range: TimeRange,
    ) -> Result<Self, String> {
        let clip = Self {
            id: id.into(),
            asset_id: asset_id.into(),
            label: label.into(),
            timeline_range,
            source_range,
            filter_ids: Vec::new(),
            effect_ids: Vec::new(),
            subtitle_ids: Vec::new(),
        };
        validate_clip_shape(&clip)?;
        Ok(clip)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub kind: TrackKind,
    #[serde(default, alias = "filterIds")]
    pub filter_ids: Vec<String>,
    #[serde(default, alias = "effectIds")]
    pub effect_ids: Vec<String>,
    #[serde(default)]
    pub clips: Vec<Clip>,
}

impl Track {
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: TrackKind) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            filter_ids: Vec::new(),
            effect_ids: Vec::new(),
            clips: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorSelection {
    #[serde(default, alias = "clipIds")]
    pub clip_ids: Vec<String>,
    #[serde(default, alias = "trackId")]
    pub track_id: Option<String>,
}

impl Default for EditorSelection {
    fn default() -> Self {
        Self {
            clip_ids: Vec::new(),
            track_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorViewport {
    #[serde(alias = "scrollXPx")]
    pub scroll_x_px: u64,
    #[serde(alias = "scrollYPx")]
    pub scroll_y_px: u64,
    #[serde(alias = "zoomPercent")]
    pub zoom_percent: u32,
}

impl Default for EditorViewport {
    fn default() -> Self {
        Self {
            scroll_x_px: 0,
            scroll_y_px: 0,
            zoom_percent: 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorState {
    pub selection: EditorSelection,
    pub playhead_ms: u64,
    pub viewport: EditorViewport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectState {
    pub id: String,
    pub name: String,
    #[serde(alias = "aspectRatio")]
    pub aspect_ratio: String,
    #[serde(default)]
    pub assets: Vec<Asset>,
    #[serde(default)]
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub subtitles: Vec<Subtitle>,
    #[serde(default = "default_subtitle_styles", alias = "subtitleStyles")]
    pub subtitle_styles: Vec<SubtitleStyle>,
    #[serde(default = "default_filter_registry")]
    pub filters: Vec<Filter>,
    #[serde(default = "default_effect_registry")]
    pub effects: Vec<Effect>,
    #[serde(default = "default_export_presets", alias = "exportPresets")]
    pub export_presets: Vec<ExportPreset>,
    #[serde(default = "default_project_presets")]
    pub presets: ProjectPresets,
    #[serde(default = "default_project_metadata")]
    pub metadata: ProjectMetadata,
    #[serde(default)]
    pub editor: EditorState,
}

impl ProjectState {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        aspect_ratio: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            aspect_ratio: aspect_ratio.into(),
            assets: Vec::new(),
            tracks: Vec::new(),
            subtitles: Vec::new(),
            subtitle_styles: default_subtitle_styles(),
            filters: default_filter_registry(),
            effects: default_effect_registry(),
            export_presets: default_export_presets(),
            presets: default_project_presets(),
            metadata: default_project_metadata(),
            editor: EditorState::default(),
        }
    }

    pub fn track(&self, track_id: &str) -> Option<&Track> {
        self.tracks.iter().find(|track| track.id == track_id)
    }

    pub fn clip(&self, clip_id: &str) -> Option<&Clip> {
        self.tracks
            .iter()
            .find_map(|track| track.clips.iter().find(|clip| clip.id == clip_id))
    }

    pub fn subtitle(&self, subtitle_id: &str) -> Option<&Subtitle> {
        self.subtitles.iter().find(|subtitle| subtitle.id == subtitle_id)
    }

    pub fn asset(&self, asset_id: &str) -> Option<&Asset> {
        self.assets.iter().find(|asset| asset.id == asset_id)
    }

    pub fn subtitle_style(&self, style_id: &str) -> Option<&SubtitleStyle> {
        self.subtitle_styles.iter().find(|style| style.id == style_id)
    }

    pub fn filter(&self, filter_id: &str) -> Option<&Filter> {
        self.filters.iter().find(|filter| filter.id == filter_id)
    }

    pub fn effect(&self, effect_id: &str) -> Option<&Effect> {
        self.effects.iter().find(|effect| effect.id == effect_id)
    }

    pub fn export_preset(&self, preset_id: &str) -> Option<&ExportPreset> {
        self.export_presets.iter().find(|preset| preset.id == preset_id)
    }

    pub fn replay(mut self, events: &[ProjectEvent]) -> Result<Self, String> {
        for event in events {
            apply_event_to_state(&mut self, event)?;
        }
        validate_project_state_shape(&self)?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_project_state_shape(self)
    }

    fn find_track_index(&self, track_id: &str) -> Option<usize> {
        self.tracks.iter().position(|track| track.id == track_id)
    }

    fn find_clip_position(&self, clip_id: &str) -> Option<(usize, usize)> {
        self.tracks.iter().enumerate().find_map(|(track_index, track)| {
            track
                .clips
                .iter()
                .position(|clip| clip.id == clip_id)
                .map(|clip_index| (track_index, clip_index))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProjectEvent {
    ProjectRenamed { new_name: String },
    SelectionSet { clip_ids: Vec<String>, track_id: Option<String> },
    SelectionCleared,
    PlayheadSet { playhead_ms: u64 },
    ViewportSet { viewport: EditorViewport },
    AssetUpserted { asset: Asset },
    AssetRemoved { asset_id: String },
    SubtitleStyleUpserted { style: SubtitleStyle },
    SubtitleStyleRemoved { style_id: String },
    SubtitleUpserted { subtitle: Subtitle },
    SubtitleRemoved { subtitle_id: String },
    FilterUpserted { filter: Filter },
    FilterRemoved { filter_id: String },
    EffectUpserted { effect: Effect },
    EffectRemoved { effect_id: String },
    ExportPresetUpserted { preset: ExportPreset },
    ExportPresetRemoved { preset_id: String },
    ProjectPresetsSet { presets: ProjectPresets },
    TrackAdded { track: Track, index: usize },
    TrackRemoved { track_id: String },
    TrackRenamed { track_id: String, new_name: String },
    ClipInserted { track_id: String, clip: Clip },
    ClipRemoved { track_id: String, clip_id: String },
    TrackClipsReplaced { track_id: String, clips: Vec<Clip> },
    ClipMoved {
        clip_id: String,
        to_track_id: String,
        new_timeline_range: TimeRange,
    },
    ClipTrimmed {
        clip_id: String,
        new_source_range: TimeRange,
        new_timeline_range: TimeRange,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct HistoryEntry {
    forward: ProjectEvent,
    inverse: ProjectEvent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedHistoryEntry {
    pub forward: ProjectEvent,
    pub inverse: ProjectEvent,
}

#[derive(Debug, Clone)]
pub struct ProjectHistory {
    pub state: ProjectState,
    pub applied_events: Vec<ProjectEvent>,
    past: Vec<HistoryEntry>,
    future: Vec<HistoryEntry>,
}

impl ProjectHistory {
    pub fn new(state: ProjectState) -> Self {
        Self {
            state,
            applied_events: Vec::new(),
            past: Vec::new(),
            future: Vec::new(),
        }
    }

    pub fn apply(&mut self, event: ProjectEvent) -> Result<(), String> {
        let inverse = compute_inverse_event(&self.state, &event)?;
        apply_event_to_state(&mut self.state, &event)?;
        self.applied_events.push(event.clone());
        self.past.push(HistoryEntry {
            forward: event,
            inverse,
        });
        self.future.clear();
        Ok(())
    }

    pub fn undo(&mut self) -> Result<Option<ProjectEvent>, String> {
        let Some(entry) = self.past.pop() else {
            return Ok(None);
        };
        apply_event_to_state(&mut self.state, &entry.inverse)?;
        self.future.push(entry.clone());
        Ok(Some(entry.inverse))
    }

    pub fn redo(&mut self) -> Result<Option<ProjectEvent>, String> {
        let Some(entry) = self.future.pop() else {
            return Ok(None);
        };
        apply_event_to_state(&mut self.state, &entry.forward)?;
        self.past.push(entry.clone());
        Ok(Some(entry.forward))
    }

    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    pub fn checkpoint_state(&self) -> Result<ProjectState, String> {
        let mut checkpoint = self.clone();
        while checkpoint.can_undo() {
            checkpoint.undo()?;
        }
        Ok(checkpoint.state)
    }

    pub fn persisted_past(&self) -> Vec<PersistedHistoryEntry> {
        self.past
            .iter()
            .map(|entry| PersistedHistoryEntry {
                forward: entry.forward.clone(),
                inverse: entry.inverse.clone(),
            })
            .collect()
    }

    pub fn persisted_future(&self) -> Vec<PersistedHistoryEntry> {
        self.future
            .iter()
            .map(|entry| PersistedHistoryEntry {
                forward: entry.forward.clone(),
                inverse: entry.inverse.clone(),
            })
            .collect()
    }

    pub fn from_persisted(
        state: ProjectState,
        past: Vec<PersistedHistoryEntry>,
        future: Vec<PersistedHistoryEntry>,
    ) -> Result<Self, String> {
        let mut history = Self::new(state);

        for entry in past {
            let expected_inverse = compute_inverse_event(&history.state, &entry.forward)?;
            if expected_inverse != entry.inverse {
                return Err(format!(
                    "persisted past entry inverse mismatch for event {:?}",
                    entry.forward
                ));
            }

            apply_event_to_state(&mut history.state, &entry.forward)?;
            history.applied_events.push(entry.forward.clone());
            history.past.push(HistoryEntry {
                forward: entry.forward,
                inverse: entry.inverse,
            });
        }

        let mut cursor = history.state.clone();
        for entry in future.iter().rev() {
            let expected_inverse = compute_inverse_event(&cursor, &entry.forward)?;
            if expected_inverse != entry.inverse {
                return Err(format!(
                    "persisted future entry inverse mismatch for event {:?}",
                    entry.forward
                ));
            }
            apply_event_to_state(&mut cursor, &entry.forward)?;
        }

        history.future = future
            .into_iter()
            .map(|entry| HistoryEntry {
                forward: entry.forward,
                inverse: entry.inverse,
            })
            .collect();

        validate_project_state_shape(&history.state)?;
        Ok(history)
    }

    pub fn set_selection(
        &mut self,
        clip_ids: Vec<String>,
        track_id: Option<String>,
    ) -> Result<(), String> {
        self.apply(ProjectEvent::SelectionSet { clip_ids, track_id })
    }

    pub fn clear_selection(&mut self) -> Result<(), String> {
        self.apply(ProjectEvent::SelectionCleared)
    }

    pub fn set_playhead(&mut self, playhead_ms: u64) -> Result<(), String> {
        self.apply(ProjectEvent::PlayheadSet { playhead_ms })
    }

    pub fn set_viewport(&mut self, viewport: EditorViewport) -> Result<(), String> {
        self.apply(ProjectEvent::ViewportSet { viewport })
    }

    pub fn split_clip(
        &mut self,
        track_id: &str,
        clip_id: &str,
        split_at_ms: u64,
        right_clip_id: impl Into<String>,
    ) -> Result<(), String> {
        let track = self
            .state
            .track(track_id)
            .ok_or_else(|| format!("track {} does not exist", track_id))?;
        let clip_index = track
            .clips
            .iter()
            .position(|clip| clip.id == clip_id)
            .ok_or_else(|| format!("clip {} does not exist on track {}", clip_id, track_id))?;
        let clip = track.clips[clip_index].clone();

        if split_at_ms <= clip.timeline_range.start_ms || split_at_ms >= clip.timeline_range.end_ms {
            return Err(format!(
                "split point {} is outside clip {} timeline range",
                split_at_ms, clip_id
            ));
        }

        let split_offset = split_at_ms - clip.timeline_range.start_ms;
        let source_split_at_ms = clip.source_range.start_ms + split_offset;
        let right_clip_id = right_clip_id.into();

        let left_clip = Clip::new(
            clip.id.clone(),
            clip.asset_id.clone(),
            clip.label.clone(),
            TimeRange::new(clip.timeline_range.start_ms, split_at_ms)?,
            TimeRange::new(clip.source_range.start_ms, source_split_at_ms)?,
        )?;
        let right_clip = Clip::new(
            right_clip_id,
            clip.asset_id.clone(),
            clip.label.clone(),
            TimeRange::new(split_at_ms, clip.timeline_range.end_ms)?,
            TimeRange::new(source_split_at_ms, clip.source_range.end_ms)?,
        )?;

        let mut clips = track.clips.clone();
        clips.splice(clip_index..=clip_index, [left_clip, right_clip]);

        self.apply(ProjectEvent::TrackClipsReplaced {
            track_id: track_id.to_string(),
            clips,
        })
    }

    pub fn ripple_move_clip(&mut self, clip_id: &str, delta_ms: i64) -> Result<i64, String> {
        let (track_index, clip_index) = self
            .state
            .find_clip_position(clip_id)
            .ok_or_else(|| format!("clip {} does not exist", clip_id))?;
        let track = &self.state.tracks[track_index];
        let clip = &track.clips[clip_index];
        let min_start_ms = if clip_index == 0 {
            0
        } else {
            track.clips[clip_index - 1].timeline_range.end_ms
        };

        let requested_start = shift_time_value(clip.timeline_range.start_ms, delta_ms)?;
        let bounded_start = requested_start.max(min_start_ms);
        let effective_delta = bounded_start as i64 - clip.timeline_range.start_ms as i64;

        if effective_delta == 0 {
            return Ok(0);
        }

        let mut clips = track.clips.clone();
        for clip in clips.iter_mut().skip(clip_index) {
            clip.timeline_range = shift_time_range(&clip.timeline_range, effective_delta)?;
        }

        self.apply(ProjectEvent::TrackClipsReplaced {
            track_id: track.id.clone(),
            clips,
        })?;

        Ok(effective_delta)
    }

    pub fn close_gap_before_clip(&mut self, clip_id: &str) -> Result<i64, String> {
        let (track_index, clip_index) = self
            .state
            .find_clip_position(clip_id)
            .ok_or_else(|| format!("clip {} does not exist", clip_id))?;
        let track = &self.state.tracks[track_index];
        let clip = &track.clips[clip_index];

        let target_start_ms = if clip_index == 0 {
            0
        } else {
            track.clips[clip_index - 1].timeline_range.end_ms
        };

        let gap = clip.timeline_range.start_ms.saturating_sub(target_start_ms);
        if gap == 0 {
            return Ok(0);
        }

        self.ripple_move_clip(clip_id, -(gap as i64))
    }
}

pub fn timeline_demo_history() -> Result<ProjectHistory, String> {
    let mut history = ProjectHistory::new(ProjectState::new(
        "timeline-demo",
        "Timeline Demo",
        "9:16",
    ));
    history.apply(ProjectEvent::TrackAdded {
        track: Track::new("v1", "Primary Video", TrackKind::Video),
        index: 0,
    })?;
    history.apply(ProjectEvent::TrackAdded {
        track: Track::new("v2", "B-Roll", TrackKind::Video),
        index: 1,
    })?;
    history.apply(ProjectEvent::ClipInserted {
        track_id: "v1".into(),
        clip: Clip::new(
            "clip-a",
            "asset-interview-001",
            "Interview Take",
            TimeRange::new(0, 4_000)?,
            TimeRange::new(10_000, 14_000)?,
        )?,
    })?;
    history.apply(ProjectEvent::ClipMoved {
        clip_id: "clip-a".into(),
        to_track_id: "v2".into(),
        new_timeline_range: TimeRange::new(6_000, 10_000)?,
    })?;
    history.apply(ProjectEvent::ClipTrimmed {
        clip_id: "clip-a".into(),
        new_source_range: TimeRange::new(11_000, 14_500)?,
        new_timeline_range: TimeRange::new(6_000, 9_500)?,
    })?;
    Ok(history)
}

fn default_true() -> bool {
    true
}

fn default_subtitle_styles() -> Vec<SubtitleStyle> {
    vec![
        SubtitleStyle {
            id: "clean_documentary".into(),
            label: "Clean Documentary".into(),
            font_family: "Aptos".into(),
            font_size_px: 46,
            placement: "bottom".into(),
            fill_color: "#FFFFFF".into(),
            stroke_color: "#141414".into(),
            background_color: None,
            max_lines: 2,
            animation_preset: None,
        },
        SubtitleStyle {
            id: "expressive_social".into(),
            label: "Expressive Social".into(),
            font_family: "Trebuchet MS".into(),
            font_size_px: 54,
            placement: "bottom".into(),
            fill_color: "#FFF7E8".into(),
            stroke_color: "#1D1D1B".into(),
            background_color: Some("rgba(29,29,27,0.58)".into()),
            max_lines: 2,
            animation_preset: Some("subtitle_pop".into()),
        },
    ]
}

fn default_filter_registry() -> Vec<Filter> {
    vec![
        Filter {
            id: "cinematic_grade".into(),
            kind: "color_grade".into(),
            label: "Cinematic Grade".into(),
            category: Some("look".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([
                ("contrast".into(), json!(1.08)),
                ("saturation".into(), json!(1.12)),
                ("brightness".into(), json!(0.01)),
            ]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
        Filter {
            id: "retro_film".into(),
            kind: "film_emulation".into(),
            label: "Retro Film".into(),
            category: Some("look".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([
                ("grain".into(), json!(0.4)),
                ("curve".into(), json!("vintage")),
            ]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
        Filter {
            id: "clean_bright".into(),
            kind: "clarity_boost".into(),
            label: "Clean Bright".into(),
            category: Some("utility".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([
                ("brightness".into(), json!(0.03)),
                ("saturation".into(), json!(1.04)),
            ]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
        Filter {
            id: "teal_orange".into(),
            kind: "split_tone".into(),
            label: "Teal Orange".into(),
            category: Some("look".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([
                ("teal".into(), json!(0.05)),
                ("orange".into(), json!(0.04)),
            ]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
    ]
}

fn default_effect_registry() -> Vec<Effect> {
    vec![
        Effect {
            id: "zoom_punch".into(),
            kind: "scale_pulse".into(),
            label: "Zoom Punch".into(),
            category: Some("motion".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([("maxZoom".into(), json!(1.2))]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
        Effect {
            id: "glitch".into(),
            kind: "signal_break".into(),
            label: "Glitch".into(),
            category: Some("stylized".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([("intensity".into(), json!(0.65))]),
            keyframes: Vec::new(),
            render_backend: Some("compositor".into()),
        },
        Effect {
            id: "speed_ramp".into(),
            kind: "time_remap".into(),
            label: "Speed Ramp".into(),
            category: Some("timing".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([("speed".into(), json!(0.92))]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
        Effect {
            id: "soft_flash".into(),
            kind: "luma_flash".into(),
            label: "Soft Flash".into(),
            category: Some("light".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([("brightness".into(), json!(0.06))]),
            keyframes: Vec::new(),
            render_backend: Some("ffmpeg".into()),
        },
        Effect {
            id: "subtitle_pop".into(),
            kind: "caption_motion".into(),
            label: "Subtitle Pop".into(),
            category: Some("text".into()),
            target: None,
            enabled: true,
            parameters: BTreeMap::from([("scale".into(), json!(1.08))]),
            keyframes: Vec::new(),
            render_backend: Some("compositor".into()),
        },
    ]
}

fn default_export_presets() -> Vec<ExportPreset> {
    vec![
        ExportPreset {
            id: "social_master".into(),
            label: "Social Master".into(),
            container: "mp4".into(),
            aspect_ratio: "9:16".into(),
            video: ExportVideoSettings {
                codec: Some("h264".into()),
                resolution: Some(Resolution {
                    width: 1080,
                    height: 1920,
                }),
                frame_rate: 30.0,
                bitrate_kbps: Some(18_000),
                pixel_format: Some("yuv420p".into()),
            },
            audio: ExportAudioSettings {
                codec: Some("aac".into()),
                sample_rate: 48_000,
                channels: 2,
                bitrate_kbps: Some(320),
            },
            destination: None,
        },
        ExportPreset {
            id: "editorial_review".into(),
            label: "Editorial Review".into(),
            container: "mov".into(),
            aspect_ratio: "16:9".into(),
            video: ExportVideoSettings {
                codec: Some("prores".into()),
                resolution: Some(Resolution {
                    width: 1920,
                    height: 1080,
                }),
                frame_rate: 25.0,
                bitrate_kbps: Some(50_000),
                pixel_format: Some("yuv422p10le".into()),
            },
            audio: ExportAudioSettings {
                codec: Some("pcm_s16le".into()),
                sample_rate: 48_000,
                channels: 2,
                bitrate_kbps: None,
            },
            destination: None,
        },
    ]
}

fn default_project_presets() -> ProjectPresets {
    ProjectPresets {
        default_aspect_ratio: Some("16:9".into()),
        default_subtitle_style_id: Some("clean_documentary".into()),
        polished_subtitle_style_id: Some("expressive_social".into()),
        export_profile: Some("social_master".into()),
    }
}

fn default_project_metadata() -> ProjectMetadata {
    ProjectMetadata {
        version: 1,
        created_at: None,
        updated_at: None,
        tags: vec!["local-first".into()],
    }
}

fn validate_unique_ids<'a>(
    label: &str,
    ids: impl IntoIterator<Item = &'a str>,
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id.to_string()) {
            return Err(format!("duplicate {} id {}", label, id));
        }
    }
    Ok(())
}

fn validate_project_state_shape(state: &ProjectState) -> Result<(), String> {
    validate_unique_ids("asset", state.assets.iter().map(|asset| asset.id.as_str()))?;
    validate_unique_ids(
        "subtitle style",
        state.subtitle_styles.iter().map(|style| style.id.as_str()),
    )?;
    validate_unique_ids("filter", state.filters.iter().map(|filter| filter.id.as_str()))?;
    validate_unique_ids("effect", state.effects.iter().map(|effect| effect.id.as_str()))?;
    validate_unique_ids(
        "export preset",
        state.export_presets.iter().map(|preset| preset.id.as_str()),
    )?;
    validate_unique_ids("track", state.tracks.iter().map(|track| track.id.as_str()))?;
    validate_unique_ids("subtitle", state.subtitles.iter().map(|subtitle| subtitle.id.as_str()))?;
    validate_unique_ids(
        "clip",
        state
            .tracks
            .iter()
            .flat_map(|track| track.clips.iter().map(|clip| clip.id.as_str())),
    )?;

    let track_ids = state
        .tracks
        .iter()
        .map(|track| track.id.as_str())
        .collect::<BTreeSet<_>>();
    let clip_ids = state
        .tracks
        .iter()
        .flat_map(|track| track.clips.iter().map(|clip| clip.id.as_str()))
        .collect::<BTreeSet<_>>();
    let subtitle_style_ids = state
        .subtitle_styles
        .iter()
        .map(|style| style.id.as_str())
        .collect::<BTreeSet<_>>();
    let export_preset_ids = state
        .export_presets
        .iter()
        .map(|preset| preset.id.as_str())
        .collect::<BTreeSet<_>>();
    let filters_by_id = state
        .filters
        .iter()
        .map(|filter| (filter.id.as_str(), filter))
        .collect::<BTreeMap<_, _>>();
    let effects_by_id = state
        .effects
        .iter()
        .map(|effect| (effect.id.as_str(), effect))
        .collect::<BTreeMap<_, _>>();
    let subtitles_by_id = state
        .subtitles
        .iter()
        .map(|subtitle| (subtitle.id.as_str(), subtitle))
        .collect::<BTreeMap<_, _>>();

    if let Some(style_id) = state.presets.default_subtitle_style_id.as_deref() {
        if !subtitle_style_ids.contains(style_id) {
            return Err(format!(
                "project preset default_subtitle_style_id {} is unknown",
                style_id
            ));
        }
    }

    if let Some(style_id) = state.presets.polished_subtitle_style_id.as_deref() {
        if !subtitle_style_ids.contains(style_id) {
            return Err(format!(
                "project preset polished_subtitle_style_id {} is unknown",
                style_id
            ));
        }
    }

    if let Some(export_profile) = state.presets.export_profile.as_deref() {
        if !export_preset_ids.contains(export_profile) {
            return Err(format!(
                "project preset export_profile {} is unknown",
                export_profile
            ));
        }
    }

    for subtitle in &state.subtitles {
        if let Some(style_id) = subtitle.style_id.as_deref() {
            if !subtitle_style_ids.contains(style_id) {
                return Err(format!(
                    "subtitle {} references unknown style {}",
                    subtitle.id, style_id
                ));
            }
        }

        if let Some(track_id) = subtitle.track_id.as_deref() {
            if !track_ids.contains(track_id) {
                return Err(format!(
                    "subtitle {} references unknown track {}",
                    subtitle.id, track_id
                ));
            }
        }

        if let Some(clip_id) = subtitle.clip_id.as_deref() {
            if !clip_ids.contains(clip_id) {
                return Err(format!(
                    "subtitle {} references unknown clip {}",
                    subtitle.id, clip_id
                ));
            }
        }
    }

    for filter in &state.filters {
        if let Some(target) = &filter.target {
            match target.target_type {
                TargetType::Project => {}
                TargetType::Track => {
                    if let Some(track_id) = target.id.as_deref() {
                        if !track_ids.contains(track_id) {
                            return Err(format!(
                                "filter {} references unknown track {}",
                                filter.id, track_id
                            ));
                        }
                    } else {
                        return Err(format!("filter {} track target is missing id", filter.id));
                    }
                }
                TargetType::Clip => {
                    if let Some(clip_id) = target.id.as_deref() {
                        if !clip_ids.contains(clip_id) {
                            return Err(format!(
                                "filter {} references unknown clip {}",
                                filter.id, clip_id
                            ));
                        }
                    } else {
                        return Err(format!("filter {} clip target is missing id", filter.id));
                    }
                }
            }
        }
    }

    for effect in &state.effects {
        if let Some(target) = &effect.target {
            match target.target_type {
                TargetType::Project => {}
                TargetType::Track => {
                    if let Some(track_id) = target.id.as_deref() {
                        if !track_ids.contains(track_id) {
                            return Err(format!(
                                "effect {} references unknown track {}",
                                effect.id, track_id
                            ));
                        }
                    } else {
                        return Err(format!("effect {} track target is missing id", effect.id));
                    }
                }
                TargetType::Clip => {
                    if let Some(clip_id) = target.id.as_deref() {
                        if !clip_ids.contains(clip_id) {
                            return Err(format!(
                                "effect {} references unknown clip {}",
                                effect.id, clip_id
                            ));
                        }
                    } else {
                        return Err(format!("effect {} clip target is missing id", effect.id));
                    }
                }
            }
        }
    }

    for track in &state.tracks {
        for filter_id in &track.filter_ids {
            let Some(filter) = filters_by_id.get(filter_id.as_str()) else {
                return Err(format!(
                    "track {} references unknown filter {}",
                    track.id, filter_id
                ));
            };
            let Some(target) = &filter.target else {
                return Err(format!(
                    "track {} references filter {} without track target",
                    track.id, filter_id
                ));
            };
            if target.target_type != TargetType::Track || target.id.as_deref() != Some(track.id.as_str()) {
                return Err(format!(
                    "track {} filter target mismatch for {}",
                    track.id, filter_id
                ));
            }
        }

        for effect_id in &track.effect_ids {
            let Some(effect) = effects_by_id.get(effect_id.as_str()) else {
                return Err(format!(
                    "track {} references unknown effect {}",
                    track.id, effect_id
                ));
            };
            let Some(target) = &effect.target else {
                return Err(format!(
                    "track {} references effect {} without track target",
                    track.id, effect_id
                ));
            };
            if target.target_type != TargetType::Track || target.id.as_deref() != Some(track.id.as_str()) {
                return Err(format!(
                    "track {} effect target mismatch for {}",
                    track.id, effect_id
                ));
            }
        }

        for clip in &track.clips {
            for filter_id in &clip.filter_ids {
                let Some(filter) = filters_by_id.get(filter_id.as_str()) else {
                    return Err(format!(
                        "clip {} references unknown filter {}",
                        clip.id, filter_id
                    ));
                };
                let Some(target) = &filter.target else {
                    return Err(format!(
                        "clip {} references filter {} without clip target",
                        clip.id, filter_id
                    ));
                };
                if target.target_type != TargetType::Clip || target.id.as_deref() != Some(clip.id.as_str()) {
                    return Err(format!(
                        "clip {} filter target mismatch for {}",
                        clip.id, filter_id
                    ));
                }
            }

            for effect_id in &clip.effect_ids {
                let Some(effect) = effects_by_id.get(effect_id.as_str()) else {
                    return Err(format!(
                        "clip {} references unknown effect {}",
                        clip.id, effect_id
                    ));
                };
                let Some(target) = &effect.target else {
                    return Err(format!(
                        "clip {} references effect {} without clip target",
                        clip.id, effect_id
                    ));
                };
                if target.target_type != TargetType::Clip || target.id.as_deref() != Some(clip.id.as_str()) {
                    return Err(format!(
                        "clip {} effect target mismatch for {}",
                        clip.id, effect_id
                    ));
                }
            }

            for subtitle_id in &clip.subtitle_ids {
                let Some(subtitle) = subtitles_by_id.get(subtitle_id.as_str()) else {
                    return Err(format!(
                        "clip {} references unknown subtitle {}",
                        clip.id, subtitle_id
                    ));
                };
                if subtitle.clip_id.as_deref() != Some(clip.id.as_str()) {
                    return Err(format!(
                        "clip {} subtitle target mismatch for {}",
                        clip.id, subtitle_id
                    ));
                }
            }
        }
    }

    Ok(())
}

fn current_selection_event(state: &ProjectState) -> ProjectEvent {
    ProjectEvent::SelectionSet {
        clip_ids: state.editor.selection.clip_ids.clone(),
        track_id: state.editor.selection.track_id.clone(),
    }
}

fn validate_viewport(viewport: &EditorViewport) -> Result<(), String> {
    if viewport.zoom_percent == 0 {
        return Err("viewport zoom_percent must be greater than zero".into());
    }
    Ok(())
}

fn normalize_selection(
    state: &ProjectState,
    clip_ids: &[String],
    track_id: &Option<String>,
) -> Result<EditorSelection, String> {
    if let Some(track_id) = track_id {
        if state.track(track_id).is_none() {
            return Err(format!("track {} does not exist", track_id));
        }
    }

    let mut normalized_clip_ids = Vec::new();
    for clip_id in clip_ids {
        if state.clip(clip_id).is_none() {
            return Err(format!("clip {} does not exist", clip_id));
        }
        if !normalized_clip_ids.iter().any(|existing| existing == clip_id) {
            normalized_clip_ids.push(clip_id.clone());
        }
    }

    Ok(EditorSelection {
        clip_ids: normalized_clip_ids,
        track_id: track_id.clone(),
    })
}

fn validate_clip_shape(clip: &Clip) -> Result<(), String> {
    if clip.id.trim().is_empty() {
        return Err("clip id must not be empty".into());
    }
    if clip.asset_id.trim().is_empty() {
        return Err(format!("clip {} asset_id must not be empty", clip.id));
    }
    if clip.timeline_range.duration_ms() != clip.source_range.duration_ms() {
        return Err(format!(
            "clip {} has mismatched timeline and source durations",
            clip.id
        ));
    }
    Ok(())
}

fn validate_asset_shape(asset: &Asset) -> Result<(), String> {
    if asset.id.trim().is_empty() {
        return Err("asset id must not be empty".into());
    }
    if asset.label.trim().is_empty() {
        return Err(format!("asset {} label must not be empty", asset.id));
    }
    if asset.source.path.trim().is_empty() {
        return Err(format!("asset {} source.path must not be empty", asset.id));
    }
    Ok(())
}

fn validate_subtitle_style_shape(style: &SubtitleStyle) -> Result<(), String> {
    if style.id.trim().is_empty() {
        return Err("subtitle style id must not be empty".into());
    }
    if style.label.trim().is_empty() {
        return Err(format!("subtitle style {} label must not be empty", style.id));
    }
    if style.font_family.trim().is_empty() {
        return Err(format!(
            "subtitle style {} font_family must not be empty",
            style.id
        ));
    }
    if style.font_size_px == 0 {
        return Err(format!(
            "subtitle style {} font_size_px must be greater than zero",
            style.id
        ));
    }
    if style.max_lines == 0 {
        return Err(format!(
            "subtitle style {} max_lines must be greater than zero",
            style.id
        ));
    }
    Ok(())
}

fn validate_subtitle_shape(subtitle: &Subtitle) -> Result<(), String> {
    if subtitle.id.trim().is_empty() {
        return Err("subtitle id must not be empty".into());
    }
    if subtitle.text.trim().is_empty() {
        return Err(format!("subtitle {} text must not be empty", subtitle.id));
    }
    if subtitle.end_ms <= subtitle.start_ms {
        return Err(format!(
            "subtitle {} end_ms must be greater than start_ms",
            subtitle.id
        ));
    }
    for word in &subtitle.words {
        if word.text.trim().is_empty() {
            return Err(format!("subtitle {} contains empty word text", subtitle.id));
        }
        if word.end_ms <= word.start_ms {
            return Err(format!(
                "subtitle {} contains invalid word timing",
                subtitle.id
            ));
        }
    }
    Ok(())
}

fn validate_keyframe_shape(keyframe: &Keyframe, owner_label: &str, owner_id: &str) -> Result<(), String> {
    if keyframe.id.trim().is_empty() {
        return Err(format!("{} {} has keyframe with empty id", owner_label, owner_id));
    }
    if keyframe.property.trim().is_empty() {
        return Err(format!(
            "{} {} keyframe {} property must not be empty",
            owner_label, owner_id, keyframe.id
        ));
    }
    Ok(())
}

fn validate_filter_shape(filter: &Filter) -> Result<(), String> {
    if filter.id.trim().is_empty() {
        return Err("filter id must not be empty".into());
    }
    if filter.kind.trim().is_empty() {
        return Err(format!("filter {} kind must not be empty", filter.id));
    }
    if filter.label.trim().is_empty() {
        return Err(format!("filter {} label must not be empty", filter.id));
    }
    for keyframe in &filter.keyframes {
        validate_keyframe_shape(keyframe, "filter", &filter.id)?;
    }
    Ok(())
}

fn validate_effect_shape(effect: &Effect) -> Result<(), String> {
    if effect.id.trim().is_empty() {
        return Err("effect id must not be empty".into());
    }
    if effect.kind.trim().is_empty() {
        return Err(format!("effect {} kind must not be empty", effect.id));
    }
    if effect.label.trim().is_empty() {
        return Err(format!("effect {} label must not be empty", effect.id));
    }
    for keyframe in &effect.keyframes {
        validate_keyframe_shape(keyframe, "effect", &effect.id)?;
    }
    Ok(())
}

fn validate_export_preset_shape(preset: &ExportPreset) -> Result<(), String> {
    if preset.id.trim().is_empty() {
        return Err("export preset id must not be empty".into());
    }
    if preset.label.trim().is_empty() {
        return Err(format!("export preset {} label must not be empty", preset.id));
    }
    if preset.container.trim().is_empty() {
        return Err(format!(
            "export preset {} container must not be empty",
            preset.id
        ));
    }
    if preset.aspect_ratio.trim().is_empty() {
        return Err(format!(
            "export preset {} aspect_ratio must not be empty",
            preset.id
        ));
    }
    if preset.video.frame_rate <= 0.0 {
        return Err(format!(
            "export preset {} video.frame_rate must be greater than zero",
            preset.id
        ));
    }
    if preset.audio.sample_rate == 0 || preset.audio.channels == 0 {
        return Err(format!(
            "export preset {} audio settings must be greater than zero",
            preset.id
        ));
    }
    Ok(())
}

fn apply_project_state_update<F>(state: &mut ProjectState, updater: F) -> Result<(), String>
where
    F: FnOnce(&mut ProjectState) -> Result<(), String>,
{
    let mut candidate = state.clone();
    updater(&mut candidate)?;
    validate_project_state_shape(&candidate)?;
    *state = candidate;
    Ok(())
}

fn push_unique_id(ids: &mut Vec<String>, id: &str) {
    if !ids.iter().any(|existing| existing == id) {
        ids.push(id.to_string());
    }
}

fn remove_id(ids: &mut Vec<String>, id: &str) {
    ids.retain(|existing| existing != id);
}

fn same_target_reference(current: &TargetReference, next: Option<&TargetReference>) -> bool {
    next.is_some_and(|candidate| {
        current.target_type == candidate.target_type && current.id == candidate.id
    })
}

fn attach_filter_target(
    state: &mut ProjectState,
    filter_id: &str,
    target: &TargetReference,
) -> Result<(), String> {
    let Some(target_id) = target.id.as_deref() else {
        return Ok(());
    };

    match target.target_type {
        TargetType::Project => Ok(()),
        TargetType::Track => {
            let index = state
                .find_track_index(target_id)
                .ok_or_else(|| format!("filter {} references unknown track {}", filter_id, target_id))?;
            push_unique_id(&mut state.tracks[index].filter_ids, filter_id);
            Ok(())
        }
        TargetType::Clip => {
            let (track_index, clip_index) = state
                .find_clip_position(target_id)
                .ok_or_else(|| format!("filter {} references unknown clip {}", filter_id, target_id))?;
            push_unique_id(
                &mut state.tracks[track_index].clips[clip_index].filter_ids,
                filter_id,
            );
            Ok(())
        }
    }
}

fn detach_filter_target(state: &mut ProjectState, filter_id: &str, target: &TargetReference) -> Result<(), String> {
    let Some(target_id) = target.id.as_deref() else {
        return Ok(());
    };

    match target.target_type {
        TargetType::Project => Ok(()),
        TargetType::Track => {
            let index = state
                .find_track_index(target_id)
                .ok_or_else(|| format!("filter {} references unknown track {}", filter_id, target_id))?;
            remove_id(&mut state.tracks[index].filter_ids, filter_id);
            Ok(())
        }
        TargetType::Clip => {
            let (track_index, clip_index) = state
                .find_clip_position(target_id)
                .ok_or_else(|| format!("filter {} references unknown clip {}", filter_id, target_id))?;
            remove_id(
                &mut state.tracks[track_index].clips[clip_index].filter_ids,
                filter_id,
            );
            Ok(())
        }
    }
}

fn sync_filter_target(
    state: &mut ProjectState,
    filter_id: &str,
    previous_target: Option<&TargetReference>,
    next_target: Option<&TargetReference>,
) -> Result<(), String> {
    if let Some(target) = previous_target {
        if !same_target_reference(target, next_target) {
            detach_filter_target(state, filter_id, target)?;
        }
    }
    if let Some(target) = next_target {
        attach_filter_target(state, filter_id, target)?;
    }
    Ok(())
}

fn attach_effect_target(
    state: &mut ProjectState,
    effect_id: &str,
    target: &TargetReference,
) -> Result<(), String> {
    let Some(target_id) = target.id.as_deref() else {
        return Ok(());
    };

    match target.target_type {
        TargetType::Project => Ok(()),
        TargetType::Track => {
            let index = state
                .find_track_index(target_id)
                .ok_or_else(|| format!("effect {} references unknown track {}", effect_id, target_id))?;
            push_unique_id(&mut state.tracks[index].effect_ids, effect_id);
            Ok(())
        }
        TargetType::Clip => {
            let (track_index, clip_index) = state
                .find_clip_position(target_id)
                .ok_or_else(|| format!("effect {} references unknown clip {}", effect_id, target_id))?;
            push_unique_id(
                &mut state.tracks[track_index].clips[clip_index].effect_ids,
                effect_id,
            );
            Ok(())
        }
    }
}

fn detach_effect_target(state: &mut ProjectState, effect_id: &str, target: &TargetReference) -> Result<(), String> {
    let Some(target_id) = target.id.as_deref() else {
        return Ok(());
    };

    match target.target_type {
        TargetType::Project => Ok(()),
        TargetType::Track => {
            let index = state
                .find_track_index(target_id)
                .ok_or_else(|| format!("effect {} references unknown track {}", effect_id, target_id))?;
            remove_id(&mut state.tracks[index].effect_ids, effect_id);
            Ok(())
        }
        TargetType::Clip => {
            let (track_index, clip_index) = state
                .find_clip_position(target_id)
                .ok_or_else(|| format!("effect {} references unknown clip {}", effect_id, target_id))?;
            remove_id(
                &mut state.tracks[track_index].clips[clip_index].effect_ids,
                effect_id,
            );
            Ok(())
        }
    }
}

fn sync_effect_target(
    state: &mut ProjectState,
    effect_id: &str,
    previous_target: Option<&TargetReference>,
    next_target: Option<&TargetReference>,
) -> Result<(), String> {
    if let Some(target) = previous_target {
        if !same_target_reference(target, next_target) {
            detach_effect_target(state, effect_id, target)?;
        }
    }
    if let Some(target) = next_target {
        attach_effect_target(state, effect_id, target)?;
    }
    Ok(())
}

fn sync_subtitle_clip(
    state: &mut ProjectState,
    subtitle_id: &str,
    previous_clip_id: Option<&str>,
    next_clip_id: Option<&str>,
) -> Result<(), String> {
    if let Some(clip_id) = previous_clip_id {
        if Some(clip_id) != next_clip_id {
            let (track_index, clip_index) = state
                .find_clip_position(clip_id)
                .ok_or_else(|| format!("subtitle {} references unknown clip {}", subtitle_id, clip_id))?;
            remove_id(
                &mut state.tracks[track_index].clips[clip_index].subtitle_ids,
                subtitle_id,
            );
        }
    }

    if let Some(clip_id) = next_clip_id {
        let (track_index, clip_index) = state
            .find_clip_position(clip_id)
            .ok_or_else(|| format!("subtitle {} references unknown clip {}", subtitle_id, clip_id))?;
        push_unique_id(
            &mut state.tracks[track_index].clips[clip_index].subtitle_ids,
            subtitle_id,
        );
    }

    Ok(())
}

fn ranges_overlap(a: &TimeRange, b: &TimeRange) -> bool {
    a.start_ms < b.end_ms && b.start_ms < a.end_ms
}

fn validate_track_for_insert(state: &ProjectState, track: &Track) -> Result<(), String> {
    if track.id.trim().is_empty() {
        return Err("track id must not be empty".into());
    }
    if track.name.trim().is_empty() {
        return Err(format!("track {} name must not be empty", track.id));
    }
    if state.tracks.iter().any(|existing| existing.id == track.id) {
        return Err(format!("track {} already exists", track.id));
    }
    for clip in &track.clips {
        if state.clip(&clip.id).is_some() {
            return Err(format!("clip {} already exists in project", clip.id));
        }
        validate_clip_shape(clip)?;
    }
    for (index, left) in track.clips.iter().enumerate() {
        for right in track.clips.iter().skip(index + 1) {
            if ranges_overlap(&left.timeline_range, &right.timeline_range) {
                return Err(format!(
                    "track {} contains overlapping clips {} and {}",
                    track.id, left.id, right.id
                ));
            }
        }
    }
    Ok(())
}

fn validate_clip_placement(
    state: &ProjectState,
    track_id: &str,
    candidate: &Clip,
    excluding_clip_id: Option<&str>,
) -> Result<(), String> {
    validate_clip_shape(candidate)?;
    let track = state
        .track(track_id)
        .ok_or_else(|| format!("track {} does not exist", track_id))?;
    if let Some(existing) = state.clip(&candidate.id) {
        let is_same_clip = excluding_clip_id.is_some_and(|clip_id| clip_id == candidate.id);
        if !is_same_clip && existing.id == candidate.id {
            return Err(format!("clip {} already exists in project", candidate.id));
        }
    }
    for clip in &track.clips {
        if excluding_clip_id.is_some_and(|clip_id| clip_id == clip.id) {
            continue;
        }
        if ranges_overlap(&clip.timeline_range, &candidate.timeline_range) {
            return Err(format!(
                "clip {} overlaps with {} on track {}",
                candidate.id, clip.id, track_id
            ));
        }
    }
    Ok(())
}

fn insert_clip_sorted(track: &mut Track, clip: Clip) {
    let index = track
        .clips
        .iter()
        .position(|existing| existing.timeline_range.start_ms > clip.timeline_range.start_ms)
        .unwrap_or(track.clips.len());
    track.clips.insert(index, clip);
}

fn shift_time_value(value: u64, delta_ms: i64) -> Result<u64, String> {
    let shifted = value as i128 + delta_ms as i128;
    if shifted < 0 || shifted > u64::MAX as i128 {
        return Err(format!("time shift overflow for value {} and delta {}", value, delta_ms));
    }
    Ok(shifted as u64)
}

fn shift_time_range(range: &TimeRange, delta_ms: i64) -> Result<TimeRange, String> {
    TimeRange::new(
        shift_time_value(range.start_ms, delta_ms)?,
        shift_time_value(range.end_ms, delta_ms)?,
    )
}

fn validate_replacement_clip_set(
    state: &ProjectState,
    track_id: &str,
    clips: &[Clip],
) -> Result<(), String> {
    if state.track(track_id).is_none() {
        return Err(format!("track {} does not exist", track_id));
    }

    for (index, left) in clips.iter().enumerate() {
        validate_clip_shape(left)?;
        for right in clips.iter().skip(index + 1) {
            if left.id == right.id {
                return Err(format!("duplicate clip id {} in replacement set", left.id));
            }
            if ranges_overlap(&left.timeline_range, &right.timeline_range) {
                return Err(format!(
                    "replacement clips {} and {} overlap on track {}",
                    left.id, right.id, track_id
                ));
            }
        }
    }

    for clip in clips {
        for other_track in state.tracks.iter().filter(|track| track.id != track_id) {
            if other_track.clips.iter().any(|other| other.id == clip.id) {
                return Err(format!(
                    "clip {} already exists on another track {}",
                    clip.id, other_track.id
                ));
            }
        }
    }

    Ok(())
}

fn compute_inverse_event(state: &ProjectState, event: &ProjectEvent) -> Result<ProjectEvent, String> {
    match event {
        ProjectEvent::ProjectRenamed { .. } => Ok(ProjectEvent::ProjectRenamed {
            new_name: state.name.clone(),
        }),
        ProjectEvent::SelectionSet { .. } | ProjectEvent::SelectionCleared => {
            Ok(current_selection_event(state))
        }
        ProjectEvent::PlayheadSet { .. } => Ok(ProjectEvent::PlayheadSet {
            playhead_ms: state.editor.playhead_ms,
        }),
        ProjectEvent::ViewportSet { .. } => Ok(ProjectEvent::ViewportSet {
            viewport: state.editor.viewport.clone(),
        }),
        ProjectEvent::AssetUpserted { asset } => {
            if let Some(existing) = state.asset(&asset.id) {
                Ok(ProjectEvent::AssetUpserted {
                    asset: existing.clone(),
                })
            } else {
                Ok(ProjectEvent::AssetRemoved {
                    asset_id: asset.id.clone(),
                })
            }
        }
        ProjectEvent::AssetRemoved { asset_id } => {
            let asset = state
                .asset(asset_id)
                .cloned()
                .ok_or_else(|| format!("asset {} does not exist", asset_id))?;
            Ok(ProjectEvent::AssetUpserted { asset })
        }
        ProjectEvent::SubtitleStyleUpserted { style } => {
            if let Some(existing) = state.subtitle_style(&style.id) {
                Ok(ProjectEvent::SubtitleStyleUpserted {
                    style: existing.clone(),
                })
            } else {
                Ok(ProjectEvent::SubtitleStyleRemoved {
                    style_id: style.id.clone(),
                })
            }
        }
        ProjectEvent::SubtitleStyleRemoved { style_id } => {
            let style = state
                .subtitle_style(style_id)
                .cloned()
                .ok_or_else(|| format!("subtitle style {} does not exist", style_id))?;
            Ok(ProjectEvent::SubtitleStyleUpserted { style })
        }
        ProjectEvent::SubtitleUpserted { subtitle } => {
            if let Some(existing) = state.subtitle(&subtitle.id) {
                Ok(ProjectEvent::SubtitleUpserted {
                    subtitle: existing.clone(),
                })
            } else {
                Ok(ProjectEvent::SubtitleRemoved {
                    subtitle_id: subtitle.id.clone(),
                })
            }
        }
        ProjectEvent::SubtitleRemoved { subtitle_id } => {
            let subtitle = state
                .subtitle(subtitle_id)
                .cloned()
                .ok_or_else(|| format!("subtitle {} does not exist", subtitle_id))?;
            Ok(ProjectEvent::SubtitleUpserted { subtitle })
        }
        ProjectEvent::FilterUpserted { filter } => {
            if let Some(existing) = state.filter(&filter.id) {
                Ok(ProjectEvent::FilterUpserted {
                    filter: existing.clone(),
                })
            } else {
                Ok(ProjectEvent::FilterRemoved {
                    filter_id: filter.id.clone(),
                })
            }
        }
        ProjectEvent::FilterRemoved { filter_id } => {
            let filter = state
                .filter(filter_id)
                .cloned()
                .ok_or_else(|| format!("filter {} does not exist", filter_id))?;
            Ok(ProjectEvent::FilterUpserted { filter })
        }
        ProjectEvent::EffectUpserted { effect } => {
            if let Some(existing) = state.effect(&effect.id) {
                Ok(ProjectEvent::EffectUpserted {
                    effect: existing.clone(),
                })
            } else {
                Ok(ProjectEvent::EffectRemoved {
                    effect_id: effect.id.clone(),
                })
            }
        }
        ProjectEvent::EffectRemoved { effect_id } => {
            let effect = state
                .effect(effect_id)
                .cloned()
                .ok_or_else(|| format!("effect {} does not exist", effect_id))?;
            Ok(ProjectEvent::EffectUpserted { effect })
        }
        ProjectEvent::ExportPresetUpserted { preset } => {
            if let Some(existing) = state.export_preset(&preset.id) {
                Ok(ProjectEvent::ExportPresetUpserted {
                    preset: existing.clone(),
                })
            } else {
                Ok(ProjectEvent::ExportPresetRemoved {
                    preset_id: preset.id.clone(),
                })
            }
        }
        ProjectEvent::ExportPresetRemoved { preset_id } => {
            let preset = state
                .export_preset(preset_id)
                .cloned()
                .ok_or_else(|| format!("export preset {} does not exist", preset_id))?;
            Ok(ProjectEvent::ExportPresetUpserted { preset })
        }
        ProjectEvent::ProjectPresetsSet { .. } => Ok(ProjectEvent::ProjectPresetsSet {
            presets: state.presets.clone(),
        }),
        ProjectEvent::TrackAdded { track, .. } => Ok(ProjectEvent::TrackRemoved {
            track_id: track.id.clone(),
        }),
        ProjectEvent::TrackRemoved { track_id } => {
            let index = state
                .find_track_index(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            Ok(ProjectEvent::TrackAdded {
                track: state.tracks[index].clone(),
                index,
            })
        }
        ProjectEvent::TrackRenamed { track_id, .. } => {
            let track = state
                .track(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            Ok(ProjectEvent::TrackRenamed {
                track_id: track_id.clone(),
                new_name: track.name.clone(),
            })
        }
        ProjectEvent::ClipInserted { track_id, clip } => Ok(ProjectEvent::ClipRemoved {
            track_id: track_id.clone(),
            clip_id: clip.id.clone(),
        }),
        ProjectEvent::ClipRemoved { track_id, clip_id } => {
            let track = state
                .track(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            let clip = track
                .clips
                .iter()
                .find(|clip| clip.id == *clip_id)
                .cloned()
                .ok_or_else(|| format!("clip {} does not exist on track {}", clip_id, track_id))?;
            Ok(ProjectEvent::ClipInserted {
                track_id: track_id.clone(),
                clip,
            })
        }
        ProjectEvent::TrackClipsReplaced { track_id, .. } => {
            let track = state
                .track(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            Ok(ProjectEvent::TrackClipsReplaced {
                track_id: track_id.clone(),
                clips: track.clips.clone(),
            })
        }
        ProjectEvent::ClipMoved { clip_id, .. } => {
            let (track_index, clip_index) = state
                .find_clip_position(clip_id)
                .ok_or_else(|| format!("clip {} does not exist", clip_id))?;
            Ok(ProjectEvent::ClipMoved {
                clip_id: clip_id.clone(),
                to_track_id: state.tracks[track_index].id.clone(),
                new_timeline_range: state.tracks[track_index].clips[clip_index]
                    .timeline_range
                    .clone(),
            })
        }
        ProjectEvent::ClipTrimmed { clip_id, .. } => {
            let clip = state
                .clip(clip_id)
                .cloned()
                .ok_or_else(|| format!("clip {} does not exist", clip_id))?;
            Ok(ProjectEvent::ClipTrimmed {
                clip_id: clip_id.clone(),
                new_source_range: clip.source_range,
                new_timeline_range: clip.timeline_range,
            })
        }
    }
}

fn apply_event_to_state(state: &mut ProjectState, event: &ProjectEvent) -> Result<(), String> {
    match event {
        ProjectEvent::ProjectRenamed { new_name } => {
            if new_name.trim().is_empty() {
                return Err("project name must not be empty".into());
            }
            state.name = new_name.clone();
            Ok(())
        }
        ProjectEvent::SelectionSet { clip_ids, track_id } => {
            state.editor.selection = normalize_selection(state, clip_ids, track_id)?;
            Ok(())
        }
        ProjectEvent::SelectionCleared => {
            state.editor.selection = EditorSelection::default();
            Ok(())
        }
        ProjectEvent::PlayheadSet { playhead_ms } => {
            state.editor.playhead_ms = *playhead_ms;
            Ok(())
        }
        ProjectEvent::ViewportSet { viewport } => {
            validate_viewport(viewport)?;
            state.editor.viewport = viewport.clone();
            Ok(())
        }
        ProjectEvent::AssetUpserted { asset } => apply_project_state_update(state, |candidate| {
            validate_asset_shape(asset)?;
            if let Some(index) = candidate.assets.iter().position(|existing| existing.id == asset.id) {
                candidate.assets[index] = asset.clone();
            } else {
                candidate.assets.push(asset.clone());
            }
            Ok(())
        }),
        ProjectEvent::AssetRemoved { asset_id } => apply_project_state_update(state, |candidate| {
            let index = candidate
                .assets
                .iter()
                .position(|asset| asset.id == *asset_id)
                .ok_or_else(|| format!("asset {} does not exist", asset_id))?;
            if candidate
                .tracks
                .iter()
                .flat_map(|track| track.clips.iter())
                .any(|clip| clip.asset_id == *asset_id)
            {
                return Err(format!("asset {} is still referenced by timeline clips", asset_id));
            }
            if candidate
                .subtitles
                .iter()
                .any(|subtitle| subtitle.asset_id.as_deref() == Some(asset_id.as_str()))
            {
                return Err(format!("asset {} is still referenced by subtitles", asset_id));
            }
            candidate.assets.remove(index);
            Ok(())
        }),
        ProjectEvent::SubtitleStyleUpserted { style } => {
            apply_project_state_update(state, |candidate| {
                validate_subtitle_style_shape(style)?;
                if let Some(index) = candidate
                    .subtitle_styles
                    .iter()
                    .position(|existing| existing.id == style.id)
                {
                    candidate.subtitle_styles[index] = style.clone();
                } else {
                    candidate.subtitle_styles.push(style.clone());
                }
                Ok(())
            })
        }
        ProjectEvent::SubtitleStyleRemoved { style_id } => {
            apply_project_state_update(state, |candidate| {
                let index = candidate
                    .subtitle_styles
                    .iter()
                    .position(|style| style.id == *style_id)
                    .ok_or_else(|| format!("subtitle style {} does not exist", style_id))?;
                if candidate
                    .subtitles
                    .iter()
                    .any(|subtitle| subtitle.style_id.as_deref() == Some(style_id.as_str()))
                {
                    return Err(format!(
                        "subtitle style {} is still referenced by subtitles",
                        style_id
                    ));
                }
                if candidate.presets.default_subtitle_style_id.as_deref() == Some(style_id.as_str())
                    || candidate.presets.polished_subtitle_style_id.as_deref() == Some(style_id.as_str())
                {
                    return Err(format!(
                        "subtitle style {} is still referenced by project presets",
                        style_id
                    ));
                }
                candidate.subtitle_styles.remove(index);
                Ok(())
            })
        }
        ProjectEvent::SubtitleUpserted { subtitle } => {
            apply_project_state_update(state, |candidate| {
                validate_subtitle_shape(subtitle)?;
                let previous_clip_id = candidate
                    .subtitles
                    .iter()
                    .find(|existing| existing.id == subtitle.id)
                    .and_then(|existing| existing.clip_id.clone());
                if let Some(index) = candidate
                    .subtitles
                    .iter()
                    .position(|existing| existing.id == subtitle.id)
                {
                    candidate.subtitles[index] = subtitle.clone();
                } else {
                    candidate.subtitles.push(subtitle.clone());
                }
                sync_subtitle_clip(
                    candidate,
                    &subtitle.id,
                    previous_clip_id.as_deref(),
                    subtitle.clip_id.as_deref(),
                )?;
                Ok(())
            })
        }
        ProjectEvent::SubtitleRemoved { subtitle_id } => {
            apply_project_state_update(state, |candidate| {
                let index = candidate
                    .subtitles
                    .iter()
                    .position(|subtitle| subtitle.id == *subtitle_id)
                    .ok_or_else(|| format!("subtitle {} does not exist", subtitle_id))?;
                let subtitle = candidate.subtitles[index].clone();
                sync_subtitle_clip(candidate, subtitle_id, subtitle.clip_id.as_deref(), None)?;
                candidate.subtitles.remove(index);
                Ok(())
            })
        }
        ProjectEvent::FilterUpserted { filter } => apply_project_state_update(state, |candidate| {
            validate_filter_shape(filter)?;
            let previous_target = candidate
                .filters
                .iter()
                .find(|existing| existing.id == filter.id)
                .and_then(|existing| existing.target.clone());
            if let Some(index) = candidate.filters.iter().position(|existing| existing.id == filter.id) {
                candidate.filters[index] = filter.clone();
            } else {
                candidate.filters.push(filter.clone());
            }
            sync_filter_target(
                candidate,
                &filter.id,
                previous_target.as_ref(),
                filter.target.as_ref(),
            )?;
            Ok(())
        }),
        ProjectEvent::FilterRemoved { filter_id } => apply_project_state_update(state, |candidate| {
            let index = candidate
                .filters
                .iter()
                .position(|filter| filter.id == *filter_id)
                .ok_or_else(|| format!("filter {} does not exist", filter_id))?;
            let filter = candidate.filters[index].clone();
            sync_filter_target(candidate, filter_id, filter.target.as_ref(), None)?;
            candidate.filters.remove(index);
            Ok(())
        }),
        ProjectEvent::EffectUpserted { effect } => apply_project_state_update(state, |candidate| {
            validate_effect_shape(effect)?;
            let previous_target = candidate
                .effects
                .iter()
                .find(|existing| existing.id == effect.id)
                .and_then(|existing| existing.target.clone());
            if let Some(index) = candidate.effects.iter().position(|existing| existing.id == effect.id) {
                candidate.effects[index] = effect.clone();
            } else {
                candidate.effects.push(effect.clone());
            }
            sync_effect_target(
                candidate,
                &effect.id,
                previous_target.as_ref(),
                effect.target.as_ref(),
            )?;
            Ok(())
        }),
        ProjectEvent::EffectRemoved { effect_id } => apply_project_state_update(state, |candidate| {
            let index = candidate
                .effects
                .iter()
                .position(|effect| effect.id == *effect_id)
                .ok_or_else(|| format!("effect {} does not exist", effect_id))?;
            let effect = candidate.effects[index].clone();
            sync_effect_target(candidate, effect_id, effect.target.as_ref(), None)?;
            candidate.effects.remove(index);
            Ok(())
        }),
        ProjectEvent::ExportPresetUpserted { preset } => {
            apply_project_state_update(state, |candidate| {
                validate_export_preset_shape(preset)?;
                if let Some(index) = candidate
                    .export_presets
                    .iter()
                    .position(|existing| existing.id == preset.id)
                {
                    candidate.export_presets[index] = preset.clone();
                } else {
                    candidate.export_presets.push(preset.clone());
                }
                Ok(())
            })
        }
        ProjectEvent::ExportPresetRemoved { preset_id } => {
            apply_project_state_update(state, |candidate| {
                let index = candidate
                    .export_presets
                    .iter()
                    .position(|preset| preset.id == *preset_id)
                    .ok_or_else(|| format!("export preset {} does not exist", preset_id))?;
                if candidate.presets.export_profile.as_deref() == Some(preset_id.as_str()) {
                    return Err(format!(
                        "export preset {} is still referenced by project presets",
                        preset_id
                    ));
                }
                candidate.export_presets.remove(index);
                Ok(())
            })
        }
        ProjectEvent::ProjectPresetsSet { presets } => apply_project_state_update(state, |candidate| {
            candidate.presets = presets.clone();
            Ok(())
        }),
        ProjectEvent::TrackAdded { track, index } => {
            validate_track_for_insert(state, track)?;
            if *index > state.tracks.len() {
                return Err(format!(
                    "track index {} is out of bounds for project with {} tracks",
                    index,
                    state.tracks.len()
                ));
            }
            let mut track = track.clone();
            track
                .clips
                .sort_by_key(|clip| clip.timeline_range.start_ms);
            state.tracks.insert(*index, track);
            Ok(())
        }
        ProjectEvent::TrackRemoved { track_id } => {
            let index = state
                .find_track_index(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            state.tracks.remove(index);
            Ok(())
        }
        ProjectEvent::TrackRenamed { track_id, new_name } => {
            if new_name.trim().is_empty() {
                return Err("track name must not be empty".into());
            }
            let index = state
                .find_track_index(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            state.tracks[index].name = new_name.clone();
            Ok(())
        }
        ProjectEvent::ClipInserted { track_id, clip } => {
            validate_clip_placement(state, track_id, clip, None)?;
            let index = state
                .find_track_index(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            insert_clip_sorted(&mut state.tracks[index], clip.clone());
            Ok(())
        }
        ProjectEvent::ClipRemoved { track_id, clip_id } => {
            let track_index = state
                .find_track_index(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            let clip_index = state.tracks[track_index]
                .clips
                .iter()
                .position(|clip| clip.id == *clip_id)
                .ok_or_else(|| format!("clip {} does not exist on track {}", clip_id, track_id))?;
            state.tracks[track_index].clips.remove(clip_index);
            Ok(())
        }
        ProjectEvent::TrackClipsReplaced { track_id, clips } => {
            validate_replacement_clip_set(state, track_id, clips)?;
            let track_index = state
                .find_track_index(track_id)
                .ok_or_else(|| format!("track {} does not exist", track_id))?;
            let mut sorted_clips = clips.clone();
            sorted_clips.sort_by_key(|clip| clip.timeline_range.start_ms);
            state.tracks[track_index].clips = sorted_clips;
            Ok(())
        }
        ProjectEvent::ClipMoved {
            clip_id,
            to_track_id,
            new_timeline_range,
        } => {
            let (from_track_index, clip_index) = state
                .find_clip_position(clip_id)
                .ok_or_else(|| format!("clip {} does not exist", clip_id))?;
            let mut updated_clip = state.tracks[from_track_index].clips[clip_index].clone();
            updated_clip.timeline_range = new_timeline_range.clone();
            validate_clip_placement(state, to_track_id, &updated_clip, Some(clip_id.as_str()))?;

            let clip = state.tracks[from_track_index].clips.remove(clip_index);
            let mut moved_clip = clip;
            moved_clip.timeline_range = new_timeline_range.clone();

            let to_track_index = state
                .find_track_index(to_track_id)
                .ok_or_else(|| format!("track {} does not exist", to_track_id))?;
            insert_clip_sorted(&mut state.tracks[to_track_index], moved_clip);
            Ok(())
        }
        ProjectEvent::ClipTrimmed {
            clip_id,
            new_source_range,
            new_timeline_range,
        } => {
            let (track_index, clip_index) = state
                .find_clip_position(clip_id)
                .ok_or_else(|| format!("clip {} does not exist", clip_id))?;
            let track_id = state.tracks[track_index].id.clone();
            let mut updated_clip = state.tracks[track_index].clips[clip_index].clone();
            updated_clip.source_range = new_source_range.clone();
            updated_clip.timeline_range = new_timeline_range.clone();
            validate_clip_placement(state, &track_id, &updated_clip, Some(clip_id.as_str()))?;

            state.tracks[track_index].clips.remove(clip_index);
            insert_clip_sorted(&mut state.tracks[track_index], updated_clip);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;

    fn range(start_ms: u64, end_ms: u64) -> TimeRange {
        TimeRange::new(start_ms, end_ms).expect("time range should be valid")
    }

    fn clip(
        id: &str,
        asset_id: &str,
        start_ms: u64,
        end_ms: u64,
        source_start_ms: u64,
        source_end_ms: u64,
    ) -> Clip {
        Clip::new(
            id,
            asset_id,
            id,
            range(start_ms, end_ms),
            range(source_start_ms, source_end_ms),
        )
        .expect("clip should be valid")
    }

    #[test]
    fn project_history_supports_undo_and_redo_for_move_and_trim() {
        let mut history = ProjectHistory::new(ProjectState::new("project-001", "Demo", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-b", "Secondary", TrackKind::Video),
            index: 1,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 4_000, 10_000, 14_000),
        }).unwrap();
        history.apply(ProjectEvent::ClipMoved {
            clip_id: "clip-1".into(),
            to_track_id: "track-b".into(),
            new_timeline_range: range(5_000, 9_000),
        }).unwrap();
        history.apply(ProjectEvent::ClipTrimmed {
            clip_id: "clip-1".into(),
            new_source_range: range(11_000, 14_000),
            new_timeline_range: range(5_000, 8_000),
        }).unwrap();

        let moved_clip = history.state.clip("clip-1").unwrap();
        assert_eq!(moved_clip.timeline_range, range(5_000, 8_000));
        assert_eq!(moved_clip.source_range, range(11_000, 14_000));

        history.undo().unwrap();
        let moved_clip = history.state.clip("clip-1").unwrap();
        assert_eq!(moved_clip.timeline_range, range(5_000, 9_000));
        assert_eq!(moved_clip.source_range, range(10_000, 14_000));

        history.undo().unwrap();
        let restored_clip = history.state.clip("clip-1").unwrap();
        assert_eq!(restored_clip.timeline_range, range(0, 4_000));
        assert_eq!(history.state.track("track-a").unwrap().clips.len(), 1);
        assert!(history.state.track("track-b").unwrap().clips.is_empty());

        history.redo().unwrap();
        history.redo().unwrap();
        let redone_clip = history.state.clip("clip-1").unwrap();
        assert_eq!(redone_clip.timeline_range, range(5_000, 8_000));
        assert_eq!(redone_clip.source_range, range(11_000, 14_000));
    }

    #[test]
    fn split_clip_creates_adjacent_left_and_right_clips() {
        let mut history = ProjectHistory::new(ProjectState::new("project-004", "Split Demo", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 4_000, 10_000, 14_000),
        }).unwrap();

        history.split_clip("track-a", "clip-1", 2_500, "clip-1b").unwrap();

        let track = history.state.track("track-a").unwrap();
        assert_eq!(track.clips.len(), 2);
        assert_eq!(track.clips[0].id, "clip-1");
        assert_eq!(track.clips[0].timeline_range, range(0, 2_500));
        assert_eq!(track.clips[0].source_range, range(10_000, 12_500));
        assert_eq!(track.clips[1].id, "clip-1b");
        assert_eq!(track.clips[1].timeline_range, range(2_500, 4_000));
        assert_eq!(track.clips[1].source_range, range(12_500, 14_000));
    }

    #[test]
    fn ripple_move_shifts_clip_and_following_clips_together() {
        let mut history = ProjectHistory::new(ProjectState::new("project-005", "Ripple Demo", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 2_000, 10_000, 12_000),
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-2", "asset-2", 3_000, 5_000, 20_000, 22_000),
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-3", "asset-3", 6_000, 8_000, 30_000, 32_000),
        }).unwrap();

        let delta = history.ripple_move_clip("clip-2", 1_500).unwrap();
        assert_eq!(delta, 1_500);

        let track = history.state.track("track-a").unwrap();
        assert_eq!(track.clips[1].timeline_range, range(4_500, 6_500));
        assert_eq!(track.clips[2].timeline_range, range(7_500, 9_500));
    }

    #[test]
    fn close_gap_before_clip_pulls_clip_and_following_left() {
        let mut history = ProjectHistory::new(ProjectState::new("project-006", "Gap Demo", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 2_000, 10_000, 12_000),
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-2", "asset-2", 5_000, 7_000, 20_000, 22_000),
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-3", "asset-3", 8_000, 10_000, 30_000, 32_000),
        }).unwrap();

        let delta = history.close_gap_before_clip("clip-2").unwrap();
        assert_eq!(delta, -3_000);

        let track = history.state.track("track-a").unwrap();
        assert_eq!(track.clips[1].timeline_range, range(2_000, 4_000));
        assert_eq!(track.clips[2].timeline_range, range(5_000, 7_000));
    }

    #[test]
    fn ui_events_support_undo_and_redo() {
        let mut history = ProjectHistory::new(ProjectState::new("project-007", "UI Demo", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 4_000, 10_000, 14_000),
        }).unwrap();

        history.set_selection(vec!["clip-1".into()], Some("track-a".into())).unwrap();
        history.set_playhead(1_250).unwrap();
        history.set_viewport(EditorViewport {
            scroll_x_px: 320,
            scroll_y_px: 140,
            zoom_percent: 175,
        }).unwrap();

        assert_eq!(history.state.editor.selection.clip_ids, vec!["clip-1"]);
        assert_eq!(history.state.editor.playhead_ms, 1_250);
        assert_eq!(history.state.editor.viewport.zoom_percent, 175);

        history.undo().unwrap();
        assert_eq!(history.state.editor.viewport, EditorViewport::default());
        history.undo().unwrap();
        assert_eq!(history.state.editor.playhead_ms, 0);
        history.undo().unwrap();
        assert!(history.state.editor.selection.clip_ids.is_empty());

        history.redo().unwrap();
        history.redo().unwrap();
        history.redo().unwrap();
        assert_eq!(history.state.editor.selection.clip_ids, vec!["clip-1"]);
        assert_eq!(history.state.editor.playhead_ms, 1_250);
        assert_eq!(history.state.editor.viewport.zoom_percent, 175);
    }

    #[test]
    fn registry_events_sync_clip_membership_and_inverse_events() {
        let mut history = ProjectHistory::new(ProjectState::new("project-009", "Registry Demo", "16:9"));
        history.apply(ProjectEvent::AssetUpserted {
            asset: Asset {
                id: "asset-1".into(),
                asset_type: AssetType::Video,
                label: "Source".into(),
                source: AssetSource {
                    path: "media/source.mp4".into(),
                    checksum: None,
                },
                media: AssetMedia::default(),
                tags: vec![],
            },
        }).unwrap();
        history.apply(ProjectEvent::SubtitleStyleUpserted {
            style: SubtitleStyle {
                id: "style-1".into(),
                label: "Clean".into(),
                font_family: "Aptos".into(),
                font_size_px: 40,
                placement: "bottom".into(),
                fill_color: "#FFFFFF".into(),
                stroke_color: "#111111".into(),
                background_color: None,
                max_lines: 2,
                animation_preset: Some("subtitle_pop".into()),
            },
        }).unwrap();
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 4_000, 10_000, 14_000),
        }).unwrap();

        history.apply(ProjectEvent::FilterUpserted {
            filter: Filter {
                id: "filter-1".into(),
                kind: "cinematic_grade".into(),
                label: "Grade".into(),
                category: Some("look".into()),
                target: Some(TargetReference {
                    target_type: TargetType::Clip,
                    id: Some("clip-1".into()),
                }),
                enabled: true,
                parameters: BTreeMap::from([("intensity".into(), json!(0.8))]),
                keyframes: vec![],
                render_backend: Some("ffmpeg".into()),
            },
        }).unwrap();
        history.apply(ProjectEvent::EffectUpserted {
            effect: Effect {
                id: "effect-1".into(),
                kind: "glitch".into(),
                label: "Glitch".into(),
                category: Some("stylized".into()),
                target: Some(TargetReference {
                    target_type: TargetType::Clip,
                    id: Some("clip-1".into()),
                }),
                enabled: true,
                parameters: BTreeMap::from([("intensity".into(), json!(0.4))]),
                keyframes: vec![],
                render_backend: Some("compositor".into()),
            },
        }).unwrap();
        history.apply(ProjectEvent::SubtitleUpserted {
            subtitle: Subtitle {
                id: "sub-1".into(),
                text: "Hello there".into(),
                start_ms: 0,
                end_ms: 1_500,
                style_id: Some("style-1".into()),
                language: Some("en".into()),
                speaker: None,
                asset_id: Some("asset-1".into()),
                track_id: Some("track-a".into()),
                clip_id: Some("clip-1".into()),
                words: vec![],
            },
        }).unwrap();

        let clip = history.state.clip("clip-1").unwrap();
        assert_eq!(clip.filter_ids, vec!["filter-1"]);
        assert_eq!(clip.effect_ids, vec!["effect-1"]);
        assert_eq!(clip.subtitle_ids, vec!["sub-1"]);

        history.apply(ProjectEvent::SubtitleRemoved {
            subtitle_id: "sub-1".into(),
        }).unwrap();
        history.apply(ProjectEvent::EffectRemoved {
            effect_id: "effect-1".into(),
        }).unwrap();
        history.apply(ProjectEvent::FilterRemoved {
            filter_id: "filter-1".into(),
        }).unwrap();

        let clip = history.state.clip("clip-1").unwrap();
        assert!(clip.subtitle_ids.is_empty());
        assert!(clip.effect_ids.is_empty());
        assert!(clip.filter_ids.is_empty());

        history.undo().unwrap();
        history.undo().unwrap();
        history.undo().unwrap();

        let clip = history.state.clip("clip-1").unwrap();
        assert_eq!(clip.filter_ids, vec!["filter-1"]);
        assert_eq!(clip.effect_ids, vec!["effect-1"]);
        assert_eq!(clip.subtitle_ids, vec!["sub-1"]);
    }

    #[test]
    fn rejects_invalid_selection_targets() {
        let mut history = ProjectHistory::new(ProjectState::new("project-008", "UI Validation", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();

        let error = history
            .set_selection(vec!["missing-clip".into()], Some("track-a".into()))
            .unwrap_err();
        assert!(error.contains("does not exist"));
    }

    #[test]
    fn project_state_can_replay_event_log() {
        let replayed = ProjectState::new("project-002", "Replay Demo", "9:16")
            .replay(&[
                ProjectEvent::TrackAdded {
                    track: Track::new("v1", "Video 1", TrackKind::Video),
                    index: 0,
                },
                ProjectEvent::ClipInserted {
                    track_id: "v1".into(),
                    clip: clip("clip-r1", "asset-r1", 0, 3_000, 20_000, 23_000),
                },
            ])
            .unwrap();
        assert_eq!(replayed.tracks.len(), 1);
        assert_eq!(replayed.tracks[0].clips[0].id, "clip-r1");
    }

    #[test]
    fn rejects_overlapping_clips_on_same_track() {
        let mut history = ProjectHistory::new(ProjectState::new("project-003", "Overlap Demo", "16:9"));
        history.apply(ProjectEvent::TrackAdded {
            track: Track::new("track-a", "Primary", TrackKind::Video),
            index: 0,
        }).unwrap();
        history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-1", "asset-1", 0, 4_000, 10_000, 14_000),
        }).unwrap();

        let error = history.apply(ProjectEvent::ClipInserted {
            track_id: "track-a".into(),
            clip: clip("clip-2", "asset-2", 3_500, 6_000, 30_000, 32_500),
        }).unwrap_err();
        assert!(error.contains("overlaps"));
    }

    #[test]
    fn timeline_demo_builds_real_history() {
        let history = timeline_demo_history().unwrap();
        assert!(history.can_undo());
        assert_eq!(history.state.tracks.len(), 2);
        assert_eq!(history.state.track("v2").unwrap().clips.len(), 1);
    }
}
