use wasm_bindgen::JsCast;
use web_sys::window;

use anyhow::anyhow;

pub fn download_zip_file(bytes: Vec<u8>, filename:String) -> anyhow::Result<()> {
    download_file(bytes, filename, "application/zip".to_owned())
}
pub fn download_file(bytes: Vec<u8>, filename:String, datatype:String) -> anyhow::Result<()> {
    let link = window()
    .ok_or(anyhow!("no window found"))
    ? 
    .document()
    .ok_or(anyhow!("no document found"))
    
    ?.get_element_by_id("download_files")
    .ok_or(anyhow!("could not find element called download_files"))
    ?;
    
    let link = link.dyn_into::<web_sys::HtmlElement>()
    .map_err(|_| ())
    .unwrap();
    let base = base64_url::encode(&bytes);

    let a = link.set_attribute("href", &format!("data:{datatype};base64,{base}"));
    let b = link.set_attribute("download", &format!(""));
    if a.is_ok() && b.is_ok() {
        link.click();
    }else {
        return Err(anyhow!("{:?} {:?}", a,b));
    }
    ;
    Ok(())
}