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
#[path = "./Option.rs"]
pub(crate) mod import_8d7d6be8;
pub use import_8d7d6be8::*;
#[path = "./Util.rs"]
pub(crate) mod import_f222008f;
pub use import_f222008f::*;
#[path = "./Types.rs"]
pub(crate) mod import_df4a7900;
pub use import_df4a7900::*;
#[path = "./List.rs"]
pub(crate) mod import_ec6ee4e9;
pub use import_ec6ee4e9::*;
#[path = "./Array.rs"]
pub(crate) mod import_c6216f2;
pub use import_c6216f2::*;
#[path = "./Seq.rs"]
pub(crate) mod import_52af85ec;
pub use import_52af85ec::*;
use std::rc::Rc;
pub mod Map {
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
    #[derive(Clone, Debug)]
    pub struct MapTreeNode_2<K: Clone + 'static, V: Clone + 'static> {
        pub Height: i32,
        pub Key: K,
        pub Value: V,
        pub Left: Option<Rc<Map::MapTreeNode_2<K, V>>>,
        pub Right: Option<Rc<Map::MapTreeNode_2<K, V>>>,
    }
    pub fn empty<K: Clone + 'static, V: Clone + 'static>()
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        None::<Rc<Map::MapTreeNode_2<K, V>>>
    }
    pub fn isEmpty<K: Clone + 'static, V: Clone +
                   'static>(m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> bool {
        m.is_none()
    }
    pub fn mkMapTreeLeaf<K: Clone + 'static, V: Clone + 'static>(k: &K, v: &V)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        Some(Rc::from(Map::MapTreeNode_2::<K,
                                           V>{Height: 1i32,
                                              Key: k.clone(),
                                              Value: v.clone(),
                                              Left: Map::empty(),
                                              Right: Map::empty(),}))
    }
    pub fn mkMapTreeNode<K: Clone + 'static, V: Clone +
                         'static>(k: &K, v: &V,
                                  left: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                  right:
                                      &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                  h: &i32)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        Some(Rc::from(Map::MapTreeNode_2::<K,
                                           V>{Height: h.clone(),
                                              Key: k.clone(),
                                              Value: v.clone(),
                                              Left: left.clone(),
                                              Right: right.clone(),}))
    }
    pub fn singleton<K: Clone + 'static, V: Clone + 'static>(k: &K, v: &V)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        Map::mkMapTreeLeaf(k, v)
    }
    pub fn sizeAux<K: Clone + 'static, V: Clone +
                   'static>(acc: &i32,
                            m: &Option<Rc<Map::MapTreeNode_2<K, V>>>) -> i32 {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    acc.clone() + 1i32
                } else {
                    Map::sizeAux(&Map::sizeAux(&(acc.clone() + 1i32),
                                               &t.Left), &t.Right)
                }
            }
            _ => acc.clone(),
        }
    }
    pub fn count<a_: Clone + 'static, b_: Clone +
                 'static>(x: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>) -> i32 {
        Map::sizeAux(&0i32, x)
    }
    pub fn height<K: Clone + 'static, V: Clone +
                  'static>(m: &Option<Rc<Map::MapTreeNode_2<K, V>>>) -> i32 {
        match m { Some(m_0_0) => m_0_0.Height, _ => 0i32, }
    }
    pub fn mk<K: Clone + 'static, V: Clone +
              'static>(l: &Option<Rc<Map::MapTreeNode_2<K, V>>>, k: &K, v: &V,
                       r: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        {
            let hl: i32 =
                {
                    let m: Option<Rc<Map::MapTreeNode_2<K, V>>> = l.clone();
                    match &m { Some(m_0_0) => m_0_0.Height, _ => 0i32, }
                };
            let hr: i32 =
                {
                    let m_1: Option<Rc<Map::MapTreeNode_2<K, V>>> = r.clone();
                    match &m_1 { Some(m_1_0_0) => m_1_0_0.Height, _ => 0i32, }
                };
            let m_2: i32 = if hl < hr { hr } else { hl };
            if m_2 == 0i32 {
                Map::mkMapTreeLeaf(k, v)
            } else { Map::mkMapTreeNode(k, v, l, r, &(m_2 + 1i32)) }
        }.clone()
    }
    pub fn rebalance<K: Clone + 'static, V: Clone +
                     'static>(t1: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                              k: &K, v: &V,
                              t2: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        {
            let t1h: i32 =
                {
                    let m: Option<Rc<Map::MapTreeNode_2<K, V>>> = t1.clone();
                    match &m { Some(m_0_0) => m_0_0.Height, _ => 0i32, }
                };
            let t2h: i32 =
                {
                    let m_1: Option<Rc<Map::MapTreeNode_2<K, V>>> =
                        t2.clone();
                    match &m_1 { Some(m_1_0_0) => m_1_0_0.Height, _ => 0i32, }
                };
            if t2h > t1h + 2i32 {
                {
                    let t2_0027: Rc<Map::MapTreeNode_2<K, V>> =
                        Option_::getValue(t2).clone();
                    if {
                           let m_2: Option<Rc<Map::MapTreeNode_2<K, V>>> =
                               t2_0027.Left.clone();
                           match &m_2 {
                               Some(m_2_0_0) => m_2_0_0.Height,
                               _ => 0i32,
                           }
                       } > t1h + 1i32 {
                        {
                            let t2l: Rc<Map::MapTreeNode_2<K, V>> =
                                Option_::getValue(&t2_0027.Left).clone();
                            Map::mk(&Map::mk(t1, k, v, &t2l.Left), &t2l.Key,
                                    &t2l.Value,
                                    &Map::mk(&t2l.Right, &t2_0027.Key,
                                             &t2_0027.Value, &t2_0027.Right))
                        }.clone()
                    } else {
                        Map::mk(&Map::mk(t1, k, v, &t2_0027.Left),
                                &t2_0027.Key, &t2_0027.Value, &t2_0027.Right)
                    }
                }.clone()
            } else {
                if t1h > t2h + 2i32 {
                    {
                        let t1_0027: Rc<Map::MapTreeNode_2<K, V>> =
                            Option_::getValue(t1).clone();
                        if {
                               let m_3: Option<Rc<Map::MapTreeNode_2<K, V>>> =
                                   t1_0027.Right.clone();
                               match &m_3 {
                                   Some(m_3_0_0) => m_3_0_0.Height,
                                   _ => 0i32,
                               }
                           } > t2h + 1i32 {
                            {
                                let t1r: Rc<Map::MapTreeNode_2<K, V>> =
                                    Option_::getValue(&t1_0027.Right).clone();
                                Map::mk(&Map::mk(&t1_0027.Left, &t1_0027.Key,
                                                 &t1_0027.Value, &t1r.Left),
                                        &t1r.Key, &t1r.Value,
                                        &Map::mk(&t1r.Right, k, v, t2))
                            }.clone()
                        } else {
                            Map::mk(&t1_0027.Left, &t1_0027.Key,
                                    &t1_0027.Value,
                                    &Map::mk(&t1_0027.Right, k, v, t2))
                        }
                    }.clone()
                } else { Map::mk(t1, k, v, t2) }
            }
        }.clone()
    }
    pub fn add<K: PartialOrd + Clone + 'static, V: Clone +
               'static>(k: &K, v: &V,
                        m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c < 0i32 {
                        Map::mkMapTreeNode(k, v, &Map::empty(), m, &2i32)
                    } else {
                        if c == 0i32 {
                            Map::mkMapTreeLeaf(k, v)
                        } else {
                            Map::mkMapTreeNode(k, v, m, &Map::empty(), &2i32)
                        }
                    }
                } else {
                    if c < 0i32 {
                        Map::rebalance(&Map::add(k, v, &t.Left), &t.Key,
                                       &t.Value, &t.Right)
                    } else {
                        if c == 0i32 {
                            Map::mkMapTreeNode(k, v, &t.Left, &t.Right,
                                               &t.Height)
                        } else {
                            Map::rebalance(&t.Left, &t.Key, &t.Value,
                                           &Map::add(k, v, &t.Right))
                        }
                    }
                }
            }.clone(),
            _ => Map::mkMapTreeLeaf(k, v),
        }
    }
    pub fn tryGetValue<K: PartialOrd + Clone + 'static, V: Clone +
                       'static>(k: &K, v: &Rc<MutCell<V>>,
                                m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if c == 0i32 {
                    v.set(t.Value.clone());
                    true
                } else {
                    if t.Height == 1i32 {
                        false
                    } else {
                        Map::tryGetValue(k, v,
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
        panic!("{}", Map::SR::keyNotFound())
    }
    pub fn find<K: PartialOrd + Clone + 'static, V: Clone +
                'static>(k: &K, m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> V {
        {
            let v: Rc<MutCell<V>> =
                Rc::from(MutCell::from(Native::defaultOf::<V>()));
            if Map::tryGetValue(k, &v, m) {
                v.get().clone()
            } else { Map::throwKeyNotFound() }
        }.clone()
    }
    pub fn tryFind<K: PartialOrd + Clone + 'static, V: Clone +
                   'static>(k: &K, m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<V> {
        {
            let v: Rc<MutCell<V>> =
                Rc::from(MutCell::from(Native::defaultOf::<V>()));
            if Map::tryGetValue(k, &v, m) {
                Some(v.get().clone())
            } else { None::<V> }
        }.clone()
    }
    pub fn partition1<a_: PartialOrd + Clone + 'static, b_: Clone +
                      'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                               k: &a_, v: &b_,
                               acc1: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>,
                               acc2: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>)
     ->
         Rc<(Option<Rc<Map::MapTreeNode_2<a_, b_>>>,
             Option<Rc<Map::MapTreeNode_2<a_, b_>>>)> {
        if f(k, v) {
            Rc::from((Map::add(k, v, acc1), acc2.clone()))
        } else { Rc::from((acc1.clone(), Map::add(k, v, acc2))) }
    }
    pub fn partitionAux<K: PartialOrd + Clone + 'static, V: Clone +
                        'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                                 m: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                 acc_0: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                 acc_1: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     ->
         Rc<(Option<Rc<Map::MapTreeNode_2<K, V>>>,
             Option<Rc<Map::MapTreeNode_2<K, V>>>)> {
        {
            let acc:
                    Rc<(Option<Rc<Map::MapTreeNode_2<K, V>>>,
                        Option<Rc<Map::MapTreeNode_2<K, V>>>)> =
                Rc::from((acc_0.clone(), acc_1.clone()));
            match m {
                Some(m_0_0) =>
                {
                    let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                    if t.Height == 1i32 {
                        Map::partition1(f, &t.Key, &t.Value, &acc.0.clone(),
                                        &acc.1.clone())
                    } else {
                        {
                            let acc_2:
                                    Rc<(Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                        Option<Rc<Map::MapTreeNode_2<K,
                                                                     V>>>)> =
                                Map::partitionAux(f, &t.Right, &acc.0.clone(),
                                                  &acc.1.clone());
                            let acc_3:
                                    Rc<(Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                        Option<Rc<Map::MapTreeNode_2<K,
                                                                     V>>>)> =
                                Map::partition1(f, &t.Key, &t.Value,
                                                &acc_2.0.clone(),
                                                &acc_2.1.clone());
                            Map::partitionAux(f, &t.Left, &acc_3.0.clone(),
                                              &acc_3.1.clone())
                        }.clone()
                    }
                }.clone(),
                _ => acc.clone(),
            }
        }.clone()
    }
    pub fn partition<a_: PartialOrd + Clone + 'static, b_: Clone +
                     'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                              m: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>)
     ->
         Rc<(Option<Rc<Map::MapTreeNode_2<a_, b_>>>,
             Option<Rc<Map::MapTreeNode_2<a_, b_>>>)> {
        Map::partitionAux(f, m, &Map::empty(), &Map::empty())
    }
    pub fn filter1<a_: PartialOrd + Clone + 'static, b_: Clone +
                   'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                            k: &a_, v: &b_,
                            acc: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>)
     -> Option<Rc<Map::MapTreeNode_2<a_, b_>>> {
        if f(k, v) { Map::add(k, v, acc) } else { acc.clone() }
    }
    pub fn filterAux<K: PartialOrd + Clone + 'static, V: Clone +
                     'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                              m: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                              acc: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map::filter1(f, &t.Key, &t.Value, acc)
                } else {
                    Map::filterAux(f, &t.Right,
                                   &Map::filter1(f, &t.Key, &t.Value,
                                                 &Map::filterAux(f, &t.Left,
                                                                 acc)))
                }
            }.clone(),
            _ => acc.clone(),
        }
    }
    pub fn filter<a_: PartialOrd + Clone + 'static, b_: Clone +
                  'static>(f: &Rc<impl Fn(&a_, &b_) -> (bool) + 'static>,
                           m: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>)
     -> Option<Rc<Map::MapTreeNode_2<a_, b_>>> {
        Map::filterAux(f, m, &Map::empty())
    }
    pub fn spliceOutSuccessor<K: Clone + 'static, V: Clone +
                              'static>(m:
                                           &Option<Rc<Map::MapTreeNode_2<K,
                                                                         V>>>)
     -> Rc<(K, V, Option<Rc<Map::MapTreeNode_2<K, V>>>)> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Rc::from((t.Key.clone(), t.Value.clone(), Map::empty()))
                } else {
                    if t.Left.is_none() {
                        Rc::from((t.Key.clone(), t.Value.clone(),
                                  t.Right.clone()))
                    } else {
                        {
                            let patternInput:
                                    Rc<(K, V,
                                        Option<Rc<Map::MapTreeNode_2<K,
                                                                     V>>>)> =
                                Map::spliceOutSuccessor(&t.Left);
                            Rc::from((patternInput.0.clone(),
                                      patternInput.1.clone(),
                                      Map::mk(&patternInput.2.clone(), &t.Key,
                                              &t.Value, &t.Right)))
                        }.clone()
                    }
                }
            }.clone(),
            _ =>
            panic!("{}",
                   Native::string(&"internal error: Map.spliceOutSuccessor")),
        }
    }
    pub fn remove<K: PartialOrd + Clone + 'static, V: Clone +
                  'static>(k: &K, m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    if c == 0i32 { Map::empty() } else { m.clone() }
                } else {
                    if c < 0i32 {
                        Map::rebalance(&Map::remove(k, &t.Left), &t.Key,
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
                                                Rc<(K, V,
                                                    Option<Rc<Map::MapTreeNode_2<K,
                                                                                 V>>>)> =
                                            Map::spliceOutSuccessor(&t.Right);
                                        Map::mk(&t.Left,
                                                &patternInput.0.clone(),
                                                &patternInput.1.clone(),
                                                &patternInput.2.clone())
                                    }.clone()
                                }
                            }
                        } else {
                            Map::rebalance(&t.Left, &t.Key, &t.Value,
                                           &Map::remove(k, &t.Right))
                        }
                    }
                }
            }.clone(),
            _ => Map::empty(),
        }
    }
    pub fn change<K: PartialOrd + Clone + 'static, V: Clone +
                  'static>(k: &K,
                           u:
                               &Rc<impl Fn(&Option<V>) -> (Option<V>) +
                                   'static>,
                           m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, V>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    {
                        let c: i32 = Util::compare(k, &t.Key);
                        if c < 0i32 {
                            {
                                let matchValue_1: Option<V> = u(&None::<V>);
                                match &matchValue_1 {
                                    Some(matchValue_1_0_0) =>
                                    Map::mkMapTreeNode(k, matchValue_1_0_0,
                                                       &Map::empty(), m,
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
                                        Map::mkMapTreeLeaf(k,
                                                           matchValue_2_0_0),
                                        _ => Map::empty(),
                                    }
                                }.clone()
                            } else {
                                {
                                    let matchValue_3: Option<V> =
                                        u(&None::<V>);
                                    match &matchValue_3 {
                                        Some(matchValue_3_0_0) =>
                                        Map::mkMapTreeNode(k,
                                                           matchValue_3_0_0,
                                                           m, &Map::empty(),
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
                            Map::rebalance(&Map::change(k, u, &t.Left),
                                           &t.Key, &t.Value, &t.Right)
                        } else {
                            if c_1 == 0i32 {
                                {
                                    let matchValue_4: Option<V> =
                                        u(&Some(t.Value.clone()));
                                    match &matchValue_4 {
                                        Some(matchValue_4_0_0) =>
                                        Map::mkMapTreeNode(k,
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
                                                            Rc<(K, V,
                                                                Option<Rc<Map::MapTreeNode_2<K,
                                                                                             V>>>)> =
                                                        Map::spliceOutSuccessor(&t.Right);
                                                    Map::mk(&t.Left,
                                                            &patternInput.0.clone(),
                                                            &patternInput.1.clone(),
                                                            &patternInput.2.clone())
                                                }.clone()
                                            }
                                        },
                                    }
                                }.clone()
                            } else {
                                Map::rebalance(&t.Left, &t.Key, &t.Value,
                                               &Map::change(k, u, &t.Right))
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
                    Map::mkMapTreeLeaf(k, matchValue_0_0),
                    _ => m.clone(),
                }
            }.clone(),
        }
    }
    pub fn containsKey<K: PartialOrd + Clone + 'static, V: Clone +
                       'static>(k: &K,
                                m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                let c: i32 = Util::compare(k, &t.Key);
                if t.Height == 1i32 {
                    c == 0i32
                } else {
                    if c < 0i32 {
                        Map::containsKey(k, &t.Left)
                    } else {
                        if c == 0i32 {
                            true
                        } else { Map::containsKey(k, &t.Right) }
                    }
                }
            }
            _ => false,
        }
    }
    pub fn iterate<K: Clone + 'static, V: Clone +
                   'static>(f: &Rc<impl Fn(&K, &V) + 'static>,
                            m: &Option<Rc<Map::MapTreeNode_2<K, V>>>) {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    Map::iterate(f, &t.Left);
                    f(&t.Key, &t.Value);
                    Map::iterate(f, &t.Right)
                }
            }
            _ => (),
        };
    }
    pub fn tryPick<K: Clone + 'static, V: Clone + 'static, a_: Clone +
                   'static>(f: &Rc<impl Fn(&K, &V) -> (Option<a_>) + 'static>,
                            m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<a_> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    {
                        let matchValue: Option<a_> = Map::tryPick(f, &t.Left);
                        match &matchValue {
                            None =>
                            {
                                let matchValue_1: Option<a_> =
                                    f(&t.Key, &t.Value);
                                match &matchValue_1 {
                                    None => Map::tryPick(f, &t.Right),
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
    pub fn exists<K: Clone + 'static, V: Clone +
                  'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                           m: &Option<Rc<Map::MapTreeNode_2<K, V>>>) -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    if if Map::exists(f, &t.Left) {
                           true
                       } else { f(&t.Key, &t.Value) } {
                        true
                    } else { Map::exists(f, &t.Right) }
                }
            }
            _ => false,
        }
    }
    pub fn forall<K: Clone + 'static, V: Clone +
                  'static>(f: &Rc<impl Fn(&K, &V) -> (bool) + 'static>,
                           m: &Option<Rc<Map::MapTreeNode_2<K, V>>>) -> bool {
        match m {
            Some(m_0_0) => {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value)
                } else {
                    if if Map::forall(f, &t.Left) {
                           f(&t.Key, &t.Value)
                       } else { false } {
                        Map::forall(f, &t.Right)
                    } else { false }
                }
            }
            _ => true,
        }
    }
    pub fn map<V: Clone + 'static, R: Clone + 'static, K: Clone +
               'static>(f: &Rc<impl Fn(&V) -> (R) + 'static>,
                        m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, R>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map::mkMapTreeLeaf(&t.Key, &f(&t.Value))
                } else {
                    {
                        let l2: Option<Rc<Map::MapTreeNode_2<K, R>>> =
                            Map::map(f, &t.Left);
                        Map::mkMapTreeNode(&t.Key, &f(&t.Value), &l2,
                                           &Map::map(f, &t.Right), &t.Height)
                    }.clone()
                }
            }.clone(),
            _ => Map::empty(),
        }
    }
    pub fn mapi<K: Clone + 'static, V: Clone + 'static, R: Clone +
                'static>(f: &Rc<impl Fn(&K, &V) -> (R) + 'static>,
                         m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> Option<Rc<Map::MapTreeNode_2<K, R>>> {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    Map::mkMapTreeLeaf(&t.Key, &f(&t.Key, &t.Value))
                } else {
                    {
                        let l2: Option<Rc<Map::MapTreeNode_2<K, R>>> =
                            Map::mapi(f, &t.Left);
                        Map::mkMapTreeNode(&t.Key, &f(&t.Key, &t.Value), &l2,
                                           &Map::mapi(f, &t.Right), &t.Height)
                    }.clone()
                }
            }.clone(),
            _ => Map::empty(),
        }
    }
    pub fn foldBack<K: Clone + 'static, V: Clone + 'static, a_: Clone +
                    'static>(f: &Rc<impl Fn(&K, &V, &a_) -> (a_) + 'static>,
                             m: &Option<Rc<Map::MapTreeNode_2<K, V>>>, x: &a_)
     -> a_ {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(&t.Key, &t.Value, x)
                } else {
                    Map::foldBack(f, &t.Left,
                                  &f(&t.Key, &t.Value,
                                     &Map::foldBack(f, &t.Right, x)))
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn fold<a_: Clone + 'static, K: Clone + 'static, V: Clone +
                'static>(f: &Rc<impl Fn(&a_, &K, &V) -> (a_) + 'static>,
                         x: &a_, m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> a_ {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                if t.Height == 1i32 {
                    f(x, &t.Key, &t.Value)
                } else {
                    Map::fold(f,
                              &f(&Map::fold(f, x, &t.Left), &t.Key, &t.Value),
                              &t.Right)
                }
            }.clone(),
            _ => x.clone(),
        }
    }
    pub fn foldFromTo<K: PartialOrd + Clone + 'static, V: Clone + 'static,
                      a_: Clone +
                      'static>(lo: &K, hi: &K,
                               f: &Rc<impl Fn(&K, &V, &a_) -> (a_) + 'static>,
                               m: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                               x: &a_) -> a_ {
        match m {
            Some(m_0_0) =>
            {
                let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
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
                                Map::foldFromTo(lo, hi, f, &t.Left, x)
                            } else { x.clone() };
                        let x_3: a_ =
                            if if cLoKey_1 <= 0i32 {
                                   cKeyHi_1 <= 0i32
                               } else { false } {
                                f(&t.Key, &t.Value, &x_2)
                            } else { x_2.clone() };
                        if cKeyHi_1 < 0i32 {
                            Map::foldFromTo(lo, hi, f, &t.Right, &x_3)
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
                                m: &Option<Rc<Map::MapTreeNode_2<K, V>>>,
                                x: &a_) -> a_ {
        if Util::compare(lo, hi) == 1i32 {
            x.clone()
        } else { Map::foldFromTo(lo, hi, f, m, x) }
    }
    pub fn toArray<a_: Clone + 'static, b_: Clone +
                   'static>(m: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>)
     -> Rc<MutCell<Vec<Rc<(a_, b_)>>>> {
        {
            let res: Rc<MutCell<Vec<Rc<(a_, b_)>>>> =
                Native::arrayWithCapacity::<Rc<(a_, b_)>>(&Map::count(m));
            Map::iterate(&Rc::from({
                                       let res = res.clone();
                                       move |k: &a_, v: &b_|
                                           res.get_mut().push(Rc::from((k.clone(),
                                                                        v.clone())))
                                   }), m);
            res.clone()
        }.clone()
    }
    pub fn toList<K: Clone + 'static, V: Clone +
                  'static>(m: &Option<Rc<Map::MapTreeNode_2<K, V>>>)
     -> List_1<Rc<(K, V)>> {
        Map::foldBack(&Rc::from(move |k: &K, v: &V, acc: &List_1<Rc<(K, V)>>|
                                    List::cons(&Rc::from((k.clone(),
                                                          v.clone())), acc)),
                      m, &List::empty::<Rc<(K, V)>>())
    }
    pub fn ofArray<a_: PartialOrd + Clone + 'static, b_: Clone +
                   'static>(xs: &Rc<MutCell<Vec<Rc<(a_, b_)>>>>)
     -> Option<Rc<Map::MapTreeNode_2<a_, b_>>> {
        Array::fold(&Rc::from(move
                                  |acc:
                                       &Option<Rc<Map::MapTreeNode_2<a_,
                                                                     b_>>>,
                                   tupledArg: &Rc<(a_, b_)>|
                                  Map::add(&tupledArg.0.clone(),
                                           &tupledArg.1.clone(), acc)),
                    &Map::empty(), xs)
    }
    pub fn ofList<a_: PartialOrd + Clone + 'static, b_: Clone +
                  'static>(xs: &List_1<Rc<(a_, b_)>>)
     -> Option<Rc<Map::MapTreeNode_2<a_, b_>>> {
        List::fold(&Rc::from(move
                                 |acc:
                                      &Option<Rc<Map::MapTreeNode_2<a_, b_>>>,
                                  tupledArg: &Rc<(a_, b_)>|
                                 Map::add(&tupledArg.0.clone(),
                                          &tupledArg.1.clone(), acc)),
                   &Map::empty(), xs)
    }
    pub fn ofSeq<a_: PartialOrd + Clone + 'static, b_: Clone +
                 'static>(xs:
                              &Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<(a_,
                                                                         b_)>>>)
     -> Option<Rc<Map::MapTreeNode_2<a_, b_>>> {
        Seq::fold(&Rc::from(move
                                |acc: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>,
                                 tupledArg: &Rc<(a_, b_)>|
                                Map::add(&tupledArg.0.clone(),
                                         &tupledArg.1.clone(), acc)),
                  &Map::empty(), xs)
    }
    #[derive(Clone, Debug)]
    pub struct MapIterator_2<K: PartialOrd + Clone + 'static, V: Clone +
                             'static> {
        pub stack: MutCell<List_1<Option<Rc<Map::MapTreeNode_2<K, V>>>>>,
        pub started: MutCell<bool>,
    }
    pub fn collapseLHS<K: Clone + 'static, V: Clone +
                       'static>(stack:
                                    &List_1<Option<Rc<Map::MapTreeNode_2<K,
                                                                         V>>>>)
     -> List_1<Option<Rc<Map::MapTreeNode_2<K, V>>>> {
        if !List::isEmpty(stack) {
            let rest: List_1<Option<Rc<Map::MapTreeNode_2<K, V>>>> =
                List::tail(stack).clone();
            let m: Option<Rc<Map::MapTreeNode_2<K, V>>> =
                List::head(stack).clone();
            match &m {
                Some(m_0_0) =>
                {
                    let t: Rc<Map::MapTreeNode_2<K, V>> = m_0_0.clone();
                    if t.Height == 1i32 {
                        stack.clone()
                    } else {
                        Map::collapseLHS(&List::cons(&t.Left,
                                                     &List::cons(&Map::mkMapTreeLeaf(&t.Key,
                                                                                     &t.Value),
                                                                 &List::cons(&t.Right,
                                                                             &rest))))
                    }
                }.clone(),
                _ => Map::collapseLHS(&rest),
            }
        } else { List::empty::<Option<Rc<Map::MapTreeNode_2<K, V>>>>() }
    }
    pub fn mkIterator<a_: PartialOrd + Clone + 'static, b_: Clone +
                      'static>(m: &Option<Rc<Map::MapTreeNode_2<a_, b_>>>)
     -> Rc<Map::MapIterator_2<a_, b_>> {
        Rc::from(Map::MapIterator_2::<a_,
                                      b_>{stack:
                                              MutCell::from(Map::collapseLHS(&List::singleton(m))),
                                          started: MutCell::from(false),})
    }
    pub fn notStarted<a_: Clone + 'static>() -> a_ {
        panic!("{}", Map::SR::enumerationNotStarted())
    }
    pub fn alreadyFinished<a_: Clone + 'static>() -> a_ {
        panic!("{}", Map::SR::enumerationAlreadyFinished())
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
