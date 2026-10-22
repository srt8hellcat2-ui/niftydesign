//! File › Print: the pages go to a printer through the system print spooler (craft-print), as PDF.

use serde_json::{Value, json};

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
use super::str_param;
use super::{CommandSpec, bad, cmd, has_doc};
use crate::{Result, Session};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "file.printers", "Printers", [], None, "{} → {printers: [names], default}", super::always, |_, _| Ok(printers())),
        cmd!(
            noundo "file.print",
            "Print…",
            ["File"],
            Some("Cmd+P"),
            "{printer?, copies?: 1, pages?: \"1-3,5\", spreads?, marks?, bleed?, systemDialog?: bool (native dialog, cancellation submits nothing), dryRun?: bool (export and preview submission without printing)} → {printer, copies, pages, command, bytes}",
            has_doc,
            print
        ),
    ]
}

fn printers() -> Value {
    let printers = craft_print::printers();
    let default = printers.iter().find(|p| p.default).map(|p| p.name.clone());
    let names: Vec<_> = printers.into_iter().map(|p| p.name).collect();
    json!({"printers": names, "default": default})
}

fn print(s: &mut Session, p: &Value) -> Result<Value> {
    #[cfg(target_arch = "wasm32")]
    {
        super::export::options(p, s.doc()?.doc.page_count())?;
        Err(bad("file.print", "printing isn't available on the web: export a PDF and print it"))
    }
    #[cfg(not(target_arch = "wasm32"))]
    print_with(s, p, craft_print::submit)
}

#[cfg(not(target_arch = "wasm32"))]
fn print_with(
    s: &mut Session,
    p: &Value,
    submit: impl FnOnce(&[u8], &craft_print::Job) -> std::result::Result<String, craft_print::Error>,
) -> Result<Value> {
    print_with_dialog(s, p, submit, native::submit)
}

