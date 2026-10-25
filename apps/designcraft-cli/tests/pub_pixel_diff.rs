// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Optional visual scoring, never an import-format oracle or an image-quality CI gate.
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use designcraft_compose::Cache;
use designcraft_render::{RenderOptions, Rendered, Renderer};
use serde_json::{Value, json};

type Result<T> = std::result::Result<T, String>;
const DATA: &str = "/tmp/niftydata";
const SNAPSHOT_SCALE: f64 = 1.0;
const REDUCTION: u32 = 3;
const WINDOW: u32 = 8;
const DIFFERENCE: u8 = 8;
const MAX_PIXELS: u64 = 16_000_000;
const CATEGORIES: [&str; 12] =
    ["tables", "text", "wordart", "fills", "lines", "shadows", "pictures", "pages", "groups", "geometry", "shapes", "other"];

#[derive(Clone)]
struct Raster {
    width: u32,
    height: u32,
    rgb: Vec<[u8; 3]>,
}

fn count(width: u32, height: u32) -> Result<usize> {
    let n = u64::from(width) * u64::from(height);
    if n == 0 || n > MAX_PIXELS || width > 8192 || height > 8192 {
        return Err(format!("unsupported raster size {width}x{height}"));
    }
    usize::try_from(n).map_err(|e| e.to_string())
}

impl Raster {
    fn new(width: u32, height: u32, rgb: Vec<[u8; 3]>) -> Result<Self> {
        if rgb.len() != count(width, height)? {
            return Err("raster buffer length does not match dimensions".into());
        }
        Ok(Self { width, height, rgb })
    }

    fn pixel(&self, x: u32, y: u32) -> Result<[u8; 3]> {
        if x >= self.width || y >= self.height {
            return Err("pixel lies outside raster".into());
        }
        let index = usize::try_from(u64::from(y) * u64::from(self.width) + u64::from(x)).map_err(|e| e.to_string())?;
        self.rgb.get(index).copied().ok_or_else(|| "missing raster pixel".into())
    }

    fn save(&self, path: &Path) -> Result<()> {
        let pixels = self.rgb.iter().flat_map(|&[r, g, b]| [r, g, b, 255]).collect();
        let encoded = Rendered { width: self.width, height: self.height, pixels }.to_png();
        if encoded.is_empty() {
            return Err("PNG encoding failed".into());
        }
        write(path, encoded)
    }

    /// Box averages including partial edge boxes, with no geometric stretching.
    fn reduced(&self, factor: u32) -> Result<Self> {
        if factor == 0 || factor > 16 {
            return Err("invalid reduction factor".into());
        }
        let (w, h) = (self.width.div_ceil(factor), self.height.div_ceil(factor));
        let mut rgb = Vec::with_capacity(count(w, h)?);
        for y in 0..h {
            for x in 0..w {
                let mut sum = [0u32; 3];
                let mut n = 0;
                for sy in y * factor..((y + 1) * factor).min(self.height) {
                    for sx in x * factor..((x + 1) * factor).min(self.width) {
                        let [r, g, b] = self.pixel(sx, sy)?;
                        sum[0] += u32::from(r);
                        sum[1] += u32::from(g);
                        sum[2] += u32::from(b);
                        n += 1;
                    }
                }
                if n == 0 {
                    return Err("empty reduction box".into());
                }
                rgb.push(sum.map(|v| ((v + n / 2) / n) as u8));
            }
        }
        Self::new(w, h, rgb)
    }
}

fn read(path: &Path) -> Result<Vec<u8>> {
    if fs::metadata(path).map_err(|e| format!("{}: {e}", path.display()))?.len() > 64 * 1024 * 1024 {
        return Err(format!("{} exceeds 64 MiB", path.display()));
    }
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn write(path: &Path, bytes: impl AsRef<[u8]>) -> Result<()> {
    if fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err(format!("refusing to overwrite symlink {}", path.display()));
    }
    fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

fn white([r, g, b, a]: [u8; 4]) -> [u8; 3] {
    [r, g, b].map(|v| v.saturating_add(255 - a))
}

fn load_png(path: &Path) -> Result<Raster> {
    let bytes = read(path)?;
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Err(format!("{} is not PNG", path.display()));
    }
    let (w, h) = designcraft_render::image_size(&bytes).ok_or("missing PNG dimensions")?;
    count(w, h)?;
    let pm = designcraft_render::decode_pixmap(&bytes).ok_or_else(|| format!("cannot decode {}", path.display()))?;
    Raster::new(w, h, pm.data().iter().map(|p| white([p.r, p.g, p.b, p.a])).collect())
}

