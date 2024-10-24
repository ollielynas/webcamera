use egui::{Color32, RichText, Ui};
use strum_macros::EnumIter;

use crate::MyApp;

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
                ui.vertical_centered(|ui| {
                    if ui.button(RichText::new("Take Photo").color(Color32::RED)).clicked() {
                        self.save_photo();
                    }
                });
                
                ui.label("text");
            },
            UiTab::SavePhoto => {

            },
        }
        return Ok(())
    }
}