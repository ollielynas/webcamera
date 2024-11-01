use egui::{Color32, Stroke, Ui};
use egui_plot::{Line, PlotPoints};
use oklab::Rgb;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{image::MyImage, MyApp};

#[derive(serde::Deserialize, serde::Serialize, EnumIter, PartialEq, Eq)]
pub enum HistogramType {
    Rgb,
    OkLab,
}
impl Default for HistogramType {
    fn default() -> Self {
        HistogramType::Rgb
    }
}

impl HistogramType {
    fn name(&self) -> &str {
        match self {
            HistogramType::Rgb => "RGB",
            HistogramType::OkLab => "OkLab",
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HistogramData {
    type_: HistogramType,
    #[serde(skip)]
    data: [[f32; 3]; 64],
}

impl Default for HistogramData {
    fn default() -> Self {
        HistogramData {
            type_: HistogramType::Rgb,
            data: [[1_f32; 3]; 64],
        }
    }

}
impl HistogramData {
    fn update(&mut self, image: &MyImage) {
        self.data = [[0_f32;3];64];
        let mut max: f32 = 0.0;
        let mut max_b: f32 = 0.0;
        for i in 0..(image.bytes.len() / 4) {
            let r = image.bytes[i * 4];
            let g = image.bytes[i * 4 + 1];
            let b = image.bytes[i * 4 + 2];
            match self.type_ {
                HistogramType::Rgb => {
                    self.data[r as usize / 4][0] += 1.0;
                    self.data[g as usize / 4][1] += 1.0;
                    self.data[b as usize / 4][2] += 1.0;
                },
                HistogramType::OkLab => {
                    let ok_lab = oklab::srgb_to_oklab(Rgb{r,g,b});
                    self.data[((ok_lab.a + 0.5)* 64.0) as usize][0] += 1.0;
                    self.data[((ok_lab.b + 0.5)* 64.0) as usize][1] += 1.0;
                    self.data[(ok_lab.l * 63.0) as usize][2] += 1.0;
                    max_b = max_b.max(self.data[(ok_lab.l * 63.0) as usize][2]);
                }
            }
        }

        

        for d in self.data {
            for p in d {
                max = max.max(p);
            }
        }
        for d in &mut self.data {
            for (i,p) in d.iter_mut().enumerate() {
                if self.type_ == HistogramType::OkLab && i == 2 {
                    *p /= max_b * 1.1;
                }else {
                *p /= max * 1.1;
            }
            }
        }

    }
}

impl MyApp {
    pub fn render_histogram(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            for i in HistogramType::iter() {
                ui.add_enabled_ui(i!=self.histogram.type_, |ui| {
                    if ui.small_button(i.name()).clicked() {
                        self.histogram.type_ = i;
                    }
                });
            }
        });
        self.histogram.update(&self.photo);
        egui_plot::Plot::new("histogram")
            .allow_scroll(false)
            .allow_zoom(false)
            .allow_drag(false)
            .include_y(1.0)
            .include_x(64)
            .show(ui, |plot_ui| {
                let mut lines = vec![];

                match self.histogram.type_ {
                    HistogramType::Rgb => {
                        for i in 0..3 {
                            lines.push(
                                Line::new(
                                    self.histogram
                                        .data
                                        .iter()
                                        .enumerate()
                                        .map(|x| [x.0 as f64, x.1[i] as f64])
                                        .collect::<Vec<[f64; 2]>>(),
                                )
                                .stroke(Stroke::new(0.8, [Color32::RED, Color32::DARK_GREEN, Color32::BLUE][i]))
                                ,
                            );
                        }
                    }
                    HistogramType::OkLab => {
                        for index in 0..5 {
                            let i = index % 3;
                            lines.push(
                                Line::new(
                                    self.histogram
                                        .data
                                        .iter()
                                        .enumerate()
                                        .filter(|x| 
                                            match index {
                                                0..=1 => x.0 <= 32,
                                                3..=4 => x.0 >= 32,
                                                _ => true,
                                            }
                                        )
                                        .map(|x| [x.0 as f64, x.1[i] as f64])
                                        .collect::<Vec<[f64; 2]>>(),
                                )
                                .stroke(
                                    Stroke::new(0.8, [Color32::GREEN, Color32::BLUE, Color32::DARK_GRAY,Color32::RED,Color32::YELLOW][index])
                                )
                                ,
                            );
                        }
                    }
                }

                for l in lines {
                    plot_ui.line(l);
                }
            });
    }
}
