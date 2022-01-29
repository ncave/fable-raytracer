#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_3bd9ae6a::*;
#[path = "./Option.rs"]
pub(crate) mod import_8d7d6be8;
pub use import_8d7d6be8::*;
#[path = "./Util.rs"]
pub(crate) mod import_f222008f;
pub use import_f222008f::*;
#[path = "./List.rs"]
pub(crate) mod import_ec6ee4e9;
pub use import_ec6ee4e9::*;
#[path = "./Seq.rs"]
pub(crate) mod import_52af85ec;
pub use import_52af85ec::*;
#[path = "./Array.rs"]
pub(crate) mod import_c6216f2;
pub use import_c6216f2::*;
pub mod Map_ {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn mapCannotBeMutated() -> Rc<str> {
            Native::string(&"Map values cannot be mutated.")
        }
        pub fn enumerationNotStarted() -> Rc<str> {
            Native::string(&"Enumeration has not started. Call MoveNext.")
        }
        pub fn enumerationAlreadyFinished() -> Rc<str> {
            Native::string(&"Enumeration already finished.")
        }
        pub fn keyNotFound() -> Rc<str> {
            Native::string(&"The item, key, or index was not found in the collection.")
        }
    }
    #[derive(Clone, Debug, Default, Hash)]
    pub struct MapTree_2<K: Clone + 'static, V: Clone + 'static> {
        pub Height: i32,
        pub Key: K,
        pub Value: V,
        pub Left: Option<Rc<Map_::MapTree_2<K, V>>>,
        pub Right: Option<Rc<Map_::MapTree_2<K, V>>>,
    }
    pub fn empty<K: Clone + 'static, V: Clone + 'static>()
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        None::<Rc<Map_::MapTree_2<K, V>>>
    }
    pub fn isEmpty<K: Clone + 'static, V: Clone +
                   'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> bool {
        m.is_none()
    }
    pub fn mkMapTreeLeaf<K: Clone + 'static, V: Clone + 'static>(k: &K, v: &V)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        Some(Rc::from(Map_::MapTree_2::<K,
                                        V>{Height: 1i32,
                                           Key: k.clone(),
                                           Value: v.clone(),
                                           Left: Map_::empty(),
                                           Right: Map_::empty(),}))
    }
    pub fn mkMapTreeNode<K: Clone + 'static, V: Clone +
                         'static>(k: &K, v: &V,
                                  left: &Option<Rc<Map_::MapTree_2<K, V>>>,
                                  right: &Option<Rc<Map_::MapTree_2<K, V>>>,
                                  h: &i32)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        Some(Rc::from(Map_::MapTree_2::<K,
                                        V>{Height: h.clone(),
                                           Key: k.clone(),
                                           Value: v.clone(),
                                           Left: left.clone(),
                                           Right: right.clone(),}))
    }
    pub fn singleton<K: Clone + 'static, V: Clone + 'static>(k: &K, v: &V)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        Map_::mkMapTreeLeaf(k, v)
    }
    pub fn sizeAux<K: Clone + 'static, V: Clone +
                   'static>(acc: &i32, m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> i32 {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    acc.clone() + 1i32
                } else {
                    Map_::sizeAux(&Map_::sizeAux(&(acc.clone() + 1i32),
                                                 &t.Left), &t.Right)
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn count<a_: Clone + 'static, b_: Clone +
                 'static>(x: &Option<Rc<Map_::MapTree_2<a_, b_>>>) -> i32 {
        Map_::sizeAux(&0i32, x)
    }
    pub fn height<K: Clone + 'static, V: Clone +
                  'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> i32 {
        match m { Some(m_0_0) => m_0_0.Height, _ => 0i32, }
    }
    pub fn mk<K: Clone + 'static, V: Clone +
              'static>(l: &Option<Rc<Map_::MapTree_2<K, V>>>, k: &K, v: &V,
                       r: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        {
            let hl: i32 =
                {
                    let m: Option<Rc<Map_::MapTree_2<K, V>>> = l.clone();
                    match &m { Some(m_0_0) => m_0_0.Height, _ => 0i32, }
                };
            let hr: i32 =
                {
                    let m_1: Option<Rc<Map_::MapTree_2<K, V>>> = r.clone();
                    match &m_1 { Some(m_1_0_0) => m_1_0_0.Height, _ => 0i32, }
                };
            let m_2: i32 = if hl < hr { hr } else { hl };
            if m_2 == 0i32 {
                Map_::mkMapTreeLeaf(k, v)
            } else { Map_::mkMapTreeNode(k, v, l, r, &(m_2 + 1i32)) }
        }.clone()
    }
    pub fn rebalance<K: Clone + 'static, V: Clone +
                     'static>(t1: &Option<Rc<Map_::MapTree_2<K, V>>>, k: &K,
                              v: &V, t2: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        {
            let t1h: i32 =
                {
                    let m: Option<Rc<Map_::MapTree_2<K, V>>> = t1.clone();
                    match &m { Some(m_0_0) => m_0_0.Height, _ => 0i32, }
                };
            let t2h: i32 =
                {
                    let m_1: Option<Rc<Map_::MapTree_2<K, V>>> = t2.clone();
                    match &m_1 { Some(m_1_0_0) => m_1_0_0.Height, _ => 0i32, }
                };
            if t2h > t1h + 2i32 {
                {
                    let t2_0027: Rc<Map_::MapTree_2<K, V>> =
                        Option_::getValue(t2).clone();
                    if {
                           let m_2: Option<Rc<Map_::MapTree_2<K, V>>> =
                               t2_0027.Left.clone();
                           match &m_2 {
                               Some(m_2_0_0) => m_2_0_0.Height,
                               _ => 0i32,
                           }
                       } > t1h + 1i32 {
                        {
                            let t2l: Rc<Map_::MapTree_2<K, V>> =
                                Option_::getValue(&t2_0027.Left).clone();
                            Map_::mk(&Map_::mk(t1, k, v, &t2l.Left), &t2l.Key,
                                     &t2l.Value,
                                     &Map_::mk(&t2l.Right, &t2_0027.Key,
                                               &t2_0027.Value,
                                               &t2_0027.Right))
                        }.clone()
                    } else {
                        Map_::mk(&Map_::mk(t1, k, v, &t2_0027.Left),
                                 &t2_0027.Key, &t2_0027.Value, &t2_0027.Right)
                    }
                }.clone()
            } else {
                if t1h > t2h + 2i32 {
                    {
                        let t1_0027: Rc<Map_::MapTree_2<K, V>> =
                            Option_::getValue(t1).clone();
                        if {
                               let m_3: Option<Rc<Map_::MapTree_2<K, V>>> =
                                   t1_0027.Right.clone();
                               match &m_3 {
                                   Some(m_3_0_0) => m_3_0_0.Height,
                                   _ => 0i32,
                               }
                           } > t2h + 1i32 {
                            {
                                let t1r: Rc<Map_::MapTree_2<K, V>> =
                                    Option_::getValue(&t1_0027.Right).clone();
                                Map_::mk(&Map_::mk(&t1_0027.Left,
                                                   &t1_0027.Key,
                                                   &t1_0027.Value, &t1r.Left),
                                         &t1r.Key, &t1r.Value,
                                         &Map_::mk(&t1r.Right, k, v, t2))
                            }.clone()
                        } else {
                            Map_::mk(&t1_0027.Left, &t1_0027.Key,
                                     &t1_0027.Value,
                                     &Map_::mk(&t1_0027.Right, k, v, t2))
                        }
                    }.clone()
                } else { Map_::mk(t1, k, v, t2) }
            }
        }.clone()
    }
    pub fn add<K: PartialOrd + Clone + 'static, V: Clone +
               'static>(k: &K, v: &V, m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c < 0i32 {
                        Map_::mkMapTreeNode(k, v, &Map_::empty(), m, &2i32)
                    } else {
                        if c == 0i32 {
                            Map_::mkMapTreeLeaf(k, v)
                        } else {
                            Map_::mkMapTreeNode(k, v, m, &Map_::empty(),
                                                &2i32)
                        }
                    }
                } else {
                    if c < 0i32 {
                        Map_::rebalance(&Map_::add(k, v, &t.Left), &t.Key,
                                        &t.Value, &t.Right)
                    } else {
                        if c == 0i32 {
                            Map_::mkMapTreeNode(k, v, &t.Left, &t.Right,
                                                &t.Height)
                        } else {
                            Map_::rebalance(&t.Left, &t.Key, &t.Value,
                                            &Map_::add(k, v, &t.Right))
                        }
                    }
                }
            }.clone(),
            _ => Map_::mkMapTreeLeaf(k, v),
        }
    }
    pub fn tryGetValue<K: PartialOrd + Clone + 'static, V: Clone +
                       'static>(k: &K, v: &Rc<MutCell<V>>,
                                m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if c == 0i32 {
                    v.set(t.Value.clone());
                    true
                } else {
                    if t.Height == 1i32 {
                        false
                    } else {
                        Map_::tryGetValue(k, v,
                                          &if c < 0i32 {
                                               t.Left.clone()
                                           } else { t.Right.clone() })
                    }
                }
            }
            _ => false,
        }
    }
    pub fn throwKeyNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", Map_::SR::keyNotFound())
    }
    pub fn find<K: PartialOrd + Clone + 'static, V: Clone +
                'static>(k: &K, m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> V {
        {
            let v: Rc<MutCell<V>> =
                Rc::from(MutCell::from(Native::defaultOf::<V>()));
            if Map_::tryGetValue(k, &v, m) {
                v.get().clone()
            } else { Map_::throwKeyNotFound() }
        }.clone()
    }
    pub fn tryFind<K: PartialOrd + Clone + 'static, V: Clone +
                   'static>(k: &K, m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<V> {
        {
            let v: Rc<MutCell<V>> =
                Rc::from(MutCell::from(Native::defaultOf::<V>()));
            if Map_::tryGetValue(k, &v, m) {
                Some(v.get().clone())
            } else { None::<V> }
        }.clone()
    }
    pub fn item<K: PartialOrd + Clone + 'static, V: Clone +
                'static>(k: &K, m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> V {
        Map_::find(k, m)
    }
    pub fn partition1<a_: PartialOrd + Clone + 'static, b_: Clone +
                      'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                               k: &a_, v: &b_,
                               acc1: &Option<Rc<Map_::MapTree_2<a_, b_>>>,
                               acc2: &Option<Rc<Map_::MapTree_2<a_, b_>>>)
     ->
         (Option<Rc<Map_::MapTree_2<a_, b_>>>,
          Option<Rc<Map_::MapTree_2<a_, b_>>>) {
        if f(k, v) {
            (Map_::add(k, v, acc1), acc2.clone())
        } else { (acc1.clone(), Map_::add(k, v, acc2)) }
    }
    pub fn partitionAux<K: PartialOrd + Clone + 'static, V: Clone +
                        'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                                 m: &Option<Rc<Map_::MapTree_2<K, V>>>,
                                 acc_0: &Option<Rc<Map_::MapTree_2<K, V>>>,
                                 acc_1: &Option<Rc<Map_::MapTree_2<K, V>>>)
     ->
         (Option<Rc<Map_::MapTree_2<K, V>>>,
          Option<Rc<Map_::MapTree_2<K, V>>>) {
        let acc:
                (Option<Rc<Map_::MapTree_2<K, V>>>,
                 Option<Rc<Map_::MapTree_2<K, V>>>) =
            (acc_0.clone(), acc_1.clone());
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map_::partition1(f, &t.Key, &t.Value, &acc.0.clone(),
                                     &acc.1.clone())
                } else {
                    let acc_2:
                            (Option<Rc<Map_::MapTree_2<K, V>>>,
                             Option<Rc<Map_::MapTree_2<K, V>>>) =
                        Map_::partitionAux(f, &t.Right, &acc.0.clone(),
                                           &acc.1.clone());
                    let acc_3:
                            (Option<Rc<Map_::MapTree_2<K, V>>>,
                             Option<Rc<Map_::MapTree_2<K, V>>>) =
                        Map_::partition1(f, &t.Key, &t.Value,
                                         &acc_2.0.clone(), &acc_2.1.clone());
                    Map_::partitionAux(f, &t.Left, &acc_3.0.clone(),
                                       &acc_3.1.clone())
                }
            }
            _ => acc,
        }
    }
    pub fn partition<a_: PartialOrd + Clone + 'static, b_: Clone +
                     'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                              m: &Option<Rc<Map_::MapTree_2<a_, b_>>>)
     ->
         (Option<Rc<Map_::MapTree_2<a_, b_>>>,
          Option<Rc<Map_::MapTree_2<a_, b_>>>) {
        Map_::partitionAux(f, m, &Map_::empty(), &Map_::empty())
    }
    pub fn filter1<a_: PartialOrd + Clone + 'static, b_: Clone +
                   'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                            k: &a_, v: &b_,
                            acc: &Option<Rc<Map_::MapTree_2<a_, b_>>>)
     -> Option<Rc<Map_::MapTree_2<a_, b_>>> {
        if f(k, v) { Map_::add(k, v, acc) } else { acc.clone() }
    }
    pub fn filterAux<K: PartialOrd + Clone + 'static, V: Clone +
                     'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                              m: &Option<Rc<Map_::MapTree_2<K, V>>>,
                              acc: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map_::filter1(f, &t.Key, &t.Value, acc)
                } else {
                    Map_::filterAux(f, &t.Right,
                                    &Map_::filter1(f, &t.Key, &t.Value,
                                                   &Map_::filterAux(f,
                                                                    &t.Left,
                                                                    acc)))
                }
            }.clone(),
            _ => acc.clone(),
        }
    }
    pub fn filter<a_: PartialOrd + Clone + 'static, b_: Clone +
                  'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                           m: &Option<Rc<Map_::MapTree_2<a_, b_>>>)
     -> Option<Rc<Map_::MapTree_2<a_, b_>>> {
        Map_::filterAux(f, m, &Map_::empty())
    }
    pub fn spliceOutSuccessor<K: Clone + 'static, V: Clone +
                              'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> (K, V, Option<Rc<Map_::MapTree_2<K, V>>>) {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    (t.Key.clone(), t.Value.clone(), Map_::empty())
                } else {
                    if t.Left.is_none() {
                        (t.Key.clone(), t.Value.clone(), t.Right.clone())
                    } else {
                        let patternInput:
                                (K, V, Option<Rc<Map_::MapTree_2<K, V>>>) =
                            Map_::spliceOutSuccessor(&t.Left);
                        (patternInput.0.clone(), patternInput.1.clone(),
                         Map_::mk(&patternInput.2.clone(), &t.Key, &t.Value,
                                  &t.Right))
                    }
                }
            }
            _ =>
            panic!("{}",
                   Native::string(&"internal error: Map.spliceOutSuccessor")),
        }
    }
    pub fn remove<K: PartialOrd + Clone + 'static, V: Clone +
                  'static>(k: &K, m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c == 0i32 { Map_::empty() } else { m.clone() }
                } else {
                    if c < 0i32 {
                        Map_::rebalance(&Map_::remove(k, &t.Left), &t.Key,
                                        &t.Value, &t.Right)
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
                                                (K, V,
                                                 Option<Rc<Map_::MapTree_2<K,
                                                                           V>>>) =
                                            Map_::spliceOutSuccessor(&t.Right);
                                        Map_::mk(&t.Left,
                                                 &patternInput.0.clone(),
                                                 &patternInput.1.clone(),
                                                 &patternInput.2.clone())
                                    }.clone()
                                }
                            }
                        } else {
                            Map_::rebalance(&t.Left, &t.Key, &t.Value,
                                            &Map_::remove(k, &t.Right))
                        }
                    }
                }
            }.clone(),
            _ => Map_::empty(),
        }
    }
    pub fn change<K: PartialOrd + Clone + 'static, V: Clone +
                  'static>(k: &K,
                           u:
                               &Rc<impl Fn(&Option<V>) -> (Option<V>) +
                                   'static>,
                           m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    {
                        let c: i32 = Util::compare(k, &t.Key);
                        if c < 0i32 {
                            {
                                let matchValue_1: Option<V> = u(&None::<V>);
                                match &matchValue_1 {
                                    Some(matchValue_1_0_0) =>
                                    Map_::mkMapTreeNode(k, matchValue_1_0_0,
                                                        &Map_::empty(), m,
                                                        &2i32),
                                    _ => m.clone(),
                                }
                            }.clone()
                        } else {
                            if c == 0i32 {
                                {
                                    let matchValue_2: Option<V> =
                                        u(&Some(t.Value.clone()));
                                    match &matchValue_2 {
                                        Some(matchValue_2_0_0) =>
                                        Map_::mkMapTreeLeaf(k,
                                                            matchValue_2_0_0),
                                        _ => Map_::empty(),
                                    }
                                }.clone()
                            } else {
                                {
                                    let matchValue_3: Option<V> =
                                        u(&None::<V>);
                                    match &matchValue_3 {
                                        Some(matchValue_3_0_0) =>
                                        Map_::mkMapTreeNode(k,
                                                            matchValue_3_0_0,
                                                            m, &Map_::empty(),
                                                            &2i32),
                                        _ => m.clone(),
                                    }
                                }.clone()
                            }
                        }
                    }.clone()
                } else {
                    {
                        let c_1: i32 = Util::compare(k, &t.Key);
                        if c_1 < 0i32 {
                            Map_::rebalance(&Map_::change(k, u, &t.Left),
                                            &t.Key, &t.Value, &t.Right)
                        } else {
                            if c_1 == 0i32 {
                                {
                                    let matchValue_4: Option<V> =
                                        u(&Some(t.Value.clone()));
                                    match &matchValue_4 {
                                        Some(matchValue_4_0_0) =>
                                        Map_::mkMapTreeNode(k,
                                                            matchValue_4_0_0,
                                                            &t.Left, &t.Right,
                                                            &t.Height),
                                        _ =>
                                        if t.Left.is_none() {
                                            t.Right.clone()
                                        } else {
                                            if t.Right.is_none() {
                                                t.Left.clone()
                                            } else {
                                                {
                                                    let patternInput:
                                                            (K, V,
                                                             Option<Rc<Map_::MapTree_2<K,
                                                                                       V>>>) =
                                                        Map_::spliceOutSuccessor(&t.Right);
                                                    Map_::mk(&t.Left,
                                                             &patternInput.0.clone(),
                                                             &patternInput.1.clone(),
                                                             &patternInput.2.clone())
                                                }.clone()
                                            }
                                        },
                                    }
                                }.clone()
                            } else {
                                Map_::rebalance(&t.Left, &t.Key, &t.Value,
                                                &Map_::change(k, u, &t.Right))
                            }
                        }
                    }.clone()
                }
            }.clone(),
            _ =>
            {
                let matchValue: Option<V> = u(&None::<V>);
                match &matchValue {
                    Some(matchValue_0_0) =>
                    Map_::mkMapTreeLeaf(k, matchValue_0_0),
                    _ => m.clone(),
                }
            }.clone(),
        }
    }
    pub fn containsKey<K: PartialOrd + Clone + 'static, V: Clone +
                       'static>(k: &K, m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    c == 0i32
                } else {
                    if c < 0i32 {
                        Map_::containsKey(k, &t.Left)
                    } else {
                        if c == 0i32 {
                            true
                        } else { Map_::containsKey(k, &t.Right) }
                    }
                }
            }
            _ => false,
        }
    }
    pub fn iterate<K: Clone + 'static, V: Clone +
                   'static>(f: &Rc<impl Fn(&K, &V) + 'static>,
                            m: &Option<Rc<Map_::MapTree_2<K, V>>>) {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    Map_::iterate(f, &t.Left);
                    f(&t.Key, &t.Value);
                    Map_::iterate(f, &t.Right)
                }
            }
            _ => (),
        };
    }
    pub fn tryPick<K: Clone + 'static, V: Clone + 'static, a_: Clone +
                   'static>(f: &Rc<impl Fn(&K, &V) -> (Option<a_>) + 'static>,
                            m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<a_> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    {
                        let matchValue: Option<a_> =
                            Map_::tryPick(f, &t.Left);
                        match &matchValue {
                            None =>
                            {
                                let matchValue_1: Option<a_> =
                                    f(&t.Key, &t.Value);
                                match &matchValue_1 {
                                    None => Map_::tryPick(f, &t.Right),
                                    _ => matchValue_1.clone(),
                                }
                            }.clone(),
                            _ => matchValue.clone(),
                        }
                    }.clone()
                }
            }.clone(),
            _ => None::<a_>,
        }
    }
    pub fn pick<K: Clone + 'static, V: Clone + 'static, a_: Clone +
                'static>(chooser:
                             &Rc<impl Fn(&K, &V) -> (Option<a_>) + 'static>,
                         m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> a_ {
        {
            let matchValue: Option<a_> = Map_::tryPick(chooser, m);
            match &matchValue {
                Some(matchValue_0_0) => matchValue_0_0.clone(),
                _ => Map_::throwKeyNotFound(),
            }
        }.clone()
    }
    pub fn findKey<K: Clone + 'static, V: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                            m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> K {
        Map_::pick(&Rc::from({
                                 let predicate = predicate.clone();
                                 move |k: &K, v: &V|
                                     if predicate(k, v) {
                                         Some(k.clone())
                                     } else { None::<K> }
                             }), m)
    }
    pub fn tryFindKey<K: Clone + 'static, V: Clone +
                      'static>(predicate:
                                   &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                               m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<K> {
        Map_::tryPick(&Rc::from({
                                    let predicate = predicate.clone();
                                    move |k: &K, v: &V|
                                        if predicate(k, v) {
                                            Some(k.clone())
                                        } else { None::<K> }
                                }), m)
    }
    pub fn exists<K: Clone + 'static, V: Clone +
                  'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                           m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    if if Map_::exists(f, &t.Left) {
                           true
                       } else { f(&t.Key, &t.Value) } {
                        true
                    } else { Map_::exists(f, &t.Right) }
                }
            }
            _ => false,
        }
    }
    pub fn forAll<K: Clone + 'static, V: Clone +
                  'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                           m: &Option<Rc<Map_::MapTree_2<K, V>>>) -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    if if Map_::forAll(f, &t.Left) {
                           f(&t.Key, &t.Value)
                       } else { false } {
                        Map_::forAll(f, &t.Right)
                    } else { false }
                }
            }
            _ => true,
        }
    }
    pub fn mapRange<V: Clone + 'static, R: Clone + 'static, K: Clone +
                    'static>(f: &Rc<impl Fn(&V) -> (R) + 'static>,
                             m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, R>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map_::mkMapTreeLeaf(&t.Key, &f(&t.Value))
                } else {
                    {
                        let l2: Option<Rc<Map_::MapTree_2<K, R>>> =
                            Map_::mapRange(f, &t.Left);
                        Map_::mkMapTreeNode(&t.Key, &f(&t.Value), &l2,
                                            &Map_::mapRange(f, &t.Right),
                                            &t.Height)
                    }.clone()
                }
            }.clone(),
            _ => Map_::empty(),
        }
    }
    pub fn map<K: Clone + 'static, V: Clone + 'static, R: Clone +
               'static>(f: &Rc<impl Fn(&K, &V) -> (R) + 'static>,
                        m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Option<Rc<Map_::MapTree_2<K, R>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map_::mkMapTreeLeaf(&t.Key, &f(&t.Key, &t.Value))
                } else {
                    {
                        let l2: Option<Rc<Map_::MapTree_2<K, R>>> =
                            Map_::map(f, &t.Left);
                        Map_::mkMapTreeNode(&t.Key, &f(&t.Key, &t.Value), &l2,
                                            &Map_::map(f, &t.Right),
                                            &t.Height)
                    }.clone()
                }
            }.clone(),
            _ => Map_::empty(),
        }
    }
    pub fn foldBack<K: Clone + 'static, V: Clone + 'static, a_: Clone +
                    'static>(f: &Rc<impl Fn(&K, &V, &a_) -> (a_) + 'static>,
                             m: &Option<Rc<Map_::MapTree_2<K, V>>>, x: &a_)
     -> a_ {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value, x)
                } else {
                    Map_::foldBack(f, &t.Left,
                                   &f(&t.Key, &t.Value,
                                      &Map_::foldBack(f, &t.Right, x)))
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn fold<a_: Clone + 'static, K: Clone + 'static, V: Clone +
                'static>(f: &Rc<impl Fn(&a_, &K, &V) -> (a_) + 'static>,
                         x: &a_, m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> a_ {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(x, &t.Key, &t.Value)
                } else {
                    Map_::fold(f,
                               &f(&Map_::fold(f, x, &t.Left), &t.Key,
                                  &t.Value), &t.Right)
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn foldFromTo<K: PartialOrd + Clone + 'static, V: Clone + 'static,
                      a_: Clone +
                      'static>(lo: &K, hi: &K,
                               f: &Rc<impl Fn(&K, &V, &a_) -> (a_) + 'static>,
                               m: &Option<Rc<Map_::MapTree_2<K, V>>>, x: &a_)
     -> a_ {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    if if Util::compare(lo, &t.Key) <= 0i32 {
                           Util::compare(&t.Key, hi) <= 0i32
                       } else { false } {
                        f(&t.Key, &t.Value, x)
                    } else { x.clone() }
                } else {
                    {
                        let cLoKey_1: i32 = Util::compare(lo, &t.Key);
                        let cKeyHi_1: i32 = Util::compare(&t.Key, hi);
                        let x_2: a_ =
                            if cLoKey_1 < 0i32 {
                                Map_::foldFromTo(lo, hi, f, &t.Left, x)
                            } else { x.clone() };
                        let x_3: a_ =
                            if if cLoKey_1 <= 0i32 {
                                   cKeyHi_1 <= 0i32
                               } else { false } {
                                f(&t.Key, &t.Value, &x_2)
                            } else { x_2.clone() };
                        if cKeyHi_1 < 0i32 {
                            Map_::foldFromTo(lo, hi, f, &t.Right, &x_3)
                        } else { x_3.clone() }
                    }.clone()
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn foldSection<K: PartialOrd + Clone + 'static, V: Clone + 'static,
                       a_: Clone +
                       'static>(lo: &K, hi: &K,
                                f:
                                    &Rc<impl Fn(&K, &V, &a_) -> (a_) +
                                        'static>,
                                m: &Option<Rc<Map_::MapTree_2<K, V>>>, x: &a_)
     -> a_ {
        if Util::compare(lo, hi) == 1i32 {
            x.clone()
        } else { Map_::foldFromTo(lo, hi, f, m, x) }
    }
    pub fn copyToArray<a_: Clone + 'static, b_: Clone +
                       'static>(m: &Option<Rc<Map_::MapTree_2<a_, b_>>>,
                                arr: &Rc<MutCell<Vec<(a_, b_)>>>, i: &i32) {
        let j: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
        Map_::iterate(&Rc::from({
                                    let arr = arr.clone();
                                    let j = j.clone();
                                    move |k: &a_, v: &b_|
                                        {
                                            arr.get_mut()[j.get() as usize] =
                                                (k.clone(), v.clone());
                                            j.set(j.get() + 1i32)
                                        }
                                }), m)
    }
    pub fn keys<K: Clone + 'static, V: Clone +
                'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Rc<MutCell<Vec<K>>> {
        {
            let res: Rc<MutCell<Vec<K>>> =
                Native::arrayWithCapacity::<K>(&Map_::count(m));
            Map_::iterate(&Rc::from({
                                        let res = res.clone();
                                        move |k: &K, v: &V|
                                            res.get_mut().push(k.clone())
                                    }), m);
            res.clone()
        }.clone()
    }
    pub fn values<K: Clone + 'static, V: Clone +
                  'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Rc<MutCell<Vec<V>>> {
        {
            let res: Rc<MutCell<Vec<V>>> =
                Native::arrayWithCapacity::<V>(&Map_::count(m));
            Map_::iterate(&Rc::from({
                                        let res = res.clone();
                                        move |k: &K, v: &V|
                                            res.get_mut().push(v.clone())
                                    }), m);
            res.clone()
        }.clone()
    }
    pub fn toArray<K: Clone + 'static, V: Clone +
                   'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Rc<MutCell<Vec<(K, V)>>> {
        {
            let res: Rc<MutCell<Vec<(K, V)>>> =
                Native::arrayWithCapacity::<(K, V)>(&Map_::count(m));
            Map_::iterate(&Rc::from({
                                        let res = res.clone();
                                        move |k: &K, v: &V|
                                            res.get_mut().push((k.clone(),
                                                                v.clone()))
                                    }), m);
            res.clone()
        }.clone()
    }
    pub fn toList<K: Clone + 'static, V: Clone +
                  'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> List_1<(K, V)> {
        Map_::foldBack(&Rc::from(move |k: &K, v: &V, acc: &List_1<(K, V)>|
                                     List_::cons(&(k.clone(), v.clone()),
                                                 acc)), m,
                       &List_::empty::<(K, V)>())
    }
    pub fn toSeq<K: Clone + 'static, V: Clone +
                 'static>(m: &Option<Rc<Map_::MapTree_2<K, V>>>)
     -> Rc<dyn Interfaces::IEnumerable_1<(K, V)>> {
        Seq::delay(&Rc::from({
                                 let m = m.clone();
                                 move || Seq::ofArray(&Map_::toArray(&m))
                             }))
    }
    pub fn compareTo<K: PartialOrd + Clone + 'static, V: PartialOrd + Clone +
                     'static>(m1: &Option<Rc<Map_::MapTree_2<K, V>>>,
                              m2: &Option<Rc<Map_::MapTree_2<K, V>>>) -> i32 {
        Util::compare(&Map_::toArray(m1), &Map_::toArray(m2))
    }
    pub fn equalsTo<K: Eq + core::hash::Hash + Clone + 'static, V: Eq +
                    core::hash::Hash + Clone +
                    'static>(m1: &Option<Rc<Map_::MapTree_2<K, V>>>,
                             m2: &Option<Rc<Map_::MapTree_2<K, V>>>) -> bool {
        Map_::toArray(m1) == Map_::toArray(m2)
    }
    pub fn ofArray<a_: PartialOrd + Clone + 'static, b_: Clone +
                   'static>(xs: &Rc<MutCell<Vec<(a_, b_)>>>)
     -> Option<Rc<Map_::MapTree_2<a_, b_>>> {
        Array_::fold(&Rc::from(move
                                   |acc: &Option<Rc<Map_::MapTree_2<a_, b_>>>,
                                    tupledArg: &(a_, b_)|
                                   Map_::add(&tupledArg.0.clone(),
                                             &tupledArg.1.clone(), acc)),
                     &Map_::empty(), xs)
    }
    pub fn ofList<a_: PartialOrd + Clone + 'static, b_: Clone +
                  'static>(xs: &List_1<(a_, b_)>)
     -> Option<Rc<Map_::MapTree_2<a_, b_>>> {
        List_::fold(&Rc::from(move
                                  |acc: &Option<Rc<Map_::MapTree_2<a_, b_>>>,
                                   tupledArg: &(a_, b_)|
                                  Map_::add(&tupledArg.0.clone(),
                                            &tupledArg.1.clone(), acc)),
                    &Map_::empty(), xs)
    }
    pub fn ofSeq<a_: PartialOrd + Clone + 'static, b_: Clone +
                 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<(a_, b_)>>)
     -> Option<Rc<Map_::MapTree_2<a_, b_>>> {
        Seq::fold(&Rc::from(move
                                |acc: &Option<Rc<Map_::MapTree_2<a_, b_>>>,
                                 tupledArg: &(a_, b_)|
                                Map_::add(&tupledArg.0.clone(),
                                          &tupledArg.1.clone(), acc)),
                  &Map_::empty(), xs)
    }
    #[derive(Clone, Debug, Default, Hash)]
    pub struct MapIterator_2<K: PartialOrd + Clone + 'static, V: Clone +
                             'static> {
        pub stack: MutCell<List_1<Option<Rc<Map_::MapTree_2<K, V>>>>>,
        pub started: MutCell<bool>,
    }
    pub fn collapseLHS<K: Clone + 'static, V: Clone +
                       'static>(stack:
                                    &List_1<Option<Rc<Map_::MapTree_2<K,
                                                                      V>>>>)
     -> List_1<Option<Rc<Map_::MapTree_2<K, V>>>> {
        if !List_::isEmpty(stack) {
            let rest: List_1<Option<Rc<Map_::MapTree_2<K, V>>>> =
                List_::tail(stack).clone();
            let m: Option<Rc<Map_::MapTree_2<K, V>>> =
                List_::head(stack).clone();
            match &m {
                Some(m_0_0) =>
                {
                    let t: Rc<Map_::MapTree_2<K, V>> = m_0_0.clone();
                    if t.Height == 1i32 {
                        stack.clone()
                    } else {
                        Map_::collapseLHS(&List_::cons(&t.Left,
                                                       &List_::cons(&Map_::mkMapTreeLeaf(&t.Key,
                                                                                         &t.Value),
                                                                    &List_::cons(&t.Right,
                                                                                 &rest))))
                    }
                }.clone(),
                _ => Map_::collapseLHS(&rest),
            }
        } else { List_::empty::<Option<Rc<Map_::MapTree_2<K, V>>>>() }
    }
    pub fn mkIterator<a_: PartialOrd + Clone + 'static, b_: Clone +
                      'static>(m: &Option<Rc<Map_::MapTree_2<a_, b_>>>)
     -> Rc<Map_::MapIterator_2<a_, b_>> {
        Rc::from(Map_::MapIterator_2::<a_,
                                       b_>{stack:
                                               MutCell::from(Map_::collapseLHS(&List_::singleton(m))),
                                           started: MutCell::from(false),})
    }
    pub fn notStarted<a_: Clone + 'static>() -> a_ {
        panic!("{}", Map_::SR::enumerationNotStarted())
    }
    pub fn alreadyFinished<a_: Clone + 'static>() -> a_ {
        panic!("{}", Map_::SR::enumerationAlreadyFinished())
    }
    pub fn unexpectedStackForCurrent<a_: Clone + 'static>() -> a_ {
        panic!("{}",
               Native::string(&"Please report error: Map iterator, unexpected stack for current"))
    }
    pub fn unexpectedStackForMoveNext<a_: Clone + 'static>() -> a_ {
        panic!("{}",
               Native::string(&"Please report error: Map iterator, unexpected stack for moveNext"))
    }
}
