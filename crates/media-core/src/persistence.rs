use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{PersistedHistoryEntry, ProjectEvent, ProjectHistory, ProjectState};

pub const PROJECT_DOCUMENT_VERSION: u32 = 2;
const LEGACY_PROJECT_DOCUMENT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub version: u32,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub checkpoint_index: u64,
    pub state: ProjectState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectEventLog {
    pub version: u32,
    pub project_id: String,
    #[serde(default)]
    pub events: Vec<ProjectEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectHistoryLog {
    pub version: u32,
    pub project_id: String,
    #[serde(default)]
    pub start_index: u64,
    #[serde(default)]
    pub past: Vec<PersistedHistoryEntry>,
    #[serde(default)]
    pub future: Vec<PersistedHistoryEntry>,
}

enum ParsedHistoryLog {
    Legacy(ProjectEventLog),
    Current(ProjectHistoryLog),
}

pub fn snapshot_from_state(state: &ProjectState) -> ProjectSnapshot {
    ProjectSnapshot {
        version: PROJECT_DOCUMENT_VERSION,
        project_id: state.id.clone(),
        checkpoint_index: 0,
        state: state.clone(),
    }
}

pub fn event_log_from_events(project_id: impl Into<String>, events: &[ProjectEvent]) -> ProjectEventLog {
    ProjectEventLog {
        version: LEGACY_PROJECT_DOCUMENT_VERSION,
        project_id: project_id.into(),
        events: events.to_vec(),
    }
}

pub fn history_log_from_history(history: &ProjectHistory, start_index: u64) -> ProjectHistoryLog {
    ProjectHistoryLog {
        version: PROJECT_DOCUMENT_VERSION,
        project_id: history.state.id.clone(),
        start_index,
        past: history.persisted_past(),
        future: history.persisted_future(),
    }
}

pub fn save_snapshot_to_path(
    state: &ProjectState,
    path: impl AsRef<Path>,
) -> Result<(), String> {
    state.validate()?;
    let snapshot = snapshot_from_state(state);
    write_json(path, &snapshot)
}

pub fn load_snapshot_from_path(path: impl AsRef<Path>) -> Result<ProjectState, String> {
    Ok(load_snapshot_document_from_path(path)?.state)
}

pub fn save_event_log_to_path(
    project_id: impl Into<String>,
    events: &[ProjectEvent],
    path: impl AsRef<Path>,
) -> Result<(), String> {
    let event_log = event_log_from_events(project_id, events);
    write_json(path, &event_log)
}

pub fn load_event_log_from_path(path: impl AsRef<Path>) -> Result<ProjectEventLog, String> {
    let event_log: ProjectEventLog = read_json(path)?;
    ensure_supported_version(event_log.version)?;
    Ok(event_log)
}

pub fn save_history_to_paths(
    history: &ProjectHistory,
    snapshot_path: impl AsRef<Path>,
    history_log_path: impl AsRef<Path>,
) -> Result<(), String> {
    let checkpoint_state = history.checkpoint_state()?;
    let snapshot = ProjectSnapshot {
        version: PROJECT_DOCUMENT_VERSION,
        project_id: history.state.id.clone(),
        checkpoint_index: 0,
        state: checkpoint_state,
    };
    let history_log = history_log_from_history(history, 0);

    write_json(snapshot_path, &snapshot)?;
    write_json(history_log_path, &history_log)
}

pub fn load_history_from_paths(
    snapshot_path: impl AsRef<Path>,
    history_log_path: impl AsRef<Path>,
) -> Result<ProjectHistory, String> {
    let snapshot = load_snapshot_document_from_path(snapshot_path)?;
    match load_history_log_document(history_log_path)? {
        ParsedHistoryLog::Legacy(event_log) => {
            if snapshot.project_id != event_log.project_id {
                return Err(format!(
                    "snapshot project id {} does not match event log project id {}",
                    snapshot.project_id, event_log.project_id
                ));
            }

            let mut history = ProjectHistory::new(snapshot.state);
            for event in event_log.events {
                history.apply(event)?;
            }
            Ok(history)
        }
        ParsedHistoryLog::Current(history_log) => {
            if snapshot.project_id != history_log.project_id {
                return Err(format!(
                    "snapshot project id {} does not match history log project id {}",
                    snapshot.project_id, history_log.project_id
                ));
            }
            if snapshot.checkpoint_index != history_log.start_index {
                return Err(format!(
                    "snapshot checkpoint index {} does not match history log start index {}",
                    snapshot.checkpoint_index, history_log.start_index
                ));
            }

            ProjectHistory::from_persisted(snapshot.state, history_log.past, history_log.future)
        }
    }
}

pub fn load_state_from_snapshot_and_log(
    snapshot_path: impl AsRef<Path>,
    history_log_path: impl AsRef<Path>,
) -> Result<ProjectState, String> {
    Ok(load_history_from_paths(snapshot_path, history_log_path)?.state)
}

fn load_snapshot_document_from_path(path: impl AsRef<Path>) -> Result<ProjectSnapshot, String> {
    let mut snapshot: ProjectSnapshot = read_json(path)?;
    ensure_supported_version(snapshot.version)?;
    if snapshot.version == LEGACY_PROJECT_DOCUMENT_VERSION {
        snapshot.project_id = snapshot.state.id.clone();
        snapshot.checkpoint_index = 0;
    }
    if snapshot.project_id.is_empty() {
        snapshot.project_id = snapshot.state.id.clone();
    }
    if snapshot.project_id != snapshot.state.id {
        return Err(format!(
            "snapshot project id {} does not match state id {}",
            snapshot.project_id, snapshot.state.id
        ));
    }
    snapshot.state.validate()?;
    Ok(snapshot)
}

fn load_history_log_document(path: impl AsRef<Path>) -> Result<ParsedHistoryLog, String> {
    let value = read_json_value(&path)?;
    let version = value
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| format!("missing or invalid version in {}", path.as_ref().display()))?
        as u32;
    ensure_supported_version(version)?;

    if version == LEGACY_PROJECT_DOCUMENT_VERSION {
        let event_log: ProjectEventLog = serde_json::from_value(value)
            .map_err(|error| format!("failed to parse {}: {}", path.as_ref().display(), error))?;
        return Ok(ParsedHistoryLog::Legacy(event_log));
    }

    let history_log: ProjectHistoryLog = serde_json::from_value(value)
        .map_err(|error| format!("failed to parse {}: {}", path.as_ref().display(), error))?;
    Ok(ParsedHistoryLog::Current(history_log))
}

