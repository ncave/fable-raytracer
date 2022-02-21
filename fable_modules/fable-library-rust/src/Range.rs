#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_3bd9ae6a::*;
#[path = "./Util.rs"]
pub(crate) mod import_f222008f;
pub use import_f222008f::*;
#[path = "./String.rs"]
pub(crate) mod import_eae1ac5e;
pub use import_eae1ac5e::*;
#[path = "./Seq.rs"]
pub(crate) mod import_52af85ec;
pub use import_52af85ec::*;
pub mod Range {
    use super::*;
    pub fn rangeNumeric<T: PartialOrd + Default + core::ops::Add<Output = T> +
                        Clone + 'static>(start: &T, step: &T, stop: &T)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        let stepComparedWithZero: i32 =
            Util::compare(step, &Native::getZero::<T>());
        if stepComparedWithZero == 0i32 {
            panic!("{}",
                   String_::string(&"The step of a range cannot be zero"));
        }
        Seq::unfold(&Rc::from({
                                  let step = step.clone();
                                  let stop = stop.clone();
                                  move |x: &T|
                                      {
                                          let comparedWithLast: i32 =
                                              Util::compare(x, &stop);
                                          if if if stepComparedWithZero > 0i32
                                                   {
                                                    comparedWithLast <= 0i32
                                                } else { false } {
                                                 true
                                             } else {
                                                 if stepComparedWithZero <
                                                        0i32 {
                                                     comparedWithLast >= 0i32
                                                 } else { false }
                                             } {
                                              Some((x.clone(),
                                                    x.clone() + step.clone()))
                                          } else { None::<(T, T)> }
                                      }
                              }), start)
    }
    pub fn rangeChar(start: &char, stop: &char)
     -> Rc<dyn Interfaces::IEnumerable_1<char>> {
        Seq::unfold(&Rc::from({
                                  let stop = stop.clone();
                                  move |c: &u32|
                                      if c.clone() <= stop as u32 {
                                          Some((String_::fromCharCode(&c.clone()),
                                                c.clone() + 1u32))
                                      } else { None::<(char, u32)> }
                              }), &(start.clone() as u32))
    }
}
