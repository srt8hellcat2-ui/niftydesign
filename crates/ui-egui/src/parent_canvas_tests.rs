use super::*;

fn app_with_document_object() -> DesignApp {
    let mut app = DesignApp::new(designcraft_engine::Session::new(), crate::Services::default());
    app.run("file.new", json!({"width": 64, "height": 64, "pages": 1, "facingPages": true, "margins": 4})).unwrap();
    app.run("frame.create", json!({"rect": [24, 24, 40, 40], "content": "unassigned"})).unwrap();
    app.run("object.fill", json!({"swatch": "[Black]"})).unwrap();
    // Exercise the production texture cache without starting a background thread.
    app.canvas.worker_started = true;
    app
}

fn render(app: &mut DesignApp, ctx: &egui::Context) -> egui::epaint::ImageDelta {
    render_at_zoom(app, ctx, 1.0)
}

fn render_at_zoom(app: &mut DesignApp, ctx: &egui::Context, zoom: f64) -> egui::epaint::ImageDelta {
    let rect = Rect::from_min_size(Pos2::ZERO, vec2(128.0, 64.0));
    let view = View { zoom, origin: Point::new(-64.0, 0.0), fitted: true, rotation: 0 };
    let mut output = ctx.run_ui(Default::default(), |ui| {
        let st = app.session.active().unwrap();
        let layout = CanvasLayout::new(&st.doc, st.editing_parents);
        render_texture(app, ui.ctx(), rect, &Xf::new(rect, &view), &layout, false);
    });
    let id = app.canvas.texture.as_ref().unwrap().id();
    let deltas = output.textures_delta.set.remove(&id);
    output.textures_delta.clear();
    let mut deltas = deltas.expect("canvas pixels must update");
    assert_eq!(deltas.len(), 1, "one canvas render should produce one texture update");
    deltas.pop().unwrap()
}

fn pixel(delta: &egui::epaint::ImageDelta, shown: crate::Shown, point: Point) -> Color32 {
    let egui::ImageData::Color(image) = &delta.image;
    let offset = delta.pos.unwrap_or([0, 0]);
    let x = ((point.x - shown.origin.x) * shown.zoom).round() as usize - offset[0];
    let y = ((point.y - shown.origin.y) * shown.zoom).round() as usize - offset[1];
    image[(x, y)]
}

