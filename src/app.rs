use std::{sync::Arc};


use anyhow::anyhow;
use egui::{pos2, Button, CentralPanel, Color32, Frame, Image, Pos2, Rect, RichText, SidePanel, TextureHandle, ThemePreference};
use strum::IntoEnumIterator;
use web_sys::{window, Navigator};

use crate::{image::MyImage, render::UiTab};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {

    pub ui_tab: UiTab,
    pub save_options: SaveImageOptions,

    pub photos: Vec<MyImage>,


    #[serde(skip)] // This how you opt-out of serialization of a field
    pub texture: Option<TextureHandle>,


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
        

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut s = MyApp::default();
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
            img = &perm_img;
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.update_texture(ctx);

        SidePanel::left("display")
        .exact_width(ctx.screen_rect().width()*0.5)
        .resizable(false)
        .show_separator_line(false)
        .show(ctx, |ui| {
            match self.texture {
                // render image
                Some(ref a) => {
                    let mut image_rect = Rect::from_x_y_ranges(0.0..=a.size()[0] as f32, 0.0..=a.size()[1] as f32);
                    image_rect.set_center(ui.available_rect_before_wrap().center());
                    image_rect= image_rect.scale_from_center(
                        (ui.available_rect_before_wrap().width()/a.size()[0] as f32)
                        .min(
                            ui.available_rect_before_wrap().height()/a.size()[1] as f32
                        )
                    );
                    if self.photos.len() == 0 || self.ui_tab != UiTab::SavePhoto {
                    ui.label(
                        RichText::new("low res preview").italics()
                        .small()
                        
                    );
                    }
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

        ctx.request_repaint_after(web_time::Duration::from_secs_f32(1.0/60.0));
    }
}