fn render(path: &Path, scale: f64) -> Result<Raster> {
    let imported = designcraft_pub::import_pub(&read(path)?).map_err(|e| format!("{}: {e}", path.display()))?;
    let page = imported.document.page(0).ok_or("document has no first page")?;
    count((page.width * scale).round() as u32, (page.height * scale).round() as u32)?;
    let opts = RenderOptions { printing_only: true, ..Default::default() };
    let image = Renderer::new().render_page(&imported.document, &Cache::new(), 0, scale, false, &opts).ok_or("first page could not be rendered")?;
    let rgb = image.pixels.as_chunks::<4>().0.iter().copied().map(white).collect();
    Raster::new(image.width, image.height, rgb)
}

/// Use the PNG's meter-based x density, uniformly on both axes. Unknown density means 300 dpi.
fn density(bytes: &[u8]) -> Result<(f64, bool, bool)> {
    let mut at = 8usize;
    while let Some(header) = bytes.get(at..at.saturating_add(8)) {
        let len = u32::from_be_bytes(header.get(..4).ok_or("short PNG chunk")?.try_into().map_err(|_| "short PNG chunk")?) as usize;
        let end = at.checked_add(12).and_then(|v| v.checked_add(len)).ok_or("PNG chunk length overflow")?;
        if end > bytes.len() {
            return Err("truncated PNG chunk".into());
        }
        if header.get(4..8) == Some(b"pHYs") && len == 9 {
            let data = bytes.get(at + 8..end - 4).ok_or("missing density data")?;
            let x = u32::from_be_bytes(data.get(..4).ok_or("short density")?.try_into().map_err(|_| "short density")?);
            let y = u32::from_be_bytes(data.get(4..8).ok_or("short density")?.try_into().map_err(|_| "short density")?);
            if data.get(8) == Some(&1) && x > 0 && y > 0 {
                let scale = f64::from(x) * 0.0254 / 72.0 / f64::from(REDUCTION);
                if !(0.01..=16.0).contains(&scale) {
                    return Err("unsupported PNG physical density".into());
                }
                return Ok((scale, x != y, false));
            }
        }
        at = end;
    }
    Ok((300.0 / 72.0 / f64::from(REDUCTION), false, true))
}

#[derive(Clone, Copy, Debug)]
struct Metrics {
    ssim: f64,
    fraction: f64,
    mismatch: bool,
    changed: bool,
    overlap: (u32, u32),
}

fn gray([r, g, b]: [u8; 3]) -> f64 {
    0.2126 * f64::from(r) + 0.7152 * f64::from(g) + 0.0722 * f64::from(b)
}

/// Non-overlapping 8x8 box windows, population moments, pixel-weighted edge windows.
fn compare(a: &Raster, b: &Raster) -> Result<Metrics> {
    let (w, h) = (a.width.min(b.width), a.height.min(b.height));
    let total = count(w, h)? as f64;
    let mut score = 0.0;
    let mut differing = 0u64;
    let mismatch = (a.width, a.height) != (b.width, b.height);
    let mut changed = mismatch;
    for y0 in (0..h).step_by(WINDOW as usize) {
        for x0 in (0..w).step_by(WINDOW as usize) {
            let (mut sx, mut sy, mut xx, mut yy, mut xy, mut n) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            for y in y0..(y0 + WINDOW).min(h) {
                for x in x0..(x0 + WINDOW).min(w) {
                    let (p, q) = (a.pixel(x, y)?, b.pixel(x, y)?);
                    let different = p.into_iter().zip(q).any(|(p, q)| p.abs_diff(q) > DIFFERENCE);
                    differing += u64::from(different);
                    changed |= p != q;
                    let (v, u) = (gray(p), gray(q));
                    sx += v;
                    sy += u;
                    xx += v * v;
                    yy += u * u;
                    xy += v * u;
                    n += 1.0;
                }
            }
            let (mx, my) = (sx / n, sy / n);
            let (vx, vy, cov) = ((xx / n - mx * mx).max(0.0), (yy / n - my * my).max(0.0), xy / n - mx * my);
            let (c1, c2) = ((0.01_f64 * 255.0).powi(2), (0.03_f64 * 255.0).powi(2));
            let local = ((2.0 * mx * my + c1) * (2.0 * cov + c2)) / ((mx * mx + my * my + c1) * (vx + vy + c2));
            score += local.clamp(-1.0, 1.0) * n;
        }
    }
    Ok(Metrics { ssim: score / total, fraction: differing as f64 / total, mismatch, changed, overlap: (w, h) })
}

