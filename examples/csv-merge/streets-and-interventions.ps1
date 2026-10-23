# streets-and-interventions.ps1
# Builds one document from TWO CSV files:
#   street 1: its street pages, then all of its interventions
#   street 2: its street pages, then all of its interventions ... and so on.
#
# Your template document has parent pages with text labels that contain {field} tokens
# (field = a column header), for example a label that says "Street: {street}".
#   -StreetParents        parent page(s) used for each street, in page order, e.g. "A,B"
#   -InterventionParent   parent page used for each intervention, e.g. "C"
#   -Key                  the column that is in BOTH csv files and links them (e.g. street)
#
# Notes: tested with single-sided pages (Document Setup: Facing Pages off). With facing pages, make sure each parent has its labels on the side(s) it is used on.
#        Close other DesignCraft windows first. The template file itself is never changed.
#
# Example:
#   .\streets-and-interventions.ps1 -Template template.designcraft -Streets streets.csv `
#       -Interventions interventions.csv -Key street -StreetParents A,B -InterventionParent C -Out result
param(
  [Parameter(Mandatory)][string]$Template,
  [Parameter(Mandatory)][string]$Streets,
  [Parameter(Mandatory)][string]$Interventions,
  [Parameter(Mandatory)][string]$Key,
  [Parameter(Mandatory)][string[]]$StreetParents,
  [Parameter(Mandatory)][string]$InterventionParent,
  [string]$Out = "result",          # writes result.designcraft and result.pdf
  [string]$Exe = ".\designcraft.exe",
  [int]$Port = 47932
)
$StreetParents = $StreetParents -split '[,;]' | ForEach-Object { $_.Trim() }   # "A,B" works with -File too
$Template = (Resolve-Path $Template).Path
$streetRows = Import-Csv $Streets -Encoding UTF8
$intRows = Import-Csv $Interventions -Encoding UTF8
$Out = [IO.Path]::GetFullPath($Out)

$app = Start-Process $Exe -ArgumentList '--control', $Port -PassThru
try {
  $client = $null
  for ($i = 0; $i -lt 30 -and -not $client; $i++) { Start-Sleep 1; try { $client = New-Object Net.Sockets.TcpClient('127.0.0.1', $Port) } catch {} }
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

  # Add one page that uses $parent, replace the {field} tokens in its labels with the row's values.
  function Add-Page($parent, $row) {
    $count = (Run 'document.inspect' @{}).pageCount
    Run 'layout.pages.insert' @{ count = 1; after = $count - 1; parent = $parent } | Out-Null
    $page = $count                                        # 0-based index of the new page
    $ids = (Run 'layout.overrideParentItems' @{ page = $page }).ids   # the page's own copies of the parent's labels
    $doc = Run 'document.inspect' @{}
    $items = $doc.spreads | ForEach-Object { $_.items } | Where-Object { $ids -contains $_.id -and $null -ne $_.story }
    foreach ($it in $items) {
      $story = $doc.stories | Where-Object { $_.id -eq $it.story }
      $text = $story.preview
      if ($text -notmatch '\{[^}]+\}') { continue }
      $new = [regex]::Replace($text, '\{([^}]+)\}', { param($m) $v = $row.($m.Groups[1].Value); if ($null -eq $v) { $m.Value } else { $v } })
      Run 'text.select' @{ story = $story.id; anchor = 0; focus = [Text.Encoding]::UTF8.GetByteCount($text) } | Out-Null
      Run 'text.insert' @{ text = $new; raw = $true } | Out-Null
    }
  }

  Run 'file.open' @{ path = $Template } | Out-Null
  $templatePages = (Run 'document.inspect' @{}).pageCount

  foreach ($s in $streetRows) {
    foreach ($p in $StreetParents) { Add-Page $p $s }
    foreach ($iv in ($intRows | Where-Object { $_.$Key -eq $s.$Key })) { Add-Page $InterventionParent $iv }
  }

  Run 'layout.pages.delete' @{ pages = @(0..($templatePages - 1)) } | Out-Null   # drop the template's own pages
  Run 'file.saveAs' @{ path = $Out } | Out-Null
  Run 'file.exportPdf' @{ path = "$Out.pdf" } | Out-Null
  $final = Run 'document.inspect' @{}
  "Done: $($final.pageCount) pages -> $Out.designcraft and $Out.pdf"
}
finally { Stop-Process -Id $app.Id -Force -ErrorAction SilentlyContinue }


