#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#[path = "./RayTracer.rs"]
pub(crate) mod import_d5a75629;
pub use import_d5a75629::*;
#[path = "./Platform.rs"]
pub(crate) mod import_2d3236e3;
pub use import_2d3236e3::*;
use std::rc::Rc;
use fable_library_rust::*;
pub mod TestApp {
    use super::*;
    pub fn main(_args: &Rc<MutCell<Vec<Rc<str>>>>) -> i32 {
        let patternInput: Rc<(i32, i32, i32, i32)> =
            Rc::from((0i32, 0i32, 2048i32, 2048i32));
        let w: i32 = patternInput.2.clone();
        let h: i32 = patternInput.3.clone();
        let data: Rc<MutCell<Vec<u8>>> =
            Native::arrayCreate(&(w * h * 4i32), &0u8);
        println!("{0}", &Native::string(&"Raytracer running..."));
        {
            let patternInput_1: Rc<((), f64)> =
                Platform::measureTime(&Rc::from({
                                                    let data = data.clone();
                                                    let patternInput =
                                                        patternInput.clone();
                                                    move ||
                                                        RayTracerDemo::renderScene(&data,
                                                                                   &patternInput.0.clone(),
                                                                                   &patternInput.1.clone(),
                                                                                   &w,
                                                                                   &h)
                                                }));
            println!("Ray tracing:
 - rendered image size: ({0}x{1})
 - elapsed: {2} ms",
                     &w, &h, &patternInput_1.1.clone());
            0i32
        }
    }
}
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<Rc<str>> = args[1..].iter().map(|s| Native::string(s)).collect();
    TestApp::main(&Native::arrayFrom(&args));
}