fn side_by_side(a: &Raster, b: &Raster) -> Result<Raster> {
    let w = a.width.checked_add(b.width).and_then(|v| v.checked_add(1)).ok_or("side-by-side width overflow")?;
    let h = a.height.max(b.height);
    let mut rgb = Vec::with_capacity(count(w, h)?);
    for y in 0..h {
        for x in 0..w {
            rgb.push(if x == a.width {
                [128; 3]
            } else if x < a.width && y < a.height {
                a.pixel(x, y)?
            } else if x > a.width && y < b.height {
                b.pixel(x - a.width - 1, y)?
            } else {
                [255; 3]
            });
        }
    }
    Raster::new(w, h, rgb)
}

fn safe_name(name: &str) -> Result<()> {
    if !name.as_bytes().first().is_some_and(u8::is_ascii_alphanumeric)
        || name.len() > 100
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.')
    {
        return Err(format!("invalid run or fixture name {name:?}"));
    }
    Ok(())
}

fn stem(path: &Path) -> Result<String> {
    let name = path.file_stem().and_then(|s| s.to_str()).ok_or("non-UTF-8 fixture name")?;
    safe_name(name)?;
    Ok(name.into())
}

fn files() -> Result<BTreeMap<String, PathBuf>> {
    let roots = std::env::var("PUB_CORPUS").map_err(|_| "set PUB_CORPUS to the controlled fixture folder")?;
    let mut result = BTreeMap::new();
    for root in roots.split(':').filter(|p| !p.is_empty()) {
        for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|s| s.eq_ignore_ascii_case("pub")) {
                let name = stem(&path)?;
                if result.insert(name.clone(), path).is_some() {
                    return Err(format!("duplicate fixture {name}"));
                }
            }
        }
    }
    if result.len() != 142 {
        return Err(format!("expected 142 controlled fixtures, found {}", result.len()));
    }
    Ok(result)
}

fn output_dir(name: &str) -> Result<PathBuf> {
    let path = Path::new(DATA).join(name);
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    let path = fs::canonicalize(path).map_err(|e| e.to_string())?;
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).ok_or("missing checkout root")?;
    if path.starts_with(fs::canonicalize(repo).map_err(|e| e.to_string())?) {
        return Err("generated output must stay outside the checkout".into());
    }
    Ok(path)
}

fn category(name: &str) -> &'static str {
    match name.split("__").next() {
        Some("table") => "tables",
        Some("text" | "text2") => "text",
        Some("wordart") => "wordart",
        Some("fill") => "fills",
        Some("line") => "lines",
        Some("shadow") => "shadows",
        Some("picture") => "pictures",
        Some("pages") => "pages",
        Some("group") => "groups",
        Some("geometry") => "geometry",
        Some("shapes") => "shapes",
        _ => "other",
    }
}

