

use std::sync::Arc;

use eframe::glow::Context;
use egui::{Button, CentralPanel, SidePanel, TextureHandle, ThemePreference, TopBottomPanel};
use strum::IntoEnumIterator;

use crate::{image::MyImage, image_info::HistogramData, render::UiTab};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {

    pub ui_tab: UiTab,
    pub save_options: SaveImageOptions,
    pub histogram: HistogramData,

    pub photos: Vec<MyImage>,
    
    #[serde(skip)] // This how you opt-out of serialization of a field
    pub gl:  Option<Arc<Context>>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub texture: Option<TextureHandle>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub photo: MyImage,


}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SaveImageOptions {
    pub raw_png: bool,
    pub jpg: bool,
    pub png: bool,
    pub image_index: i32,
}

impl Default for SaveImageOptions {
    fn default() -> Self {
        SaveImageOptions {
            raw_png: false,
            jpg: true,
            png: false,
            image_index: 0,
        }
    }
}
impl Default for MyApp {
    fn default() -> Self {
        
        Self {
            texture: None,
            photos: vec![],
            ui_tab: UiTab::default(),
            save_options: SaveImageOptions::default(),
            gl: None,
            photo: MyImage::default(),
            histogram: HistogramData::default(),
        }
    }
}

impl MyApp {



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
        cc.egui_ctx.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });


        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut s = MyApp::default();
        s.gl = cc.gl.clone();
        return s;
    }

    pub fn update_texture(&mut self, ctx: &egui::Context) -> anyhow::Result<()> {
        let mut perm_img = MyImage::default();
        let img;
        self.save_options.image_index = self.save_options.image_index.clamp(0, (self.photos.len() as i32 -1).max(0));
        if self.ui_tab == UiTab::SavePhoto && self.save_options.image_index < self.photos.len() as i32 && self.photos.len()>0 {
            
            img = &self.photos[self.save_options.image_index as usize];
            self.process_image(&mut perm_img);
        }else {
            perm_img = self.capture_frame(false)?;
            self.process_image(&mut perm_img);
            self.photo = perm_img;
            img = &self.photo;
        }
        match self.texture {
            Some(ref mut a) if a.size() == [img.width as usize, img.height as usize]  => {
                (*a).set_partial([0,0], egui::ColorImage::from_rgba_premultiplied([img.width as usize,img.height as usize], &img.bytes), egui::TextureOptions {
                    ..Default::default()
                });
            },
            _ => {
                self.texture = Some(ctx.load_texture("texture", egui::ColorImage::from_rgba_premultiplied([img.width as usize,img.height as usize], &img.bytes), egui::TextureOptions {
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        self.update_texture(ctx);
        let landscape = ctx.screen_rect().aspect_ratio() > 1.0;

        if landscape {
        SidePanel::left("display")
        .exact_width(ctx.screen_rect().width()*0.5)
        .resizable(false)
        .show_separator_line(false)
        .show(ctx, |ui| {
            self.render_viewport(ui);
        });
        }else {
            TopBottomPanel::top("display")
        .exact_height(ctx.screen_rect().height()*0.5)
        .resizable(false)
        .show_separator_line(false)
        .show(ctx, |ui| {
            self.render_viewport(ui);
        });
        };
        SidePanel::right("right hand panel").resizable(false)
        .show_separator_line(false)
        .exact_width(32.0)
        .resizable(false)
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
            egui::ScrollArea::vertical().show(ui, |ui| {
            match self.render_center(ui) {
                Ok(_) => {},
                Err(e) => {
                    ui.label(format!("{:#?}", e));
                },
            }});
        });

        ctx.request_repaint_after(web_time::Duration::from_secs_f32(1.0/60.0));
    }
}

