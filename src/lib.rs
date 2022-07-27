mod RayTracer;
use wasm_bindgen::prelude::*;
use RayTracer::RayTracerDemo;

use fable_library_rust::Native_::arrayCreate;
use fable_library_rust::Native_::MutCell;
use fable_library_rust::Native_::Lrc;

const WMAX: i32 = 2048;
const HMAX: i32 = 2048;
const BUF_LEN: i32 = WMAX * HMAX * 4;

fn get_buffer() -> Lrc<MutCell<Vec<u8>>> {
    thread_local! {
        static DATA: Lrc<MutCell<Vec<u8>>> = arrayCreate(&BUF_LEN, &0u8);
    }
    DATA.with(|data| data.clone())
}

#[wasm_bindgen]
pub fn get_buffer_offset() -> *const u8 {
    get_buffer().as_ptr()
}

#[wasm_bindgen]
pub fn get_buffer_length() -> i32 {
    BUF_LEN
}

#[wasm_bindgen]
pub fn render_scene(x: i32, y: i32, w: i32, h: i32, angle: f64) {
    RayTracerDemo::renderScene(get_buffer(), x, y, w, h, angle);
}