fn publisher() -> Result<()> {
    let fixtures = files()?;
    let out = output_dir("pixel-report")?;
    let mut references = BTreeMap::new();
    let folders = fixtures
        .values()
        .map(|p| p.parent().map(|p| p.join("golden")).ok_or_else(|| "fixture has no parent".to_string()))
        .collect::<Result<BTreeSet<_>>>()?;
    for folder in folders {
        if folder.is_dir() {
            for entry in fs::read_dir(folder).map_err(|e| e.to_string())? {
                let p = entry.map_err(|e| e.to_string())?.path();
                if p.extension().is_some_and(|s| s.eq_ignore_ascii_case("png")) && references.insert(stem(&p)?, p).is_some() {
                    return Err("duplicate reference name across corpus folders".into());
                }
            }
        }
    }
    if references.is_empty() {
        return Err("no Publisher reference PNGs found in golden/".into());
    }
    let mut rows = Vec::new();
    let mut csv = String::from(
        "name,category,ssim,differing_pixel_fraction,size_mismatch,ours_width,ours_height,golden_width,golden_height,overlap_width,overlap_height,assumed_300dpi\n",
    );
    for (name, path) in references {
        let (scale, anisotropic, assumed) = density(&read(&path)?)?;
        let golden = load_png(&path)?.reduced(REDUCTION)?;
        let input = fixtures.get(&name).ok_or_else(|| format!("no matching fixture for {name}"))?;
        let ours = render(input, scale)?;
        let mut m = compare(&ours, &golden)?;
        m.mismatch |= anisotropic;
        csv.push_str(&format!(
            "{name}.pub,{},{:.9},{:.9},{},{},{},{},{},{},{},{assumed}\n",
            category(&name),
            m.ssim,
            m.fraction,
            if m.mismatch { "yes" } else { "no" },
            ours.width,
            ours.height,
            golden.width,
            golden.height,
            m.overlap.0,
            m.overlap.1
        ));
        side_by_side(&ours, &golden)?.save(&out.join(format!("{name}-side-by-side.png")))?;
        println!("{name}: SSIM {:.6}, differing {:.6}, size mismatch {}", m.ssim, m.fraction, m.mismatch);
        rows.push((name, m));
    }
    let mut report = String::from(
        "# Publisher reference pixel baseline\n\nFirst page only. Left image: ours; right: Publisher. References are scoring data only.\n\nGrayscale SSIM: non-overlapping 8x8 box windows, population variance/covariance, K1=0.01, K2=0.03, L=255; partial windows weighted by pixels. Grayscale uses Rec.709 coefficients on sRGB bytes, without gamma linearization. Differing fraction: max RGB channel difference >8/255 after compositing on white. Golden images use 3x3 box reduction; ours renders directly at one third the PNG physical density (300 dpi assumed if unspecified). No stretching or alignment search. Size mismatches use only the top-left overlapping area; outside pixels are excluded.\n\nFonts and antialiasing affect scores. White margins can dominate. A good score does not prove correct reading. Only nine Publisher references are currently available, six WordArt; absent categories are unmeasured. No pass/fail thresholds.\n\n| Category | Count | Mean SSIM | Mean differing fraction |\n| --- | ---: | ---: | ---: |\n",
    );
    for cat in CATEGORIES {
        let selected: Vec<_> = rows.iter().filter(|(n, _)| category(n) == cat).collect();
        if selected.is_empty() {
            if cat != "other" {
                report.push_str(&format!("| {cat} | 0 | — | — |\n"));
            }
        } else {
            let n = selected.len() as f64;
            report.push_str(&format!(
                "| {cat} | {} | {:.6} | {:.6} |\n",
                selected.len(),
                selected.iter().map(|(_, m)| m.ssim).sum::<f64>() / n,
                selected.iter().map(|(_, m)| m.fraction).sum::<f64>() / n
            ));
        }
    }
    rows.sort_by(|a, b| a.1.ssim.total_cmp(&b.1.ssim).then_with(|| a.0.cmp(&b.0)));
    report.push_str("\n## References, lowest SSIM first\n\n| Fixture | SSIM | Differing fraction | Size mismatch |\n| --- | ---: | ---: | --- |\n");
    for (name, m) in rows {
        report.push_str(&format!("| {name}.pub | {:.6} | {:.6} | {} |\n", m.ssim, m.fraction, if m.mismatch { "yes" } else { "no" }));
    }
    write(&out.join("scores.csv"), csv)?;
    write(&out.join("report.md"), report)?;
    println!("Report: {}", out.display());
    Ok(())
}

fn snapshot(run: &str) -> Result<()> {
    safe_name(run)?;
    let fixtures = files()?;
    let root = output_dir("pixel-snapshots")?;
    let out = root.join(run);
    fs::create_dir(&out).map_err(|e| format!("new snapshot run {}: {e}; choose a new run name", out.display()))?;
    let start = Instant::now();
    let mut entries = Vec::new();
    for (name, file) in fixtures {
        let image = render(&file, SNAPSHOT_SCALE)?;
        image.save(&out.join(format!("{name}.png")))?;
        entries.push(json!({"name": name, "width": image.width, "height": image.height}));
    }
    let revision = Command::new("git").args(["rev-parse", "HEAD"]).current_dir(env!("CARGO_MANIFEST_DIR")).output().map_err(|e| e.to_string())?;
    if !revision.status.success() {
        return Err("cannot record source revision".into());
    }
    let seconds = start.elapsed().as_secs_f64();
    let manifest = json!({"schema":1,"kind":"self-regression, not Publisher","scale":SNAPSHOT_SCALE,"source_revision":String::from_utf8_lossy(&revision.stdout).trim(),"platform":std::env::consts::OS,"architecture":std::env::consts::ARCH,"elapsed_seconds":seconds,"fixtures":entries});
    write(&out.join("manifest.json"), serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?)?;
    println!("Snapshot {run}: 142 first pages in {seconds:.3} seconds at {SNAPSHOT_SCALE} px/pt; {}", out.display());
    Ok(())
}

