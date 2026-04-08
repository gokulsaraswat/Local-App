param(
  [string]$TargetPath = (Get-Location).Path,
  [switch]$Force
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$nodeArgs = @("$scriptDir/apply_patch.mjs", $TargetPath)

if ($Force) {
  $nodeArgs += "--force"
}

node @nodeArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}
