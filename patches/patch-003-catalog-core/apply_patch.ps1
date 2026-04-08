param(
  [string]$TargetDir = "."
)

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
node "$scriptRoot/apply_patch.mjs" $TargetDir --install
