use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    load_history_from_paths, save_history_to_paths, Asset, Clip, EditorViewport, Effect,
    ExportPreset, Filter, ProjectEvent, ProjectHistory, ProjectPresets, ProjectState, Subtitle,
    SubtitleStyle, TimeRange, Track, TrackKind,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DocumentCommand {
    SetSelection {
        clip_ids: Vec<String>,
        track_id: Option<String>,
    },
    ClearSelection,
    SetPlayhead {
        playhead_ms: u64,
    },
    SetViewport {
        viewport: EditorViewport,
    },
    UpsertAsset {
        asset: Asset,
    },
    RemoveAsset {
        asset_id: String,
    },
    UpsertSubtitleStyle {
        style: SubtitleStyle,
    },
    RemoveSubtitleStyle {
        style_id: String,
    },
    UpsertSubtitle {
        subtitle: Subtitle,
    },
    RemoveSubtitle {
        subtitle_id: String,
    },
    UpsertFilter {
        filter: Filter,
    },
    RemoveFilter {
        filter_id: String,
    },
    UpsertEffect {
        effect: Effect,
    },
    RemoveEffect {
        effect_id: String,
    },
    UpsertExportPreset {
        preset: ExportPreset,
    },
    RemoveExportPreset {
        preset_id: String,
    },
    SetProjectPresets {
        presets: ProjectPresets,
    },
    AddTrack {
        track_id: String,
        name: String,
        kind: TrackKind,
        index: usize,
    },
    RenameTrack {
        track_id: String,
        new_name: String,
    },
    RemoveTrack {
        track_id: String,
    },
    InsertClip {
        track_id: String,
        clip: Clip,
    },
    RemoveClip {
        track_id: String,
        clip_id: String,
    },
    MoveClip {
        clip_id: String,
        to_track_id: String,
        new_timeline_range: TimeRange,
    },
    TrimClip {
        clip_id: String,
        new_source_range: TimeRange,
        new_timeline_range: TimeRange,
    },
    SplitClip {
        track_id: String,
        clip_id: String,
        split_at_ms: u64,
        right_clip_id: String,
    },
    RippleMoveClip {
        clip_id: String,
        delta_ms: i64,
    },
    CloseGapBeforeClip {
        clip_id: String,
    },
    Undo,
    Redo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AdapterRequest {
    CreateProjectDocument {
        project_id: String,
        name: String,
        aspect_ratio: String,
        snapshot_path: PathBuf,
        event_log_path: PathBuf,
    },
    GetDocumentState {
        snapshot_path: PathBuf,
        event_log_path: PathBuf,
    },
    ApplyCommands {
        snapshot_path: PathBuf,
        event_log_path: PathBuf,
        commands: Vec<DocumentCommand>,
        save: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterResponse {
    pub ok: bool,
    pub message: String,
    pub snapshot_path: Option<PathBuf>,
    pub event_log_path: Option<PathBuf>,
    pub emitted_events: Vec<ProjectEvent>,
    pub state: Option<ProjectState>,
}

pub fn execute_adapter_request(request: AdapterRequest) -> Result<AdapterResponse, String> {
    match request {
        AdapterRequest::CreateProjectDocument {
            project_id,
            name,
            aspect_ratio,
            snapshot_path,
            event_log_path,
        } => {
            let state = ProjectState::new(project_id.clone(), name, aspect_ratio);
            save_history_to_paths(&ProjectHistory::new(state.clone()), &snapshot_path, &event_log_path)?;
            Ok(AdapterResponse {
                ok: true,
                message: "project document created".into(),
                snapshot_path: Some(snapshot_path),
                event_log_path: Some(event_log_path),
                emitted_events: Vec::new(),
                state: Some(state),
            })
        }
        AdapterRequest::GetDocumentState {
            snapshot_path,
            event_log_path,
        } => {
            let state = load_history_from_documents(&snapshot_path, &event_log_path)?.state;
            Ok(AdapterResponse {
                ok: true,
                message: "document state loaded".into(),
                snapshot_path: Some(snapshot_path),
                event_log_path: Some(event_log_path),
                emitted_events: Vec::new(),
                state: Some(state),
            })
        }
        AdapterRequest::ApplyCommands {
            snapshot_path,
            event_log_path,
            commands,
            save,
        } => {
            let mut history = load_history_from_documents(&snapshot_path, &event_log_path)?;
            let emitted_events = apply_commands(&mut history, &commands)?;

            if save {
                save_history_to_paths(&history, &snapshot_path, &event_log_path)?;
            }

            Ok(AdapterResponse {
                ok: true,
                message: format!("applied {} command(s)", commands.len()),
                snapshot_path: Some(snapshot_path),
                event_log_path: Some(event_log_path),
                emitted_events,
                state: Some(history.state),
            })
        }
    }
}

fn load_history_from_documents(
    snapshot_path: impl AsRef<Path>,
    event_log_path: impl AsRef<Path>,
) -> Result<ProjectHistory, String> {
    load_history_from_paths(snapshot_path, event_log_path)
}

fn apply_commands(
    history: &mut ProjectHistory,
    commands: &[DocumentCommand],
) -> Result<Vec<ProjectEvent>, String> {
    let mut emitted_events = Vec::new();

    for command in commands {
        let previous_len = history.applied_events.len();
        match command {
            DocumentCommand::SetSelection { clip_ids, track_id } => {
                history.set_selection(clip_ids.clone(), track_id.clone())?
            }
            DocumentCommand::ClearSelection => history.clear_selection()?,
            DocumentCommand::SetPlayhead { playhead_ms } => history.set_playhead(*playhead_ms)?,
            DocumentCommand::SetViewport { viewport } => history.set_viewport(viewport.clone())?,
            DocumentCommand::UpsertAsset { asset } => history.apply(ProjectEvent::AssetUpserted {
                asset: asset.clone(),
            })?,
            DocumentCommand::RemoveAsset { asset_id } => history.apply(ProjectEvent::AssetRemoved {
                asset_id: asset_id.clone(),
            })?,
            DocumentCommand::UpsertSubtitleStyle { style } => {
                history.apply(ProjectEvent::SubtitleStyleUpserted {
                    style: style.clone(),
                })?
            }
            DocumentCommand::RemoveSubtitleStyle { style_id } => {
                history.apply(ProjectEvent::SubtitleStyleRemoved {
                    style_id: style_id.clone(),
                })?
            }
            DocumentCommand::UpsertSubtitle { subtitle } => {
                history.apply(ProjectEvent::SubtitleUpserted {
                    subtitle: subtitle.clone(),
                })?
            }
            DocumentCommand::RemoveSubtitle { subtitle_id } => {
                history.apply(ProjectEvent::SubtitleRemoved {
                    subtitle_id: subtitle_id.clone(),
                })?
            }
            DocumentCommand::UpsertFilter { filter } => history.apply(ProjectEvent::FilterUpserted {
                filter: filter.clone(),
            })?,
            DocumentCommand::RemoveFilter { filter_id } => history.apply(ProjectEvent::FilterRemoved {
                filter_id: filter_id.clone(),
            })?,
            DocumentCommand::UpsertEffect { effect } => history.apply(ProjectEvent::EffectUpserted {
                effect: effect.clone(),
            })?,
            DocumentCommand::RemoveEffect { effect_id } => history.apply(ProjectEvent::EffectRemoved {
                effect_id: effect_id.clone(),
            })?,
            DocumentCommand::UpsertExportPreset { preset } => {
                history.apply(ProjectEvent::ExportPresetUpserted {
                    preset: preset.clone(),
                })?
            }
            DocumentCommand::RemoveExportPreset { preset_id } => {
                history.apply(ProjectEvent::ExportPresetRemoved {
                    preset_id: preset_id.clone(),
                })?
            }
            DocumentCommand::SetProjectPresets { presets } => {
                history.apply(ProjectEvent::ProjectPresetsSet {
                    presets: presets.clone(),
                })?
            }
            DocumentCommand::AddTrack {
                track_id,
                name,
                kind,
                index,
            } => history.apply(ProjectEvent::TrackAdded {
                track: Track::new(track_id.clone(), name.clone(), kind.clone()),
                index: *index,
            })?,
            DocumentCommand::RenameTrack { track_id, new_name } => history.apply(ProjectEvent::TrackRenamed {
                track_id: track_id.clone(),
                new_name: new_name.clone(),
            })?,
            DocumentCommand::RemoveTrack { track_id } => history.apply(ProjectEvent::TrackRemoved {
                track_id: track_id.clone(),
            })?,
            DocumentCommand::InsertClip { track_id, clip } => history.apply(ProjectEvent::ClipInserted {
                track_id: track_id.clone(),
                clip: clip.clone(),
            })?,
            DocumentCommand::RemoveClip { track_id, clip_id } => history.apply(ProjectEvent::ClipRemoved {
                track_id: track_id.clone(),
                clip_id: clip_id.clone(),
            })?,
            DocumentCommand::MoveClip {
                clip_id,
                to_track_id,
                new_timeline_range,
            } => history.apply(ProjectEvent::ClipMoved {
                clip_id: clip_id.clone(),
                to_track_id: to_track_id.clone(),
                new_timeline_range: new_timeline_range.clone(),
            })?,
            DocumentCommand::TrimClip {
                clip_id,
                new_source_range,
                new_timeline_range,
            } => history.apply(ProjectEvent::ClipTrimmed {
                clip_id: clip_id.clone(),
                new_source_range: new_source_range.clone(),
                new_timeline_range: new_timeline_range.clone(),
            })?,
            DocumentCommand::SplitClip {
                track_id,
                clip_id,
                split_at_ms,
                right_clip_id,
            } => history.split_clip(track_id, clip_id, *split_at_ms, right_clip_id.clone())?,
            DocumentCommand::RippleMoveClip { clip_id, delta_ms } => {
                history.ripple_move_clip(clip_id, *delta_ms)?;
            }
            DocumentCommand::CloseGapBeforeClip { clip_id } => {
                history.close_gap_before_clip(clip_id)?;
            }
            DocumentCommand::Undo => {
                history.undo()?;
            }
            DocumentCommand::Redo => {
                history.redo()?;
            }
        }

        emitted_events.extend(history.applied_events.iter().skip(previous_len).cloned());
    }

    Ok(emitted_events)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;

    use serde_json::json;

    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("cineweave-adapter-{}-{}.json", name, std::process::id()));
        path
    }

    #[test]
    fn adapter_can_create_and_load_project_document() {
        let snapshot_path = temp_file("snapshot-create");
        let event_log_path = temp_file("event-log-create");

        let create_response = execute_adapter_request(AdapterRequest::CreateProjectDocument {
            project_id: "adapter-1".into(),
            name: "Adapter Demo".into(),
            aspect_ratio: "16:9".into(),
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
        })
        .unwrap();

        assert!(create_response.ok);
        assert_eq!(create_response.state.unwrap().id, "adapter-1");

        let load_response = execute_adapter_request(AdapterRequest::GetDocumentState {
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
        })
        .unwrap();

        assert_eq!(load_response.state.unwrap().name, "Adapter Demo");

        let _ = fs::remove_file(snapshot_path);
        let _ = fs::remove_file(event_log_path);
    }

    #[test]
    fn adapter_can_apply_frontend_commands_and_persist() {
        let snapshot_path = temp_file("snapshot-apply");
        let event_log_path = temp_file("event-log-apply");

        execute_adapter_request(AdapterRequest::CreateProjectDocument {
            project_id: "adapter-2".into(),
            name: "Adapter Edit".into(),
            aspect_ratio: "9:16".into(),
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
        })
        .unwrap();

        let clip = Clip::new(
            "clip-1",
            "asset-1",
            "clip-1",
            TimeRange::new(0, 2_000).unwrap(),
            TimeRange::new(10_000, 12_000).unwrap(),
        )
        .unwrap();

        let response = execute_adapter_request(AdapterRequest::ApplyCommands {
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
            commands: vec![
                DocumentCommand::AddTrack {
                    track_id: "v1".into(),
                    name: "Video 1".into(),
                    kind: TrackKind::Video,
                    index: 0,
                },
                DocumentCommand::InsertClip {
                    track_id: "v1".into(),
                    clip,
                },
                DocumentCommand::SetSelection {
                    clip_ids: vec!["clip-1".into()],
                    track_id: Some("v1".into()),
                },
                DocumentCommand::SetPlayhead { playhead_ms: 500 },
            ],
            save: true,
        })
        .unwrap();

        let state = response.state.unwrap();
        assert_eq!(state.tracks.len(), 1);
        assert_eq!(state.editor.playhead_ms, 500);
        assert_eq!(state.editor.selection.clip_ids, vec!["clip-1"]);
        assert_eq!(response.emitted_events.len(), 4);

        let reloaded = execute_adapter_request(AdapterRequest::GetDocumentState {
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
        })
        .unwrap();
        assert_eq!(reloaded.state.unwrap().editor.playhead_ms, 500);

        let persisted_snapshot = crate::load_snapshot_from_path(&snapshot_path).unwrap();
        let persisted_history = load_history_from_documents(&snapshot_path, &event_log_path).unwrap();
        assert_eq!(persisted_snapshot.editor.playhead_ms, 0);
        assert!(persisted_snapshot.tracks.is_empty());
        assert_eq!(persisted_history.state.editor.playhead_ms, 500);
        assert_eq!(persisted_history.state.tracks.len(), 1);
        assert!(persisted_history.can_undo());
        assert!(!persisted_history.can_redo());

        let undone = execute_adapter_request(AdapterRequest::ApplyCommands {
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
            commands: vec![DocumentCommand::Undo],
            save: true,
        })
        .unwrap();
        assert_eq!(undone.state.unwrap().editor.playhead_ms, 0);

        let _ = fs::remove_file(snapshot_path);
        let _ = fs::remove_file(event_log_path);
    }

    #[test]
    fn adapter_can_upsert_registry_entities_and_project_presets() {
        let snapshot_path = temp_file("snapshot-registry");
        let event_log_path = temp_file("event-log-registry");

        execute_adapter_request(AdapterRequest::CreateProjectDocument {
            project_id: "adapter-3".into(),
            name: "Registry Edit".into(),
            aspect_ratio: "16:9".into(),
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
        })
        .unwrap();

        let asset = Asset {
            id: "asset-1".into(),
            asset_type: crate::AssetType::Video,
            label: "Interview Source".into(),
            source: crate::AssetSource {
                path: "media/interview.mp4".into(),
                checksum: None,
            },
            media: crate::AssetMedia::default(),
            tags: vec!["source".into()],
        };
        let style = SubtitleStyle {
            id: "style-custom".into(),
            label: "Custom Style".into(),
            font_family: "Aptos".into(),
            font_size_px: 42,
            placement: "bottom".into(),
            fill_color: "#FFFFFF".into(),
            stroke_color: "#111111".into(),
            background_color: None,
            max_lines: 2,
            animation_preset: Some("subtitle_pop".into()),
        };
        let clip = Clip::new(
            "clip-1",
            "asset-1",
            "clip-1",
            TimeRange::new(0, 2_000).unwrap(),
            TimeRange::new(10_000, 12_000).unwrap(),
        )
        .unwrap();

        let filter = Filter {
            id: "filter-1".into(),
            kind: "cinematic_grade".into(),
            label: "Clip Grade".into(),
            category: Some("look".into()),
            target: Some(crate::TargetReference {
                target_type: crate::TargetType::Clip,
                id: Some("clip-1".into()),
            }),
            enabled: true,
            parameters: BTreeMap::from([("intensity".into(), json!(0.8))]),
            keyframes: vec![crate::Keyframe {
                id: "kf-1".into(),
                property: "intensity".into(),
                offset_ms: 0,
                value: json!(0.8),
                easing: crate::KeyframeEasing::Linear,
            }],
            render_backend: Some("ffmpeg".into()),
        };
        let effect = Effect {
            id: "effect-1".into(),
            kind: "glitch".into(),
            label: "Glitch Accent".into(),
            category: Some("stylized".into()),
            target: Some(crate::TargetReference {
                target_type: crate::TargetType::Clip,
                id: Some("clip-1".into()),
            }),
            enabled: true,
            parameters: BTreeMap::from([("intensity".into(), json!(0.4))]),
            keyframes: vec![],
            render_backend: Some("compositor".into()),
        };
        let subtitle = Subtitle {
            id: "sub-1".into(),
            text: "Hello there".into(),
            start_ms: 0,
            end_ms: 1_800,
            style_id: Some("style-custom".into()),
            language: Some("en".into()),
            speaker: None,
            asset_id: Some("asset-1".into()),
            track_id: Some("v1".into()),
            clip_id: Some("clip-1".into()),
            words: vec![crate::SubtitleWord {
                text: "Hello".into(),
                start_ms: 0,
                end_ms: 700,
            }],
        };
        let preset = ExportPreset {
            id: "preset-custom".into(),
            label: "Custom Master".into(),
            container: "mp4".into(),
            aspect_ratio: "16:9".into(),
            video: crate::ExportVideoSettings {
                codec: Some("h264".into()),
                resolution: Some(crate::Resolution {
                    width: 1920,
                    height: 1080,
                }),
                frame_rate: 30.0,
                bitrate_kbps: Some(20_000),
                pixel_format: Some("yuv420p".into()),
            },
            audio: crate::ExportAudioSettings {
                codec: Some("aac".into()),
                sample_rate: 48_000,
                channels: 2,
                bitrate_kbps: Some(320),
            },
            destination: None,
        };

        let response = execute_adapter_request(AdapterRequest::ApplyCommands {
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
            commands: vec![
                DocumentCommand::UpsertAsset { asset },
                DocumentCommand::UpsertSubtitleStyle { style },
                DocumentCommand::AddTrack {
                    track_id: "v1".into(),
                    name: "Video 1".into(),
                    kind: TrackKind::Video,
                    index: 0,
                },
                DocumentCommand::InsertClip {
                    track_id: "v1".into(),
                    clip,
                },
                DocumentCommand::UpsertFilter { filter },
                DocumentCommand::UpsertEffect { effect },
                DocumentCommand::UpsertSubtitle { subtitle },
                DocumentCommand::UpsertExportPreset { preset },
                DocumentCommand::SetProjectPresets {
                    presets: ProjectPresets {
                        default_aspect_ratio: Some("16:9".into()),
                        default_subtitle_style_id: Some("style-custom".into()),
                        polished_subtitle_style_id: Some("style-custom".into()),
                        export_profile: Some("preset-custom".into()),
                    },
                },
            ],
            save: true,
        })
        .unwrap();

        let state = response.state.unwrap();
        assert_eq!(state.assets.len(), 1);
        assert_eq!(state.subtitle_styles.iter().any(|style| style.id == "style-custom"), true);
        assert_eq!(state.subtitles.len(), 1);
        assert_eq!(state.filters.iter().any(|filter| filter.id == "filter-1"), true);
        assert_eq!(state.effects.iter().any(|effect| effect.id == "effect-1"), true);
        assert_eq!(state.export_presets.iter().any(|preset| preset.id == "preset-custom"), true);
        assert_eq!(state.presets.export_profile.as_deref(), Some("preset-custom"));
        assert_eq!(state.clip("clip-1").unwrap().filter_ids, vec!["filter-1"]);
        assert_eq!(state.clip("clip-1").unwrap().effect_ids, vec!["effect-1"]);
        assert_eq!(state.clip("clip-1").unwrap().subtitle_ids, vec!["sub-1"]);
        assert_eq!(response.emitted_events.len(), 9);

        let reloaded = execute_adapter_request(AdapterRequest::GetDocumentState {
            snapshot_path: snapshot_path.clone(),
            event_log_path: event_log_path.clone(),
        })
        .unwrap();
        let reloaded_state = reloaded.state.unwrap();
        assert_eq!(reloaded_state.subtitles[0].clip_id.as_deref(), Some("clip-1"));
        assert_eq!(reloaded_state.filters.iter().any(|filter| filter.id == "filter-1"), true);
        assert_eq!(reloaded_state.presets.default_subtitle_style_id.as_deref(), Some("style-custom"));
        assert_eq!(reloaded_state.clip("clip-1").unwrap().filter_ids, vec!["filter-1"]);
        assert_eq!(reloaded_state.clip("clip-1").unwrap().effect_ids, vec!["effect-1"]);
        assert_eq!(reloaded_state.clip("clip-1").unwrap().subtitle_ids, vec!["sub-1"]);

        let _ = fs::remove_file(snapshot_path);
        let _ = fs::remove_file(event_log_path);
    }
}
