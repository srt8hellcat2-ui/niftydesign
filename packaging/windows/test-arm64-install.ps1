# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
# Check the actual per-machine WiX contract; the executable retains its compatible file name.
param([string] $Dist = 'dist')
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$inPackage = $false
$Version = $null
foreach ($line in Get-Content (Join-Path $Root 'Cargo.toml')) {
  if ($line -match '^\s*\[') { $inPackage = ($line.Trim() -eq '[workspace.package]'); continue }
  if ($inPackage -and $line -match '^version\s*=\s*"([^"]+)"') { $Version = $Matches[1]; break }
}
if (-not $Version) { throw 'missing workspace version' }
$packages = @(Get-ChildItem (Join-Path $Dist "niftydesign-$Version-windows-arm64.msi"))
if ($packages.Count -ne 1) { throw "expected one NiftyDesign $Version ARM64 MSI" }
$msi = $packages[0].FullName
$dir = Join-Path $env:ProgramFiles 'NiftyDesign'
$shortcut = Join-Path ([Environment]::GetFolderPath('CommonPrograms')) 'NiftyDesign.lnk'
$application = 'HKLM:\Software\Classes\Applications\designcraft.exe'
$appPath = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\App Paths\designcraft.exe'
$document = 'HKLM:\Software\Classes\Designcraft.Document'
$idml = 'HKLM:\Software\Classes\Designcraft.Idml'
$settings = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Craft.designcraft'
$uninstall = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'
function Require([bool] $Condition, [string] $Message) {
  if (-not $Condition) { throw $Message }
  Write-Output "ok $Message"
}
function Run-Msi([string] $Action, [string] $Log) {
  $p = Start-Process msiexec.exe -ArgumentList $Action, "`"$msi`"", '/qn', '/norestart', '/l*v', $Log -Wait -PassThru
  if ($p.ExitCode -notin @(0, 3010)) {
    Get-Content $Log -Tail 60
    throw "msiexec $Action exited $($p.ExitCode)"
  }
}
Require ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString() -eq 'Arm64') 'native ARM64 runner'
Run-Msi '/i' 'install.log'
try {
  foreach ($name in @('designcraft.exe', 'designcraft-cli.exe')) {
    $exe = Join-Path $dir $name
    Require (Test-Path $exe) "$name installed in $dir"
    $bytes = [System.IO.File]::ReadAllBytes($exe)
    $pe = [BitConverter]::ToInt32($bytes, 0x3C)
    Require ([BitConverter]::ToUInt16($bytes, $pe + 4) -eq 0xAA64) "$name is ARM64"
  }
  $exe = Join-Path $dir 'designcraft.exe'
  $meta = (Get-Item $exe).VersionInfo
  Require ($meta.ProductName -eq 'NiftyDesign') 'PE product name'
  Require ($meta.ProductVersion -eq $Version -and $meta.FileVersion -eq $Version) 'PE alpha version strings'
  Require (Test-Path $shortcut) 'NiftyDesign Start Menu shortcut'
  # WiX creates an advertised MSI shortcut, so resolve its component rather than treating
  # it as an ordinary WScript .lnk target.
  $installer = New-Object -ComObject WindowsInstaller.Installer
  $target = $installer.GetType().InvokeMember('ShortcutTarget', 'GetProperty', $null, $installer, @($shortcut))
  $product = $target.GetType().InvokeMember('StringData', 'GetProperty', $null, $target, @(1))
  $component = $target.GetType().InvokeMember('StringData', 'GetProperty', $null, $target, @(3))
  $targetPath = $installer.GetType().InvokeMember('ComponentPath', 'GetProperty', $null, $installer, @($product, $component))
  Require ($targetPath -eq $exe) 'advertised shortcut targets designcraft.exe'
  Require ((Get-ItemProperty $application).FriendlyAppName -eq 'NiftyDesign') 'Open with name'
  Require ((Get-Item $appPath).GetValue('') -eq $exe) 'App Paths executable'
  Require ((Get-Item $document).GetValue('') -eq 'NiftyDesign Document') 'document association name'
  $command = '"' + $exe + '" "%1"'
  foreach ($key in @("$application\shell\open\command", "$document\shell\open\command", "$idml\shell\open\command")) {
    Require ((Get-Item $key).GetValue('') -eq $command) "open command: $key"
  }
  Require ((Get-Item 'HKLM:\Software\Classes\.designcraft').GetValue('') -eq 'Designcraft.Document') 'document extension'
  Require ($null -ne (Get-ItemProperty 'HKLM:\Software\Classes\.idml\OpenWithProgids').PSObject.Properties['Designcraft.Idml']) 'IDML offered under Open with'
  $arp = @(Get-ItemProperty $uninstall | Where-Object DisplayName -eq "NiftyDesign $Version")
  Require ($arp.Count -eq 1) 'uninstall entry displays full alpha version'
  Require ($arp[0].DisplayVersion -eq ($Version -split '-')[0]) 'numeric MSI version'
  $arpKey = $arp[0].PSPath
  $entry = Get-ItemProperty $settings
  Require ($entry.DisplayName -eq "NiftyDesign $Version (Craft Apps)") 'Settings entry displays full alpha identity'
  Require ($entry.DisplayVersion -eq ($Version -split '-')[0]) 'Settings numeric MSI version'
  Require ($entry.InstallLocation.TrimEnd('\') -eq $dir) 'Settings install directory'
  Require ($entry.Publisher -eq 'NiftyDesign (unofficial)') 'Settings publisher'
  Require ($entry.QuietUninstallString -match '/x \{[0-9A-Fa-f-]+\} /qn /norestart$') 'Settings MSI uninstall command'
  $out = & (Join-Path $dir 'designcraft-cli.exe') --version 2>&1 | Out-String
  Require ($LASTEXITCODE -eq 0 -and $out.Trim() -eq "NiftyDesign CLI $Version") 'native CLI alpha version'
} finally {
  Run-Msi '/x' 'uninstall.log'
}
foreach ($left in @((Join-Path $dir 'designcraft.exe'), (Join-Path $dir 'designcraft-cli.exe'), $shortcut, $application, $appPath, $document, $idml, $settings)) {
  Require (-not (Test-Path $left)) "uninstall removed $left"
}
if ($arpKey) { Require (-not (Test-Path $arpKey)) 'uninstall removed product registration' }
$extension = Get-Item 'HKLM:\Software\Classes\.designcraft' -ErrorAction SilentlyContinue
Require ($null -eq $extension -or $extension.GetValue('') -ne 'Designcraft.Document') 'uninstall removed document association'
$openWith = Get-ItemProperty 'HKLM:\Software\Classes\.idml\OpenWithProgids' -ErrorAction SilentlyContinue
Require ($null -eq $openWith -or $null -eq $openWith.PSObject.Properties['Designcraft.Idml']) 'uninstall removed IDML offer'
