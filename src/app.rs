use std::time::Duration;

use anyhow::anyhow;
use egui::{pos2, Button, CentralPanel, Color32, Frame, Image, Pos2, Rect, SidePanel, TextureHandle, ThemePreference};
use strum::IntoEnumIterator;
use web_sys::{window, Navigator};

use crate::{image::capture_frame, render::UiTab};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {

    pub ui_tab: UiTab,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub texture: Option<TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        
        Self {
            texture: None,
            ui_tab: UiTab::default(),
        }
    }
}

impl MyApp {

    
    
    fn init_webcam() -> anyhow::Result<()> {
        
        let elem = window().ok_or(anyhow!("no window"))?
        .document().ok_or(anyhow!("no document"))?
        .get_element_by_id("videoElement").ok_or(anyhow!("video element not found"))?;
        let canvas = window().ok_or(anyhow!("no window"))?
        .document().ok_or(anyhow!("no document"))?
        .get_element_by_id("canvas").ok_or(anyhow!("video element not found"))?;

        canvas.set_attribute("width", &(elem.client_width()/10).to_string());
        canvas.set_attribute("height", &(elem.client_height()/10).to_string());
        Ok(())
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_zoom_factor(2.5);
        cc.egui_ctx.set_theme(ThemePreference::Light);

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
        
        cc.egui_ctx.all_styles_mut(|style| {
            
        });
        

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn update_texture(&mut self, ctx: &egui::Context) -> anyhow::Result<()> {
        let (width, height, data) = capture_frame()?;

        match self.texture {
            Some(ref mut a) if a.size() == [width as usize, height as usize]  => {
                (*a).set_partial([0,0], egui::ColorImage::from_rgba_premultiplied([width as usize,height as usize], &data), egui::TextureOptions {
                    ..Default::default()
                });
            },
            _ => {
                self.texture = Some(ctx.load_texture("texture", egui::ColorImage::from_rgba_premultiplied([width as usize,height as usize], &data), egui::TextureOptions {
                    ..Default::default()
                }));
            },
        }

        Ok(())
    }
}

impl eframe::App for MyApp {

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.update_texture(ctx);
        
        

        SidePanel::left("display")
        .exact_width(ctx.screen_rect().width()*0.5)
        .show(ctx, |ui| {
            match self.texture {
                Some(ref a) => {
                    let mut image_rect = Rect::from_x_y_ranges(0.0..=a.size()[0] as f32, 0.0..=a.size()[1] as f32);
                    image_rect.set_center(ui.available_rect_before_wrap().center());
                    image_rect= image_rect.scale_from_center(
                        (ui.available_rect_before_wrap().width()/a.size()[0] as f32)
                        .min(
                            ui.available_rect_before_wrap().height()/a.size()[1] as f32
                        )
                    );
                    ui.painter_at(image_rect).image(a.id(), 
                    image_rect
                    , Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                },
                None => {
                    ui.label("failed to get video");
                },
            }
        });
        SidePanel::right("right hand panel").resizable(false)
        .show_separator_line(false)
        .exact_width(32.0)
        .show(ctx, |ui| {
            
            ui.vertical_centered_justified(|ui| {
            for i in UiTab::iter() {
                if ui.add_enabled(i != self.ui_tab, Button::new(egui::RichText::new(i.icon()))).clicked() {
                    self.ui_tab = i;
                }
            }
            });
        });
        CentralPanel::default()
        .show(ctx, |ui| {
            match self.render_center(ui) {
                Ok(_) => {},
                Err(e) => {
                    ui.label(format!("{:#?}", e));
                },
            }
        });
        





        ctx.request_repaint_after(Duration::from_secs_f32((1.0/60.0)));
    }
}