#[cfg(not(target_arch = "wasm32"))]
fn print_with_dialog(
    s: &mut Session,
    p: &Value,
    submit: impl FnOnce(&[u8], &craft_print::Job) -> std::result::Result<String, craft_print::Error>,
    dialog: impl FnOnce(&[u8], &str) -> std::result::Result<bool, String>,
) -> Result<Value> {
    let title = s.doc()?.title();
    let copies = p.get("copies").and_then(Value::as_u64).unwrap_or(1).clamp(1, 999) as u32;
    let job = craft_print::Job {
        printer: str_param(p, "printer").map(str::to_string),
        copies,
        scaling: craft_print::Scaling::ShrinkToFit,
        title,
        ..craft_print::Job::default()
    };
    let dry = p.get("dryRun").and_then(Value::as_bool).unwrap_or(false);
    let prepared = super::export::prepare_pdf(s, p)?;
    let pdf = prepared.bytes;
    let command = craft_print::command_preview(&job).map_err(|e| bad("file.print", e.to_string()))?;
    let pages = prepared.pages;
    let system = p.get("systemDialog").and_then(Value::as_bool).unwrap_or(false);
    let mut out = json!({"printer": job.printer, "copies": copies, "pages": pages, "command": if system { Value::Null } else { json!(command) }, "bytes": pdf.len(), "systemDialog": system, "warnings": prepared.warnings});
    if !dry {
        if system {
            let printed = dialog(&pdf, &job.title).map_err(|e| bad("file.print", e))?;
            out["cancelled"] = json!(!printed);
        } else {
            submit(&pdf, &job).map_err(|e| bad("file.print", e.to_string()))?;
        }
    }
    Ok(out)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    // PDF timestamps and document IDs vary between exports; compare the actual printed pages.
    fn assert_same_pdf_pages(actual: &[u8], expected: &[u8]) -> std::result::Result<(), String> {
        let pages = designcraft_render::pdf_page_count(expected).ok_or_else(|| "expected PDF has no readable page count".to_string())?;
        assert_eq!(designcraft_render::pdf_page_count(actual), Some(pages));
        for page in 0..pages {
            assert_eq!(designcraft_render::pdf_page_size(actual, page), designcraft_render::pdf_page_size(expected, page));
            let actual = designcraft_render::pdf_page_png(actual, page, 512, &[], None)
                .ok_or_else(|| format!("could not render actual PDF page {}", page + 1))?;
            let expected = designcraft_render::pdf_page_png(expected, page, 512, &[], None)
                .ok_or_else(|| format!("could not render expected PDF page {}", page + 1))?;
            assert!(actual == expected, "print and export differ on page {}", page + 1);
        }
        Ok(())
    }

    #[test]
    fn system_print_dialog_receives_pages_and_cancellation_never_spools() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"title": "flyer", "pages": 3})).unwrap();
        let params = json!({"systemDialog": true, "pages": "2-3"});
        for printed in [true, false] {
            let r = print_with_dialog(
                &mut s,
                &params,
                |_, _| panic!("dialog route must not also spool"),
                |pdf, title| {
                    assert_eq!(title, "flyer");
                    assert_eq!(designcraft_render::pdf_page_count(pdf), Some(2));
                    Ok(printed)
                },
            )
            .unwrap();
            assert_eq!(r["cancelled"], !printed);
            assert_eq!(r["pages"], 2);
            assert!(r["command"].is_null());
        }
        assert!(
            print_with_dialog(&mut s, &params, |_, _| panic!("no fallback job"), |_, _| Err("portal unavailable".into()))
                .unwrap_err()
                .to_string()
                .contains("portal unavailable")
        );
        print_with_dialog(
            &mut s,
            &json!({"systemDialog": true, "dryRun": true}),
            |_, _| panic!("dry run must not spool"),
            |_, _| panic!("dry run must not open a dialog"),
        )
        .unwrap();
        assert!(
            print_with_dialog(
                &mut s,
                &json!({"systemDialog": true, "pages": "9"}),
                |_, _| panic!("invalid range"),
                |_, _| panic!("invalid range must not open a dialog")
            )
            .is_err()
        );
    }

    #[test]
    fn print_keeps_shadows_in_the_same_pdf_as_export() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"title": "flyer"})).unwrap();
        let id = s.execute("frame.create", &json!({"rect": [72,72,172,172]})).unwrap()["id"].as_u64().unwrap();
        s.execute("object.fill", &json!({"ids": [id], "swatch": "[Black]"})).unwrap();
        s.execute("object.dropShadow", &json!({"ids": [id], "on": true, "distance": 6, "angle": 135, "size": 0, "opacity": 0.5})).unwrap();
        let exported = s.execute("file.exportPdf", &json!({})).unwrap();
        let expected = super::super::base64_decode(exported["base64"].as_str().unwrap());
        assert!(exported["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("soft effects")));
        let r = print_with_dialog(
            &mut s,
            &json!({"systemDialog": true}),
            |_, _| panic!("no duplicate spool"),
            |pdf, _| {
                assert_same_pdf_pages(pdf, &expected)?;
                Ok(true)
            },
        )
        .unwrap();
        assert_eq!(r["warnings"], exported["warnings"]);
    }

    #[test]
    fn print_uses_the_current_display_title_after_save_as() -> crate::Result<()> {
        let mut s = Session::new();
        s.execute("file.new", &json!({"title": "Original"}))?;
        let dir = tempfile::tempdir().map_err(|e| crate::EngineError::Other(e.to_string()))?;
        let path = dir.path().join("Coffee Shop.designcraft");
        s.execute("file.saveAs", &json!({"path": path.to_string_lossy()}))?;
        // The persisted document title can still be the original name. The UI title is current.
        assert_eq!(s.doc()?.doc.title, "Original");
        print_with(&mut s, &json!({}), |_, job| {
            assert_eq!(job.title, "Coffee Shop.designcraft");
            Ok("submitted".into())
        })?;
        Ok(())
    }

    #[test]
    fn print_builds_the_spool_command() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"pages": 4})).unwrap();
        let r = s.execute("file.print", &json!({"printer": "Office", "copies": 2, "pages": "2-3", "dryRun": true})).unwrap();
        assert_eq!(r["pages"], 2);
        let command: Vec<&str> = r["command"].as_array().unwrap().iter().filter_map(Value::as_str).collect();
        if craft_print::backend() == Some(craft_print::Backend::Cups) {
            assert_eq!(&command[..5], ["lp", "-d", "Office", "-n", "2"]);
            assert!(command.contains(&"print-scaling=auto-fit"));
            assert!(!command.iter().any(|a| a.starts_with("sides=")));
        } else {
            assert_eq!(command[0], "powershell");
        }
        assert!(r["bytes"].as_u64().unwrap() > 500);
        assert!(s.execute("file.print", &json!({"pages": "9", "dryRun": true})).is_err());
        assert!(s.execute("file.printers", &json!({})).unwrap()["printers"].is_array());
    }

    #[test]
    fn print_submits_the_selected_export_and_reports_refusal() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"pages": 4})).unwrap();
        let params = json!({"printer": "Office (east)", "copies": 2, "pages": "2-3", "marks": true, "bleed": true});
        let exported = s.execute("file.exportPdf", &json!({"pages": params["pages"], "marks": params["marks"], "bleed": params["bleed"]})).unwrap();
        let expected = super::super::base64_decode(exported["base64"].as_str().unwrap());
        let r = print_with(&mut s, &params, |pdf, job| {
            // The production export options and full bytes reach the shared spooler boundary.
            assert_same_pdf_pages(pdf, &expected).map_err(craft_print::Error::Spool)?;
            assert!(pdf.starts_with(b"%PDF-"));
            assert_eq!(job.printer.as_deref(), Some("Office (east)"));
            assert_eq!(job.copies, 2);
            assert_eq!(job.duplex, None);
            assert_eq!(job.scaling, craft_print::Scaling::ShrinkToFit);
            assert!(!job.title.is_empty());
            Ok("request id is Office-1".into())
        })
        .unwrap();
        assert_eq!(r["pages"], 2);
        let refusal = print_with(&mut s, &params, |_, _| Err(craft_print::Error::Spool("printer offline".into()))).unwrap_err();
        assert!(refusal.to_string().contains("printer offline"));
        print_with(&mut s, &json!({"dryRun": true}), |_, _| panic!("dry runs must not spool")).unwrap();
        assert!(print_with(&mut s, &json!({"pages": "9"}), |_, _| panic!("invalid ranges must not spool")).is_err());
    }
}
