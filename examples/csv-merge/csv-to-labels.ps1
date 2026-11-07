# csv-to-labels.ps1  -  fill labels from a CSV with DesignCraft's data merge.
# Usage:
#   .\csv-to-labels.ps1 -Csv people.csv -Template "Name: {name} from {city}" -Out labels.pdf
# {name} and {city} must match column headers in the CSV's first row.
# Needs designcraft.exe (set -Exe if it is not in the same folder). Make sure no other DesignCraft is running.
param(
  [Parameter(Mandatory)][string]$Csv,
  [Parameter(Mandatory)][string]$Template,
  [string]$Out = "labels.pdf",
  [string]$Exe = ".\designcraft.exe",
  [int]$Port = 47931
)
$Csv = (Resolve-Path $Csv).Path
$Out = [IO.Path]::GetFullPath($Out)
$parts = [regex]::Split($Template, '(\{[^}]+\})') | Where-Object { $_ -ne '' }
$literal = -join ($parts | Where-Object { $_ -notmatch '^\{.+\}$' })

$app = Start-Process $Exe -ArgumentList '--control', $Port -PassThru
try {
  $client = $null
  for ($i = 0; $i -lt 30 -and -not $client; $i++) {
    Start-Sleep 1
    try { $client = New-Object Net.Sockets.TcpClient('127.0.0.1', $Port) } catch {}
  }
  if (-not $client) { throw "Could not connect to DesignCraft." }
  $w = New-Object IO.StreamWriter($client.GetStream()); $w.AutoFlush = $true
  $r = New-Object IO.StreamReader($client.GetStream())
  $script:id = 0
  function Run($cmd, $params) {
    $script:id++
    $w.WriteLine((@{ id = $script:id; method = 'engine.execute'; params = @{ command = $cmd; params = $params } } | ConvertTo-Json -Depth 10 -Compress))
    $reply = $r.ReadLine() | ConvertFrom-Json
    if (-not $reply.ok) { throw "$cmd failed: $($reply | ConvertTo-Json -Depth 5 -Compress)" }
    $reply.result
  }

  Run 'file.new' @{} | Out-Null
  $frame = Run 'frame.create' @{ rect = @(50, 50, 300, 100); content = 'text'; text = $literal }
  Run 'data.source.select' @{ path = $Csv; name = (Split-Path $Csv -Leaf) } | Out-Null

  # put each {field} where it sits in the template; go from the last one to the first so earlier positions stay valid
  $spots = @(); $pos = 0
  foreach ($p in $parts) {
    if ($p -match '^\{(.+)\}$') { $spots += @{ field = $Matches[1]; at = $pos } } else { $pos += $p.Length }
  }
  [array]::Reverse($spots)
  foreach ($s in $spots) {
    Run 'data.placeholder.add' @{ field = $s.field; story = $frame.story; at = $s.at } | Out-Null
  }

  $m = Run 'data.merge' @{ path = $Csv; name = (Split-Path $Csv -Leaf) }
  Run 'file.exportPdf' @{ path = $Out } | Out-Null
  "Done: $($m.records) records, $($m.pages) pages -> $Out"
}
finally { Stop-Process -Id $app.Id -Force -ErrorAction SilentlyContinue }

