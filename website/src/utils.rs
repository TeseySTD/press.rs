use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, Url, HtmlAnchorElement};

pub fn download_file(name: &str, data: &[u8]) {
    let array = js_sys::Uint8Array::from(data);
    let parts = js_sys::Array::new();
    parts.push(&array.buffer());
    
    let mut properties = BlobPropertyBag::new();
    properties.type_("application/octet-stream");
    
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &properties).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    
    let document = web_sys::window().unwrap().document().unwrap();
    let a = document.create_element("a").unwrap().dyn_into::<HtmlAnchorElement>().unwrap();
    a.set_href(&url);
    a.set_download(name);
    a.click();
    
    Url::revoke_object_url(&url).unwrap();
}