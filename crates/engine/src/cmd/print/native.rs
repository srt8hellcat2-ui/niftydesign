// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Native print dialogs. Only a generated, owned PDF is handed to the operating system.

#[cfg(target_os = "linux")]
pub(super) fn submit(pdf: &[u8], title: &str) -> Result<bool, String> {
    use ashpd::desktop::{ResponseError, print::PrintProxy};
    use std::io::{Seek, Write};
    let mut file = tempfile::tempfile().map_err(|e| e.to_string())?;
    file.write_all(pdf).map_err(|e| e.to_string())?;
    file.rewind().map_err(|e| e.to_string())?;
    let result: ashpd::Result<()> = pollster::block_on(async {
        let proxy = PrintProxy::new().await?;
        let prepared = proxy.prepare_print(None, title, Default::default(), Default::default(), None, true).await?.response()?;
        proxy.print(None, title, &file, Some(prepared.token), true).await?.response()?;
        Ok(())
    });
    match result {
        Ok(()) => Ok(true),
        Err(ashpd::Error::Response(ResponseError::Cancelled)) => Ok(false),
        Err(e) => Err(format!("The system print dialog is unavailable: {e}. Check the desktop print portal, or export a PDF and print it.")),
    }
}

#[cfg(target_os = "windows")]
pub(super) fn submit(pdf: &[u8], title: &str) -> Result<bool, String> {
    // PrinterSettings belongs to the native driver dialog. PDF rendering/submission stays in
    // the existing craft-print backend. Page ranges are already applied by the PDF exporter.
    const SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding($false)
Add-Type -AssemblyName System.Windows.Forms
$d = New-Object System.Windows.Forms.PrintDialog
$d.UseEXDialog = $true
$d.AllowSomePages = $false
$d.AllowPrintToFile = $false
try {
    if ($d.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        @{ printer = $d.PrinterSettings.PrinterName; copies = $d.PrinterSettings.Copies; duplex = [int]$d.PrinterSettings.Duplex; collate = $d.PrinterSettings.Collate } | ConvertTo-Json -Compress
    } else { '{"cancelled":true}' }
} finally { $d.Dispose() }
"#;
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-STA", "-NonInteractive", "-Command", SCRIPT])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() || output.stdout.len() > 65_536 {
        return Err("The system print dialog failed. Export a PDF to print it from another application.".into());
    }
    let settings: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if settings["cancelled"].as_bool() == Some(true) {
        return Ok(false);
    }
    let printer = settings["printer"].as_str().filter(|p| !p.is_empty()).ok_or("The print dialog returned no printer")?;
    let copies = settings["copies"].as_u64().filter(|c| (1..=999).contains(c)).ok_or("The print dialog returned invalid copies")? as u32;
    let job = craft_print::Job {
        printer: Some(printer.into()),
        copies,
        title: title.into(),
        scaling: craft_print::Scaling::ShrinkToFit,
        ..Default::default()
    };
    craft_print::submit(pdf, &job).map_err(|e| e.to_string())?;
    Ok(true)
}

#[cfg(target_os = "macos")]
pub(super) fn submit(pdf: &[u8], _: &str) -> Result<bool, String> {
    // Preview owns the macOS print panel and its page/printer settings. The argument is passed
    // separately, never interpolated into AppleScript. Keep the PDF until the dialog completes.
    const SCRIPT: &str = r#"on run argv
try
    tell application "Preview"
        activate
        print (POSIX file (item 1 of argv)) with print dialog
    end tell
    return "printed"
on error messageText number errorNumber
    if errorNumber is -128 then return "cancelled"
    error messageText number errorNumber
end try
end run"#;
    use std::io::Write;
    let mut file = tempfile::Builder::new().suffix(".pdf").tempfile().map_err(|e| e.to_string())?;
    file.write_all(pdf).map_err(|e| e.to_string())?;
    let output = std::process::Command::new("osascript").args(["-e", SCRIPT]).arg(file.path()).output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("The macOS print dialog failed. Check Preview/Automation permissions, or export a PDF to print.".into());
    }
    match output.stdout.trim_ascii() {
        b"printed" => Ok(true),
        b"cancelled" => Ok(false),
        _ => Err("The macOS print dialog returned an unexpected result.".into()),
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub(super) fn submit(_: &[u8], _: &str) -> Result<bool, String> {
    Err("No system print dialog is available on this platform. Export a PDF to print it.".into())
}
