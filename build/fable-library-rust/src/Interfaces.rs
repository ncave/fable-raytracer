#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#[path = "./Native.rs"]
pub(crate) mod import_3bd9ae6a;
pub use import_3bd9ae6a::*;
pub mod Interfaces {
    use super::*;
    pub trait IEnumerator_1<T: Clone + 'static> {
        fn MoveNext(&self)
        -> bool;
        fn Current(&self)
        -> T;
        fn Dispose(&self);
    }
    pub trait IEnumerable_1<T: Clone + 'static> {
        fn GetEnumerator(&self)
        -> Rc<dyn Interfaces::IEnumerator_1<T>>;
    }
    pub trait ICollection_1<T: Clone + 'static> {
        fn Add(&self, arg0: &T);
        fn Clear(&self);
        fn Contains(&self, arg0: &T)
        -> bool;
        fn CopyTo(&self, arg0: &Rc<MutCell<Vec<T>>>, arg1: &i32);
        fn Remove(&self, arg0: &T)
        -> bool;
        fn Count(&self)
        -> i32;
        fn IsReadOnly(&self)
        -> bool;
    }
    pub trait IDictionary_2<K: Clone + 'static, V: Clone + 'static> {
        fn Add(&self, arg0: &K, arg1: &V);
        fn ContainsKey(&self, arg0: &K)
        -> bool;
        fn Remove(&self, arg0: &K)
        -> bool;
        fn TryGetValue(&self, arg0: &K, arg1: &Rc<MutCell<V>>)
        -> bool;
        fn Item(&self, arg0: &K)
        -> V;
        fn Keys(&self)
        -> Rc<dyn Interfaces::ICollection_1<K>>;
        fn Values(&self)
        -> Rc<dyn Interfaces::ICollection_1<V>>;
    }
}
