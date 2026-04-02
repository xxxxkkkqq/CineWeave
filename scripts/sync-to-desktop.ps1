param(
  [string]$Source = "C:\Users\xkq\Desktop\Codex-OSS\CineWeave",
  [string]$Target = "C:\Users\xkq\Desktop\CineWeave"
)

robocopy $Source $Target /E /R:1 /W:1 /NFL /NDL /NJH /NJS /NP | Out-Null
Write-Host "Synced $Source -> $Target"
