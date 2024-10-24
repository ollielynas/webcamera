use anyhow::anyhow;
use egui::{epaint::image, CentralPanel};
use wasm_bindgen::JsCast;
use web_sys::{window, Navigator};


pub fn capture_frame() -> anyhow::Result<(u32,u32,Vec<u8>)> {

        
    let video = window().ok_or(anyhow!("no window"))?
    .document().ok_or(anyhow!("no document"))?
    .get_element_by_id("videoElement").ok_or(anyhow!("video element not found"))?;
    let canvas = window().ok_or(anyhow!("no window"))?
    .document().ok_or(anyhow!("no document"))?
    .get_element_by_id("canvas").ok_or(anyhow!("video element not found"))?;

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

    context.draw_image_with_html_video_element(&video, 0.0, 0.0);

    let data = context.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64).ok().ok_or(anyhow!("failed to capture"))?;
    data.data();
    Ok((canvas.width(), canvas.height(), data.data().to_vec()))
}