use std::io::{self, Read};
use std::path::PathBuf;

use media_core::{
    compile_render_plan, compile_task_graph, default_project, execute_adapter_request,
    load_history_from_paths, save_history_to_paths, timeline_demo_history, AdapterRequest,
    EditorViewport,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "plan".to_string());
    let prompt = args.collect::<Vec<_>>().join(" ");
    let prompt = if prompt.trim().is_empty() {
        "把采访视频剪成 9:16 版本，自动字幕，电影胶片风，加一点 glitch 和 zoom，小心思要有，节奏更利落"
            .to_string()
    } else {
        prompt
    };

    match command.as_str() {
        "plan" => {
            let project = default_project();
            let graph = compile_task_graph(&prompt, &project).expect("task graph should compile");
            let render_plan = compile_render_plan(&graph);
            println!("Task Graph:\n{graph:#?}\n");
            println!("Render Plan:\n{render_plan:#?}");
        }
        "timeline-demo" => {
            let mut history = timeline_demo_history().expect("timeline demo should compile");
            println!("Timeline State After Apply:\n{:#?}\n", history.state);
            history.undo().expect("undo should succeed");
            println!("Timeline State After Undo:\n{:#?}\n", history.state);
            history.redo().expect("redo should succeed");
            println!("Timeline State After Redo:\n{:#?}", history.state);
        }
        "document-demo" => {
            let mut history = timeline_demo_history().expect("timeline demo should compile");
            history
                .set_selection(vec!["clip-a".into()], Some("v2".into()))
                .expect("selection should set");
            history.set_playhead(6_200).expect("playhead should set");
            history
                .set_viewport(EditorViewport {
                    scroll_x_px: 540,
                    scroll_y_px: 180,
                    zoom_percent: 160,
                })
                .expect("viewport should set");
            let demo_dir = PathBuf::from("target").join("demo-ui");
            let snapshot_path = demo_dir.join("snapshot.json");
            let event_log_path = demo_dir.join("event-log.json");
            save_history_to_paths(&history, &snapshot_path, &event_log_path)
                .expect("history documents should save");

            let loaded_state = load_history_from_paths(&snapshot_path, &event_log_path)
                .expect("history should load")
                .state;

            println!("Snapshot Path: {}", snapshot_path.display());
            println!("Event Log Path: {}", event_log_path.display());
            println!("Reloaded State:\n{loaded_state:#?}");
        }
        "adapter" => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("stdin should be readable");
            let request: AdapterRequest =
                serde_json::from_str(&input).expect("adapter request JSON should be valid");
            let response =
                execute_adapter_request(request).expect("adapter request should execute");
            println!(
                "{}",
                serde_json::to_string_pretty(&response)
                    .expect("adapter response should serialize")
            );
        }
        other => {
            eprintln!("Unknown command: {other}");
            std::process::exit(1);
        }
    }
}
