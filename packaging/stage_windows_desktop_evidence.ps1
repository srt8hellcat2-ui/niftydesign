# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Source,
    [Parameter(Mandatory)][string]$Destination,
    [int]$Attempts = 3,
    [int]$DelayMilliseconds = 500
)
$ErrorActionPreference = 'Stop'
if ($Attempts -lt 1 -or $Attempts -gt 10) { throw 'Attempts must be 1..10' }
if ($DelayMilliseconds -lt 0 -or $DelayMilliseconds -gt 5000) { throw 'Delay must be 0..5000 ms' }
# Never hand an uploader a live settings/log directory. Start with new copies.
if (Test-Path -LiteralPath $Destination) { throw 'Choose a fresh staging directory' }
$report = Join-Path $Destination 'report'
$shots = Join-Path $Destination 'shots'
$pending = Join-Path $Destination 'pending-not-uploaded'
New-Item -ItemType Directory -Path $report, $shots, $pending -Force | Out-Null
$skipped = [System.Collections.Generic.List[string]]::new()
if (Test-Path -LiteralPath $Source) {
    $root = (Resolve-Path -LiteralPath $Source).Path
    foreach ($file in Get-ChildItem -LiteralPath $root -File -Recurse) {
        $relative = [System.IO.Path]::GetRelativePath($root, $file.FullName)
        # Build output is already streamed to Actions; retain its tail, not the full log.
        if ($relative -eq 'build.log') {
            $target = Join-Path $report 'build-tail.log'
        } else {
            $kind = if ($file.Extension.ToLowerInvariant() -in @('.png', '.pdf', '.xps', '.designcraft')) { $shots } else { $report }
            $target = Join-Path $kind $relative
        }
        $copied = $false
        for ($attempt = 1; $attempt -le $Attempts; $attempt++) {
            $candidate = Join-Path $pending ([guid]::NewGuid().ToString())
            try {
                New-Item -ItemType Directory -Path (Split-Path -Parent $target) -Force | Out-Null
                if ($relative -eq 'build.log') {
                    Get-Content -LiteralPath $file.FullName -Tail 400 -ErrorAction Stop | Set-Content -LiteralPath $candidate -Encoding utf8
                } else {
                    Copy-Item -LiteralPath $file.FullName -Destination $candidate -Force -ErrorAction Stop
                }
                Move-Item -LiteralPath $candidate -Destination $target -ErrorAction Stop
                $copied = $true
                break
            } catch {
                $reason = $_.Exception.Message
                if ($attempt -lt $Attempts) { Start-Sleep -Milliseconds $DelayMilliseconds }
            } finally {
                # An undeletable partial copy stays outside BOTH upload roots.
                Remove-Item -LiteralPath $candidate -Force -ErrorAction SilentlyContinue
            }
        }
        if (-not $copied) {
            $message = "$relative : $reason (after $Attempts attempts)"
            $skipped.Add($message)
            Write-Warning "SKIPPED $message"
        }
    }
} else {
    $skipped.Add('Evidence source does not exist; build or initialization did not run.')
}
$listing = if ($skipped.Count) { $skipped.ToArray() } else { @('None. All evidence files copied successfully.') }
foreach ($folder in @($report, $shots)) {
    $listing | Set-Content -LiteralPath (Join-Path $folder 'SKIPPED.txt') -Encoding utf8
}
if (-not (Test-Path -LiteralPath (Join-Path $report 'REPORT.md'))) {
    'FAILED / INCOMPLETE: REPORT.md unavailable. See SKIPPED.txt and the Actions console.' |
        Set-Content -LiteralPath (Join-Path $report 'REPORT.md') -Encoding utf8
}
Write-Host "Evidence staged: $Destination; skipped files: $($skipped.Count)"
