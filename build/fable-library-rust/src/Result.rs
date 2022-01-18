#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use std::rc::Rc;
pub mod Result_ {
    use super::*;
    pub fn map<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
               'static>(mapping: &Rc<impl Fn(&a_) -> (b_) + 'static>,
                        result: &Result<a_, c_>) -> Result<b_, c_> {
        match result {
            Err(result_1_0) => Err(result_1_0.clone()),
            Ok(result_0_0) => Ok(mapping(result_0_0)),
        }
    }
    pub fn mapError<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
                    'static>(mapping: &Rc<impl Fn(&a_) -> (b_) + 'static>,
                             result: &Result<c_, a_>) -> Result<c_, b_> {
        match result {
            Err(result_1_0) => Err(mapping(result_1_0)),
            Ok(result_0_0) => Ok(result_0_0.clone()),
        }
    }
    pub fn bind<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
                'static>(binder:
                             &Rc<impl Fn(&a_) -> (Result<b_, c_>) + 'static>,
                         result: &Result<a_, c_>) -> Result<b_, c_> {
        match result {
            Err(result_1_0) => Err(result_1_0.clone()),
            Ok(result_0_0) => binder(result_0_0),
        }
    }
}