fn ensure_supported_version(version: u32) -> Result<(), String> {
    if version != PROJECT_DOCUMENT_VERSION && version != LEGACY_PROJECT_DOCUMENT_VERSION {
        return Err(format!("unsupported project document version {}", version));
    }
    Ok(())
}

fn write_json<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {}", parent.display(), error))?;
    }

    let json = serde_json::to_string_pretty(value)
        .map_err(|error| format!("failed to serialize JSON: {}", error))?;
    fs::write(path, json).map_err(|error| format!("failed to write {}: {}", path.display(), error))
}

fn read_json<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T, String> {
    let path = path.as_ref();
    let json = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str(&json)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn read_json_value(path: impl AsRef<Path>) -> Result<serde_json::Value, String> {
    let path = path.as_ref();
    let json = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str(&json)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Clip, EditorViewport, ProjectEvent, ProjectHistory, ProjectState, TimeRange, Track,
        TrackKind,
    };

    fn temp_file(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("cineweave-{}-{}.json", name, std::process::id()));
        path
    }

    fn range(start_ms: u64, end_ms: u64) -> TimeRange {
        TimeRange::new(start_ms, end_ms).expect("range should be valid")
    }

    fn clip(id: &str) -> Clip {
        Clip::new(id, "asset-1", id, range(0, 3_000), range(10_000, 13_000))
            .expect("clip should be valid")
    }

    #[test]
    fn history_documents_roundtrip_preserves_undo_and_redo() {
        let mut history = ProjectHistory::new(ProjectState::new("persist-1", "Persist Demo", "16:9"));
        history
            .apply(ProjectEvent::TrackAdded {
                track: Track::new("v1", "Video 1", TrackKind::Video),
                index: 0,
            })
            .unwrap();
        history
            .apply(ProjectEvent::ClipInserted {
                track_id: "v1".into(),
                clip: clip("clip-p1"),
            })
            .unwrap();
        history
            .set_selection(vec!["clip-p1".into()], Some("v1".into()))
            .unwrap();
        history.set_playhead(1_500).unwrap();
        history
            .set_viewport(EditorViewport {
                scroll_x_px: 240,
                scroll_y_px: 80,
                zoom_percent: 150,
            })
            .unwrap();
        history.undo().unwrap();
        history.undo().unwrap();

        let snapshot_path = temp_file("snapshot");
        let history_log_path = temp_file("history-log");

        save_history_to_paths(&history, &snapshot_path, &history_log_path).unwrap();

        let loaded_history = load_history_from_paths(&snapshot_path, &history_log_path).unwrap();
        let loaded_snapshot = load_snapshot_from_path(&snapshot_path).unwrap();

        assert_eq!(loaded_snapshot.id, "persist-1");
        assert!(loaded_snapshot.tracks.is_empty());
        assert_eq!(loaded_history.state.editor.selection.clip_ids, vec!["clip-p1"]);
        assert_eq!(loaded_history.state.editor.playhead_ms, 0);
        assert!(loaded_history.can_undo());
        assert!(loaded_history.can_redo());

        let mut rehydrated = loaded_history.clone();
        rehydrated.redo().unwrap();
        rehydrated.redo().unwrap();
        assert_eq!(rehydrated.state.editor.playhead_ms, 1_500);
        assert_eq!(rehydrated.state.editor.viewport.zoom_percent, 150);

        let _ = fs::remove_file(snapshot_path);
        let _ = fs::remove_file(history_log_path);
    }

    #[test]
    fn load_state_from_snapshot_and_log_replays_legacy_events() {
        let base_state = ProjectState::new("persist-2", "Replay Base", "9:16");
        let events = vec![
            ProjectEvent::TrackAdded {
                track: Track::new("v1", "Video 1", TrackKind::Video),
                index: 0,
            },
            ProjectEvent::PlayheadSet { playhead_ms: 900 },
            ProjectEvent::ViewportSet {
                viewport: EditorViewport {
                    scroll_x_px: 120,
                    scroll_y_px: 40,
                    zoom_percent: 125,
                },
            },
        ];

        let snapshot_path = temp_file("snapshot-replay");
        let event_log_path = temp_file("event-log-replay");

        save_snapshot_to_path(&base_state, &snapshot_path).unwrap();
        save_event_log_to_path("persist-2", &events, &event_log_path).unwrap();

        let loaded_state = load_state_from_snapshot_and_log(&snapshot_path, &event_log_path).unwrap();
        assert_eq!(loaded_state.tracks.len(), 1);
        assert_eq!(loaded_state.tracks[0].id, "v1");
        assert_eq!(loaded_state.editor.playhead_ms, 900);
        assert_eq!(loaded_state.editor.viewport.zoom_percent, 125);

        let _ = fs::remove_file(snapshot_path);
        let _ = fs::remove_file(event_log_path);
    }

    #[test]
    fn load_snapshot_supports_camel_case_schema_fields() {
        let snapshot_path = temp_file("snapshot-camel");
        let snapshot_json = r##"{
  "version": 1,
  "state": {
    "id": "persist-camel",
    "name": "Camel Case Snapshot",
    "aspectRatio": "16:9",
    "assets": [],
    "tracks": [
      {
        "id": "v1",
        "name": "Video 1",
        "kind": "video",
        "clips": [
          {
            "id": "clip-1",
            "assetId": "asset-1",
            "label": "Clip 1",
            "timelineRange": { "startMs": 0, "endMs": 2000 },
            "sourceRange": { "startMs": 10000, "endMs": 12000 },
            "filterIds": [],
            "effectIds": [],
            "subtitleIds": ["sub-1"]
          }
        ]
      }
    ],
    "subtitles": [
      {
        "id": "sub-1",
        "text": "Hello world",
        "startMs": 0,
        "endMs": 1800,
        "styleId": "clean_documentary",
        "trackId": "v1",
        "clipId": "clip-1",
        "words": [
          { "text": "Hello", "startMs": 0, "endMs": 800 }
        ]
      }
    ],
    "subtitleStyles": [
      {
        "id": "clean_documentary",
        "label": "Clean Documentary",
        "fontFamily": "Aptos",
        "fontSizePx": 44,
        "placement": "bottom",
        "fillColor": "#FFFFFF",
        "strokeColor": "#141414",
        "maxLines": 2
      }
    ],
    "filters": [],
    "effects": [],
    "exportPresets": [
      {
        "id": "social_master",
        "label": "Social Master",
        "container": "mp4",
        "aspectRatio": "16:9",
        "video": {
          "codec": "h264",
          "resolution": { "width": 1080, "height": 1920 },
          "frameRate": 30
        },
        "audio": {
          "codec": "aac",
          "sampleRate": 48000,
          "channels": 2
        }
      }
    ],
    "presets": {
      "defaultAspectRatio": "16:9",
      "defaultSubtitleStyleId": "clean_documentary",
      "exportProfile": "social_master"
    },
    "metadata": {
      "version": 1,
      "tags": ["schema"]
    }
  }
}"##;

        fs::write(&snapshot_path, snapshot_json).unwrap();

        let loaded_state = load_snapshot_from_path(&snapshot_path).unwrap();
        assert_eq!(loaded_state.aspect_ratio, "16:9");
        assert_eq!(loaded_state.tracks[0].kind, TrackKind::Video);
        assert_eq!(loaded_state.tracks[0].clips[0].subtitle_ids, vec!["sub-1"]);
        assert_eq!(loaded_state.subtitles[0].style_id.as_deref(), Some("clean_documentary"));
        assert_eq!(
            loaded_state.presets.export_profile.as_deref(),
            Some("social_master")
        );

        let _ = fs::remove_file(snapshot_path);
    }
}