#[test]
fn switching_parent_editing_repaints_pages_and_document_objects() {
    let mut app = app_with_document_object();
    let ctx = egui::Context::default();
    let left_page = Point::new(-32.0, 32.0);
    let document_object = Point::new(32.0, 32.0);
    let before = render(&mut app, &ctx);
    let shown = app.canvas.shown.unwrap();
    assert_eq!(pixel(&before, shown, left_page), Color32::TRANSPARENT);
    let document_color = pixel(&before, shown, document_object);
    assert_ne!(document_color, Color32::WHITE);
    assert_eq!(document_color.a(), 255);

    let doc = app.session.active().unwrap().doc.clone();
    app.run("layout.parents.edit", json!({"on": true})).unwrap();
    assert!(std::sync::Arc::ptr_eq(&doc, &app.session.active().unwrap().doc), "switching canvas mode does not edit the document");
    let parent = render(&mut app, &ctx);
    assert!(parent.pos.is_none(), "the parent canvas needs a complete repaint");
    let shown = app.canvas.shown.unwrap();
    assert_eq!(pixel(&parent, shown, left_page), Color32::WHITE, "the parent's left page must become visible");
    assert_eq!(pixel(&parent, shown, document_object), Color32::WHITE, "document-only objects must disappear in parent editing");

    app.run("layout.parents.edit", json!({"on": false})).unwrap();
    let restored = render(&mut app, &ctx);
    assert!(restored.pos.is_none());
    let shown = app.canvas.shown.unwrap();
    assert_eq!(pixel(&restored, shown, left_page), Color32::TRANSPARENT);
    assert_eq!(pixel(&restored, shown, document_object), document_color);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn switching_parent_editing_with_a_new_zoom_repaints_before_the_worker() {
    let mut app = app_with_document_object();
    let ctx = egui::Context::default();
    app.canvas.worker = Some(crate::render_worker::Worker::spawn(ctx.clone()).expect("render worker must start"));
    let before = render(&mut app, &ctx);
    assert_ne!(pixel(&before, app.canvas.shown.unwrap(), Point::new(32.0, 32.0)), Color32::WHITE);

    // Fitting the parent spread can change the zoom as well as the canvas mode.
    app.run("layout.parents.edit", json!({"on": true})).unwrap();
    let parent = render_at_zoom(&mut app, &ctx, 0.75);
    assert!(parent.pos.is_none(), "a mode switch must upload the new canvas in this frame");
    assert!(app.canvas.pending.is_none(), "parent pages must not wait behind an asynchronous zoom render");
    let shown = app.canvas.shown.unwrap();
    assert_eq!(shown.zoom, 0.75);
    assert_eq!(pixel(&parent, shown, Point::new(-32.0, 32.0)), Color32::WHITE);
    assert_eq!(pixel(&parent, shown, Point::new(32.0, 32.0)), Color32::WHITE);
}

#[test]
fn small_edits_in_the_same_canvas_mode_still_repaint_partially() {
    let mut app = app_with_document_object();
    let ctx = egui::Context::default();
    let before = render(&mut app, &ctx);
    let point = Point::new(32.0, 32.0);
    assert_ne!(pixel(&before, app.canvas.shown.unwrap(), point), Color32::WHITE);

    app.run("object.fill", json!({"swatch": "[Paper]"})).unwrap();
    let patch = render(&mut app, &ctx);
    assert!(patch.pos.is_some(), "an ordinary small edit should keep the partial repaint optimization");
    assert_eq!(app.canvas.patches, 1);
    assert_eq!(pixel(&patch, app.canvas.shown.unwrap(), point), Color32::WHITE);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn screenshot_after_native_reopen_refreshes_the_canvas_before_readback() {
    for (name, bytes) in [
        (
            "red.svg",
            br##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16"><rect width="16" height="16" fill="#ff0000"/></svg>"##.to_vec(),
        ),
        ("blue.png", designcraft_render::Rendered { width: 2, height: 2, pixels: [0, 0, 255, 255].repeat(4) }.to_png()),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let picture = dir.path().join(name);
        std::fs::write(&picture, &bytes).unwrap();
        let saved = dir.path().join("placed.designcraft");
        let mut app = app_with_document_object();
        app.run("file.place", json!({"path": picture, "spread":0, "x":24, "y":24, "width":16})).unwrap();
        app.run("file.saveAs", json!({"path":saved})).unwrap();
        let ctx = egui::Context::default();
        app.canvas.worker = Some(crate::render_worker::Worker::spawn(ctx.clone()).unwrap());
        let before = render(&mut app, &ctx);
        let expected = pixel(&before, app.canvas.shown.unwrap(), Point::new(32.0, 32.0));
        assert_ne!(expected, Color32::WHITE, "native picture must be visible before reopen");
        app.run("file.open", json!({"path":saved})).unwrap();
        assert!(app.session.active().unwrap().doc.assets.values().any(|a| a.data.as_slice() == bytes));
        let (request, _) = crate::ControlRequest::new("ui.screenshot", json!({"focus":false}));
        assert!(matches!(crate::control::handle(&mut app, &ctx, &request), crate::control::Outcome::Screenshot { .. }));
        // A different fitted zoom would normally queue a background render and
        // leave the previous document's texture visible until a later frame.
        let after = render_at_zoom(&mut app, &ctx, 0.75);
        assert!(after.pos.is_none(), "capture needs a complete current viewport");
        assert!(app.canvas.pending.is_none(), "capture must not depend on worker timing");
        assert_eq!(pixel(&after, app.canvas.shown.unwrap(), Point::new(32.0, 32.0)), expected, "{name}");
    }
}