fn manifest(root: &Path, run: &str) -> Result<(PathBuf, BTreeSet<String>)> {
    safe_name(run)?;
    let dir = fs::canonicalize(root.join(run)).map_err(|e| e.to_string())?;
    if !dir.starts_with(root) {
        return Err("snapshot path escapes its root".into());
    }
    let data: Value = serde_json::from_slice(&read(&dir.join("manifest.json"))?).map_err(|e| e.to_string())?;
    if data["schema"] != 1 || data["scale"].as_f64() != Some(SNAPSHOT_SCALE) {
        return Err("incompatible snapshot manifest".into());
    }
    let rows = data["fixtures"].as_array().ok_or("missing snapshot inventory")?;
    if rows.len() != 142 {
        return Err("snapshot inventory must contain 142 fixtures".into());
    }
    let mut names = BTreeSet::new();
    for row in rows {
        let name = row["name"].as_str().ok_or("snapshot entry has no name")?;
        safe_name(name)?;
        if !names.insert(name.to_string()) {
            return Err("duplicate snapshot entry".into());
        }
    }
    Ok((dir, names))
}

fn runs(before: &str, after: &str) -> Result<()> {
    let root = output_dir("pixel-snapshots")?;
    let (a, an) = manifest(&root, before)?;
    let (b, bn) = manifest(&root, after)?;
    let mut rows = Vec::new();
    let mut missing = Vec::new();
    for name in an.union(&bn) {
        if !an.contains(name) || !bn.contains(name) {
            missing.push(name);
            continue;
        }
        let m = compare(&load_png(&a.join(format!("{name}.png")))?, &load_png(&b.join(format!("{name}.png")))?)?;
        if m.changed {
            rows.push((name, m));
        }
    }
    rows.sort_by(|a, b| a.1.ssim.total_cmp(&b.1.ssim).then_with(|| b.1.fraction.total_cmp(&a.1.fraction)).then_with(|| a.0.cmp(b.0)));
    println!(
        "Own-output regression only, not a Publisher comparison. Compared {} common fixtures; {} changed; {} added/removed.",
        an.intersection(&bn).count(),
        rows.len(),
        missing.len()
    );
    println!("name,ssim,differing_pixel_fraction,size_mismatch");
    for (name, m) in rows {
        println!("{name}.pub,{:.9},{:.9},{}", m.ssim, m.fraction, m.mismatch);
    }
    for name in missing {
        println!("{name}.pub,not compared,not compared,added/removed");
    }
    Ok(())
}

