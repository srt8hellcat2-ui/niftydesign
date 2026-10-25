// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Default ECMA textArchUp/textCircle paths; no unverified adjustment conversion.
use designcraft_geom::{BezPath, PathData, Rect, kurbo::Arc};
use std::f64::consts::PI;

/// Only the two fixture-confirmed MS-ODRAW numbers are mapped. The XML's default
/// guides reduce to a left-middle start, half radii and these sweeps. Circle is
/// deliberately just short of a full turn (21599998 / 60000 degrees), not closed.
pub(crate) fn default_path(kind: u16, rect: Rect) -> Option<PathData> {
    if !matches!(kind, 144 | 146)
        || ![rect.x0, rect.y0, rect.x1, rect.y1].iter().all(|v| v.is_finite())
        || rect.width() <= 0.0
        || rect.height() <= 0.0
        || rect.width() > 1_000_000.0
        || rect.height() > 1_000_000.0
    {
        return None;
    }
    let sweep = if kind == 144 {
        PI
    } else {
        // DrawingML angles are radial directions. On a noncircular ellipse the
        // final two-unit gap needs conversion to the ellipse parameter angle.
        let gap = (2.0 / 60_000.0_f64).to_radians();
        2.0 * PI - (rect.width() * gap.sin()).atan2(rect.height() * gap.cos())
    };
    let mut path = BezPath::new();
    path.move_to((rect.x0, rect.center().y));
    Arc::new(rect.center(), (rect.width() / 2.0, rect.height() / 2.0), PI, sweep, 0.0).to_cubic_beziers(0.001, |a, b, end| path.curve_to(a, b, end));
    Some(PathData::from_bezpath(&path))
}

/// A conservative average advance estimate, not a stored Publisher size or font
/// measurement. Only curved text uses this; straight WordArt keeps its old size.
pub(crate) fn estimated_size(path: &PathData, letters: usize) -> Option<f64> {
    let len = designcraft_geom::warp::PathWarp::new(&path.to_bezpath(), false).length();
    let size = len / (letters.max(1) as f64 * 0.6);
    (size.is_finite() && size >= 0.1).then_some(size.min(1296.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use designcraft_geom::{Point, warp::PathWarp};

    fn near(a: f64, b: f64) -> Result<(), String> {
        if (a - b).abs() < 0.002 { Ok(()) } else { Err(format!("{a} != {b}")) }
    }

    #[test]
    fn default_arch_and_glyph_tangent_have_hand_computed_coordinates() -> Result<(), String> {
        // Circle of radius 50: left (10,70), top (60,20), right (110,70).
        let path = default_path(144, Rect::new(10.0, 20.0, 110.0, 120.0)).ok_or("no arch")?;
        let warp = PathWarp::new(&path.to_bezpath(), false);
        near(warp.length(), PI * 50.0)?;
        let (start, a) = warp.at(0.0).ok_or("no start")?;
        near(start.x, 10.0)?;
        near(start.y, 70.0)?;
        near(a, -PI / 2.0)?;
        let mid = warp.length() / 2.0;
        let (top, a) = warp.at(mid).ok_or("no midpoint")?;
        near(top.x, 60.0)?;
        near(top.y, 20.0)?;
        near(a, 0.0)?;
        // A glyph centre at x=mid, baseline 12 lands at the top, upright.
        let xf = warp.glyph(mid, 12.0).ok_or("no glyph transform")?;
        let p = xf * Point::new(mid, 12.0);
        near(p.x, 60.0)?;
        near(p.y, 20.0)?;
        let above = xf * Point::new(mid, 2.0);
        near(above.x, 60.0)?;
        near(above.y, 10.0)?;
        let end = warp.at(warp.length()).ok_or("no end")?.0;
        near(end.x, 110.0)?;
        near(end.y, 70.0)?;
        near(estimated_size(&path, 10).ok_or("no size")?, PI * 50.0 / 6.0)
    }

    #[test]
    fn default_circle_is_open_and_almost_one_turn() -> Result<(), String> {
        let path = default_path(146, Rect::new(0.0, 0.0, 100.0, 100.0)).ok_or("no circle")?;
        if path.subpaths.iter().any(|p| p.closed) {
            return Err("circle closed".into());
        }
        let warp = PathWarp::new(&path.to_bezpath(), false);
        near(warp.length(), PI * 100.0)?;
        let (p, a) = warp.at(warp.length() / 2.0).ok_or("no opposite point")?;
        near(p.x, 100.0)?;
        near(p.y, 50.0)?;
        near(a, PI / 2.0)?;
        let xf = warp.glyph(warp.length() / 2.0, 12.0).ok_or("no glyph")?;
        let p = xf * Point::new(warp.length() / 2.0, 2.0);
        near(p.x, 110.0)?;
        near(p.y, 50.0)
    }

    #[test]
    fn ellipse_circle_endpoint_uses_radial_angle_not_parameter_angle() -> Result<(), String> {
        let path = default_path(146, Rect::new(0.0, 0.0, 200.0, 100.0)).ok_or("no ellipse")?;
        let warp = PathWarp::new(&path.to_bezpath(), false);
        let end = warp.at(warp.length()).ok_or("no ellipse endpoint")?.0;
        // A two-unit gap = pi/5400000 radians. At the left radius of 100,
        // dy is 100*pi/5400000 = 0.000058177641733; dx differs only at order gap².
        if end.x.abs() > 1e-8 || (end.y - 50.000_058_177_641_73).abs() > 1e-8 {
            return Err(format!("wrong radial-angle endpoint: {end:?}"));
        }
        Ok(())
    }

    #[test]
    fn unsupported_and_invalid_paths_are_declined() -> Result<(), String> {
        for (kind, r) in [
            (145, Rect::new(0.0, 0.0, 100.0, 100.0)),
            (156, Rect::ZERO),
            (144, Rect::ZERO),
            (146, Rect::new(f64::NAN, 0.0, 100.0, 100.0)),
            (144, Rect::new(0.0, 0.0, 1e20, 100.0)),
        ] {
            if default_path(kind, r).is_some() {
                return Err("invalid path accepted".into());
            }
        }
        Ok(())
    }
}
