#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_3bd9ae6a::*;
use crate::import_8d7d6be8::*;
use crate::import_f222008f::*;
use crate::import_52af85ec::*;
use crate::import_df4a7900::*;
use crate::import_ec6ee4e9::*;
use crate::import_c6216f2::*;
use std::rc::Rc;
pub mod Set {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn setContainsNoElements() -> Rc<str> {
            Native::string(&"Set contains no elements.")
        }
        pub fn enumerationNotStarted() -> Rc<str> {
            Native::string(&"Enumeration has not started. Call MoveNext.")
        }
        pub fn enumerationAlreadyFinished() -> Rc<str> {
            Native::string(&"Enumeration already finished.")
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetTreeNode_1<T: Clone + 'static> {
        pub Height: i32,
        pub Key: T,
        pub Left: Option<Rc<Set::SetTreeNode_1<T>>>,
        pub Right: Option<Rc<Set::SetTreeNode_1<T>>>,
    }
    pub fn empty<T: Clone + 'static>() -> Option<Rc<Set::SetTreeNode_1<T>>> {
        None::<Rc<Set::SetTreeNode_1<T>>>
    }
    pub fn isEmpty<T: Clone + 'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> bool {
        s.is_none()
    }
    pub fn mkSetTreeLeaf<T: Clone + 'static>(key: &T)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        Some(Rc::from(Set::SetTreeNode_1::<T>{Height: 1i32,
                                              Key: key.clone(),
                                              Left: Set::empty(),
                                              Right: Set::empty(),}))
    }
    pub fn mkSetTreeNode<T: Clone +
                         'static>(key: &T,
                                  left: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                  right: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                  height: &i32)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        Some(Rc::from(Set::SetTreeNode_1::<T>{Height: height.clone(),
                                              Key: key.clone(),
                                              Left: left.clone(),
                                              Right: right.clone(),}))
    }
    pub fn singleton<T: Clone + 'static>(value: &T)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        Set::mkSetTreeLeaf(value)
    }
    pub fn countAux<T: Clone +
                    'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>, acc: &i32)
     -> i32 {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    acc.clone() + 1i32
                } else {
                    Set::countAux(&t.Left,
                                  &Set::countAux(&t.Right,
                                                 &(acc.clone() + 1i32)))
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn count<a_: Clone + 'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> i32 {
        Set::countAux(s, &0i32)
    }
    pub fn height<T: Clone + 'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> i32 {
        match s { Some(s_0_0) => s_0_0.Height, _ => 0i32, }
    }
    pub fn mk<T: Clone +
              'static>(l: &Option<Rc<Set::SetTreeNode_1<T>>>, k: &T,
                       r: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        {
            let hl: i32 =
                {
                    let s: Option<Rc<Set::SetTreeNode_1<T>>> = l.clone();
                    match &s { Some(s_0_0) => s_0_0.Height, _ => 0i32, }
                };
            let hr: i32 =
                {
                    let s_1: Option<Rc<Set::SetTreeNode_1<T>>> = r.clone();
                    match &s_1 { Some(s_1_0_0) => s_1_0_0.Height, _ => 0i32, }
                };
            let m: i32 = if hl < hr { hr } else { hl };
            if m == 0i32 {
                Set::mkSetTreeLeaf(k)
            } else { Set::mkSetTreeNode(k, l, r, &(m + 1i32)) }
        }.clone()
    }
    pub fn rebalance<T: Clone +
                     'static>(t1: &Option<Rc<Set::SetTreeNode_1<T>>>, v: &T,
                              t2: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        {
            let t1h: i32 =
                {
                    let s: Option<Rc<Set::SetTreeNode_1<T>>> = t1.clone();
                    match &s { Some(s_0_0) => s_0_0.Height, _ => 0i32, }
                };
            let t2h: i32 =
                {
                    let s_1: Option<Rc<Set::SetTreeNode_1<T>>> = t2.clone();
                    match &s_1 { Some(s_1_0_0) => s_1_0_0.Height, _ => 0i32, }
                };
            if t2h > t1h + 2i32 {
                {
                    let t2_0027: Rc<Set::SetTreeNode_1<T>> =
                        Option_::getValue(t2).clone();
                    if {
                           let s_2: Option<Rc<Set::SetTreeNode_1<T>>> =
                               t2_0027.Left.clone();
                           match &s_2 {
                               Some(s_2_0_0) => s_2_0_0.Height,
                               _ => 0i32,
                           }
                       } > t1h + 1i32 {
                        {
                            let t2l: Rc<Set::SetTreeNode_1<T>> =
                                Option_::getValue(&t2_0027.Left).clone();
                            Set::mk(&Set::mk(t1, v, &t2l.Left), &t2l.Key,
                                    &Set::mk(&t2l.Right, &t2_0027.Key,
                                             &t2_0027.Right))
                        }.clone()
                    } else {
                        Set::mk(&Set::mk(t1, v, &t2_0027.Left), &t2_0027.Key,
                                &t2_0027.Right)
                    }
                }.clone()
            } else {
                if t1h > t2h + 2i32 {
                    {
                        let t1_0027: Rc<Set::SetTreeNode_1<T>> =
                            Option_::getValue(t1).clone();
                        if {
                               let s_3: Option<Rc<Set::SetTreeNode_1<T>>> =
                                   t1_0027.Right.clone();
                               match &s_3 {
                                   Some(s_3_0_0) => s_3_0_0.Height,
                                   _ => 0i32,
                               }
                           } > t2h + 1i32 {
                            {
                                let t1r: Rc<Set::SetTreeNode_1<T>> =
                                    Option_::getValue(&t1_0027.Right).clone();
                                Set::mk(&Set::mk(&t1_0027.Left, &t1_0027.Key,
                                                 &t1r.Left), &t1r.Key,
                                        &Set::mk(&t1r.Right, v, t2))
                            }.clone()
                        } else {
                            Set::mk(&t1_0027.Left, &t1_0027.Key,
                                    &Set::mk(&t1_0027.Right, v, t2))
                        }
                    }.clone()
                } else { Set::mk(t1, v, t2) }
            }
        }.clone()
    }
    pub fn add<T: PartialOrd + Clone +
               'static>(k: &T, s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c < 0i32 {
                        Set::mkSetTreeNode(k, &Set::empty(), s, &2i32)
                    } else {
                        if c == 0i32 {
                            s.clone()
                        } else {
                            Set::mkSetTreeNode(k, s, &Set::empty(), &2i32)
                        }
                    }
                } else {
                    if c < 0i32 {
                        Set::rebalance(&Set::add(k, &t.Left), &t.Key,
                                       &t.Right)
                    } else {
                        if c == 0i32 {
                            s.clone()
                        } else {
                            Set::rebalance(&t.Left, &t.Key,
                                           &Set::add(k, &t.Right))
                        }
                    }
                }
            }.clone(),
            _ => Set::mkSetTreeLeaf(k),
        }
    }
    pub fn balance<T: PartialOrd + Clone +
                   'static>(s1: &Option<Rc<Set::SetTreeNode_1<T>>>, k: &T,
                            s2: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match s1 {
            Some(s1_0_0) =>
            {
                let t1: Rc<Set::SetTreeNode_1<T>> = s1_0_0.clone();
                match s2 {
                    Some(s2_0_0) =>
                    {
                        let t2: Rc<Set::SetTreeNode_1<T>> = s2_0_0.clone();
                        if t1.Height == 1i32 {
                            Set::add(k, &Set::add(&t1.Key, s2))
                        } else {
                            if t2.Height == 1i32 {
                                Set::add(k, &Set::add(&t2.Key, s1))
                            } else {
                                if t1.Height + 2i32 < t2.Height {
                                    Set::rebalance(&Set::balance(s1, k,
                                                                 &t2.Left),
                                                   &t2.Key, &t2.Right)
                                } else {
                                    if t2.Height + 2i32 < t1.Height {
                                        Set::rebalance(&t1.Left, &t1.Key,
                                                       &Set::balance(&t1.Right,
                                                                     k, s2))
                                    } else { Set::mk(s1, k, s2) }
                                }
                            }
                        }
                    }.clone(),
                    _ => Set::add(k, s1),
                }
            }.clone(),
            _ => Set::add(k, s2),
        }
    }
    pub fn split<T: PartialOrd + Clone +
                 'static>(pivot: &T, s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     ->
         Rc<(Option<Rc<Set::SetTreeNode_1<T>>>, bool,
             Option<Rc<Set::SetTreeNode_1<T>>>)> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    {
                        let c: i32 = Util::compare(&t.Key, pivot);
                        if c < 0i32 {
                            Rc::from((s.clone(), false, Set::empty()))
                        } else {
                            if c == 0i32 {
                                Rc::from((Set::empty(), true, Set::empty()))
                            } else {
                                Rc::from((Set::empty(), false, s.clone()))
                            }
                        }
                    }.clone()
                } else {
                    {
                        let c_1: i32 = Util::compare(pivot, &t.Key);
                        if c_1 < 0i32 {
                            {
                                let patternInput:
                                        Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                                            bool,
                                            Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                    Set::split(pivot, &t.Left);
                                Rc::from((patternInput.0.clone(),
                                          patternInput.1.clone(),
                                          Set::balance(&patternInput.2.clone(),
                                                       &t.Key, &t.Right)))
                            }.clone()
                        } else {
                            if c_1 == 0i32 {
                                Rc::from((t.Left.clone(), true,
                                          t.Right.clone()))
                            } else {
                                {
                                    let patternInput_1:
                                            Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                                                bool,
                                                Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                        Set::split(pivot, &t.Right);
                                    Rc::from((Set::balance(&t.Left, &t.Key,
                                                           &patternInput_1.0.clone()),
                                              patternInput_1.1.clone(),
                                              patternInput_1.2.clone()))
                                }.clone()
                            }
                        }
                    }.clone()
                }
            }.clone(),
            _ => Rc::from((Set::empty(), false, Set::empty())),
        }
    }
    pub fn spliceOutSuccessor<T: Clone +
                              'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Rc<(T, Option<Rc<Set::SetTreeNode_1<T>>>)> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    Rc::from((t.Key.clone(), Set::empty()))
                } else {
                    if t.Left.is_none() {
                        Rc::from((t.Key.clone(), t.Right.clone()))
                    } else {
                        {
                            let patternInput:
                                    Rc<(T,
                                        Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                Set::spliceOutSuccessor(&t.Left);
                            Rc::from((patternInput.0.clone(),
                                      Set::mk(&patternInput.1.clone(), &t.Key,
                                              &t.Right)))
                        }.clone()
                    }
                }
            }.clone(),
            _ =>
            panic!("{}",
                   Native::string(&"internal error: Set.spliceOutSuccessor")),
        }
    }
    pub fn remove<T: PartialOrd + Clone +
                  'static>(k: &T, s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c == 0i32 { Set::empty() } else { s.clone() }
                } else {
                    if c < 0i32 {
                        Set::rebalance(&Set::remove(k, &t.Left), &t.Key,
                                       &t.Right)
                    } else {
                        if c == 0i32 {
                            if t.Left.is_none() {
                                t.Right.clone()
                            } else {
                                if t.Right.is_none() {
                                    t.Left.clone()
                                } else {
                                    {
                                        let patternInput:
                                                Rc<(T,
                                                    Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                            Set::spliceOutSuccessor(&t.Right);
                                        Set::mk(&t.Left,
                                                &patternInput.0.clone(),
                                                &patternInput.1.clone())
                                    }.clone()
                                }
                            }
                        } else {
                            Set::rebalance(&t.Left, &t.Key,
                                           &Set::remove(k, &t.Right))
                        }
                    }
                }
            }.clone(),
            _ => s.clone(),
        }
    }
    pub fn contains<T: PartialOrd + Clone +
                    'static>(k: &T, s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> bool {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    c == 0i32
                } else {
                    if c < 0i32 {
                        Set::contains(k, &t.Left)
                    } else {
                        if c == 0i32 {
                            true
                        } else { Set::contains(k, &t.Right) }
                    }
                }
            }
            _ => false,
        }
    }
    pub fn iterate<T: Clone +
                   'static>(f: &Rc<impl Fn(&T) + 'static>,
                            s: &Option<Rc<Set::SetTreeNode_1<T>>>) {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key)
                } else {
                    Set::iterate(f, &t.Left);
                    f(&t.Key);
                    Set::iterate(f, &t.Right)
                }
            }
            _ => (),
        };
    }
    pub fn foldBack<T: Clone + 'static, a_: Clone +
                    'static>(f: &Rc<impl Fn(&T, &a_) -> (a_) + 'static>,
                             s: &Option<Rc<Set::SetTreeNode_1<T>>>, x: &a_)
     -> a_ {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, x)
                } else {
                    Set::foldBack(f, &t.Left,
                                  &f(&t.Key, &Set::foldBack(f, &t.Right, x)))
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn fold<a_: Clone + 'static, T: Clone +
                'static>(f: &Rc<impl Fn(&a_, &T) -> (a_) + 'static>, x: &a_,
                         s: &Option<Rc<Set::SetTreeNode_1<T>>>) -> a_ {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(x, &t.Key)
                } else {
                    Set::fold(f, &f(&Set::fold(f, x, &t.Left), &t.Key),
                              &t.Right)
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn map<T: Clone + 'static, a_: PartialOrd + Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (a_) + 'static>,
                        s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        Set::fold(&Rc::from({
                                let mapping = mapping.clone();
                                move
                                    |acc: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                     k: &T| Set::add(&mapping(k), acc)
                            }), &Set::empty(), s)
    }
    pub fn forall<T: Clone +
                  'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           s: &Option<Rc<Set::SetTreeNode_1<T>>>) -> bool {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key)
                } else {
                    if if f(&t.Key) { Set::forall(f, &t.Left) } else { false }
                       {
                        Set::forall(f, &t.Right)
                    } else { false }
                }
            }
            _ => true,
        }
    }
    pub fn exists<T: Clone +
                  'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           s: &Option<Rc<Set::SetTreeNode_1<T>>>) -> bool {
        match s {
            Some(s_0_0) => {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key)
                } else {
                    if if f(&t.Key) { true } else { Set::exists(f, &t.Left) }
                       {
                        true
                    } else { Set::exists(f, &t.Right) }
                }
            }
            _ => false,
        }
    }
    pub fn isSubset<a_: PartialOrd + Clone +
                    'static>(a: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                             b: &Option<Rc<Set::SetTreeNode_1<a_>>>) -> bool {
        Set::forall(&Rc::from({
                                  let b = b.clone();
                                  move |x: &a_| Set::contains(x, &b)
                              }), a)
    }
    pub fn isSuperset<a_: PartialOrd + Clone +
                      'static>(a: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                               b: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> bool {
        Set::isSubset(b, a)
    }
    pub fn isProperSubset<a_: PartialOrd + Clone +
                          'static>(a: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                   b: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> bool {
        if Set::forall(&Rc::from({
                                     let b = b.clone();
                                     move |x: &a_| Set::contains(x, &b)
                                 }), a) {
            Set::exists(&Rc::from({
                                      let a = a.clone();
                                      move |x_1: &a_| !Set::contains(x_1, &a)
                                  }), b)
        } else { false }
    }
    pub fn isProperSuperset<a_: PartialOrd + Clone +
                            'static>(a: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                     b: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> bool {
        Set::isProperSubset(b, a)
    }
    pub fn filterAux<T: PartialOrd + Clone +
                     'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              s: &Option<Rc<Set::SetTreeNode_1<T>>>,
                              acc: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    if f(&t.Key) {
                        Set::add(&t.Key, acc)
                    } else { acc.clone() }
                } else {
                    Set::filterAux(f, &t.Left,
                                   &Set::filterAux(f, &t.Right,
                                                   &if f(&t.Key) {
                                                        Set::add(&t.Key, acc)
                                                    } else { acc.clone() }))
                }
            }.clone(),
            _ => acc.clone(),
        }
    }
    pub fn filter<a_: PartialOrd + Clone +
                  'static>(f: &Rc<impl Fn(&a_) -> (bool) + 'static>,
                           s: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        Set::filterAux(f, s, &Set::empty())
    }
    pub fn diffAux<T: PartialOrd + Clone +
                   'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>,
                            acc: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match acc {
            Some(acc_0_0) =>
            {
                let _acc: Rc<Set::SetTreeNode_1<T>> = acc_0_0.clone();
                match s {
                    Some(s_0_0) =>
                    {
                        let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                        if t.Height == 1i32 {
                            Set::remove(&t.Key, acc)
                        } else {
                            Set::diffAux(&t.Left,
                                         &Set::diffAux(&t.Right,
                                                       &Set::remove(&t.Key,
                                                                    acc)))
                        }
                    }.clone(),
                    _ => acc.clone(),
                }
            }.clone(),
            _ => acc.clone(),
        }
    }
    pub fn difference<a_: PartialOrd + Clone +
                      'static>(a: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                               b: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        Set::diffAux(b, a)
    }
    pub fn r#union<T: PartialOrd + Clone +
                   'static>(s1: &Option<Rc<Set::SetTreeNode_1<T>>>,
                            s2: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match s1 {
            Some(s1_0_0) =>
            {
                let t1: Rc<Set::SetTreeNode_1<T>> = s1_0_0.clone();
                match s2 {
                    Some(s2_0_0) =>
                    {
                        let t2: Rc<Set::SetTreeNode_1<T>> = s2_0_0.clone();
                        if t1.Height == 1i32 {
                            Set::add(&t1.Key, s2)
                        } else {
                            if t2.Height == 1i32 {
                                Set::add(&t2.Key, s1)
                            } else {
                                if t1.Height > t2.Height {
                                    {
                                        let patternInput:
                                                Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                                                    bool,
                                                    Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                            Set::split(&t1.Key, s2);
                                        Set::balance(&Set::r#union(&t1.Left,
                                                                   &patternInput.0.clone()),
                                                     &t1.Key,
                                                     &Set::r#union(&t1.Right,
                                                                   &patternInput.2.clone()))
                                    }.clone()
                                } else {
                                    {
                                        let patternInput_1:
                                                Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                                                    bool,
                                                    Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                            Set::split(&t2.Key, s1);
                                        Set::balance(&Set::r#union(&t2.Left,
                                                                   &patternInput_1.0.clone()),
                                                     &t2.Key,
                                                     &Set::r#union(&t2.Right,
                                                                   &patternInput_1.2.clone()))
                                    }.clone()
                                }
                            }
                        }
                    }.clone(),
                    _ => s1.clone(),
                }
            }.clone(),
            _ => s2.clone(),
        }
    }
    pub fn intersectionAux<T: PartialOrd + Clone +
                           'static>(b: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                    s: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                    acc: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    if Set::contains(&t.Key, b) {
                        Set::add(&t.Key, acc)
                    } else { acc.clone() }
                } else {
                    {
                        let acc_1: Option<Rc<Set::SetTreeNode_1<T>>> =
                            Set::intersectionAux(b, &t.Right, acc);
                        Set::intersectionAux(b, &t.Left,
                                             &if Set::contains(&t.Key, b) {
                                                  Set::add(&t.Key, &acc_1)
                                              } else { acc_1.clone() })
                    }.clone()
                }
            }.clone(),
            _ => acc.clone(),
        }
    }
    pub fn intersect<a_: PartialOrd + Clone +
                     'static>(a: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                              b: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        if b.clone().is_none() {
            b.clone()
        } else { Set::intersectionAux(b, a, &Set::empty()) }
    }
    pub fn intersectMany<T: PartialOrd + Clone +
                         'static>(sets:
                                      &Rc<dyn Seq::Enumerable::IEnumerable_1<Option<Rc<Set::SetTreeNode_1<T>>>>>)
     -> Option<Rc<Set::SetTreeNode_1<T>>> {
        Seq::reduce(&Rc::from(move
                                  |a: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                   b: &Option<Rc<Set::SetTreeNode_1<T>>>|
                                  Set::intersect(a, b)), sets)
    }
    pub fn partition1<a_: PartialOrd + Clone +
                      'static>(f: &Rc<impl Fn(&a_) -> (bool) + 'static>,
                               k: &a_,
                               acc1: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                               acc2: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     ->
         Rc<(Option<Rc<Set::SetTreeNode_1<a_>>>,
             Option<Rc<Set::SetTreeNode_1<a_>>>)> {
        if f(k) {
            Rc::from((Set::add(k, acc1), acc2.clone()))
        } else { Rc::from((acc1.clone(), Set::add(k, acc2))) }
    }
    pub fn partitionAux<T: PartialOrd + Clone +
                        'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 s: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                 acc_0: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                 acc_1: &Option<Rc<Set::SetTreeNode_1<T>>>)
     ->
         Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
             Option<Rc<Set::SetTreeNode_1<T>>>)> {
        {
            let acc:
                    Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                        Option<Rc<Set::SetTreeNode_1<T>>>)> =
                Rc::from((acc_0.clone(), acc_1.clone()));
            match s {
                Some(s_0_0) =>
                {
                    let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                    if t.Height == 1i32 {
                        Set::partition1(f, &t.Key, &acc.0.clone(),
                                        &acc.1.clone())
                    } else {
                        {
                            let acc_2:
                                    Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                                        Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                Set::partitionAux(f, &t.Right, &acc.0.clone(),
                                                  &acc.1.clone());
                            let acc_3:
                                    Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                                        Option<Rc<Set::SetTreeNode_1<T>>>)> =
                                Set::partition1(f, &t.Key, &acc_2.0.clone(),
                                                &acc_2.1.clone());
                            Set::partitionAux(f, &t.Left, &acc_3.0.clone(),
                                              &acc_3.1.clone())
                        }.clone()
                    }
                }.clone(),
                _ => acc.clone(),
            }
        }.clone()
    }
    pub fn partition<a_: PartialOrd + Clone +
                     'static>(f: &Rc<impl Fn(&a_) -> (bool) + 'static>,
                              s: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     ->
         Rc<(Option<Rc<Set::SetTreeNode_1<a_>>>,
             Option<Rc<Set::SetTreeNode_1<a_>>>)> {
        Set::partitionAux(f, s, &Set::empty(), &Set::empty())
    }
    pub fn minimumElementAux<T: Clone +
                             'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                      n: &T) -> T {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    t.Key.clone()
                } else { Set::minimumElementAux(&t.Left, &t.Key) }
            }.clone(),
            _ => n.clone(),
        }
    }
    pub fn minimumElementOpt<T: Clone +
                             'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<T> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    Some(t.Key.clone())
                } else { Some(Set::minimumElementAux(&t.Left, &t.Key)) }
            }.clone(),
            _ => None::<T>,
        }
    }
    pub fn maximumElementAux<T: Clone +
                             'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                      n: &T) -> T {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    t.Key.clone()
                } else { Set::maximumElementAux(&t.Right, &t.Key) }
            }.clone(),
            _ => n.clone(),
        }
    }
    pub fn maximumElementOpt<T: Clone +
                             'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> Option<T> {
        match s {
            Some(s_0_0) =>
            {
                let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                if t.Height == 1i32 {
                    Some(t.Key.clone())
                } else { Some(Set::maximumElementAux(&t.Right, &t.Key)) }
            }.clone(),
            _ => None::<T>,
        }
    }
    pub fn minElement<a_: Clone +
                      'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>) -> a_ {
        {
            let matchValue: Option<a_> = Set::minimumElementOpt(s);
            match &matchValue {
                None =>
                panic!("{}",
                       Rc::from((Rc::from(Set::SR::setContainsNoElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"s")) as Rc<str>),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn maxElement<a_: Clone +
                      'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>) -> a_ {
        {
            let matchValue: Option<a_> = Set::maximumElementOpt(s);
            match &matchValue {
                None =>
                panic!("{}",
                       Rc::from((Rc::from(Set::SR::setContainsNoElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"s")) as Rc<str>),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    #[derive(Clone, Debug)]
    pub struct SetIterator_1<T: PartialOrd + Clone + 'static> {
        pub stack: MutCell<List_1<Option<Rc<Set::SetTreeNode_1<T>>>>>,
        pub started: MutCell<bool>,
    }
    pub fn collapseLHS<T: Clone +
                       'static>(stack:
                                    &List_1<Option<Rc<Set::SetTreeNode_1<T>>>>)
     -> List_1<Option<Rc<Set::SetTreeNode_1<T>>>> {
        if !List::isEmpty(stack) {
            let s: Option<Rc<Set::SetTreeNode_1<T>>> =
                List::head(stack).clone();
            let rest: List_1<Option<Rc<Set::SetTreeNode_1<T>>>> =
                List::tail(stack).clone();
            match &s {
                Some(s_0_0) =>
                {
                    let t: Rc<Set::SetTreeNode_1<T>> = s_0_0.clone();
                    if t.Height == 1i32 {
                        stack.clone()
                    } else {
                        Set::collapseLHS(&List::cons(&t.Left,
                                                     &List::cons(&Set::mkSetTreeLeaf(&t.Key),
                                                                 &List::cons(&t.Right,
                                                                             &rest))))
                    }
                }.clone(),
                _ => Set::collapseLHS(&rest),
            }
        } else { List::empty::<Option<Rc<Set::SetTreeNode_1<T>>>>() }
    }
    pub fn mkIterator<a_: PartialOrd + Clone +
                      'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> Rc<Set::SetIterator_1<a_>> {
        Rc::from(Set::SetIterator_1::<a_>{stack:
                                              MutCell::from(Set::collapseLHS(&List::singleton(s))),
                                          started: MutCell::from(false),})
    }
    pub fn notStarted<a_: Clone + 'static>() -> a_ {
        panic!("{}", Set::SR::enumerationNotStarted())
    }
    pub fn alreadyFinished<a_: Clone + 'static>() -> a_ {
        panic!("{}", Set::SR::enumerationAlreadyFinished())
    }
    pub fn current<T: PartialOrd + Clone +
                   'static>(i: &Rc<Set::SetIterator_1<T>>) -> T {
        if i.started.get() {
            {
                let matchValue: List_1<Option<Rc<Set::SetTreeNode_1<T>>>> =
                    i.stack.get().clone();
                if !List::isEmpty(&matchValue) {
                    if List::head(&matchValue).is_some() {
                        let k: Rc<Set::SetTreeNode_1<T>> =
                            Option_::getValue(&List::head(&matchValue)).clone();
                        k.Key.clone()
                    } else { Set::alreadyFinished() }
                } else { Set::alreadyFinished() }
            }.clone()
        } else { Set::notStarted() }
    }
    pub fn unexpectedStackForMoveNext<a_: Clone + 'static>() -> a_ {
        panic!("{}",
               Native::string(&"Please report error: Set iterator, unexpected stack for moveNext"))
    }
    pub fn unexpectedstateInSetTreeCompareStacks<a_: Clone + 'static>()
     -> a_ {
        panic!("{}",
               Native::string(&"unexpected state in SetTree.compareStacks"))
    }
    pub fn moveNext<T: PartialOrd + Clone +
                    'static>(i: &Rc<Set::SetIterator_1<T>>) -> bool {
        if i.started.get() {
            let matchValue: List_1<Option<Rc<Set::SetTreeNode_1<T>>>> =
                i.stack.get().clone();
            if !List::isEmpty(&matchValue) {
                if List::head(&matchValue).is_some() {
                    let t: Rc<Set::SetTreeNode_1<T>> =
                        Option_::getValue(&List::head(&matchValue)).clone();
                    if t.Height == 1i32 {
                        i.stack.set(Set::collapseLHS(&List::tail(&matchValue)));
                        !List::isEmpty(&i.stack.get())
                    } else { Set::unexpectedStackForMoveNext() }
                } else { false }
            } else { false }
        } else { i.started.set(true); !List::isEmpty(&i.stack.get()) }
    }
    pub fn compareStacks<T: PartialOrd + Clone +
                         'static>(l1:
                                      &List_1<Option<Rc<Set::SetTreeNode_1<T>>>>,
                                  l2:
                                      &List_1<Option<Rc<Set::SetTreeNode_1<T>>>>)
     -> i32 {
        let cont =
            Rc::from({
                         let l1 = l1.clone();
                         let l2 = l2.clone();
                         move ||
                             {
                                 let matchValue:
                                         Rc<(List_1<Option<Rc<Set::SetTreeNode_1<T>>>>,
                                             List_1<Option<Rc<Set::SetTreeNode_1<T>>>>)> =
                                     Rc::from((l1.clone(), l2.clone()));
                                 if !List::isEmpty(&matchValue.0.clone()) {
                                     if List::head(&matchValue.0.clone()).is_some()
                                        {
                                         let x1: Rc<Set::SetTreeNode_1<T>> =
                                             Option_::getValue(&List::head(&matchValue.0.clone())).clone();
                                         if x1.Height == 1i32 {
                                             Set::compareStacks(&List::cons(&Set::empty(),
                                                                            &List::cons(&Set::mkSetTreeLeaf(&x1.Key),
                                                                                        &List::tail(&matchValue.0.clone()))),
                                                                &l2)
                                         } else {
                                             Set::compareStacks(&List::cons(&x1.Left,
                                                                            &List::cons(&Set::mkSetTreeNode(&x1.Key,
                                                                                                            &Set::empty(),
                                                                                                            &x1.Right,
                                                                                                            &0i32),
                                                                                        &List::tail(&matchValue.0.clone()))),
                                                                &l2)
                                         }
                                     } else {
                                         if !List::isEmpty(&matchValue.1.clone())
                                            {
                                             if List::head(&matchValue.1.clone()).is_some()
                                                {
                                                 let x2:
                                                         Rc<Set::SetTreeNode_1<T>> =
                                                     Option_::getValue(&List::head(&matchValue.1.clone())).clone();
                                                 if x2.Height == 1i32 {
                                                     Set::compareStacks(&l1,
                                                                        &List::cons(&Set::empty(),
                                                                                    &List::cons(&Set::mkSetTreeLeaf(&x2.Key),
                                                                                                &List::tail(&matchValue.1.clone()))))
                                                 } else {
                                                     Set::compareStacks(&l1,
                                                                        &List::cons(&x2.Left,
                                                                                    &List::cons(&Set::mkSetTreeNode(&x2.Key,
                                                                                                                    &Set::empty(),
                                                                                                                    &x2.Right,
                                                                                                                    &0i32),
                                                                                                &List::tail(&matchValue.1.clone()))))
                                                 }
                                             } else {
                                                 Set::unexpectedstateInSetTreeCompareStacks()
                                             }
                                         } else {
                                             Set::unexpectedstateInSetTreeCompareStacks()
                                         }
                                     }
                                 } else {
                                     if !List::isEmpty(&matchValue.1.clone())
                                        {
                                         if List::head(&matchValue.1.clone()).is_some()
                                            {
                                             let x2:
                                                     Rc<Set::SetTreeNode_1<T>> =
                                                 Option_::getValue(&List::head(&matchValue.1.clone())).clone();
                                             if x2.Height == 1i32 {
                                                 Set::compareStacks(&l1,
                                                                    &List::cons(&Set::empty(),
                                                                                &List::cons(&Set::mkSetTreeLeaf(&x2.Key),
                                                                                            &List::tail(&matchValue.1.clone()))))
                                             } else {
                                                 Set::compareStacks(&l1,
                                                                    &List::cons(&x2.Left,
                                                                                &List::cons(&Set::mkSetTreeNode(&x2.Key,
                                                                                                                &Set::empty(),
                                                                                                                &x2.Right,
                                                                                                                &0i32),
                                                                                            &List::tail(&matchValue.1.clone()))))
                                             }
                                         } else {
                                             Set::unexpectedstateInSetTreeCompareStacks()
                                         }
                                     } else {
                                         Set::unexpectedstateInSetTreeCompareStacks()
                                     }
                                 }
                             }
                     });
        let matchValue_1:
                Rc<(List_1<Option<Rc<Set::SetTreeNode_1<T>>>>,
                    List_1<Option<Rc<Set::SetTreeNode_1<T>>>>)> =
            Rc::from((l1.clone(), l2.clone()));
        if !List::isEmpty(&matchValue_1.0.clone()) {
            if !List::isEmpty(&matchValue_1.1.clone()) {
                let matchValue_2:
                        Rc<(Option<Rc<Set::SetTreeNode_1<T>>>,
                            Option<Rc<Set::SetTreeNode_1<T>>>)> =
                    Rc::from((List::head(&matchValue_1.0.clone()).clone(),
                              List::head(&matchValue_1.1.clone()).clone()));
                if matchValue_2.0.clone().is_some() {
                    if matchValue_2.1.clone().is_some() {
                        let x1_1: Rc<Set::SetTreeNode_1<T>> =
                            Option_::getValue(&matchValue_2.0.clone()).clone();
                        let x2_1: Rc<Set::SetTreeNode_1<T>> =
                            Option_::getValue(&matchValue_2.1.clone()).clone();
                        if x1_1.Height == 1i32 {
                            if x2_1.Height == 1i32 {
                                let c: i32 =
                                    Util::compare(&x1_1.Key, &x2_1.Key);
                                if c != 0i32 {
                                    c
                                } else {
                                    Set::compareStacks(&List::tail(&matchValue_1.0.clone()),
                                                       &List::tail(&matchValue_1.1.clone()))
                                }
                            } else {
                                if x2_1.Left.is_none() {
                                    let c_1: i32 =
                                        Util::compare(&x1_1.Key, &x2_1.Key);
                                    if c_1 != 0i32 {
                                        c_1
                                    } else {
                                        Set::compareStacks(&List::cons(&Set::empty(),
                                                                       &List::tail(&matchValue_1.0.clone())),
                                                           &List::cons(&x2_1.Right,
                                                                       &List::tail(&matchValue_1.1.clone())))
                                    }
                                } else { cont() }
                            }
                        } else {
                            if x1_1.Left.is_none() {
                                if x2_1.Height == 1i32 {
                                    let c_2: i32 =
                                        Util::compare(&x1_1.Key, &x2_1.Key);
                                    if c_2 != 0i32 {
                                        c_2
                                    } else {
                                        Set::compareStacks(&List::cons(&x1_1.Right,
                                                                       &List::tail(&matchValue_1.0.clone())),
                                                           &List::cons(&Set::empty(),
                                                                       &List::tail(&matchValue_1.1.clone())))
                                    }
                                } else {
                                    if x2_1.Left.is_none() {
                                        let c_3: i32 =
                                            Util::compare(&x1_1.Key,
                                                          &x2_1.Key);
                                        if c_3 != 0i32 {
                                            c_3
                                        } else {
                                            Set::compareStacks(&List::cons(&x1_1.Right,
                                                                           &List::tail(&matchValue_1.0.clone())),
                                                               &List::cons(&x2_1.Right,
                                                                           &List::tail(&matchValue_1.1.clone())))
                                        }
                                    } else { cont() }
                                }
                            } else { cont() }
                        }
                    } else { cont() }
                } else {
                    if matchValue_2.1.clone().is_none() {
                        Set::compareStacks(&List::tail(&matchValue_1.0.clone()),
                                           &List::tail(&matchValue_1.1.clone()))
                    } else { cont() }
                }
            } else { 1i32 }
        } else {
            if List::isEmpty(&matchValue_1.1.clone()) { 0i32 } else { -1i32 }
        }
    }
    pub fn compareTrees<T: PartialOrd + Clone +
                        'static>(t1: &Option<Rc<Set::SetTreeNode_1<T>>>,
                                 t2: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> i32 {
        if t1.clone().is_none() {
            if t2.clone().is_none() { 0i32 } else { -1i32 }
        } else {
            if t2.clone().is_none() {
                1i32
            } else {
                Set::compareStacks(&List::singleton(t1), &List::singleton(t2))
            }
        }
    }
    pub fn choose<a_: Clone + 'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> a_ {
        Set::minElement(s)
    }
    pub fn copyToArray<a_: Clone +
                       'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                arr: &Rc<MutCell<Vec<a_>>>, i: &i32) {
        let j: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
        Set::iterate(&Rc::from({
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
    pub fn toArray<a_: Clone +
                   'static>(s: &Option<Rc<Set::SetTreeNode_1<a_>>>)
     -> Rc<MutCell<Vec<a_>>> {
        {
            let res: Rc<MutCell<Vec<a_>>> =
                Native::arrayWithCapacity::<a_>(&Set::count(s));
            Set::iterate(&Rc::from({
                                       let res = res.clone();
                                       move |x: &a_|
                                           res.get_mut().push(x.clone())
                                   }), s);
            res.clone()
        }.clone()
    }
    pub fn toList<T: Clone + 'static>(s: &Option<Rc<Set::SetTreeNode_1<T>>>)
     -> List_1<T> {
        Set::foldBack(&Rc::from(move |k: &T, acc: &List_1<T>|
                                    List::cons(k, acc)), s,
                      &List::empty::<T>())
    }
    pub fn ofArray<a_: PartialOrd + Clone +
                   'static>(xs: &Rc<MutCell<Vec<a_>>>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        Array::fold(&Rc::from(move
                                  |acc: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                   k: &a_| Set::add(k, acc)), &Set::empty(),
                    xs)
    }
    pub fn ofList<a_: PartialOrd + Clone + 'static>(xs: &List_1<a_>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        List::fold(&Rc::from(move
                                 |acc: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                  k: &a_| Set::add(k, acc)), &Set::empty(),
                   xs)
    }
    pub fn ofSeq<a_: PartialOrd + Clone +
                 'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<a_>>)
     -> Option<Rc<Set::SetTreeNode_1<a_>>> {
        Seq::fold(&Rc::from(move
                                |acc: &Option<Rc<Set::SetTreeNode_1<a_>>>,
                                 k: &a_| Set::add(k, acc)), &Set::empty(), xs)
    }
}
