#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_52af85ec::*;
use crate::import_3bd9ae6a::*;
use crate::import_f222008f::*;
use std::rc::Rc;
pub mod Range {
    use super::*;
    pub fn rangeNumeric<T: PartialOrd + Default + core::ops::Add<Output = T> +
                        Clone + 'static>(start: &T, step: &T, stop: &T)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        {
            let stepComparedWithZero: i32 =
                Util::compare(step, &Native::getZero::<T>());
            if stepComparedWithZero == 0i32 {
                panic!("{}",
                       Native::string(&"The step of a range cannot be zero"));
            }
            Seq::unfold(&Rc::from({
                                      let step = step.clone();
                                      let stop = stop.clone();
                                      move |x: &T|
                                          {
                                              let comparedWithLast: i32 =
                                                  Util::compare(x, &stop);
                                              if if if stepComparedWithZero >
                                                           0i32 {
                                                        comparedWithLast <=
                                                            0i32
                                                    } else { false } {
                                                     true
                                                 } else {
                                                     if stepComparedWithZero <
                                                            0i32 {
                                                         comparedWithLast >=
                                                             0i32
                                                     } else { false }
                                                 } {
                                                  Some(Rc::from((x.clone(),
                                                                 x.clone() +
                                                                     step.clone())))
                                              } else { None::<Rc<(T, T)>> }
                                          }.clone()
                                  }), start)
        }.clone()
    }
    pub fn rangeChar(start: &char, stop: &char)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<char>> {
        Seq::unfold(&Rc::from({
                                  let stop = stop.clone();
                                  move |c: &u32|
                                      if c.clone() <= stop as u32 {
                                          Some(Rc::from((Native::toChar(&c.clone()),
                                                         c.clone() + 1u32)))
                                      } else { None::<Rc<(char, u32)>> }
                              }), &(start.clone() as u32))
    }
}
