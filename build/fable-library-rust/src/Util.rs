#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use std::rc::Rc;
pub mod Util {
    use super::*;
    pub fn equals<T: PartialEq + Clone + 'static>(x: &T, y: &T) -> bool {
        x.clone() == y.clone()
    }
    pub fn compare<T: PartialOrd + Clone + 'static>(x: &T, y: &T) -> i32 {
        if x.clone() > y.clone() {
            1i32
        } else { if x.clone() < y.clone() { -1i32 } else { 0i32 } }
    }
    pub fn ignore<T: Clone + 'static>(x: &T) { (); }
}
