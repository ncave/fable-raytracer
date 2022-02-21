#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_eae1ac5e::*;
use crate::import_3bd9ae6a::*;
#[path = "./Option.rs"]
pub(crate) mod import_8d7d6be8;
pub use import_8d7d6be8::*;
use crate::import_f222008f::*;
use crate::import_52af85ec::*;
#[path = "./List.rs"]
pub(crate) mod import_ec6ee4e9;
pub use import_ec6ee4e9::*;
#[path = "./Array.rs"]
pub(crate) mod import_c6216f2;
pub use import_c6216f2::*;
pub mod Set_ {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn setContainsNoElements() -> Rc<str> {
            thread_local! {
                static setContainsNoElements: Rc<str> =
    String_::string(&"Set contains no elements.");
            };
            setContainsNoElements.with(|value| value.clone())
        }
        pub fn enumerationNotStarted() -> Rc<str> {
            thread_local! {
                static enumerationNotStarted: Rc<str> =
    String_::string(&"Enumeration has not started. Call MoveNext.");
            };
            enumerationNotStarted.with(|value| value.clone())
        }
        pub fn enumerationAlreadyFinished() -> Rc<str> {
            thread_local! {
                static enumerationAlreadyFinished: Rc<str> =
    String_::string(&"Enumeration already finished.");
            };
            enumerationAlreadyFinished.with(|value| value.clone())
        }
    }
    #[derive(Clone, Debug, Default, Hash)]
    pub struct SetTree_1<T: Clone + 'static> {
        pub Height: i32,
        pub Key: T,
        pub Left: Option<Rc<Set_::SetTree_1<T>>>,
        pub Right: Option<Rc<Set_::SetTree_1<T>>>,
    }
    pub fn empty<T: Clone + 'static>() -> Option<Rc<Set_::SetTree_1<T>>> {
        None::<Rc<Set_::SetTree_1<T>>>
    }
    pub fn isEmpty<T: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> bool {
        s.is_none()
    }
    pub fn mkSetTreeLeaf<T: Clone + 'static>(key: &T)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        Some(Rc::from(Set_::SetTree_1::<T>{Height: 1i32,
                                           Key: key.clone(),
                                           Left: Set_::empty(),
                                           Right: Set_::empty(),}))
    }
    pub fn mkSetTreeNode<T: Clone +
                         'static>(key: &T,
                                  left: &Option<Rc<Set_::SetTree_1<T>>>,
                                  right: &Option<Rc<Set_::SetTree_1<T>>>,
                                  height: &i32)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        Some(Rc::from(Set_::SetTree_1::<T>{Height: height.clone(),
                                           Key: key.clone(),
                                           Left: left.clone(),
                                           Right: right.clone(),}))
    }
    pub fn singleton<T: Clone + 'static>(value: &T)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        Set_::mkSetTreeLeaf(value)
    }
    pub fn countAux<T: Clone +
                    'static>(s: &Option<Rc<Set_::SetTree_1<T>>>, acc: &i32)
     -> i32 {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    acc.clone() + 1i32
                } else {
                    Set_::countAux(&t.Left,
                                   &Set_::countAux(&t.Right,
                                                   &(acc.clone() + 1i32)))
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn count<a_: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> i32 {
        Set_::countAux(s, &0i32)
    }
    pub fn height<T: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> i32 {
        match s { Some(s_0_0) => s_0_0.Height, _ => 0i32, }
    }
    pub fn mk<T: Clone +
              'static>(l: &Option<Rc<Set_::SetTree_1<T>>>, k: &T,
                       r: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        let hl: i32 =
            {
                let s: Option<Rc<Set_::SetTree_1<T>>> = l.clone();
                match &s { Some(s_0_0) => s_0_0.Height, _ => 0i32, }
            };
        let hr: i32 =
            {
                let s_1: Option<Rc<Set_::SetTree_1<T>>> = r.clone();
                match &s_1 { Some(s_1_0_0) => s_1_0_0.Height, _ => 0i32, }
            };
        let m: i32 = if hl < hr { hr } else { hl };
        if m == 0i32 {
            Set_::mkSetTreeLeaf(k)
        } else { Set_::mkSetTreeNode(k, l, r, &(m + 1i32)) }
    }
    pub fn rebalance<T: Clone +
                     'static>(t1: &Option<Rc<Set_::SetTree_1<T>>>, v: &T,
                              t2: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        let t1h: i32 =
            {
                let s: Option<Rc<Set_::SetTree_1<T>>> = t1.clone();
                match &s { Some(s_0_0) => s_0_0.Height, _ => 0i32, }
            };
        let t2h: i32 =
            {
                let s_1: Option<Rc<Set_::SetTree_1<T>>> = t2.clone();
                match &s_1 { Some(s_1_0_0) => s_1_0_0.Height, _ => 0i32, }
            };
        if t2h > t1h + 2i32 {
            let t2_: Rc<Set_::SetTree_1<T>> = Option_::getValue(t2).clone();
            if {
                   let s_2: Option<Rc<Set_::SetTree_1<T>>> = t2_.Left.clone();
                   match &s_2 { Some(s_2_0_0) => s_2_0_0.Height, _ => 0i32, }
               } > t1h + 1i32 {
                let t2l: Rc<Set_::SetTree_1<T>> =
                    Option_::getValue(&t2_.Left).clone();
                Set_::mk(&Set_::mk(t1, v, &t2l.Left), &t2l.Key,
                         &Set_::mk(&t2l.Right, &t2_.Key, &t2_.Right))
            } else {
                Set_::mk(&Set_::mk(t1, v, &t2_.Left), &t2_.Key, &t2_.Right)
            }
        } else {
            if t1h > t2h + 2i32 {
                let t1_: Rc<Set_::SetTree_1<T>> =
                    Option_::getValue(t1).clone();
                if {
                       let s_3: Option<Rc<Set_::SetTree_1<T>>> =
                           t1_.Right.clone();
                       match &s_3 {
                           Some(s_3_0_0) => s_3_0_0.Height,
                           _ => 0i32,
                       }
                   } > t2h + 1i32 {
                    let t1r: Rc<Set_::SetTree_1<T>> =
                        Option_::getValue(&t1_.Right).clone();
                    Set_::mk(&Set_::mk(&t1_.Left, &t1_.Key, &t1r.Left),
                             &t1r.Key, &Set_::mk(&t1r.Right, v, t2))
                } else {
                    Set_::mk(&t1_.Left, &t1_.Key,
                             &Set_::mk(&t1_.Right, v, t2))
                }
            } else { Set_::mk(t1, v, t2) }
        }
    }
    pub fn add<T: PartialOrd + Clone +
               'static>(k: &T, s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c < 0i32 {
                        Set_::mkSetTreeNode(k, &Set_::empty(), s, &2i32)
                    } else {
                        if c == 0i32 {
                            s.clone()
                        } else {
                            Set_::mkSetTreeNode(k, s, &Set_::empty(), &2i32)
                        }
                    }
                } else {
                    if c < 0i32 {
                        Set_::rebalance(&Set_::add(k, &t.Left), &t.Key,
                                        &t.Right)
                    } else {
                        if c == 0i32 {
                            s.clone()
                        } else {
                            Set_::rebalance(&t.Left, &t.Key,
                                            &Set_::add(k, &t.Right))
                        }
                    }
                }
            }
            _ => Set_::mkSetTreeLeaf(k),
        }
    }
    pub fn balance<T: PartialOrd + Clone +
                   'static>(s1: &Option<Rc<Set_::SetTree_1<T>>>, k: &T,
                            s2: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match s1 {
            Some(s1_0_0) => {
                let t1: Rc<Set_::SetTree_1<T>> = s1_0_0.clone();
                match s2 {
                    Some(s2_0_0) => {
                        let t2: Rc<Set_::SetTree_1<T>> = s2_0_0.clone();
                        if t1.Height == 1i32 {
                            Set_::add(k, &Set_::add(&t1.Key, s2))
                        } else {
                            if t2.Height == 1i32 {
                                Set_::add(k, &Set_::add(&t2.Key, s1))
                            } else {
                                if t1.Height + 2i32 < t2.Height {
                                    Set_::rebalance(&Set_::balance(s1, k,
                                                                   &t2.Left),
                                                    &t2.Key, &t2.Right)
                                } else {
                                    if t2.Height + 2i32 < t1.Height {
                                        Set_::rebalance(&t1.Left, &t1.Key,
                                                        &Set_::balance(&t1.Right,
                                                                       k, s2))
                                    } else { Set_::mk(s1, k, s2) }
                                }
                            }
                        }
                    }
                    _ => Set_::add(k, s1),
                }
            }
            _ => Set_::add(k, s2),
        }
    }
    pub fn split<T: PartialOrd + Clone +
                 'static>(pivot: &T, s: &Option<Rc<Set_::SetTree_1<T>>>)
     ->
         (Option<Rc<Set_::SetTree_1<T>>>, bool,
          Option<Rc<Set_::SetTree_1<T>>>) {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    let c: i32 = Util::compare(&t.Key, pivot);
                    if c < 0i32 {
                        (s.clone(), false, Set_::empty())
                    } else {
                        if c == 0i32 {
                            (Set_::empty(), true, Set_::empty())
                        } else { (Set_::empty(), false, s.clone()) }
                    }
                } else {
                    let c_1: i32 = Util::compare(pivot, &t.Key);
                    if c_1 < 0i32 {
                        let patternInput:
                                (Option<Rc<Set_::SetTree_1<T>>>, bool,
                                 Option<Rc<Set_::SetTree_1<T>>>) =
                            Set_::split(pivot, &t.Left);
                        (patternInput.0.clone(), patternInput.1.clone(),
                         Set_::balance(&patternInput.2.clone(), &t.Key,
                                       &t.Right))
                    } else {
                        if c_1 == 0i32 {
                            (t.Left.clone(), true, t.Right.clone())
                        } else {
                            let patternInput_1:
                                    (Option<Rc<Set_::SetTree_1<T>>>, bool,
                                     Option<Rc<Set_::SetTree_1<T>>>) =
                                Set_::split(pivot, &t.Right);
                            (Set_::balance(&t.Left, &t.Key,
                                           &patternInput_1.0.clone()),
                             patternInput_1.1.clone(),
                             patternInput_1.2.clone())
                        }
                    }
                }
            }
            _ => (Set_::empty(), false, Set_::empty()),
        }
    }
    pub fn spliceOutSuccessor<T: Clone +
                              'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> (T, Option<Rc<Set_::SetTree_1<T>>>) {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    (t.Key.clone(), Set_::empty())
                } else {
                    if t.Left.is_none() {
                        (t.Key.clone(), t.Right.clone())
                    } else {
                        let patternInput:
                                (T, Option<Rc<Set_::SetTree_1<T>>>) =
                            Set_::spliceOutSuccessor(&t.Left);
                        (patternInput.0.clone(),
                         Set_::mk(&patternInput.1.clone(), &t.Key, &t.Right))
                    }
                }
            }
            _ =>
            panic!("{}",
                   String_::string(&"internal error: Set.spliceOutSuccessor")),
        }
    }
    pub fn remove<T: PartialOrd + Clone +
                  'static>(k: &T, s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c == 0i32 { Set_::empty() } else { s.clone() }
                } else {
                    if c < 0i32 {
                        Set_::rebalance(&Set_::remove(k, &t.Left), &t.Key,
                                        &t.Right)
                    } else {
                        if c == 0i32 {
                            if t.Left.is_none() {
                                t.Right.clone()
                            } else {
                                if t.Right.is_none() {
                                    t.Left.clone()
                                } else {
                                    let patternInput:
                                            (T,
                                             Option<Rc<Set_::SetTree_1<T>>>) =
                                        Set_::spliceOutSuccessor(&t.Right);
                                    Set_::mk(&t.Left, &patternInput.0.clone(),
                                             &patternInput.1.clone())
                                }
                            }
                        } else {
                            Set_::rebalance(&t.Left, &t.Key,
                                            &Set_::remove(k, &t.Right))
                        }
                    }
                }
            }
            _ => s.clone(),
        }
    }
    pub fn contains<T: PartialOrd + Clone +
                    'static>(k: &T, s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> bool {
        let k: Rc<MutCell<T>> = Rc::from(MutCell::from(k.clone()));
        let s: Rc<MutCell<Option<Rc<Set_::SetTree_1<T>>>>> =
            Rc::from(MutCell::from(s.clone()));
        '_contains:
            loop  {
                break '_contains
                    (match &s.get() {
                         Some(s_0_0) => {
                             let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                             let c: i32 = Util::compare(&k.get(), &t.Key);
                             if t.Height == 1i32 {
                                 c == 0i32
                             } else {
                                 if c < 0i32 {
                                     let k_temp: T = k.get();
                                     let s_temp:
                                             Option<Rc<Set_::SetTree_1<T>>> =
                                         t.Left.clone();
                                     k.set(k_temp);
                                     s.set(s_temp);
                                     continue '_contains
                                 } else {
                                     if c == 0i32 {
                                         true
                                     } else {
                                         let k_temp: T = k.get();
                                         let s_temp:
                                                 Option<Rc<Set_::SetTree_1<T>>> =
                                             t.Right.clone();
                                         k.set(k_temp);
                                         s.set(s_temp);
                                         continue '_contains
                                     }
                                 }
                             }
                         }
                         _ => false,
                     }) ;
            }
    }
    pub fn iterate<T: Clone +
                   'static>(f: &Rc<impl Fn(&T) + 'static>,
                            s: &Option<Rc<Set_::SetTree_1<T>>>) {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key)
                } else {
                    Set_::iterate(f, &t.Left);
                    f(&t.Key);
                    Set_::iterate(f, &t.Right)
                }
            }
            _ => (),
        };
    }
    pub fn foldBack<T: Clone + 'static, a_: Clone +
                    'static>(f: &Rc<impl Fn(&T, &a_) -> (a_) + 'static>,
                             s: &Option<Rc<Set_::SetTree_1<T>>>, x: &a_)
     -> a_ {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, x)
                } else {
                    Set_::foldBack(f, &t.Left,
                                   &f(&t.Key,
                                      &Set_::foldBack(f, &t.Right, x)))
                }
            }
            _ => x.clone(),
        }
    }
    pub fn fold<a_: Clone + 'static, T: Clone +
                'static>(f: &Rc<impl Fn(&a_, &T) -> (a_) + 'static>, x: &a_,
                         s: &Option<Rc<Set_::SetTree_1<T>>>) -> a_ {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(x, &t.Key)
                } else {
                    Set_::fold(f, &f(&Set_::fold(f, x, &t.Left), &t.Key),
                               &t.Right)
                }
            }
            _ => x.clone(),
        }
    }
    pub fn map<T: Clone + 'static, a_: PartialOrd + Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (a_) + 'static>,
                        s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        Set_::fold(&Rc::from({
                                 let mapping = mapping.clone();
                                 move
                                     |acc: &Option<Rc<Set_::SetTree_1<a_>>>,
                                      k: &T| Set_::add(&mapping(k), acc)
                             }), &Set_::empty(), s)
    }
    pub fn forAll<T: Clone +
                  'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           s: &Option<Rc<Set_::SetTree_1<T>>>) -> bool {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key)
                } else {
                    if if f(&t.Key) {
                           Set_::forAll(f, &t.Left)
                       } else { false } {
                        Set_::forAll(f, &t.Right)
                    } else { false }
                }
            }
            _ => true,
        }
    }
    pub fn exists<T: Clone +
                  'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           s: &Option<Rc<Set_::SetTree_1<T>>>) -> bool {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key)
                } else {
                    if if f(&t.Key) { true } else { Set_::exists(f, &t.Left) }
                       {
                        true
                    } else { Set_::exists(f, &t.Right) }
                }
            }
            _ => false,
        }
    }
    pub fn isSubset<a_: PartialOrd + Clone +
                    'static>(a: &Option<Rc<Set_::SetTree_1<a_>>>,
                             b: &Option<Rc<Set_::SetTree_1<a_>>>) -> bool {
        Set_::forAll(&Rc::from({
                                   let b = b.clone();
                                   move |x: &a_| Set_::contains(x, &b)
                               }), a)
    }
    pub fn isSuperset<a_: PartialOrd + Clone +
                      'static>(a: &Option<Rc<Set_::SetTree_1<a_>>>,
                               b: &Option<Rc<Set_::SetTree_1<a_>>>) -> bool {
        Set_::isSubset(b, a)
    }
    pub fn isProperSubset<a_: PartialOrd + Clone +
                          'static>(a: &Option<Rc<Set_::SetTree_1<a_>>>,
                                   b: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> bool {
        if Set_::forAll(&Rc::from({
                                      let b = b.clone();
                                      move |x: &a_| Set_::contains(x, &b)
                                  }), a) {
            Set_::exists(&Rc::from({
                                       let a = a.clone();
                                       move |x_1: &a_|
                                           !Set_::contains(x_1, &a)
                                   }), b)
        } else { false }
    }
    pub fn isProperSuperset<a_: PartialOrd + Clone +
                            'static>(a: &Option<Rc<Set_::SetTree_1<a_>>>,
                                     b: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> bool {
        Set_::isProperSubset(b, a)
    }
    pub fn filterAux<T: PartialOrd + Clone +
                     'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              s: &Option<Rc<Set_::SetTree_1<T>>>,
                              acc: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    if f(&t.Key) {
                        Set_::add(&t.Key, acc)
                    } else { acc.clone() }
                } else {
                    Set_::filterAux(f, &t.Left,
                                    &Set_::filterAux(f, &t.Right,
                                                     &if f(&t.Key) {
                                                          Set_::add(&t.Key,
                                                                    acc)
                                                      } else { acc.clone() }))
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn filter<a_: PartialOrd + Clone +
                  'static>(f: &Rc<impl Fn(&a_) -> (bool) + 'static>,
                           s: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        Set_::filterAux(f, s, &Set_::empty())
    }
    pub fn diffAux<T: PartialOrd + Clone +
                   'static>(s: &Option<Rc<Set_::SetTree_1<T>>>,
                            acc: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match acc {
            Some(acc_0_0) => {
                let _acc: Rc<Set_::SetTree_1<T>> = acc_0_0.clone();
                match s {
                    Some(s_0_0) => {
                        let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                        if t.Height == 1i32 {
                            Set_::remove(&t.Key, acc)
                        } else {
                            Set_::diffAux(&t.Left,
                                          &Set_::diffAux(&t.Right,
                                                         &Set_::remove(&t.Key,
                                                                       acc)))
                        }
                    }
                    _ => acc.clone(),
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn difference<a_: PartialOrd + Clone +
                      'static>(a: &Option<Rc<Set_::SetTree_1<a_>>>,
                               b: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        Set_::diffAux(b, a)
    }
    pub fn r#union<T: PartialOrd + Clone +
                   'static>(s1: &Option<Rc<Set_::SetTree_1<T>>>,
                            s2: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match s1 {
            Some(s1_0_0) => {
                let t1: Rc<Set_::SetTree_1<T>> = s1_0_0.clone();
                match s2 {
                    Some(s2_0_0) => {
                        let t2: Rc<Set_::SetTree_1<T>> = s2_0_0.clone();
                        if t1.Height == 1i32 {
                            Set_::add(&t1.Key, s2)
                        } else {
                            if t2.Height == 1i32 {
                                Set_::add(&t2.Key, s1)
                            } else {
                                if t1.Height > t2.Height {
                                    let patternInput:
                                            (Option<Rc<Set_::SetTree_1<T>>>,
                                             bool,
                                             Option<Rc<Set_::SetTree_1<T>>>) =
                                        Set_::split(&t1.Key, s2);
                                    Set_::balance(&Set_::r#union(&t1.Left,
                                                                 &patternInput.0.clone()),
                                                  &t1.Key,
                                                  &Set_::r#union(&t1.Right,
                                                                 &patternInput.2.clone()))
                                } else {
                                    let patternInput_1:
                                            (Option<Rc<Set_::SetTree_1<T>>>,
                                             bool,
                                             Option<Rc<Set_::SetTree_1<T>>>) =
                                        Set_::split(&t2.Key, s1);
                                    Set_::balance(&Set_::r#union(&t2.Left,
                                                                 &patternInput_1.0.clone()),
                                                  &t2.Key,
                                                  &Set_::r#union(&t2.Right,
                                                                 &patternInput_1.2.clone()))
                                }
                            }
                        }
                    }
                    _ => s1.clone(),
                }
            }
            _ => s2.clone(),
        }
    }
    pub fn unionMany<T: PartialOrd + Clone +
                     'static>(sets:
                                  &Rc<dyn Interfaces::IEnumerable_1<Option<Rc<Set_::SetTree_1<T>>>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        Seq::fold(&Rc::from(move
                                |s1: &Option<Rc<Set_::SetTree_1<T>>>,
                                 s2: &Option<Rc<Set_::SetTree_1<T>>>|
                                Set_::r#union(s1, s2)), &Set_::empty(), sets)
    }
    pub fn intersectionAux<T: PartialOrd + Clone +
                           'static>(b: &Option<Rc<Set_::SetTree_1<T>>>,
                                    s: &Option<Rc<Set_::SetTree_1<T>>>,
                                    acc: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    if Set_::contains(&t.Key, b) {
                        Set_::add(&t.Key, acc)
                    } else { acc.clone() }
                } else {
                    let acc_1: Option<Rc<Set_::SetTree_1<T>>> =
                        Set_::intersectionAux(b, &t.Right, acc);
                    Set_::intersectionAux(b, &t.Left,
                                          &if Set_::contains(&t.Key, b) {
                                               Set_::add(&t.Key, &acc_1)
                                           } else { acc_1.clone() })
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn intersect<a_: PartialOrd + Clone +
                     'static>(a: &Option<Rc<Set_::SetTree_1<a_>>>,
                              b: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        if b.clone().is_none() {
            b.clone()
        } else { Set_::intersectionAux(b, a, &Set_::empty()) }
    }
    pub fn intersectMany<T: PartialOrd + Clone +
                         'static>(sets:
                                      &Rc<dyn Interfaces::IEnumerable_1<Option<Rc<Set_::SetTree_1<T>>>>>)
     -> Option<Rc<Set_::SetTree_1<T>>> {
        Seq::reduce(&Rc::from(move
                                  |a: &Option<Rc<Set_::SetTree_1<T>>>,
                                   b: &Option<Rc<Set_::SetTree_1<T>>>|
                                  Set_::intersect(a, b)), sets)
    }
    pub fn partition1<a_: PartialOrd + Clone +
                      'static>(f: &Rc<impl Fn(&a_) -> (bool) + 'static>,
                               k: &a_, acc1: &Option<Rc<Set_::SetTree_1<a_>>>,
                               acc2: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> (Option<Rc<Set_::SetTree_1<a_>>>, Option<Rc<Set_::SetTree_1<a_>>>) {
        if f(k) {
            (Set_::add(k, acc1), acc2.clone())
        } else { (acc1.clone(), Set_::add(k, acc2)) }
    }
    pub fn partitionAux<T: PartialOrd + Clone +
                        'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 s: &Option<Rc<Set_::SetTree_1<T>>>,
                                 acc_0: &Option<Rc<Set_::SetTree_1<T>>>,
                                 acc_1: &Option<Rc<Set_::SetTree_1<T>>>)
     -> (Option<Rc<Set_::SetTree_1<T>>>, Option<Rc<Set_::SetTree_1<T>>>) {
        let acc:
                (Option<Rc<Set_::SetTree_1<T>>>,
                 Option<Rc<Set_::SetTree_1<T>>>) =
            (acc_0.clone(), acc_1.clone());
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    Set_::partition1(f, &t.Key, &acc.0.clone(),
                                     &acc.1.clone())
                } else {
                    let acc_2:
                            (Option<Rc<Set_::SetTree_1<T>>>,
                             Option<Rc<Set_::SetTree_1<T>>>) =
                        Set_::partitionAux(f, &t.Right, &acc.0.clone(),
                                           &acc.1.clone());
                    let acc_3:
                            (Option<Rc<Set_::SetTree_1<T>>>,
                             Option<Rc<Set_::SetTree_1<T>>>) =
                        Set_::partition1(f, &t.Key, &acc_2.0.clone(),
                                         &acc_2.1.clone());
                    Set_::partitionAux(f, &t.Left, &acc_3.0.clone(),
                                       &acc_3.1.clone())
                }
            }
            _ => acc,
        }
    }
    pub fn partition<a_: PartialOrd + Clone +
                     'static>(f: &Rc<impl Fn(&a_) -> (bool) + 'static>,
                              s: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> (Option<Rc<Set_::SetTree_1<a_>>>, Option<Rc<Set_::SetTree_1<a_>>>) {
        Set_::partitionAux(f, s, &Set_::empty(), &Set_::empty())
    }
    pub fn minimumElementAux<T: Clone +
                             'static>(s: &Option<Rc<Set_::SetTree_1<T>>>,
                                      n: &T) -> T {
        let s: Rc<MutCell<Option<Rc<Set_::SetTree_1<T>>>>> =
            Rc::from(MutCell::from(s.clone()));
        let n: Rc<MutCell<T>> = Rc::from(MutCell::from(n.clone()));
        '_minimumElementAux:
            loop  {
                break '_minimumElementAux
                    (match &s.get() {
                         Some(s_0_0) => {
                             let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                             if t.Height == 1i32 {
                                 t.Key.clone()
                             } else {
                                 let s_temp: Option<Rc<Set_::SetTree_1<T>>> =
                                     t.Left.clone();
                                 let n_temp: T = t.Key.clone();
                                 s.set(s_temp);
                                 n.set(n_temp);
                                 continue '_minimumElementAux
                             }
                         }
                         _ => n.get(),
                     }) ;
            }
    }
    pub fn minimumElementOpt<T: Clone +
                             'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<T> {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    Some(t.Key.clone())
                } else { Some(Set_::minimumElementAux(&t.Left, &t.Key)) }
            }
            _ => None::<T>,
        }
    }
    pub fn maximumElementAux<T: Clone +
                             'static>(s: &Option<Rc<Set_::SetTree_1<T>>>,
                                      n: &T) -> T {
        let s: Rc<MutCell<Option<Rc<Set_::SetTree_1<T>>>>> =
            Rc::from(MutCell::from(s.clone()));
        let n: Rc<MutCell<T>> = Rc::from(MutCell::from(n.clone()));
        '_maximumElementAux:
            loop  {
                break '_maximumElementAux
                    (match &s.get() {
                         Some(s_0_0) => {
                             let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                             if t.Height == 1i32 {
                                 t.Key.clone()
                             } else {
                                 let s_temp: Option<Rc<Set_::SetTree_1<T>>> =
                                     t.Right.clone();
                                 let n_temp: T = t.Key.clone();
                                 s.set(s_temp);
                                 n.set(n_temp);
                                 continue '_maximumElementAux
                             }
                         }
                         _ => n.get(),
                     }) ;
            }
    }
    pub fn maximumElementOpt<T: Clone +
                             'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Option<T> {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set_::SetTree_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    Some(t.Key.clone())
                } else { Some(Set_::maximumElementAux(&t.Right, &t.Key)) }
            }
            _ => None::<T>,
        }
    }
    pub fn minElement<a_: Clone +
                      'static>(s: &Option<Rc<Set_::SetTree_1<a_>>>) -> a_ {
        let matchValue: Option<a_> = Set_::minimumElementOpt(s);
        match &matchValue {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(Set_::SR::setContainsNoElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"s")) as Rc<str>),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn maxElement<a_: Clone +
                      'static>(s: &Option<Rc<Set_::SetTree_1<a_>>>) -> a_ {
        let matchValue: Option<a_> = Set_::maximumElementOpt(s);
        match &matchValue {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(Set_::SR::setContainsNoElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"s")) as Rc<str>),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    #[derive(Clone, Debug, Default, Hash)]
    pub struct SetIterator_1<T: PartialOrd + Clone + 'static> {
        pub stack: MutCell<List_1<Option<Rc<Set_::SetTree_1<T>>>>>,
        pub started: MutCell<bool>,
    }
    pub fn collapseLHS<T: Clone +
                       'static>(stack:
                                    &List_1<Option<Rc<Set_::SetTree_1<T>>>>)
     -> List_1<Option<Rc<Set_::SetTree_1<T>>>> {
        let stack: Rc<MutCell<List_1<Option<Rc<Set_::SetTree_1<T>>>>>> =
            Rc::from(MutCell::from(stack.clone()));
        '_collapseLHS:
            loop  {
                break '_collapseLHS
                    (if !List_::isEmpty(&stack.get()) {
                         let s: Option<Rc<Set_::SetTree_1<T>>> =
                             List_::head(&stack.get()).clone();
                         let rest: List_1<Option<Rc<Set_::SetTree_1<T>>>> =
                             List_::tail(&stack.get()).clone();
                         match &s {
                             Some(s_0_0) => {
                                 let t: Rc<Set_::SetTree_1<T>> =
                                     s_0_0.clone();
                                 if t.Height == 1i32 {
                                     stack.get()
                                 } else {
                                     let stack_temp:
                                             List_1<Option<Rc<Set_::SetTree_1<T>>>> =
                                         List_::cons(&t.Left,
                                                     &List_::cons(&Set_::mkSetTreeLeaf(&t.Key),
                                                                  &List_::cons(&t.Right,
                                                                               &rest)));
                                     stack.set(stack_temp);
                                     continue '_collapseLHS
                                 }
                             }
                             _ => {
                                 let stack_temp:
                                         List_1<Option<Rc<Set_::SetTree_1<T>>>> =
                                     rest.clone();
                                 stack.set(stack_temp);
                                 continue '_collapseLHS
                             }
                         }
                     } else {
                         List_::empty::<Option<Rc<Set_::SetTree_1<T>>>>()
                     }) ;
            }
    }
    pub fn mkIterator<a_: PartialOrd + Clone +
                      'static>(s: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> Rc<Set_::SetIterator_1<a_>> {
        Rc::from(Set_::SetIterator_1::<a_>{stack:
                                               MutCell::from(Set_::collapseLHS(&List_::singleton(s))),
                                           started: MutCell::from(false),})
    }
    pub fn notStarted<a_: Clone + 'static>() -> a_ {
        panic!("{}", Set_::SR::enumerationNotStarted())
    }
    pub fn alreadyFinished<a_: Clone + 'static>() -> a_ {
        panic!("{}", Set_::SR::enumerationAlreadyFinished())
    }
    pub fn current<T: PartialOrd + Clone +
                   'static>(i: &Rc<Set_::SetIterator_1<T>>) -> T {
        if i.started.get() {
            let matchValue: List_1<Option<Rc<Set_::SetTree_1<T>>>> =
                i.stack.get().clone();
            if !List_::isEmpty(&matchValue) {
                if List_::head(&matchValue).is_some() {
                    let k: Rc<Set_::SetTree_1<T>> =
                        Option_::getValue(&List_::head(&matchValue)).clone();
                    k.Key.clone()
                } else { Set_::alreadyFinished() }
            } else { Set_::alreadyFinished() }
        } else { Set_::notStarted() }
    }
    pub fn unexpectedStackForMoveNext<a_: Clone + 'static>() -> a_ {
        panic!("{}",
               String_::string(&"Please report error: Set iterator, unexpected stack for moveNext"))
    }
    pub fn unexpectedstateInSetTreeCompareStacks<a_: Clone + 'static>()
     -> a_ {
        panic!("{}",
               String_::string(&"unexpected state in SetTree.compareStacks"))
    }
    pub fn moveNext<T: PartialOrd + Clone +
                    'static>(i: &Rc<Set_::SetIterator_1<T>>) -> bool {
        if i.started.get() {
            let matchValue: List_1<Option<Rc<Set_::SetTree_1<T>>>> =
                i.stack.get().clone();
            if !List_::isEmpty(&matchValue) {
                if List_::head(&matchValue).is_some() {
                    let t: Rc<Set_::SetTree_1<T>> =
                        Option_::getValue(&List_::head(&matchValue)).clone();
                    if t.Height == 1i32 {
                        i.stack.set(Set_::collapseLHS(&List_::tail(&matchValue)));
                        !List_::isEmpty(&i.stack.get())
                    } else { Set_::unexpectedStackForMoveNext() }
                } else { false }
            } else { false }
        } else { i.started.set(true); !List_::isEmpty(&i.stack.get()) }
    }
    pub fn choose<a_: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<a_>>>)
     -> a_ {
        Set_::minElement(s)
    }
    pub fn copyToArray<a_: Clone +
                       'static>(s: &Option<Rc<Set_::SetTree_1<a_>>>,
                                arr: &Rc<MutCell<Vec<a_>>>, i: &i32) {
        let j: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
        Set_::iterate(&Rc::from({
                                    let arr = arr.clone();
                                    let j = j.clone();
                                    move |x: &a_|
                                        {
                                            arr.get_mut()[j.get() as usize] =
                                                x.clone();
                                            j.set(j.get() + 1i32)
                                        }
                                }), s)
    }
    pub fn toArray<T: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> =
            Native::arrayWithCapacity::<T>(&Set_::count(s));
        Set_::iterate(&Rc::from({
                                    let res = res.clone();
                                    move |x: &T| res.get_mut().push(x.clone())
                                }), s);
        res
    }
    pub fn toList<T: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> List_1<T> {
        Set_::foldBack(&Rc::from(move |k: &T, acc: &List_1<T>|
                                     List_::cons(k, acc)), s,
                       &List_::empty::<T>())
    }
    pub fn toSeq<T: Clone + 'static>(s: &Option<Rc<Set_::SetTree_1<T>>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let s = s.clone();
                                 move || Seq::ofArray(&Set_::toArray(&s))
                             }))
    }
    pub fn compareTo<T: PartialOrd + Clone +
                     'static>(s1: &Option<Rc<Set_::SetTree_1<T>>>,
                              s2: &Option<Rc<Set_::SetTree_1<T>>>) -> i32 {
        Util::compare(&Set_::toArray(s1), &Set_::toArray(s2))
    }
    pub fn equalsTo<T: Eq + core::hash::Hash + Clone +
                    'static>(s1: &Option<Rc<Set_::SetTree_1<T>>>,
                             s2: &Option<Rc<Set_::SetTree_1<T>>>) -> bool {
        Set_::toArray(s1) == Set_::toArray(s2)
    }
    pub fn ofArray<a_: PartialOrd + Clone +
                   'static>(xs: &Rc<MutCell<Vec<a_>>>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        Array_::fold(&Rc::from(move
                                   |acc: &Option<Rc<Set_::SetTree_1<a_>>>,
                                    k: &a_| Set_::add(k, acc)),
                     &Set_::empty(), xs)
    }
    pub fn ofList<a_: PartialOrd + Clone + 'static>(xs: &List_1<a_>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        List_::fold(&Rc::from(move
                                  |acc: &Option<Rc<Set_::SetTree_1<a_>>>,
                                   k: &a_| Set_::add(k, acc)), &Set_::empty(),
                    xs)
    }
    pub fn ofSeq<a_: PartialOrd + Clone +
                 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<a_>>)
     -> Option<Rc<Set_::SetTree_1<a_>>> {
        Seq::fold(&Rc::from(move
                                |acc: &Option<Rc<Set_::SetTree_1<a_>>>,
                                 k: &a_| Set_::add(k, acc)), &Set_::empty(),
                  xs)
    }
}
