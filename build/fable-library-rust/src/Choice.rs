#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use std::rc::Rc;
pub mod Choice {
    use super::*;
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum Choice_2<T1: Clone + 'static, T2: Clone + 'static> {
        Choice1Of2(T1),
        Choice2Of2(T2),
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum Choice_3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                      'static> {
        Choice1Of3(T1),
        Choice2Of3(T2),
        Choice3Of3(T3),
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum Choice_4<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                      'static, T4: Clone + 'static> {
        Choice1Of4(T1),
        Choice2Of4(T2),
        Choice3Of4(T3),
        Choice4Of4(T4),
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum Choice_5<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                      'static, T4: Clone + 'static, T5: Clone + 'static> {
        Choice1Of5(T1),
        Choice2Of5(T2),
        Choice3Of5(T3),
        Choice4Of5(T4),
        Choice5Of5(T5),
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum Choice_6<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                      'static, T4: Clone + 'static, T5: Clone + 'static,
                      T6: Clone + 'static> {
        Choice1Of6(T1),
        Choice2Of6(T2),
        Choice3Of6(T3),
        Choice4Of6(T4),
        Choice5Of6(T5),
        Choice6Of6(T6),
    }
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum Choice_7<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                      'static, T4: Clone + 'static, T5: Clone + 'static,
                      T6: Clone + 'static, T7: Clone + 'static> {
        Choice1Of7(T1),
        Choice2Of7(T2),
        Choice3Of7(T3),
        Choice4Of7(T4),
        Choice5Of7(T5),
        Choice6Of7(T6),
        Choice7Of7(T7),
    }
}
