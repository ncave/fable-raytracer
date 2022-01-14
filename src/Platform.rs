#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use std::rc::Rc;
use fable_library_rust::*;
pub mod Platform {
    use super::*;
    pub mod Performance {
        use super::*;
        pub trait Duration {
            fn as_secs_f64(&self)
            -> f64;
        }
        pub trait Instant {
            fn elapsed(&self)
            -> Rc<dyn Platform::Performance::Duration>;
        }
    }
    pub fn measureTime<T: Clone + 'static>(f: &Rc<impl Fn() -> (T) + 'static>)
     -> Rc<(T, f64)> {
        {
            let t0 = std::time::Instant::now();
            let res: T = f();
            let t1 = std::time::Instant::now();
            Rc::from((res, t0.elapsed().as_secs_f64() * 1000.0f64))
        }.clone()
    }
}
