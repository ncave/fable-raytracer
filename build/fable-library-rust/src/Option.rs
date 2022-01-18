#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_3bd9ae6a::*;
use std::rc::Rc;
pub mod Option_ {
    use super::*;
    pub fn bind<T: Clone + 'static, U: Clone +
                'static>(binder: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                         opt: &Option<T>) -> Option<U> {
        match opt { None => None::<U>, Some(opt_0_0) => binder(opt_0_0), }
    }
    pub fn contains<T: PartialEq + Clone +
                    'static>(value: &T, opt: &Option<T>) -> bool {
        match opt {
            None => false,
            Some(opt_0_0) => opt_0_0.clone() == value.clone(),
        }
    }
    pub fn count<T: Clone + 'static>(opt: &Option<T>) -> i32 {
        match opt { None => 0i32, _ => 1i32, }
    }
    pub fn defaultArg<T: Clone + 'static>(opt: &Option<T>, defaultValue_1: &T)
     -> T {
        match opt {
            None => defaultValue_1.clone(),
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn defaultValue<T: Clone +
                        'static>(defaultValue_1: &T, opt: &Option<T>) -> T {
        match opt {
            None => defaultValue_1.clone(),
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn defaultWith<T: Clone +
                       'static>(defThunk: &Rc<impl Fn() -> (T) + 'static>,
                                opt: &Option<T>) -> T {
        match opt { None => defThunk(), Some(opt_0_0) => opt_0_0.clone(), }
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           opt: &Option<T>) -> bool {
        match opt { None => false, Some(opt_0_0) => predicate(opt_0_0), }
    }
    pub fn filter<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           opt: &Option<T>) -> Option<T> {
        match opt {
            None => None::<T>,
            Some(opt_0_0) =>
            if predicate(opt_0_0) { opt.clone() } else { None::<T> },
        }
    }
    pub fn flatten<T: Clone + 'static>(opt: &Option<Option<T>>) -> Option<T> {
        match opt { None => None::<T>, Some(opt_0_0) => opt_0_0.clone(), }
    }
    pub fn fold<S: Clone + 'static, T: Clone +
                'static>(folder: &Rc<impl Fn(&S, &T) -> (S) + 'static>,
                         state: &S, opt: &Option<T>) -> S {
        match opt {
            None => state.clone(),
            Some(opt_0_0) => folder(state, opt_0_0),
        }
    }
    pub fn foldBack<T: Clone + 'static, S: Clone +
                    'static>(folder: &Rc<impl Fn(&T, &S) -> (S) + 'static>,
                             opt: &Option<T>, state: &S) -> S {
        match opt {
            None => state.clone(),
            Some(opt_0_0) => folder(opt_0_0, state),
        }
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           opt: &Option<T>) -> bool {
        match opt { None => true, Some(opt_0_0) => predicate(opt_0_0), }
    }
    pub fn getValue<T: Clone + 'static>(opt: &Option<T>) -> T {
        match opt {
            None => panic!("{}", Native::string(&"Option has no value")),
            Some(opt_0_0) => opt_0_0.clone(),
        }
    }
    pub fn iterate<T: Clone +
                   'static>(action: &Rc<impl Fn(&T) + 'static>,
                            opt: &Option<T>) {
        match opt { None => (), Some(opt_0_0) => action(opt_0_0), };
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        opt: &Option<T>) -> Option<U> {
        match opt {
            None => None::<U>,
            Some(opt_0_0) => Some(mapping(opt_0_0)),
        }
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         opt1: &Option<T1>, opt2: &Option<T2>) -> Option<U> {
        match opt1 {
            Some(opt1_0_0) =>
            {
                let x: T1 = opt1_0_0.clone();
                match opt2 {
                    Some(opt2_0_0) => Some(mapping(&x, opt2_0_0)),
                    _ => None::<U>,
                }
            }.clone(),
            _ => None::<U>,
        }
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         opt1: &Option<T1>, opt2: &Option<T2>,
                         opt3: &Option<T3>) -> Option<U> {
        match opt1 {
            Some(opt1_0_0) =>
            {
                let x: T1 = opt1_0_0.clone();
                match opt2 {
                    Some(opt2_0_0) =>
                    {
                        let y: T2 = opt2_0_0.clone();
                        match opt3 {
                            Some(opt3_0_0) => Some(mapping(&x, &y, opt3_0_0)),
                            _ => None::<U>,
                        }
                    }.clone(),
                    _ => None::<U>,
                }
            }.clone(),
            _ => None::<U>,
        }
    }
    pub fn orElse<T: Clone + 'static>(ifNone: &Option<T>, opt: &Option<T>)
     -> Option<T> {
        match opt { None => ifNone.clone(), _ => opt.clone(), }
    }
    pub fn orElseWith<T: Clone +
                      'static>(ifNoneThunk:
                                   &Rc<impl Fn() -> (Option<T>) + 'static>,
                               opt: &Option<T>) -> Option<T> {
        match opt { None => ifNoneThunk(), _ => opt.clone(), }
    }
}
