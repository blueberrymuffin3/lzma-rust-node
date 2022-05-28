use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Result<String, JsValue> {
    let mut reader = std::io::BufReader::new(input);
    let mut output: Vec<u8> = Vec::new();

    if let Err(err) = lzma_rs::lzma_decompress(&mut reader, &mut output) {
        return Err(JsValue::from(format!("Error decomressing: {}", err)));
    };

    return match String::from_utf8(output) {
        Ok(string) => Ok(string),
        Err(err) => Err(JsValue::from(format!("Error decoding UTF-8: {}", err))),
    };
}

#[wasm_bindgen]
pub fn compress(input: &str) -> Result<Vec<u8>, JsValue> {
    let mut reader = std::io::BufReader::new(input.as_bytes());
    let mut output: Vec<u8> = Vec::new();
    return match lzma_rs::lzma_compress(&mut reader, &mut output) {
        Ok(_) => Ok(output),
        Err(err) => Err(JsValue::from(format!("Error decomressing: {}", err))),
    };
}
