use std::io::{BufWriter, Cursor, Read};

use async_zip::{base::write::ZipFileWriter, ZipEntryBuilder};
use chrono::{DateTime, Local, TimeZone, Utc};
use eframe::glow::Buffer;
use ::image::{DynamicImage, ImageBuffer, Pixel, Rgba};
use anyhow::anyhow;
use egui::{
    epaint::{image, TextureManager},
    load::SizedTexture,
    CentralPanel, Context, Image, LayerId,
};
use pollster::FutureExt;
// use ::image::{DynamicImage, ImageBuffer};
use crate::MyApp;
use wasm_bindgen::JsCast;
use web_sys::{window, Navigator};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyImage {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
    pub save: bool,
    pub name: String,
    pub del: bool,
}
impl Default for MyImage {
    fn default() -> Self {
        let local: DateTime<Local> = Local::now();
        MyImage {
            name: local.format("%Y-%m-%d %H-%M-%S").to_string(),
            save: true,
            width: 0,
            height: 0,
            bytes: vec![],
            del: false,
        }
    }
}

impl MyApp {
    pub fn process_image(&self, img: &mut MyImage) {
        
    }

    pub fn capture_frame(&self, full_quality: bool) -> anyhow::Result<(MyImage)> {
        let video = window()
            .ok_or(anyhow!("no window"))?
            .document()
            .ok_or(anyhow!("no document"))?
            .get_element_by_id("videoElement")
            .ok_or(anyhow!("video element not found"))?;
        let canvas = window()
            .ok_or(anyhow!("no window"))?
            .document()
            .ok_or(anyhow!("no document"))?
            .get_element_by_id("canvas")
            .ok_or(anyhow!("video element not found"))?;
        let devi = if full_quality { 1 } else { 5 };

        canvas.set_attribute("width", &(video.client_width()).to_string());
        canvas.set_attribute("height", &(video.client_height()).to_string());

        let video: web_sys::HtmlVideoElement = video
            .dyn_into::<web_sys::HtmlVideoElement>()
            .map_err(|_| ())
            .unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context
            .draw_image_with_html_video_element_and_dw_and_dh(
                &video,
                0.0,
                0.0,
                (canvas.width() as i32 / devi) as f64,
                (canvas.height() as i32 / devi) as f64,
            )
            .ok()
            .ok_or(anyhow!("failed to write image"))?;

        let data = context
            .get_image_data(0.0, 0.0, (canvas.width() as i32 / devi) as f64, (canvas.width() as i32 / devi) as f64)
            .ok()
            .ok_or(anyhow!("failed to capture"))?;
        let local: DateTime<Local> = Local::now();
        Ok(
            MyImage {
                name: local.format("%Y-%m-%d %H-%M-%S").to_string(),
                height: (canvas.width() / devi as u32),
                width: (canvas.width() / devi as u32),
                bytes: data.data().to_vec(),
                save: true,
                del: false,
            },
        )
    }

    pub fn save_photos(&mut self, ctx: &Context) -> anyhow::Result<Vec<u8>> {
        self.save_photos_async(ctx).block_on()
    }

    async fn save_photos_async(&mut self, ctx: &Context) -> anyhow::Result<Vec<u8>> {
        let mut file: Vec<u8> = vec![];
        let mut writer = ZipFileWriter::new(&mut file);
        let mut photo_vec: Vec<DynamicImage> = vec![];
        for p in &self.photos {
            let buf: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::from_fn(p.width, p.height, |x,y| {
                Rgba::from(
                    [p.bytes[4 * (x + y * p.height as u32) as usize],
                        p.bytes[4 * (x + y * p.height as u32) as usize + 1],
                        p.bytes[4 * (x + y * p.height as u32) as usize + 2],
                        p.bytes[4 * (x + y * p.height as u32) as usize + 3],
                    ])
            });
            let img = DynamicImage::from(buf);
            
            let builder = ZipEntryBuilder::new((p.name).clone().into(), async_zip::Compression::Deflate);
            
            if self.save_options.jpg {
                let mut a = vec![];
                let mut buf2 : Cursor<&mut Vec<u8>> = Cursor::new(&mut a);
                let img3 = img.to_rgb8();
                img3.write_to(&mut buf2, ::image::ImageFormat::Jpeg)?;
                writer.write_entry_whole(builder, &buf2.bytes().map(|x| x.unwrap_or(0)).collect::<Vec<u8>>()).await?;
            }
            photo_vec.push(img);
        }

        return Ok(writer.close().await?.to_vec());
    }
    pub fn take_photo(&mut self, ctx: &Context) -> anyhow::Result<()> {
        let image = self.capture_frame(true);
        if let Ok(image) = image {
            self.save_options.image_index = self.photos.len() as i32 - 1;

            self.photos.push(image);
        };
        
        return Ok(());
    }
}
