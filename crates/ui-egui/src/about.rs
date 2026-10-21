//! NiftyDesign About screen and compiled-in contributor/model credits.
use crate::DesignApp;
use crate::theme::{Tokens, semibold};
use egui::{Color32, Rect, Sense, pos2, vec2};

pub const ABOUT_TEXT: &str = "NiftyDesign opens and edits older Microsoft Publisher (.pub) files, and also saves to PDF and prints. Based on DesignCraft by the ArtCraft team. Unofficial build; not supported by ArtCraft.";
pub const ABOUT_TABS: [&str; 3] = ["About", "Contributors", "Models"];

/// Paint the owner's original Nifty mark, preserving its colours and aspect ratio.
pub fn paint_mark(ui: &egui::Ui, rect: Rect) {
    let key = egui::Id::new("nifty_mark");
    let tex: Option<egui::TextureHandle> = ui.ctx().data(|d| d.get_temp(key));
    let tex = tex.or_else(|| {
        let pm = designcraft_render::decode_pixmap(include_bytes!("../../../assets/nifty/nifty-mark.png"))?;
        let image = egui::ColorImage::from_rgba_premultiplied([pm.width() as usize, pm.height() as usize], pm.data_as_u8_slice());
        let tex = crate::widgets::load_texture(ui.ctx(), "nifty_mark", image, egui::TextureOptions::LINEAR);
        ui.ctx().data_mut(|d| d.insert_temp(key, tex.clone()));
        Some(tex)
    });
    if let Some(tex) = tex {
        let size = tex.size_vec2();
        let scale = (rect.width() / size.x).min(rect.height() / size.y);
        let bounds = Rect::from_center_size(rect.center(), size * scale);
        ui.painter().image(tex.id(), bounds, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
    }
}

pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = Tokens::get(ctx);
    let area = egui::Id::new("about");
    let instance = area.with("placement");
    let resp = crate::modal_drag::modal(ctx, area, instance).show(ctx, |ui| {
        ui.set_width(if app.ui.about_tab == 0 { 460.0 } else { 640.0 });
        // The tab row is the title row. The tabs register after the drag area, so they stay clickable.
        let margin = egui::Frame::popup(ui.style()).inner_margin;
        let top = ui.cursor().top();
        let title_row = Rect::from_min_max(
            pos2(ui.max_rect().left() - margin.leftf(), top - margin.topf()),
            pos2(ui.max_rect().right() + margin.rightf(), top + ui.spacing().interact_size.y),
        );
        crate::modal_drag::title_drag(ui, instance, title_row);
        ui.horizontal(|ui| {
            for (i, l) in ABOUT_TABS.iter().enumerate() {
                let i = i as u8;
                if ui.selectable_label(app.ui.about_tab == i, crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, l))).clicked() {
                    app.ui.about_tab = i;
                }
            }
        });
        ui.separator();
        if app.ui.about_tab != 0 {
            let h = 400.0_f32.min(ctx.content_rect().height() * 0.7).max(160.0);
            ui.allocate_ui_with_layout(vec2(ui.available_width(), h), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_min_height(h);
                if app.ui.about_tab == 1 {
                    crate::credits::contributors_ui(ui, &app.ui.language);
                } else {
                    crate::credits::models_ui(ui, &app.ui.language);
                }
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                    app.ui.about = false;
                }
            });
            return;
        }
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            let (r, _) = ui.allocate_exact_size(vec2(72.0, 72.0), Sense::hover());
            paint_mark(ui, r);
            ui.add_space(6.0);
            ui.label(egui::RichText::new("NiftyDesign").font(semibold(26.0)).color(t.text_strong));
            ui.label(egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).size(12.0).color(t.text_dim));
            ui.add_space(12.0);
            ui.label(egui::RichText::new(ABOUT_TEXT).size(13.0).color(t.text));
            ui.add_space(8.0);
            if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                app.ui.about = false;
            }
        });
    });
    if resp.should_close() {
        app.ui.about = false;
    }
}
