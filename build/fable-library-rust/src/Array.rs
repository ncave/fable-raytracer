#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_3bd9ae6a::*;
use crate::import_f222008f::*;
use std::rc::Rc;
pub mod Array {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn indexOutOfBounds() -> Rc<str> {
            Native::string(&"The index was outside the range of elements in the array.")
        }
        pub fn inputArrayWasEmpty() -> Rc<str> {
            Native::string(&"The input array was empty")
        }
        pub fn inputArrayWasTooLong() -> Rc<str> {
            Native::string(&"The input array was too long")
        }
        pub fn inputArrayWasTooShort() -> Rc<str> {
            Native::string(&"The input array has not enough elements")
        }
        pub fn inputMustBeNonNegative() -> Rc<str> {
            Native::string(&"The input must be non-negative")
        }
        pub fn inputMustBePositive() -> Rc<str> {
            Native::string(&"The input must be positive")
        }
        pub fn keyNotFoundAlt() -> Rc<str> {
            Native::string(&"An index satisfying the predicate was not found in the collection.")
        }
        pub fn differentLengths() -> Rc<str> {
            Native::string(&"Arrays had different lengths")
        }
        pub fn invalidPermutation() -> Rc<str> {
            Native::string(&"Not a valid permutation")
        }
    }
    pub fn indexNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", Array::SR::keyNotFoundAlt())
    }
    pub fn differentLengths<a_: Clone + 'static>() -> a_ {
        panic!("{}", Array::SR::differentLengths())
    }
    pub fn empty<T: Clone + 'static>() -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&0i32, &Native::defaultOf::<T>())
    }
    pub fn create<T: Clone + 'static>(count: &i32, value: &T)
     -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&count, &value)
    }
    pub fn zeroCreate<T: Clone + 'static>(count: &i32)
     -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&count, &Native::defaultOf::<T>())
    }
    pub fn singleton<T: Clone + 'static>(value: &T) -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&1i32, &value)
    }
    pub fn isEmpty<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> bool {
        source.clone().is_empty()
    }
    pub fn length<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> i32 {
        source.clone().len() as i32
    }
    pub fn item<T: Clone + 'static>(index: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> T {
        source[index.clone()].clone()
    }
    pub fn get_<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>, index: &i32)
     -> T {
        source[index.clone()].clone()
    }
    pub fn set_<T: Clone +
                'static>(source: &Rc<MutCell<Vec<T>>>, index: &i32,
                         value: &T) {
        source.get_mut()[index.clone() as usize] = value.clone();
    }
    pub fn copy<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Native::arrayCopy(source)
    }
    pub fn tryItem<T: Clone +
                   'static>(index: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if if index.clone() < 0i32 {
               true
           } else { index.clone() >= source.clone().len() as i32 } {
            None::<T>
        } else { Some(source[index.clone()].clone()) }
    }
    pub fn reverse<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
            res.get_mut().reverse();
            res.clone()
        }.clone()
    }
    pub fn fill<T: Clone +
                'static>(target: &Rc<MutCell<Vec<T>>>, targetIndex: &i32,
                         count: &i32, value: &T) {
        if if targetIndex.clone() < 0i32 {
               true
           } else {
               targetIndex.clone() + count.clone() >
                   target.clone().len() as i32
           } {
            panic!("{}",
                   Rc::from((Rc::from(Array::SR::indexOutOfBounds().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>);
        }
        {
            let len: i32 = target.clone().len() as i32;
            for i in
                targetIndex.clone()..=targetIndex.clone() + count.clone() -
                                          1i32 {
                target.get_mut()[i as usize] = value.clone();
            }
        }
    }
    pub fn getSubArray<T: Clone +
                       'static>(source: &Rc<MutCell<Vec<T>>>,
                                startIndex: &i32, count: &i32)
     -> Rc<MutCell<Vec<T>>> {
        {
            if if startIndex.clone() < 0i32 {
                   true
               } else {
                   startIndex.clone() + count.clone() >
                       source.clone().len() as i32
               } {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::indexOutOfBounds().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>);
            }
            {
                let res: Rc<MutCell<Vec<T>>> =
                    Native::arrayWithCapacity::<T>(count);
                for i in 0i32..=count.clone() - 1i32 {
                    res.get_mut().push(source[startIndex.clone() +
                                                  i].clone());
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn exactlyOne<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.clone().len() as i32 == 1i32 {
            source[0i32].clone()
        } else {
            if source.clone().is_empty() {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>)
            } else {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasTooLong().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>)
            }
        }
    }
    pub fn tryExactlyOne<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if source.clone().len() as i32 == 1i32 {
            Some(source[0i32].clone())
        } else { None::<T> }
    }
    pub fn head<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.clone().is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array::SR::inputArrayWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>)
        } else { source[0i32].clone() }
    }
    pub fn tryHead<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if source.clone().is_empty() {
            None::<T>
        } else { Some(source[0i32].clone()) }
    }
    pub fn last<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.clone().is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array::SR::inputArrayWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>)
        } else { source[source.clone().len() as i32 - 1i32].clone() }
    }
    pub fn tryLast<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if source.clone().is_empty() {
            None::<T>
        } else { Some(source[source.clone().len() as i32 - 1i32].clone()) }
    }
    pub fn tail<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            if source.clone().is_empty() {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasTooShort().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
            }
            Array::getSubArray(source, &1i32,
                               &(source.clone().len() as i32 - 1i32))
        }.clone()
    }
    pub fn append<T: Clone +
                  'static>(source1: &Rc<MutCell<Vec<T>>>,
                           source2: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let len1: i32 = source1.clone().len() as i32;
            let len2: i32 = source2.clone().len() as i32;
            let res: Rc<MutCell<Vec<T>>> =
                Native::arrayWithCapacity::<T>(&(len1 + len2));
            for i in 0i32..=len1 - 1i32 {
                res.get_mut().push(source1[i].clone());
            }
            for i_1 in 0i32..=len2 - 1i32 {
                res.get_mut().push(source2[i_1].clone());
            }
            res.clone()
        }.clone()
    }
    pub fn choose<T: Clone + 'static, U: Clone +
                  'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<U>>> {
        {
            let res: Rc<MutCell<Vec<U>>> = Native::arrayEmpty::<U>();
            for i in 0i32..=source.clone().len() as i32 - 1i32 {
                let matchValue: Option<U> = chooser(&source[i].clone());
                match &matchValue {
                    None => (),
                    Some(matchValue_0_0) => {
                        let x: U = matchValue_0_0.clone();
                        res.get_mut().push(x)
                    }
                }
            }
            res.clone()
        }.clone()
    }
    pub fn compareWith<T: Clone +
                       'static>(comparer:
                                    &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                                source1: &Rc<MutCell<Vec<T>>>,
                                source2: &Rc<MutCell<Vec<T>>>) -> i32 {
        let length1: i32 = source1.clone().len() as i32;
        let length2: i32 = source2.clone().len() as i32;
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let result: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        if length1 < length2 {
            while if i.get() < source1.clone().len() as i32 {
                      result.get() == 0i32
                  } else { false } {
                result.set(comparer(&source1[i.get()].clone(),
                                    &source2[i.get()].clone()));
                i.set(i.get() + 1i32)
            }
        } else {
            while if i.get() < source2.clone().len() as i32 {
                      result.get() == 0i32
                  } else { false } {
                result.set(comparer(&source1[i.get()].clone(),
                                    &source2[i.get()].clone()));
                i.set(i.get() + 1i32)
            }
        }
        if result.get() != 0i32 {
            result.get()
        } else {
            if length1 == length2 {
                0i32
            } else { if length1 < length2 { -1i32 } else { 1i32 } }
        }
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone +
                      'static>(mapping:
                                   &Rc<impl Fn(&i32, &T) -> (U) + 'static>,
                               source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<U>>> {
        {
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push(mapping(&i, &source[i].clone()));
            }
            res.clone()
        }.clone()
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        source: &Rc<MutCell<Vec<T>>>) -> Rc<MutCell<Vec<U>>> {
        {
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push(mapping(&source[i].clone()));
            }
            res.clone()
        }.clone()
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&i32, &T1, &T2) -> (U) +
                                        'static>,
                                source1: &Rc<MutCell<Vec<T1>>>,
                                source2: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<U>>> {
        {
            if source1.clone().len() as i32 != source2.clone().len() as i32 {
                panic!("{}", Array::SR::differentLengths());
            }
            {
                let len: i32 = source1.clone().len() as i32;
                let res: Rc<MutCell<Vec<U>>> =
                    Native::arrayWithCapacity::<U>(&len);
                for i in 0i32..=len - 1i32 {
                    res.get_mut().push(mapping(&i, &source1[i].clone(),
                                               &source2[i].clone()));
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         source1: &Rc<MutCell<Vec<T1>>>,
                         source2: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<U>>> {
        {
            if source1.clone().len() as i32 != source2.clone().len() as i32 {
                panic!("{}", Array::SR::differentLengths());
            }
            {
                let len: i32 = source1.clone().len() as i32;
                let res: Rc<MutCell<Vec<U>>> =
                    Native::arrayWithCapacity::<U>(&len);
                for i in 0i32..=len - 1i32 {
                    res.get_mut().push(mapping(&source1[i].clone(),
                                               &source2[i].clone()));
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         source1: &Rc<MutCell<Vec<T1>>>,
                         source2: &Rc<MutCell<Vec<T2>>>,
                         source3: &Rc<MutCell<Vec<T3>>>)
     -> Rc<MutCell<Vec<U>>> {
        {
            if if source1.clone().len() as i32 != source2.clone().len() as i32
                  {
                   true
               } else {
                   source2.clone().len() as i32 !=
                       source3.clone().len() as i32
               } {
                panic!("{}", Array::SR::differentLengths());
            }
            {
                let len: i32 = source1.clone().len() as i32;
                let res: Rc<MutCell<Vec<U>>> =
                    Native::arrayWithCapacity::<U>(&len);
                for i in 0i32..=len - 1i32 {
                    res.get_mut().push(mapping(&source1[i].clone(),
                                               &source2[i].clone(),
                                               &source3[i].clone()));
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&State, &T) -> (Rc<(U, State)>) +
                                    'static>, state: &State,
                            source: &Rc<MutCell<Vec<T>>>)
     -> Rc<(Rc<MutCell<Vec<U>>>, State)> {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in 0i32..=len - 1i32 {
                let m: Rc<(U, State)> =
                    mapping(&acc.get(), &source[i].clone());
                res.get_mut().push(m.0.clone());
                acc.set(m.1.clone())
            }
            Rc::from((res.clone(), acc.get().clone()))
        }.clone()
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&T, &State)
                                        -> (Rc<(U, State)>) + 'static>,
                                source: &Rc<MutCell<Vec<T>>>, state: &State)
     -> Rc<(Rc<MutCell<Vec<U>>>, State)> {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in (0i32..=len - 1i32).rev() {
                let m: Rc<(U, State)> =
                    mapping(&source[i].clone(), &acc.get());
                res.get_mut().push(m.0.clone());
                acc.set(m.1.clone())
            }
            res.get_mut().reverse();
            Rc::from((res.clone(), acc.get().clone()))
        }.clone()
    }
    pub fn indexed<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<(i32, T)>>>> {
        {
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<Rc<(i32, T)>>>> =
                Native::arrayWithCapacity::<Rc<(i32, T)>>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push(Rc::from((i, source[i].clone())));
            }
            res.clone()
        }.clone()
    }
    pub fn concat<T: Clone +
                  'static>(sources: &Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let len: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            for idx in 0i32..=sources.clone().len() as i32 - 1i32 {
                let arr: Rc<MutCell<Vec<T>>> = sources[idx].clone();
                len.set(len.get() + arr.len() as i32)
            }
            {
                let res: Rc<MutCell<Vec<T>>> =
                    Native::arrayWithCapacity::<T>(&len.get());
                for idx_1 in 0i32..=sources.clone().len() as i32 - 1i32 {
                    let arr_1: Rc<MutCell<Vec<T>>> = sources[idx_1].clone();
                    for idx_2 in 0i32..=arr_1.len() as i32 - 1i32 {
                        let x: T = arr_1[idx_2].clone();
                        res.get_mut().push(x)
                    }
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn collect<T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&T) -> (Rc<MutCell<Vec<U>>>) +
                                    'static>, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<U>>> {
        Array::concat(&Native::arrayCopy(&Array::map(mapping, source)))
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>) -> bool {
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
        while if i.get() < source.clone().len() as i32 {
                  !res.get()
              } else { false } {
            res.set(predicate(&source[i.get()].clone()));
            i.set(i.get() + 1i32)
        }
        res.get()
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            source1: &Rc<MutCell<Vec<T1>>>,
                            source2: &Rc<MutCell<Vec<T2>>>) -> bool {
        if source1.clone().len() as i32 != source2.clone().len() as i32 {
            panic!("{}", Array::SR::differentLengths());
        }
        {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
            while if i.get() < source1.clone().len() as i32 {
                      !res.get()
                  } else { false } {
                res.set(predicate(&source1[i.get()].clone(),
                                  &source2[i.get()].clone()));
                i.set(i.get() + 1i32)
            }
            res.get()
        }
    }
    pub fn contains<T: PartialEq + Clone +
                    'static>(value: &T, source: &Rc<MutCell<Vec<T>>>)
     -> bool {
        Array::exists(&Rc::from({
                                    let value = value.clone();
                                    move |x: &T| x.clone() == value.clone()
                                }), source)
    }
    pub fn filter<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
            for i in 0i32..=source.clone().len() as i32 - 1i32 {
                if predicate(&source[i].clone()) {
                    res.get_mut().push(source[i].clone());
                };
            }
            res.clone()
        }.clone()
    }
    pub fn initialize<T: Clone +
                      'static>(count: &i32,
                               initializer:
                                   &Rc<impl Fn(&i32) -> (T) + 'static>)
     -> Rc<MutCell<Vec<T>>> {
        {
            if count.clone() < 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputMustBeNonNegative().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"count")) as Rc<str>);
            }
            {
                let res: Rc<MutCell<Vec<T>>> =
                    Native::arrayWithCapacity::<T>(count);
                for i in 0i32..=count.clone() - 1i32 {
                    res.get_mut().push(initializer(&i));
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn pairwise<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<(T, T)>>>> {
        if (source.clone().len() as i32) < 2i32 {
            Native::arrayEmpty::<Rc<(T, T)>>()
        } else {
            {
                let len: i32 = source.clone().len() as i32 - 1i32;
                let res: Rc<MutCell<Vec<Rc<(T, T)>>>> =
                    Native::arrayWithCapacity::<Rc<(T, T)>>(&len);
                for i in 0i32..=len - 1i32 {
                    res.get_mut().push(Rc::from((source[i].clone(),
                                                 source[i + 1i32].clone())));
                }
                res.clone()
            }.clone()
        }
    }
    pub fn partition<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>)
     -> Rc<(Rc<MutCell<Vec<T>>>, Rc<MutCell<Vec<T>>>)> {
        {
            let res1: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
            let res2: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
            for i in 0i32..=source.clone().len() as i32 - 1i32 {
                if predicate(&source[i].clone()) {
                    res1.get_mut().push(source[i].clone())
                } else { res2.get_mut().push(source[i].clone()) };
            }
            Rc::from((res1.clone(), res2.clone()))
        }.clone()
    }
    pub fn reduce<T: Clone +
                  'static>(reduction: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>) -> T {
        {
            if source.clone().is_empty() {
                panic!("{}", Array::SR::inputArrayWasEmpty());
            }
            {
                let acc_1: Rc<MutCell<T>> =
                    Rc::from(MutCell::from(source[0i32].clone()));
                for i_1 in 0i32..=source.clone().len() as i32 - 1i32 {
                    acc_1.set({
                                  let x: T = source[i_1].clone();
                                  if i_1 == 0i32 {
                                      x.clone()
                                  } else { reduction(&acc_1.get(), &x) }
                              }.clone());
                }
                acc_1.get().clone()
            }.clone()
        }.clone()
    }
    pub fn reduceBack<T: Clone +
                      'static>(reduction:
                                   &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                               source: &Rc<MutCell<Vec<T>>>) -> T {
        {
            if source.clone().is_empty() {
                panic!("{}", Array::SR::inputArrayWasEmpty());
            }
            {
                let len: i32 = source.clone().len() as i32;
                let acc_1: Rc<MutCell<T>> =
                    Rc::from(MutCell::from(source[len - 1i32].clone()));
                for i_1 in 1i32..=len {
                    acc_1.set({
                                  let acc: T = acc_1.get().clone();
                                  let x: T = source[len - i_1].clone();
                                  if i_1 - 1i32 == 0i32 {
                                      x.clone()
                                  } else { reduction(&acc, &x) }
                              }.clone());
                }
                acc_1.get().clone()
            }.clone()
        }.clone()
    }
    pub fn replicate<a_: Clone + 'static>(count: &i32, initial: &a_)
     -> Rc<MutCell<Vec<a_>>> {
        Array::initialize(count,
                          &Rc::from({
                                        let initial = initial.clone();
                                        move |_arg1: &i32| initial.clone()
                                    }))
    }
    pub fn scan<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<State>>> {
        {
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<State>>> =
                Native::arrayCreate(&(len + 1i32), &state);
            res.get_mut()[0i32 as usize] = state.clone();
            for i in 0i32..=len - 1i32 {
                res.get_mut()[(i + 1i32) as usize] =
                    folder(&res[i].clone(), &source[i].clone());
            }
            res.clone()
        }.clone()
    }
    pub fn scanBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>, source: &Rc<MutCell<Vec<T>>>,
                             state: &State) -> Rc<MutCell<Vec<State>>> {
        {
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<State>>> =
                Native::arrayCreate(&(len + 1i32), &state);
            res.get_mut()[len as usize] = state.clone();
            for i in (0i32..=len - 1i32).rev() {
                res.get_mut()[i as usize] =
                    folder(&source[i].clone(), &res[i + 1i32].clone());
            }
            res.clone()
        }.clone()
    }
    pub fn skip<T: Clone + 'static>(count: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            if count.clone() > source.clone().len() as i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasTooShort().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
            }
            {
                let count_1: i32 =
                    if count.clone() < 0i32 { 0i32 } else { count.clone() };
                Array::getSubArray(source, &count_1,
                                   &(source.clone().len() as i32 - count_1))
            }.clone()
        }.clone()
    }
    pub fn skipWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            while if count.get() < source.clone().len() as i32 {
                      predicate(&source[count.get()].clone())
                  } else { false } {
                count.set(count.get() + 1i32);
            }
            Array::getSubArray(source, &count.get(),
                               &(source.clone().len() as i32 - count.get()))
        }.clone()
    }
    pub fn take<T: Clone + 'static>(count: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            if count.clone() < 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputMustBeNonNegative().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"count")) as Rc<str>);
            }
            if count.clone() > source.clone().len() as i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasTooShort().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
            }
            Array::getSubArray(source, &0i32, count)
        }.clone()
    }
    pub fn takeWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            while if count.get() < source.clone().len() as i32 {
                      predicate(&source[count.get()].clone())
                  } else { false } {
                count.set(count.get() + 1i32);
            }
            Array::getSubArray(source, &0i32, &count.get())
        }.clone()
    }
    pub fn truncate<T: Clone +
                    'static>(count: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array::getSubArray(source, &0i32,
                           &if count.clone() < 0i32 {
                                0i32
                            } else {
                                if count.clone() > source.clone().len() as i32
                                   {
                                    source.clone().len() as i32
                                } else { count.clone() }
                            })
    }
    pub fn copyTo<T: Clone +
                  'static>(source: &Rc<MutCell<Vec<T>>>, sourceIndex: &i32,
                           target: &Rc<MutCell<Vec<T>>>, targetIndex: &i32,
                           count: &i32) {
        let diff: i32 = targetIndex.clone() - sourceIndex.clone();
        for i in
            sourceIndex.clone()..=sourceIndex.clone() + count.clone() - 1i32 {
            target.get_mut()[(i + diff) as usize] = source[i].clone();
        }
    }
    pub fn tryFind<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>) -> Option<T> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   predicate_1:
                                       &Rc<impl Fn(&T) -> (bool) + 'static>,
                                   source_1: &Rc<MutCell<Vec<T>>>)
             -> Option<T> {
                if i.clone() >= source_1.clone().len() as i32 {
                    None::<T>
                } else {
                    if predicate_1(&source_1[i.clone()].clone()) {
                        Some(source_1[i.clone()].clone())
                    } else {
                        inner_loop(&(i.clone() + 1i32), predicate_1, source_1)
                    }
                }
            }
            inner_loop(&0i32, predicate, source)
        }.clone()
    }
    pub fn find<T: Clone +
                'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                         source: &Rc<MutCell<Vec<T>>>) -> T {
        {
            let matchValue: Option<T> = Array::tryFind(predicate, source);
            match &matchValue {
                None => panic!("{}", Array::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFindIndex<T: Clone +
                        'static>(predicate:
                                     &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 source: &Rc<MutCell<Vec<T>>>)
     -> Option<i32> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   predicate_1:
                                       &Rc<impl Fn(&T) -> (bool) + 'static>,
                                   source_1: &Rc<MutCell<Vec<T>>>)
             -> Option<i32> {
                if i.clone() >= source_1.clone().len() as i32 {
                    None::<i32>
                } else {
                    if predicate_1(&source_1[i.clone()].clone()) {
                        Some(i.clone())
                    } else {
                        inner_loop(&(i.clone() + 1i32), predicate_1, source_1)
                    }
                }
            }
            inner_loop(&0i32, predicate, source)
        }.clone()
    }
    pub fn findIndex<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>) -> i32 {
        let matchValue: Option<i32> = Array::tryFindIndex(predicate, source);
        match &matchValue {
            None => panic!("{}", Array::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn indexOf<T: PartialEq + Clone +
                   'static>(source: &Rc<MutCell<Vec<T>>>, item: &T) -> i32 {
        let matchValue: Option<i32> =
            Array::tryFindIndex(&Rc::from({
                                              let item = item.clone();
                                              move |x: &T|
                                                  x.clone() == item.clone()
                                          }), source);
        match &matchValue {
            None => -1i32,
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindBack<T: Clone +
                       'static>(predicate:
                                    &Rc<impl Fn(&T) -> (bool) + 'static>,
                                source: &Rc<MutCell<Vec<T>>>) -> Option<T> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   predicate_1:
                                       &Rc<impl Fn(&T) -> (bool) + 'static>,
                                   source_1: &Rc<MutCell<Vec<T>>>)
             -> Option<T> {
                if i.clone() < 0i32 {
                    None::<T>
                } else {
                    if predicate_1(&source_1[i.clone()].clone()) {
                        Some(source_1[i.clone()].clone())
                    } else {
                        inner_loop(&(i.clone() - 1i32), predicate_1, source_1)
                    }
                }
            }
            inner_loop(&(source.clone().len() as i32 - 1i32), predicate,
                       source)
        }.clone()
    }
    pub fn findBack<T: Clone +
                    'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                             source: &Rc<MutCell<Vec<T>>>) -> T {
        {
            let matchValue: Option<T> = Array::tryFindBack(predicate, source);
            match &matchValue {
                None => panic!("{}", Array::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFindIndexBack<T: Clone +
                            'static>(predicate:
                                         &Rc<impl Fn(&T) -> (bool) + 'static>,
                                     source: &Rc<MutCell<Vec<T>>>)
     -> Option<i32> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   predicate_1:
                                       &Rc<impl Fn(&T) -> (bool) + 'static>,
                                   source_1: &Rc<MutCell<Vec<T>>>)
             -> Option<i32> {
                if i.clone() < 0i32 {
                    None::<i32>
                } else {
                    if predicate_1(&source_1[i.clone()].clone()) {
                        Some(i.clone())
                    } else {
                        inner_loop(&(i.clone() - 1i32), predicate_1, source_1)
                    }
                }
            }
            inner_loop(&(source.clone().len() as i32 - 1i32), predicate,
                       source)
        }.clone()
    }
    pub fn findIndexBack<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  source: &Rc<MutCell<Vec<T>>>) -> i32 {
        let matchValue: Option<i32> =
            Array::tryFindIndexBack(predicate, source);
        match &matchValue {
            None => panic!("{}", Array::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn findLastIndex<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  source: &Rc<MutCell<Vec<T>>>) -> i32 {
        let matchValue: Option<i32> =
            Array::tryFindIndexBack(predicate, source);
        match &matchValue {
            None => -1i32,
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryPick<T: Clone + 'static, U: Clone +
                   'static>(chooser:
                                &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>) -> Option<U> {
        {
            fn inner_loop<T: Clone + 'static, U: Clone +
                          'static>(i: &i32,
                                   chooser_1:
                                       &Rc<impl Fn(&T) -> (Option<U>) +
                                           'static>,
                                   source_1: &Rc<MutCell<Vec<T>>>)
             -> Option<U> {
                if i.clone() >= source_1.clone().len() as i32 {
                    None::<U>
                } else {
                    {
                        let matchValue: Option<U> =
                            chooser_1(&source_1[i.clone()].clone());
                        if matchValue.is_none() {
                            inner_loop(&(i.clone() + 1i32), chooser_1,
                                       source_1)
                        } else { matchValue.clone() }
                    }.clone()
                }
            }
            inner_loop(&0i32, chooser, source)
        }.clone()
    }
    pub fn pick<T: Clone + 'static, U: Clone +
                'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                         source: &Rc<MutCell<Vec<T>>>) -> U {
        {
            let matchValue: Option<U> = Array::tryPick(chooser, source);
            match &matchValue {
                None => panic!("{}", Array::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn fold<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, source: &Rc<MutCell<Vec<T>>>)
     -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            for i in 0i32..=source.clone().len() as i32 - 1i32 {
                acc.set(folder(&acc.get(), &source[i].clone()));
            }
            acc.get().clone()
        }.clone()
    }
    pub fn foldBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>, source: &Rc<MutCell<Vec<T>>>,
                             state: &State) -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            let len: i32 = source.clone().len() as i32;
            for i in 1i32..=len {
                acc.set(folder(&source[len - i].clone(), &acc.get()));
            }
            acc.get().clone()
        }.clone()
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone +
                 'static>(folder:
                              &Rc<impl Fn(&State, &T1, &T2) -> (State) +
                                  'static>, state: &State,
                          source1: &Rc<MutCell<Vec<T1>>>,
                          source2: &Rc<MutCell<Vec<T2>>>) -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            if source1.clone().len() as i32 != source2.clone().len() as i32 {
                panic!("{}", Array::SR::differentLengths());
            }
            for i in 0i32..=source1.clone().len() as i32 - 1i32 {
                acc.set(folder(&acc.get(), &source1[i].clone(),
                               &source2[i].clone()));
            }
            acc.get().clone()
        }.clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone +
                     'static>(folder:
                                  &Rc<impl Fn(&T1, &T2, &State) -> (State) +
                                      'static>,
                              source1: &Rc<MutCell<Vec<T1>>>,
                              source2: &Rc<MutCell<Vec<T2>>>, state: &State)
     -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            if source1.clone().len() as i32 != source2.clone().len() as i32 {
                panic!("{}", Array::SR::differentLengths());
            }
            {
                let len: i32 = source1.clone().len() as i32;
                for i in 1i32..=len {
                    acc.set(folder(&source1[len - i].clone(),
                                   &source2[len - i].clone(), &acc.get()));
                }
                acc.get().clone()
            }.clone()
        }.clone()
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>) -> bool {
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(true));
        while if i.get() < source.clone().len() as i32 {
                  res.get()
              } else { false } {
            res.set(predicate(&source[i.get()].clone()));
            i.set(i.get() + 1i32)
        }
        res.get()
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            source1: &Rc<MutCell<Vec<T1>>>,
                            source2: &Rc<MutCell<Vec<T2>>>) -> bool {
        if source1.clone().len() as i32 != source2.clone().len() as i32 {
            panic!("{}", Array::SR::differentLengths());
        }
        {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(true));
            while if i.get() < source1.clone().len() as i32 {
                      res.get()
                  } else { false } {
                res.set(predicate(&source1[i.get()].clone(),
                                  &source2[i.get()].clone()));
                i.set(i.get() + 1i32)
            }
            res.get()
        }
    }
    pub fn iterate<T: Clone +
                   'static>(action: &Rc<impl Fn(&T) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>) {
        for i in 0i32..=source.clone().len() as i32 - 1i32 {
            action(&source[i].clone());
        };
    }
    pub fn iterateIndexed<T: Clone +
                          'static>(action: &Rc<impl Fn(&i32, &T) + 'static>,
                                   source: &Rc<MutCell<Vec<T>>>) {
        for i in 0i32..=source.clone().len() as i32 - 1i32 {
            action(&i, &source[i].clone());
        };
    }
    pub fn iterate2<T: Clone +
                    'static>(action: &Rc<impl Fn(&T, &T) + 'static>,
                             source1: &Rc<MutCell<Vec<T>>>,
                             source2: &Rc<MutCell<Vec<T>>>) {
        if source1.clone().len() as i32 != source2.clone().len() as i32 {
            panic!("{}", Array::SR::differentLengths());
        }
        for i in 0i32..=source1.clone().len() as i32 - 1i32 {
            action(&source1[i].clone(), &source2[i].clone());
        }
    }
    pub fn iterateIndexed2<T: Clone +
                           'static>(action:
                                        &Rc<impl Fn(&i32, &T, &T) + 'static>,
                                    source1: &Rc<MutCell<Vec<T>>>,
                                    source2: &Rc<MutCell<Vec<T>>>) {
        if source1.clone().len() as i32 != source2.clone().len() as i32 {
            panic!("{}", Array::SR::differentLengths());
        }
        for i in 0i32..=source1.clone().len() as i32 - 1i32 {
            action(&i, &source1[i].clone(), &source2[i].clone());
        }
    }
    pub fn permute<T: Clone +
                   'static>(indexMap: &Rc<impl Fn(&i32) -> (i32) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let len: i32 = source.clone().len() as i32;
            let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
            let checkFlags: Rc<MutCell<Vec<i32>>> =
                Native::arrayCreate(&len, &0i32);
            Array::iterateIndexed(&Rc::from({
                                                let checkFlags =
                                                    checkFlags.clone();
                                                let indexMap =
                                                    indexMap.clone();
                                                let res = res.clone();
                                                move |i: &i32, x: &T|
                                                    {
                                                        let j: i32 =
                                                            indexMap(i);
                                                        if if j < 0i32 {
                                                               true
                                                           } else { j >= len }
                                                           {
                                                            panic!("{}",
                                                                   Array::SR::invalidPermutation());
                                                        }
                                                        res.get_mut()[j as
                                                                          usize]
                                                            = x.clone();
                                                        checkFlags.get_mut()[j
                                                                                 as
                                                                                 usize]
                                                            = 1i32
                                                    }
                                            }), source);
            if !Array::forAll(&Rc::from(move |y: &i32| 1i32 == y.clone()),
                              &checkFlags) {
                panic!("{}", Array::SR::invalidPermutation());
            }
            res.clone()
        }.clone()
    }
    pub fn sortInPlaceWith<T: Clone +
                           'static>(comparer:
                                        &Rc<impl Fn(&T, &T) -> (i32) +
                                            'static>,
                                    source: &Rc<MutCell<Vec<T>>>) {
        source.clone().get_mut().sort_by(&Native::compare(&Rc::from({
                                                                        let comparer
                                                                            =
                                                                            comparer.clone();
                                                                        move
                                                                            |delegateArg0:
                                                                                 &T,
                                                                             delegateArg1:
                                                                                 &T|
                                                                            comparer(delegateArg0,
                                                                                     delegateArg1)
                                                                    })));
    }
    pub fn sortInPlace<T: PartialOrd + Clone +
                       'static>(source: &Rc<MutCell<Vec<T>>>) {
        Array::sortInPlaceWith(&Rc::from(move |e1: &T, e2: &T|
                                             Util::compare(e1, e2)), source);
    }
    pub fn sortInPlaceBy<T: Clone + 'static, U: PartialOrd + Clone +
                         'static>(projection:
                                      &Rc<impl Fn(&T) -> (U) + 'static>,
                                  source: &Rc<MutCell<Vec<T>>>) {
        Array::sortInPlaceWith(&Rc::from({
                                             let projection =
                                                 projection.clone();
                                             move |x: &T, y: &T|
                                                 Util::compare(&projection(x),
                                                               &projection(y))
                                         }), source);
    }
    pub fn sort<T: PartialOrd + Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
            Array::sortInPlace(&res);
            res.clone()
        }.clone()
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone +
                  'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
            Array::sortInPlaceBy(projection, &res);
            res.clone()
        }.clone()
    }
    pub fn sortWith<T: Clone +
                    'static>(comparer:
                                 &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                             source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
            Array::sortInPlaceWith(comparer, &res);
            res.clone()
        }.clone()
    }
    pub fn sortDescending<T: PartialOrd + Clone +
                          'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array::sortWith(&Rc::from(move |x: &T, y: &T|
                                      Util::compare(x, y) * -1i32), source)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone +
                            'static>(projection:
                                         &Rc<impl Fn(&T) -> (U) + 'static>,
                                     source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array::sortWith(&Rc::from({
                                      let projection = projection.clone();
                                      move |x: &T, y: &T|
                                          Util::compare(&projection(x),
                                                        &projection(y)) *
                                              -1i32
                                  }), source)
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone +
                    'static>(xs: &Rc<MutCell<Vec<T1>>>,
                             ys: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<Rc<(T1, T2)>>>> {
        {
            let len1: i32 = xs.clone().len() as i32;
            let len2: i32 = ys.clone().len() as i32;
            let res: Rc<MutCell<Vec<Rc<(T1, T2)>>>> =
                Native::arrayWithCapacity::<Rc<(T1, T2)>>(&(len1 * len2));
            for i in 0i32..=len1 - 1i32 {
                for j in 0i32..=len2 - 1i32 {
                    res.get_mut().push(Rc::from((xs[i].clone(),
                                                 ys[j].clone())));
                };
            }
            res.clone()
        }.clone()
    }
    pub fn unfold<State: Clone + 'static, T: Clone +
                  'static>(generator:
                               &Rc<impl Fn(&State)
                                   -> (Option<Rc<(T, State)>>) + 'static>,
                           state: &State) -> Rc<MutCell<Vec<T>>> {
        {
            fn inner_loop<a_: Clone + 'static, T: Clone +
                          'static>(generator_1:
                                       &Rc<impl Fn(&a_)
                                           -> (Option<Rc<(T, a_)>>) +
                                           'static>, state_1: &a_,
                                   res: &Rc<MutCell<Vec<T>>>) {
                let matchValue: Option<Rc<(T, a_)>> = generator_1(state_1);
                match &matchValue {
                    Some(matchValue_0_0) => {
                        let x: T = matchValue_0_0.0.clone();
                        let s: a_ = matchValue_0_0.1.clone();
                        res.clone().get_mut().push(x);
                        inner_loop(generator_1, &s, res)
                    }
                    _ => (),
                }
            }
            let res_1: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
            inner_loop(generator, state, &res_1);
            res_1.clone()
        }.clone()
    }
    pub fn unzip<T1: Clone + 'static, T2: Clone +
                 'static>(source: &Rc<MutCell<Vec<Rc<(T1, T2)>>>>)
     -> Rc<(Rc<MutCell<Vec<T1>>>, Rc<MutCell<Vec<T2>>>)> {
        {
            let len: i32 = source.clone().len() as i32;
            let res1: Rc<MutCell<Vec<T1>>> =
                Native::arrayWithCapacity::<T1>(&len);
            let res2: Rc<MutCell<Vec<T2>>> =
                Native::arrayWithCapacity::<T2>(&len);
            Array::iterateIndexed(&Rc::from({
                                                let res1 = res1.clone();
                                                let res2 = res2.clone();
                                                move
                                                    |i: &i32,
                                                     tupledArg: &Rc<(T1, T2)>|
                                                    {
                                                        res1.get_mut().push(tupledArg.0.clone());
                                                        res2.get_mut().push(tupledArg.1.clone())
                                                    }
                                            }), source);
            Rc::from((res1.clone(), res2.clone()))
        }.clone()
    }
    pub fn unzip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                  'static>(source: &Rc<MutCell<Vec<Rc<(T1, T2, T3)>>>>)
     ->
         Rc<(Rc<MutCell<Vec<T1>>>, Rc<MutCell<Vec<T2>>>,
             Rc<MutCell<Vec<T3>>>)> {
        {
            let len: i32 = source.clone().len() as i32;
            let res1: Rc<MutCell<Vec<T1>>> =
                Native::arrayWithCapacity::<T1>(&len);
            let res2: Rc<MutCell<Vec<T2>>> =
                Native::arrayWithCapacity::<T2>(&len);
            let res3: Rc<MutCell<Vec<T3>>> =
                Native::arrayWithCapacity::<T3>(&len);
            Array::iterateIndexed(&Rc::from({
                                                let res1 = res1.clone();
                                                let res2 = res2.clone();
                                                let res3 = res3.clone();
                                                move
                                                    |i: &i32,
                                                     tupledArg:
                                                         &Rc<(T1, T2, T3)>|
                                                    {
                                                        res1.get_mut().push(tupledArg.0.clone());
                                                        res2.get_mut().push(tupledArg.1.clone());
                                                        res3.get_mut().push(tupledArg.2.clone())
                                                    }
                                            }), source);
            Rc::from((res1.clone(), res2.clone(), res3.clone()))
        }.clone()
    }
    pub fn zip<T1: Clone + 'static, T2: Clone +
               'static>(source1: &Rc<MutCell<Vec<T1>>>,
                        source2: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<Rc<(T1, T2)>>>> {
        Array::map2(&Rc::from(move |x: &T1, y: &T2|
                                  Rc::from((x.clone(), y.clone()))), source1,
                    source2)
    }
    pub fn zip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                'static>(source1: &Rc<MutCell<Vec<T1>>>,
                         source2: &Rc<MutCell<Vec<T2>>>,
                         source3: &Rc<MutCell<Vec<T3>>>)
     -> Rc<MutCell<Vec<Rc<(T1, T2, T3)>>>> {
        Array::map3(&Rc::from(move |x: &T1, y: &T2, z: &T3|
                                  Rc::from((x.clone(), y.clone(),
                                            z.clone()))), source1, source2,
                    source3)
    }
    pub fn chunkBySize<T: Clone +
                       'static>(chunkSize: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        {
            if chunkSize.clone() <= 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputMustBePositive().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"size")) as Rc<str>);
            }
            {
                let len: i32 = source.clone().len() as i32;
                let chunkCount: i32 = (len - 1i32) / chunkSize.clone() + 1i32;
                let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                    Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(&chunkCount);
                for i in 0i32..=chunkCount - 1i32 {
                    let start: i32 = i * chunkSize.clone();
                    let slice: Rc<MutCell<Vec<T>>> =
                        Array::getSubArray(source, &start,
                                           &chunkSize.clone().min(len -
                                                                      start));
                    res.get_mut().push(slice)
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn splitAt<T: Clone +
                   'static>(index: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<(Rc<MutCell<Vec<T>>>, Rc<MutCell<Vec<T>>>)> {
        {
            if if index.clone() < 0i32 {
                   true
               } else { index.clone() > source.clone().len() as i32 } {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::indexOutOfBounds().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>);
            }
            Rc::from((Array::getSubArray(source, &0i32, index),
                      Array::getSubArray(source, index,
                                         &(source.clone().len() as i32 -
                                               index.clone()))))
        }.clone()
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone +
               'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        {
            let acc: Rc<MutCell<T>> =
                Rc::from(MutCell::from(Native::getZero::<T>()));
            for i in 0i32..=source.clone().len() as i32 - 1i32 {
                acc.set(acc.get().clone() + source[i].clone());
            }
            acc.get().clone()
        }.clone()
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default +
                 Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          source: &Rc<MutCell<Vec<T>>>) -> U {
        {
            let acc: Rc<MutCell<U>> =
                Rc::from(MutCell::from(Native::getZero::<U>()));
            for i in 0i32..=source.clone().len() as i32 - 1i32 {
                acc.set(acc.get().clone() + projection(&source[i].clone()));
            }
            acc.get().clone()
        }.clone()
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<MutCell<Vec<T>>>) -> T {
        Array::reduce(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        if projection(x) > projection(y) {
                                            x.clone()
                                        } else { y.clone() }
                                }), xs)
    }
    pub fn max<T: PartialOrd + Clone + 'static>(xs: &Rc<MutCell<Vec<T>>>)
     -> T {
        Array::reduce(&Rc::from(move |x: &T, y: &T|
                                    if x.clone() > y.clone() {
                                        x.clone()
                                    } else { y.clone() }), xs)
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<MutCell<Vec<T>>>) -> T {
        Array::reduce(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        if projection(x) < projection(y) {
                                            x.clone()
                                        } else { y.clone() }
                                }), xs)
    }
    pub fn min<T: PartialOrd + Clone + 'static>(xs: &Rc<MutCell<Vec<T>>>)
     -> T {
        Array::reduce(&Rc::from(move |x: &T, y: &T|
                                    if x.clone() < y.clone() {
                                        x.clone()
                                    } else { y.clone() }), xs)
    }
    pub fn average<T: core::ops::Add<Output = T> +
                   core::ops::Div<Output = T> + From<i32> + Default + Clone +
                   'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        {
            if source.clone().is_empty() {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
            }
            {
                let total: Rc<MutCell<T>> =
                    Rc::from(MutCell::from(Native::getZero::<T>()));
                for i in 0i32..=source.clone().len() as i32 - 1i32 {
                    total.set(total.get().clone() + source[i].clone());
                }
                total.get().clone() /
                    T::from(source.clone().len() as i32).clone()
            }.clone()
        }.clone()
    }
    pub fn averageBy<T: Clone + 'static, U: core::ops::Add<Output = U> +
                     core::ops::Div<Output = U> + From<i32> + Default +
                     Clone +
                     'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>) -> U {
        {
            if source.clone().is_empty() {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputArrayWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
            }
            {
                let total: Rc<MutCell<U>> =
                    Rc::from(MutCell::from(Native::getZero::<U>()));
                for i in 0i32..=source.clone().len() as i32 - 1i32 {
                    total.set(total.get().clone() +
                                  projection(&source[i].clone()));
                }
                total.get().clone() /
                    U::from(source.clone().len() as i32).clone()
            }.clone()
        }.clone()
    }
    pub fn ofOption<T: Clone + 'static>(opt: &Option<T>)
     -> Rc<MutCell<Vec<T>>> {
        match opt {
            None => Native::arrayCreate(&0i32, &Native::defaultOf::<T>()),
            Some(opt_0_0) => Native::arrayCreate(&1i32, &opt_0_0),
        }
    }
    pub fn r#where<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array::filter(predicate, source)
    }
    pub fn windowed<T: Clone +
                    'static>(windowSize: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        {
            if windowSize.clone() <= 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputMustBePositive().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"size")) as Rc<str>);
            }
            {
                let len: i32 =
                    0i32.max(source.clone().len() as i32 - windowSize.clone()
                                 + 1i32);
                let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                    Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(&len);
                for i in 0i32..=len - 1i32 {
                    let slice: Rc<MutCell<Vec<T>>> =
                        Array::getSubArray(source, &i, windowSize);
                    res.get_mut().push(slice)
                }
                res.clone()
            }.clone()
        }.clone()
    }
    pub fn splitInto<T: Clone +
                     'static>(chunks: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        {
            if chunks.clone() <= 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(Array::SR::inputMustBePositive().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"chunks")) as Rc<str>);
            }
            if source.clone().is_empty() {
                Native::arrayEmpty::<Rc<MutCell<Vec<T>>>>()
            } else {
                {
                    let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                        Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(chunks);
                    let chunks_1: i32 =
                        chunks.clone().min(source.clone().len() as i32);
                    let minChunkSize: i32 =
                        source.clone().len() as i32 / chunks_1;
                    let chunksWithExtraItem: i32 =
                        source.clone().len() as i32 % chunks_1;
                    for i in 0i32..=chunks_1 - 1i32 {
                        let chunkSize: i32 =
                            if i < chunksWithExtraItem {
                                minChunkSize + 1i32
                            } else { minChunkSize };
                        let slice: Rc<MutCell<Vec<T>>> =
                            Array::getSubArray(source,
                                               &(i * minChunkSize +
                                                     chunksWithExtraItem.min(i)),
                                               &chunkSize);
                        res.get_mut().push(slice)
                    }
                    res.clone()
                }.clone()
            }
        }.clone()
    }
    pub fn transpose<T: Clone +
                     'static>(arrays: &Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        if arrays.clone().is_empty() {
            Native::arrayEmpty::<Rc<MutCell<Vec<T>>>>()
        } else {
            {
                let len: i32 = arrays.clone().len() as i32;
                let firstArray: Rc<MutCell<Vec<T>>> = arrays[0i32].clone();
                let innerLen: i32 = firstArray.len() as i32;
                if !Array::forAll(&Rc::from(move |a: &Rc<MutCell<Vec<T>>>|
                                                a.clone().len() as i32 ==
                                                    innerLen), arrays) {
                    panic!("{}", Array::SR::differentLengths());
                }
                {
                    let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                        Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(&innerLen);
                    for i in 0i32..=innerLen - 1i32 {
                        let res2: Rc<MutCell<Vec<T>>> =
                            Native::arrayWithCapacity::<T>(&len);
                        for j in 0i32..=len - 1i32 {
                            res2.get_mut().push(arrays[j].clone()[i].clone());
                        }
                        res.get_mut().push(res2)
                    }
                    res.clone()
                }.clone()
            }.clone()
        }
    }
}
