use anyhow::anyhow;
use chrono::{DateTime, Local};
use egui::{Color32, RichText, Ui};
use egui_phosphor::regular::{CARET_LEFT, CARET_RIGHT};
use strum_macros::EnumIter;
use web_sys::window;

use crate::{app::SaveImageOptions, file_stuff::download_zip_file, MyApp};

#[derive(serde::Deserialize, serde::Serialize, EnumIter, PartialEq)]
pub enum UiTab {
    TakePhoto,
    SavePhoto,
}

impl Default for UiTab {
    fn default() -> Self {
        UiTab::TakePhoto
    }
}

impl UiTab {
    pub fn icon(&self) -> &str {
        match self {
            UiTab::TakePhoto => egui_phosphor::regular::APERTURE,
            UiTab::SavePhoto => egui_phosphor::regular::FLOPPY_DISK,
            
        }
    }
}



impl MyApp {
    pub fn render_center(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        match self.ui_tab {
            UiTab::TakePhoto => {
                self.render_photo_ui(ui)?;
            },
            UiTab::SavePhoto => {
                self.render_save_ui(ui);
            },
        }
        return Ok(())
    }


    fn render_photo_ui(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        ui.vertical_centered(|ui| {
            if ui.button(RichText::new("Take Photo").color(Color32::RED)).clicked() {
                self.take_photo(ui.ctx());
            }
        });
        
        match &self.texture {
            Some(a) => {
            }
            None => {
                ui.label("no image found");
                if ui.small_button("reloading may help").clicked() {
                    window().ok_or(anyhow!("failed to get window"))?.location().reload_with_forceget(true);
                }
            }
            
        }
        return Ok(());
    }
    fn render_save_ui(&mut self, ui: &mut Ui) -> anyhow::Result<(())> {
        
        let index = self.save_options.image_index.max(0) as usize;
        if index < self.photos.len() && !self.photos[index as usize].del {
            ui.label(RichText::new(self.photos[index].name.clone()).strong());
            ui.horizontal(|ui| {
                if ui.button(CARET_LEFT).clicked() {
                    self.save_options.image_index -= 1;
                    if self.save_options.image_index < 0 {
                        self.save_options.image_index = self.photos.len() as i32 -1;
                    }

                    self.save_options.image_index = self.save_options.image_index % self.photos.len() as i32;
                };
                ui.label(
                    RichText::new(
                    format!("{}/{}",(index+1).min(self.photos.len()),self.photos.len())).strong());
                if ui.button(CARET_RIGHT).clicked() {
                    self.save_options.image_index += 1;
                    self.save_options.image_index = self.save_options.image_index % self.photos.len() as i32;
                };
            });
            ui.checkbox(&mut self.photos[index].save, "save this image");
            if ui.button("delete photo").clicked() {
                self.photos[index as usize].del = true;
            };

                ui.separator();
                ui.label(format!("{} photo/s selected to be saved", self.photos.iter().filter(|x|x.save).count()));
                
                // come back to this when i decide if I want to stor3 both raw and cooked photos
                // ui.checkbox(&mut self.save_options.raw_png, "raw png");

                ui.checkbox(&mut self.save_options.jpg, "jpeg");
                ui.checkbox(&mut self.save_options.png, "png");

                if ui.button("download").clicked() {
                    let local: DateTime<Local> = Local::now();
                    let photos = self.save_photos(ui.ctx()).unwrap();
                    download_zip_file(photos, format!("{} Photos.zip", local.format("%Y-%m-%d %H-%M-%S").to_string())).unwrap();
                }

        } else if self.photos.len() > 0 && self.photos[index as usize].del  && index < self.photos.len() {
            ui.label(format!("are you sure you want to delete \"{}\"", self.photos[index].name));
            if ui.button("no, cancel").clicked() {
                self.photos[index].del = false;
            }
            if ui.button("yes, delete").clicked() {
                self.photos.remove(index);
            }
        }else {
            ui.label("no photos have been taken");
        }
        Ok(())
    }
}


