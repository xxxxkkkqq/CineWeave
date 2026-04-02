$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

Write-Host "Running Rust tests with GNU toolchain..."
cargo +stable-x86_64-pc-windows-gnu test

Write-Host ""
Write-Host "Running sample Rust planning flow..."
$prompt = 'Create a 9:16 cut with subtitles, cinematic film style, a little glitch and zoom, plus micro-polish.'
cargo +stable-x86_64-pc-windows-gnu run -p media-core -- plan $prompt

Write-Host ""
Write-Host "Running timeline kernel demo..."
cargo +stable-x86_64-pc-windows-gnu run -p media-core -- timeline-demo

Write-Host ""
Write-Host "Running document persistence demo..."
cargo +stable-x86_64-pc-windows-gnu run -p media-core -- document-demo