#[test]
#[ignore = "optional visual baseline; needs external PUB_CORPUS and PUB_PIXEL_MODE"]
fn pub_pixel_diff() -> Result<()> {
    match std::env::var("PUB_PIXEL_MODE").as_deref() {
        Ok("publisher") => publisher(),
        Ok("snapshot") => snapshot(&std::env::var("PUB_PIXEL_RUN").map_err(|_| "set PUB_PIXEL_RUN")?),
        Ok("compare") => runs(
            &std::env::var("PUB_PIXEL_BEFORE").map_err(|_| "set PUB_PIXEL_BEFORE")?,
            &std::env::var("PUB_PIXEL_AFTER").map_err(|_| "set PUB_PIXEL_AFTER")?,
        ),
        _ => Err("set PUB_PIXEL_MODE=publisher, snapshot, or compare".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn require(ok: bool, message: &str) -> Result<()> {
        if ok { Ok(()) } else { Err(message.into()) }
    }
    fn solid(w: u32, h: u32, v: u8) -> Result<Raster> {
        Raster::new(w, h, vec![[v; 3]; count(w, h)?])
    }

    #[test]
    fn identical_and_opposite_constants_have_hand_computed_scores() -> Result<()> {
        let white = solid(8, 8, 255)?;
        let m = compare(&white, &white)?;
        require((m.ssim - 1.0).abs() < 1e-12 && m.fraction == 0.0 && !m.changed, "identity")?;
        let m = compare(&solid(8, 8, 0)?, &white)?;
        let c1 = (0.01_f64 * 255.0).powi(2);
        require((m.ssim - c1 / (255.0_f64.powi(2) + c1)).abs() < 1e-12 && m.fraction == 1.0, "opposite constants")
    }

    #[test]
    fn checkerboard_covariance_can_make_ssim_negative() -> Result<()> {
        let a = Raster::new(8, 8, (0..64).map(|i| [if (i % 8 + i / 8) % 2 == 0 { 0 } else { 255 }; 3]).collect())?;
        let b = Raster::new(8, 8, a.rgb.iter().map(|p| p.map(|v| 255 - v)).collect())?;
        let m = compare(&a, &b)?;
        let c2 = (0.03_f64 * 255.0).powi(2);
        let variance = 127.5_f64.powi(2);
        require((m.ssim - (c2 - 2.0 * variance) / (c2 + 2.0 * variance)).abs() < 1e-12 && m.fraction == 1.0, "negative covariance")
    }

    #[test]
    fn threshold_does_not_hide_small_changes_and_dimensions_never_stretch() -> Result<()> {
        let a = solid(1, 1, 0)?;
        let b = Raster::new(1, 1, vec![[8, 0, 0]])?;
        let m = compare(&a, &b)?;
        require(m.changed && m.fraction == 0.0, "small exact pixel change must be listed")?;
        require(compare(&a, &Raster::new(1, 1, vec![[9, 0, 0]])?)?.fraction == 1.0, "strict threshold")?;
        let m = compare(&solid(4, 4, 255)?, &solid(2, 2, 255)?)?;
        require(m.mismatch && m.changed && m.overlap == (2, 2) && (m.ssim - 1.0).abs() < 1e-12, "overlap, no stretch")
    }

    #[test]
    fn reduction_and_partial_windows_have_hand_computed_weights() -> Result<()> {
        let a = Raster::new(4, 4, (0..16).map(|i| [(i % 4 + 10 * (i / 4)) as u8; 3]).collect())?.reduced(3)?;
        require(a.rgb == vec![[11; 3], [13; 3], [31; 3], [33; 3]], "partial box means")?;
        let a = solid(9, 8, 255)?;
        let b = Raster::new(9, 8, (0..72).map(|i| [if i % 9 == 8 { 0 } else { 255 }; 3]).collect())?;
        let c1 = (0.01_f64 * 255.0).powi(2);
        let expected = 8.0 / 9.0 + c1 / (255.0_f64.powi(2) + c1) / 9.0;
        require((compare(&a, &b)?.ssim - expected).abs() < 1e-12, "edge windows weighted by area")?;
        require(white([128, 0, 0, 128]) == [255, 127, 127], "premultiplied alpha on paper")?;
        require(safe_name("text__size-36.5").is_ok(), "decimal fixture names")?;
        require(safe_name("../main-baseline").is_err(), "run traversal")
    }

    #[test]
    fn png_density_uses_one_uniform_scale_and_detects_anisotropy() -> Result<()> {
        let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
        bytes.extend(9u32.to_be_bytes());
        bytes.extend(b"pHYs");
        bytes.extend(11811u32.to_be_bytes());
        bytes.extend(11811u32.to_be_bytes());
        bytes.push(1);
        bytes.extend([0; 4]); // Chunk CRC is checked by the image decoder, not this metadata reader.
        let (scale, mismatch, assumed) = density(&bytes)?;
        require((scale - 1.388886111111111).abs() < 1e-12 && !mismatch && !assumed, "meter density")?;
        let y = bytes.get_mut(20..24).ok_or("test density position")?;
        y.copy_from_slice(&10000u32.to_be_bytes());
        require(density(&bytes)?.1, "non-square physical density flagged")?;
        require(density(b"\x89PNG\r\n\x1a\n")?.2, "missing density explicitly assumed")?;
        require((gray([255, 0, 0]) - 54.213).abs() < 1e-12, "grayscale coefficient")
    }
}
