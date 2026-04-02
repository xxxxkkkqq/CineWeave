$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

$request = @'
{
  "type": "create_project_document",
  "project_id": "adapter-script-demo",
  "name": "Adapter Script Demo",
  "aspect_ratio": "16:9",
  "snapshot_path": "target/demo-ui/adapter-snapshot.json",
  "event_log_path": "target/demo-ui/adapter-event-log.json"
}
'@

Write-Host "Creating adapter document..."
$request | cargo +stable-x86_64-pc-windows-gnu run -p media-core -- adapter

$request = @'
{
  "type": "apply_commands",
  "snapshot_path": "target/demo-ui/adapter-snapshot.json",
  "event_log_path": "target/demo-ui/adapter-event-log.json",
  "save": true,
  "commands": [
    {
      "type": "add_track",
      "track_id": "v1",
      "name": "Video 1",
      "kind": "Video",
      "index": 0
    },
    {
      "type": "set_playhead",
      "playhead_ms": 1200
    }
  ]
}
'@

Write-Host ""
Write-Host "Applying adapter commands..."
$request | cargo +stable-x86_64-pc-windows-gnu run -p media-core -- adapter
