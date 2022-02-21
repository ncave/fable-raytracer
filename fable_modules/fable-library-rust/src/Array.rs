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
use crate::import_f222008f::*;
pub mod Array_ {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn indexOutOfBounds() -> Rc<str> {
            thread_local! {
                static indexOutOfBounds: Rc<str> =
    String_::string(&"The index was outside the range of elements in the array.");
            };
            indexOutOfBounds.with(|value| value.clone())
        }
        pub fn inputArrayWasEmpty() -> Rc<str> {
            thread_local! {
                static inputArrayWasEmpty: Rc<str> =
    String_::string(&"The input array was empty");
            };
            inputArrayWasEmpty.with(|value| value.clone())
        }
        pub fn inputArrayWasTooLong() -> Rc<str> {
            thread_local! {
                static inputArrayWasTooLong: Rc<str> =
    String_::string(&"The input array was too long");
            };
            inputArrayWasTooLong.with(|value| value.clone())
        }
        pub fn inputArrayWasTooShort() -> Rc<str> {
            thread_local! {
                static inputArrayWasTooShort: Rc<str> =
    String_::string(&"The input array has not enough elements");
            };
            inputArrayWasTooShort.with(|value| value.clone())
        }
        pub fn inputMustBeNonNegative() -> Rc<str> {
            thread_local! {
                static inputMustBeNonNegative: Rc<str> =
    String_::string(&"The input must be non-negative");
            };
            inputMustBeNonNegative.with(|value| value.clone())
        }
        pub fn inputMustBePositive() -> Rc<str> {
            thread_local! {
                static inputMustBePositive: Rc<str> =
    String_::string(&"The input must be positive");
            };
            inputMustBePositive.with(|value| value.clone())
        }
        pub fn keyNotFoundAlt() -> Rc<str> {
            thread_local! {
                static keyNotFoundAlt: Rc<str> =
    String_::string(&"An index satisfying the predicate was not found in the collection.");
            };
            keyNotFoundAlt.with(|value| value.clone())
        }
        pub fn differentLengths() -> Rc<str> {
            thread_local! {
                static differentLengths: Rc<str> =
    String_::string(&"Arrays had different lengths");
            };
            differentLengths.with(|value| value.clone())
        }
        pub fn invalidPermutation() -> Rc<str> {
            thread_local! {
                static invalidPermutation: Rc<str> =
    String_::string(&"Not a valid permutation");
            };
            invalidPermutation.with(|value| value.clone())
        }
    }
    pub fn indexNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", Array_::SR::keyNotFoundAlt())
    }
    pub fn differentLengths<a_: Clone + 'static>() -> a_ {
        panic!("{}", Array_::SR::differentLengths())
    }
    pub fn empty<T: Clone + 'static>() -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&0i32, &Native::defaultOf::<T>())
    }
    pub fn create<T: Clone + 'static>(count: &i32, value: &T)
     -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&count.clone(), &value.clone())
    }
    pub fn zeroCreate<T: Clone + 'static>(count: &i32)
     -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&count.clone(), &Native::defaultOf::<T>())
    }
    pub fn singleton<T: Clone + 'static>(value: &T) -> Rc<MutCell<Vec<T>>> {
        Native::arrayCreate(&1i32, &value.clone())
    }
    pub fn isEmpty<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> bool {
        source.is_empty()
    }
    pub fn length<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> i32 {
        source.len() as i32
    }
    pub fn item<T: Clone + 'static>(index: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> T {
        source[index.clone()].clone()
    }
    pub fn get<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>, index: &i32)
     -> T {
        source[index.clone()].clone()
    }
    pub fn set<T: Clone +
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
           } else { index.clone() >= source.len() as i32 } {
            None::<T>
        } else { Some(source[index.clone()].clone()) }
    }
    pub fn reverse<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
        res.get_mut().reverse();
        res.clone()
    }
    pub fn fill<T: Clone +
                'static>(target: &Rc<MutCell<Vec<T>>>, targetIndex: &i32,
                         count: &i32, value: &T) {
        if if targetIndex.clone() < 0i32 {
               true
           } else {
               targetIndex.clone() + count.clone() > target.len() as i32
           } {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::indexOutOfBounds().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>);
        }
        {
            let len: i32 = target.len() as i32;
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
        if if startIndex.clone() < 0i32 {
               true
           } else { startIndex.clone() + count.clone() > source.len() as i32 }
           {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::indexOutOfBounds().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>);
        }
        {
            let res: Rc<MutCell<Vec<T>>> =
                Native::arrayWithCapacity::<T>(count);
            for i in 0i32..=count.clone() - 1i32 {
                res.get_mut().push(source[startIndex.clone() + i].clone());
            }
            res
        }
    }
    pub fn exactlyOne<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.len() as i32 == 1i32 {
            source[0i32].clone()
        } else {
            if source.is_empty() {
                panic!("{}",
                       Rc::from((Rc::from(Array_::SR::inputArrayWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>)
            } else {
                panic!("{}",
                       Rc::from((Rc::from(Array_::SR::inputArrayWasTooLong().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>)
            }
        }
    }
    pub fn tryExactlyOne<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if source.len() as i32 == 1i32 {
            Some(source[0i32].clone())
        } else { None::<T> }
    }
    pub fn head<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>)
        } else { source[0i32].clone() }
    }
    pub fn tryHead<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if source.is_empty() { None::<T> } else { Some(source[0i32].clone()) }
    }
    pub fn last<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>)
        } else { source[source.len() as i32 - 1i32].clone() }
    }
    pub fn tryLast<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Option<T> {
        if source.is_empty() {
            None::<T>
        } else { Some(source[source.len() as i32 - 1i32].clone()) }
    }
    pub fn tail<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        if source.is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasTooShort().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
        }
        Array_::getSubArray(source, &1i32, &(source.len() as i32 - 1i32))
    }
    pub fn append<T: Clone +
                  'static>(source1: &Rc<MutCell<Vec<T>>>,
                           source2: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let len1: i32 = source1.len() as i32;
        let len2: i32 = source2.len() as i32;
        let res: Rc<MutCell<Vec<T>>> =
            Native::arrayWithCapacity::<T>(&(len1 + len2));
        for i in 0i32..=len1 - 1i32 {
            res.get_mut().push(source1[i].clone());
        }
        for i_1 in 0i32..=len2 - 1i32 {
            res.get_mut().push(source2[i_1].clone());
        }
        res
    }
    pub fn choose<T: Clone + 'static, U: Clone +
                  'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<U>>> {
        let res: Rc<MutCell<Vec<U>>> = Native::arrayEmpty::<U>();
        for i in 0i32..=source.len() as i32 - 1i32 {
            let matchValue: Option<U> = chooser(&source[i].clone());
            match &matchValue {
                None => (),
                Some(matchValue_0_0) => {
                    let x: U = matchValue_0_0.clone();
                    res.get_mut().push(x)
                }
            }
        }
        res
    }
    pub fn compareWith<T: Clone +
                       'static>(comparer:
                                    &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                                source1: &Rc<MutCell<Vec<T>>>,
                                source2: &Rc<MutCell<Vec<T>>>) -> i32 {
        let length1: i32 = source1.len() as i32;
        let length2: i32 = source2.len() as i32;
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let result: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        if length1 < length2 {
            while if i.get() < source1.len() as i32 {
                      result.get() == 0i32
                  } else { false } {
                result.set(comparer(&source1[i.get()].clone(),
                                    &source2[i.get()].clone()));
                i.set(i.get() + 1i32)
            }
        } else {
            while if i.get() < source2.len() as i32 {
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
    pub fn compareTo<T: PartialOrd + Clone +
                     'static>(source1: &Rc<MutCell<Vec<T>>>,
                              source2: &Rc<MutCell<Vec<T>>>) -> i32 {
        Util::compare(source1, source2)
    }
    pub fn equalsTo<T: Eq + core::hash::Hash + Clone +
                    'static>(source1: &Rc<MutCell<Vec<T>>>,
                             source2: &Rc<MutCell<Vec<T>>>) -> bool {
        source1.clone() == source2.clone()
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone +
                      'static>(mapping:
                                   &Rc<impl Fn(&i32, &T) -> (U) + 'static>,
                               source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<U>>> {
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<U>>> = Native::arrayWithCapacity::<U>(&len);
        for i in 0i32..=len - 1i32 {
            res.get_mut().push(mapping(&i, &source[i].clone()));
        }
        res
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        source: &Rc<MutCell<Vec<T>>>) -> Rc<MutCell<Vec<U>>> {
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<U>>> = Native::arrayWithCapacity::<U>(&len);
        for i in 0i32..=len - 1i32 {
            res.get_mut().push(mapping(&source[i].clone()));
        }
        res
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&i32, &T1, &T2) -> (U) +
                                        'static>,
                                source1: &Rc<MutCell<Vec<T1>>>,
                                source2: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<U>>> {
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        {
            let len: i32 = source1.len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push(mapping(&i, &source1[i].clone(),
                                           &source2[i].clone()));
            }
            res
        }
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         source1: &Rc<MutCell<Vec<T1>>>,
                         source2: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<U>>> {
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        {
            let len: i32 = source1.len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push(mapping(&source1[i].clone(),
                                           &source2[i].clone()));
            }
            res
        }
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         source1: &Rc<MutCell<Vec<T1>>>,
                         source2: &Rc<MutCell<Vec<T2>>>,
                         source3: &Rc<MutCell<Vec<T3>>>)
     -> Rc<MutCell<Vec<U>>> {
        if if source1.len() as i32 != source2.len() as i32 {
               true
           } else { source2.len() as i32 != source3.len() as i32 } {
            panic!("{}", Array_::SR::differentLengths());
        }
        {
            let len: i32 = source1.len() as i32;
            let res: Rc<MutCell<Vec<U>>> =
                Native::arrayWithCapacity::<U>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push(mapping(&source1[i].clone(),
                                           &source2[i].clone(),
                                           &source3[i].clone()));
            }
            res
        }
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&State, &T) -> ((U, State)) +
                                    'static>, state: &State,
                            source: &Rc<MutCell<Vec<T>>>)
     -> (Rc<MutCell<Vec<U>>>, State) {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<U>>> = Native::arrayWithCapacity::<U>(&len);
        for i in 0i32..=len - 1i32 {
            let m: (U, State) = mapping(&acc.get(), &source[i].clone());
            res.get_mut().push(m.0.clone());
            acc.set(m.1.clone())
        }
        (res, acc.get().clone())
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&T, &State) -> ((U, State)) +
                                        'static>,
                                source: &Rc<MutCell<Vec<T>>>, state: &State)
     -> (Rc<MutCell<Vec<U>>>, State) {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<U>>> = Native::arrayWithCapacity::<U>(&len);
        for i in (0i32..=len - 1i32).rev() {
            let m: (U, State) = mapping(&source[i].clone(), &acc.get());
            res.get_mut().push(m.0.clone());
            acc.set(m.1.clone())
        }
        res.get_mut().reverse();
        (res, acc.get().clone())
    }
    pub fn indexed<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<(i32, T)>>> {
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<(i32, T)>>> =
            Native::arrayWithCapacity::<(i32, T)>(&len);
        for i in 0i32..=len - 1i32 {
            res.get_mut().push((i, source[i].clone()));
        }
        res
    }
    pub fn concat<T: Clone +
                  'static>(sources: &Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>>)
     -> Rc<MutCell<Vec<T>>> {
        let len: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        for idx in 0i32..=sources.len() as i32 - 1i32 {
            let arr: Rc<MutCell<Vec<T>>> = sources[idx].clone();
            len.set(len.get() + arr.len() as i32)
        }
        {
            let res: Rc<MutCell<Vec<T>>> =
                Native::arrayWithCapacity::<T>(&len.get());
            for idx_1 in 0i32..=sources.len() as i32 - 1i32 {
                let arr_1: Rc<MutCell<Vec<T>>> = sources[idx_1].clone();
                for idx_2 in 0i32..=arr_1.len() as i32 - 1i32 {
                    let x: T = arr_1[idx_2].clone();
                    res.get_mut().push(x)
                }
            }
            res
        }
    }
    pub fn collect<T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&T) -> (Rc<MutCell<Vec<U>>>) +
                                    'static>, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<U>>> {
        Array_::concat(&Array_::map(mapping, source))
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>) -> bool {
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
        while if i.get() < source.len() as i32 { !res.get() } else { false } {
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
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
            while if i.get() < source1.len() as i32 {
                      !res.get()
                  } else { false } {
                res.set(predicate(&source1[i.get()].clone(),
                                  &source2[i.get()].clone()));
                i.set(i.get() + 1i32)
            }
            res.get()
        }
    }
    pub fn contains<T: Eq + core::hash::Hash + Clone +
                    'static>(value: &T, source: &Rc<MutCell<Vec<T>>>)
     -> bool {
        Array_::exists(&Rc::from({
                                     let value = value.clone();
                                     move |x: &T| x.clone() == value.clone()
                                 }), source)
    }
    pub fn filter<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
        for i in 0i32..=source.len() as i32 - 1i32 {
            if predicate(&source[i].clone()) {
                res.get_mut().push(source[i].clone());
            };
        }
        res
    }
    pub fn initialize<T: Clone +
                      'static>(count: &i32,
                               initializer:
                                   &Rc<impl Fn(&i32) -> (T) + 'static>)
     -> Rc<MutCell<Vec<T>>> {
        if count.clone() < 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputMustBeNonNegative().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"count")) as Rc<str>);
        }
        {
            let res: Rc<MutCell<Vec<T>>> =
                Native::arrayWithCapacity::<T>(count);
            for i in 0i32..=count.clone() - 1i32 {
                res.get_mut().push(initializer(&i));
            }
            res
        }
    }
    pub fn pairwise<T: Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<(T, T)>>> {
        if (source.len() as i32) < 2i32 {
            Native::arrayEmpty::<(T, T)>()
        } else {
            let len: i32 = source.len() as i32 - 1i32;
            let res: Rc<MutCell<Vec<(T, T)>>> =
                Native::arrayWithCapacity::<(T, T)>(&len);
            for i in 0i32..=len - 1i32 {
                res.get_mut().push((source[i].clone(),
                                    source[i + 1i32].clone()));
            }
            res
        }
    }
    pub fn partition<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>)
     -> (Rc<MutCell<Vec<T>>>, Rc<MutCell<Vec<T>>>) {
        let res1: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
        let res2: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
        for i in 0i32..=source.len() as i32 - 1i32 {
            if predicate(&source[i].clone()) {
                res1.get_mut().push(source[i].clone())
            } else { res2.get_mut().push(source[i].clone()) };
        }
        (res1, res2)
    }
    pub fn reduce<T: Clone +
                  'static>(reduction: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.is_empty() {
            panic!("{}", Array_::SR::inputArrayWasEmpty());
        }
        {
            let acc_1: Rc<MutCell<T>> =
                Rc::from(MutCell::from(source[0i32].clone()));
            for i_1 in 0i32..=source.len() as i32 - 1i32 {
                acc_1.set({
                              let x: T = source[i_1].clone();
                              if i_1 == 0i32 {
                                  x.clone()
                              } else { reduction(&acc_1.get(), &x) }
                          });
            }
            acc_1.get().clone()
        }
    }
    pub fn reduceBack<T: Clone +
                      'static>(reduction:
                                   &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                               source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.is_empty() {
            panic!("{}", Array_::SR::inputArrayWasEmpty());
        }
        {
            let len: i32 = source.len() as i32;
            let acc_1: Rc<MutCell<T>> =
                Rc::from(MutCell::from(source[len - 1i32].clone()));
            for i_1 in 1i32..=len {
                acc_1.set({
                              let acc: T = acc_1.get().clone();
                              let x: T = source[len - i_1].clone();
                              if i_1 - 1i32 == 0i32 {
                                  x.clone()
                              } else { reduction(&acc, &x) }
                          });
            }
            acc_1.get().clone()
        }
    }
    pub fn replicate<T: Clone + 'static>(count: &i32, initial: &T)
     -> Rc<MutCell<Vec<T>>> {
        Array_::initialize(count,
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
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<State>>> =
            Native::arrayCreate(&(len + 1i32), &state.clone());
        res.get_mut()[0i32 as usize] = state.clone();
        for i in 0i32..=len - 1i32 {
            res.get_mut()[(i + 1i32) as usize] =
                folder(&res[i].clone(), &source[i].clone());
        }
        res.clone()
    }
    pub fn scanBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>, source: &Rc<MutCell<Vec<T>>>,
                             state: &State) -> Rc<MutCell<Vec<State>>> {
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<State>>> =
            Native::arrayCreate(&(len + 1i32), &state.clone());
        res.get_mut()[len as usize] = state.clone();
        for i in (0i32..=len - 1i32).rev() {
            res.get_mut()[i as usize] =
                folder(&source[i].clone(), &res[i + 1i32].clone());
        }
        res.clone()
    }
    pub fn skip<T: Clone + 'static>(count: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        if count.clone() > source.len() as i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasTooShort().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
        }
        {
            let count_1: i32 =
                if count.clone() < 0i32 { 0i32 } else { count.clone() };
            Array_::getSubArray(source, &count_1,
                                &(source.len() as i32 - count_1))
        }
    }
    pub fn skipWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        while if count.get() < source.len() as i32 {
                  predicate(&source[count.get()].clone())
              } else { false } {
            count.set(count.get() + 1i32);
        }
        Array_::getSubArray(source, &count.get(),
                            &(source.len() as i32 - count.get()))
    }
    pub fn take<T: Clone + 'static>(count: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        if count.clone() < 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputMustBeNonNegative().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"count")) as Rc<str>);
        }
        if count.clone() > source.len() as i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasTooShort().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
        }
        Array_::getSubArray(source, &0i32, count)
    }
    pub fn takeWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        while if count.get() < source.len() as i32 {
                  predicate(&source[count.get()].clone())
              } else { false } {
            count.set(count.get() + 1i32);
        }
        Array_::getSubArray(source, &0i32, &count.get())
    }
    pub fn truncate<T: Clone +
                    'static>(count: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array_::getSubArray(source, &0i32,
                            &if count.clone() < 0i32 {
                                 0i32
                             } else {
                                 if count.clone() > source.len() as i32 {
                                     source.len() as i32
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
        fn inner_loop<T: Clone +
                      'static>(i: &i32,
                               predicate_1:
                                   &Rc<impl Fn(&T) -> (bool) + 'static>,
                               source_1: &Rc<MutCell<Vec<T>>>) -> Option<T> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let predicate_1 = Rc::from(MutCell::from(predicate_1.clone()));
            let source_1: Rc<MutCell<Rc<MutCell<Vec<T>>>>> =
                Rc::from(MutCell::from(source_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if i.get() >= source_1.get().len() as i32 {
                             None::<T>
                         } else {
                             if predicate_1.get()(&source_1[i.get()].clone())
                                {
                                 Some(source_1[i.get()].clone())
                             } else {
                                 let i_temp: i32 = i.get() + 1i32;
                                 let predicate_1_temp = predicate_1.get();
                                 let source_1_temp: Rc<MutCell<Vec<T>>> =
                                     source_1.get();
                                 i.set(i_temp);
                                 predicate_1.set(predicate_1_temp);
                                 source_1.set(source_1_temp);
                                 continue '_inner_loop
                             }
                         }) ;
                }
        }
        inner_loop(&0i32, predicate, source)
    }
    pub fn find<T: Clone +
                'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                         source: &Rc<MutCell<Vec<T>>>) -> T {
        let matchValue: Option<T> = Array_::tryFind(predicate, source);
        match &matchValue {
            None => panic!("{}", Array_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndex<T: Clone +
                        'static>(predicate:
                                     &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 source: &Rc<MutCell<Vec<T>>>)
     -> Option<i32> {
        fn inner_loop<T: Clone +
                      'static>(i: &i32,
                               predicate_1:
                                   &Rc<impl Fn(&T) -> (bool) + 'static>,
                               source_1: &Rc<MutCell<Vec<T>>>)
         -> Option<i32> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let predicate_1 = Rc::from(MutCell::from(predicate_1.clone()));
            let source_1: Rc<MutCell<Rc<MutCell<Vec<T>>>>> =
                Rc::from(MutCell::from(source_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if i.get() >= source_1.get().len() as i32 {
                             None::<i32>
                         } else {
                             if predicate_1.get()(&source_1[i.get()].clone())
                                {
                                 Some(i.get())
                             } else {
                                 let i_temp: i32 = i.get() + 1i32;
                                 let predicate_1_temp = predicate_1.get();
                                 let source_1_temp: Rc<MutCell<Vec<T>>> =
                                     source_1.get();
                                 i.set(i_temp);
                                 predicate_1.set(predicate_1_temp);
                                 source_1.set(source_1_temp);
                                 continue '_inner_loop
                             }
                         }) ;
                }
        }
        inner_loop(&0i32, predicate, source)
    }
    pub fn findIndex<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>) -> i32 {
        let matchValue: Option<i32> = Array_::tryFindIndex(predicate, source);
        match &matchValue {
            None => panic!("{}", Array_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn indexOf<T: Eq + core::hash::Hash + Clone +
                   'static>(source: &Rc<MutCell<Vec<T>>>, item: &T) -> i32 {
        let matchValue: Option<i32> =
            Array_::tryFindIndex(&Rc::from({
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
        fn inner_loop<T: Clone +
                      'static>(i: &i32,
                               predicate_1:
                                   &Rc<impl Fn(&T) -> (bool) + 'static>,
                               source_1: &Rc<MutCell<Vec<T>>>) -> Option<T> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let predicate_1 = Rc::from(MutCell::from(predicate_1.clone()));
            let source_1: Rc<MutCell<Rc<MutCell<Vec<T>>>>> =
                Rc::from(MutCell::from(source_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if i.get() < 0i32 {
                             None::<T>
                         } else {
                             if predicate_1.get()(&source_1[i.get()].clone())
                                {
                                 Some(source_1[i.get()].clone())
                             } else {
                                 let i_temp: i32 = i.get() - 1i32;
                                 let predicate_1_temp = predicate_1.get();
                                 let source_1_temp: Rc<MutCell<Vec<T>>> =
                                     source_1.get();
                                 i.set(i_temp);
                                 predicate_1.set(predicate_1_temp);
                                 source_1.set(source_1_temp);
                                 continue '_inner_loop
                             }
                         }) ;
                }
        }
        inner_loop(&(source.len() as i32 - 1i32), predicate, source)
    }
    pub fn findBack<T: Clone +
                    'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                             source: &Rc<MutCell<Vec<T>>>) -> T {
        let matchValue: Option<T> = Array_::tryFindBack(predicate, source);
        match &matchValue {
            None => panic!("{}", Array_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndexBack<T: Clone +
                            'static>(predicate:
                                         &Rc<impl Fn(&T) -> (bool) + 'static>,
                                     source: &Rc<MutCell<Vec<T>>>)
     -> Option<i32> {
        fn inner_loop<T: Clone +
                      'static>(i: &i32,
                               predicate_1:
                                   &Rc<impl Fn(&T) -> (bool) + 'static>,
                               source_1: &Rc<MutCell<Vec<T>>>)
         -> Option<i32> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let predicate_1 = Rc::from(MutCell::from(predicate_1.clone()));
            let source_1: Rc<MutCell<Rc<MutCell<Vec<T>>>>> =
                Rc::from(MutCell::from(source_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if i.get() < 0i32 {
                             None::<i32>
                         } else {
                             if predicate_1.get()(&source_1[i.get()].clone())
                                {
                                 Some(i.get())
                             } else {
                                 let i_temp: i32 = i.get() - 1i32;
                                 let predicate_1_temp = predicate_1.get();
                                 let source_1_temp: Rc<MutCell<Vec<T>>> =
                                     source_1.get();
                                 i.set(i_temp);
                                 predicate_1.set(predicate_1_temp);
                                 source_1.set(source_1_temp);
                                 continue '_inner_loop
                             }
                         }) ;
                }
        }
        inner_loop(&(source.len() as i32 - 1i32), predicate, source)
    }
    pub fn findIndexBack<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  source: &Rc<MutCell<Vec<T>>>) -> i32 {
        let matchValue: Option<i32> =
            Array_::tryFindIndexBack(predicate, source);
        match &matchValue {
            None => panic!("{}", Array_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn findLastIndex<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  source: &Rc<MutCell<Vec<T>>>) -> i32 {
        let matchValue: Option<i32> =
            Array_::tryFindIndexBack(predicate, source);
        match &matchValue {
            None => -1i32,
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryPick<T: Clone + 'static, U: Clone +
                   'static>(chooser:
                                &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>) -> Option<U> {
        fn inner_loop<T: Clone + 'static, U: Clone +
                      'static>(i: &i32,
                               chooser_1:
                                   &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                               source_1: &Rc<MutCell<Vec<T>>>) -> Option<U> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let chooser_1 = Rc::from(MutCell::from(chooser_1.clone()));
            let source_1: Rc<MutCell<Rc<MutCell<Vec<T>>>>> =
                Rc::from(MutCell::from(source_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if i.get() >= source_1.get().len() as i32 {
                             None::<U>
                         } else {
                             let matchValue: Option<U> =
                                 chooser_1.get()(&source_1[i.get()].clone());
                             if matchValue.is_none() {
                                 let i_temp: i32 = i.get() + 1i32;
                                 let chooser_1_temp = chooser_1.get();
                                 let source_1_temp: Rc<MutCell<Vec<T>>> =
                                     source_1.get();
                                 i.set(i_temp);
                                 chooser_1.set(chooser_1_temp);
                                 source_1.set(source_1_temp);
                                 continue '_inner_loop
                             } else { matchValue.clone() }
                         }) ;
                }
        }
        inner_loop(&0i32, chooser, source)
    }
    pub fn pick<T: Clone + 'static, U: Clone +
                'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                         source: &Rc<MutCell<Vec<T>>>) -> U {
        let matchValue: Option<U> = Array_::tryPick(chooser, source);
        match &matchValue {
            None => panic!("{}", Array_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn fold<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, source: &Rc<MutCell<Vec<T>>>)
     -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        for i in 0i32..=source.len() as i32 - 1i32 {
            acc.set(folder(&acc.get(), &source[i].clone()));
        }
        acc.get().clone()
    }
    pub fn foldBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>, source: &Rc<MutCell<Vec<T>>>,
                             state: &State) -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        let len: i32 = source.len() as i32;
        for i in 1i32..=len {
            acc.set(folder(&source[len - i].clone(), &acc.get()));
        }
        acc.get().clone()
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone +
                 'static>(folder:
                              &Rc<impl Fn(&State, &T1, &T2) -> (State) +
                                  'static>, state: &State,
                          source1: &Rc<MutCell<Vec<T1>>>,
                          source2: &Rc<MutCell<Vec<T2>>>) -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        for i in 0i32..=source1.len() as i32 - 1i32 {
            acc.set(folder(&acc.get(), &source1[i].clone(),
                           &source2[i].clone()));
        }
        acc.get().clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone +
                     'static>(folder:
                                  &Rc<impl Fn(&T1, &T2, &State) -> (State) +
                                      'static>,
                              source1: &Rc<MutCell<Vec<T1>>>,
                              source2: &Rc<MutCell<Vec<T2>>>, state: &State)
     -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        {
            let len: i32 = source1.len() as i32;
            for i in 1i32..=len {
                acc.set(folder(&source1[len - i].clone(),
                               &source2[len - i].clone(), &acc.get()));
            }
            acc.get().clone()
        }
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>) -> bool {
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(true));
        while if i.get() < source.len() as i32 { res.get() } else { false } {
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
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            let res: Rc<MutCell<bool>> = Rc::from(MutCell::from(true));
            while if i.get() < source1.len() as i32 {
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
        for i in 0i32..=source.len() as i32 - 1i32 {
            action(&source[i].clone());
        };
    }
    pub fn iterateIndexed<T: Clone +
                          'static>(action: &Rc<impl Fn(&i32, &T) + 'static>,
                                   source: &Rc<MutCell<Vec<T>>>) {
        for i in 0i32..=source.len() as i32 - 1i32 {
            action(&i, &source[i].clone());
        };
    }
    pub fn iterate2<T: Clone +
                    'static>(action: &Rc<impl Fn(&T, &T) + 'static>,
                             source1: &Rc<MutCell<Vec<T>>>,
                             source2: &Rc<MutCell<Vec<T>>>) {
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        for i in 0i32..=source1.len() as i32 - 1i32 {
            action(&source1[i].clone(), &source2[i].clone());
        }
    }
    pub fn iterateIndexed2<T: Clone +
                           'static>(action:
                                        &Rc<impl Fn(&i32, &T, &T) + 'static>,
                                    source1: &Rc<MutCell<Vec<T>>>,
                                    source2: &Rc<MutCell<Vec<T>>>) {
        if source1.len() as i32 != source2.len() as i32 {
            panic!("{}", Array_::SR::differentLengths());
        }
        for i in 0i32..=source1.len() as i32 - 1i32 {
            action(&i, &source1[i].clone(), &source2[i].clone());
        }
    }
    pub fn permute<T: Clone +
                   'static>(indexMap: &Rc<impl Fn(&i32) -> (i32) + 'static>,
                            source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let len: i32 = source.len() as i32;
        let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
        let checkFlags: Rc<MutCell<Vec<i32>>> =
            Native::arrayCreate(&len, &0i32);
        Array_::iterateIndexed(&Rc::from({
                                             let checkFlags =
                                                 checkFlags.clone();
                                             let indexMap = indexMap.clone();
                                             let res = res.clone();
                                             move |i: &i32, x: &T|
                                                 {
                                                     let j: i32 = indexMap(i);
                                                     if if j < 0i32 {
                                                            true
                                                        } else { j >= len } {
                                                         panic!("{}",
                                                                Array_::SR::invalidPermutation());
                                                     }
                                                     res.get_mut()[j as usize]
                                                         = x.clone();
                                                     checkFlags.get_mut()[j as
                                                                              usize]
                                                         = 1i32
                                                 }
                                         }), source);
        if !Array_::forAll(&Rc::from(move |y: &i32| 1i32 == y.clone()),
                           &checkFlags) {
            panic!("{}", Array_::SR::invalidPermutation());
        }
        res.clone()
    }
    pub fn sortInPlaceWith<T: Clone +
                           'static>(comparer:
                                        &Rc<impl Fn(&T, &T) -> (i32) +
                                            'static>,
                                    source: &Rc<MutCell<Vec<T>>>) {
        source.clone().get_mut().sort_by(&Native::comparer(&Rc::from({
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
        Array_::sortInPlaceWith(&Rc::from(move |e1: &T, e2: &T|
                                              Util::compare(e1, e2)), source);
    }
    pub fn sortInPlaceBy<T: Clone + 'static, U: PartialOrd + Clone +
                         'static>(projection:
                                      &Rc<impl Fn(&T) -> (U) + 'static>,
                                  source: &Rc<MutCell<Vec<T>>>) {
        Array_::sortInPlaceWith(&Rc::from({
                                              let projection =
                                                  projection.clone();
                                              move |x: &T, y: &T|
                                                  Util::compare(&projection(x),
                                                                &projection(y))
                                          }), source);
    }
    pub fn sort<T: PartialOrd + Clone + 'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
        Array_::sortInPlace(&res);
        res.clone()
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone +
                  'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                           source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
        Array_::sortInPlaceBy(projection, &res);
        res.clone()
    }
    pub fn sortWith<T: Clone +
                    'static>(comparer:
                                 &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                             source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> = Native::arrayCopy(source);
        Array_::sortInPlaceWith(comparer, &res);
        res.clone()
    }
    pub fn sortDescending<T: PartialOrd + Clone +
                          'static>(source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array_::sortWith(&Rc::from(move |x: &T, y: &T|
                                       Util::compare(x, y) * -1i32), source)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone +
                            'static>(projection:
                                         &Rc<impl Fn(&T) -> (U) + 'static>,
                                     source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        Array_::sortWith(&Rc::from({
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
     -> Rc<MutCell<Vec<(T1, T2)>>> {
        let len1: i32 = xs.len() as i32;
        let len2: i32 = ys.len() as i32;
        let res: Rc<MutCell<Vec<(T1, T2)>>> =
            Native::arrayWithCapacity::<(T1, T2)>(&(len1 * len2));
        for i in 0i32..=len1 - 1i32 {
            for j in 0i32..=len2 - 1i32 {
                res.get_mut().push((xs[i].clone(), ys[j].clone()));
            };
        }
        res
    }
    pub fn unfold<State: Clone + 'static, T: Clone +
                  'static>(generator:
                               &Rc<impl Fn(&State) -> (Option<(T, State)>) +
                                   'static>, state: &State)
     -> Rc<MutCell<Vec<T>>> {
        fn inner_loop<a_: Clone + 'static, T: Clone +
                      'static>(generator_1:
                                   &Rc<impl Fn(&a_) -> (Option<(T, a_)>) +
                                       'static>, state_1: &a_,
                               res: &Rc<MutCell<Vec<T>>>) {
            let generator_1 = Rc::from(MutCell::from(generator_1.clone()));
            let state_1: Rc<MutCell<a_>> =
                Rc::from(MutCell::from(state_1.clone()));
            let res: Rc<MutCell<Rc<MutCell<Vec<T>>>>> =
                Rc::from(MutCell::from(res.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        ({
                             let matchValue: Option<(T, a_)> =
                                 generator_1.get()(&state_1.get());
                             match &matchValue {
                                 Some(matchValue_0_0) => {
                                     let x: T = matchValue_0_0.0.clone();
                                     let s: a_ = matchValue_0_0.1.clone();
                                     res.get().get_mut().push(x);
                                     {
                                         let generator_1_temp =
                                             generator_1.get();
                                         let state_1_temp: a_ = s;
                                         let res_temp: Rc<MutCell<Vec<T>>> =
                                             res.get();
                                         generator_1.set(generator_1_temp);
                                         state_1.set(state_1_temp);
                                         res.set(res_temp);
                                         continue '_inner_loop
                                     }
                                 }
                                 _ => (),
                             }
                         }) ;
                }
        }
        let res_1: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
        inner_loop(generator, state, &res_1);
        res_1
    }
    pub fn unzip<T1: Clone + 'static, T2: Clone +
                 'static>(source: &Rc<MutCell<Vec<(T1, T2)>>>)
     -> (Rc<MutCell<Vec<T1>>>, Rc<MutCell<Vec<T2>>>) {
        let len: i32 = source.len() as i32;
        let res1: Rc<MutCell<Vec<T1>>> =
            Native::arrayWithCapacity::<T1>(&len);
        let res2: Rc<MutCell<Vec<T2>>> =
            Native::arrayWithCapacity::<T2>(&len);
        Array_::iterateIndexed(&Rc::from({
                                             let res1 = res1.clone();
                                             let res2 = res2.clone();
                                             move
                                                 |i: &i32,
                                                  tupledArg: &(T1, T2)|
                                                 {
                                                     res1.get_mut().push(tupledArg.0.clone());
                                                     res2.get_mut().push(tupledArg.1.clone())
                                                 }
                                         }), source);
        (res1, res2)
    }
    pub fn unzip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                  'static>(source: &Rc<MutCell<Vec<(T1, T2, T3)>>>)
     -> (Rc<MutCell<Vec<T1>>>, Rc<MutCell<Vec<T2>>>, Rc<MutCell<Vec<T3>>>) {
        let len: i32 = source.len() as i32;
        let res1: Rc<MutCell<Vec<T1>>> =
            Native::arrayWithCapacity::<T1>(&len);
        let res2: Rc<MutCell<Vec<T2>>> =
            Native::arrayWithCapacity::<T2>(&len);
        let res3: Rc<MutCell<Vec<T3>>> =
            Native::arrayWithCapacity::<T3>(&len);
        Array_::iterateIndexed(&Rc::from({
                                             let res1 = res1.clone();
                                             let res2 = res2.clone();
                                             let res3 = res3.clone();
                                             move
                                                 |i: &i32,
                                                  tupledArg: &(T1, T2, T3)|
                                                 {
                                                     res1.get_mut().push(tupledArg.0.clone());
                                                     res2.get_mut().push(tupledArg.1.clone());
                                                     res3.get_mut().push(tupledArg.2.clone())
                                                 }
                                         }), source);
        (res1, res2, res3)
    }
    pub fn zip<T1: Clone + 'static, T2: Clone +
               'static>(source1: &Rc<MutCell<Vec<T1>>>,
                        source2: &Rc<MutCell<Vec<T2>>>)
     -> Rc<MutCell<Vec<(T1, T2)>>> {
        Array_::map2(&Rc::from(move |x: &T1, y: &T2| (x.clone(), y.clone())),
                     source1, source2)
    }
    pub fn zip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                'static>(source1: &Rc<MutCell<Vec<T1>>>,
                         source2: &Rc<MutCell<Vec<T2>>>,
                         source3: &Rc<MutCell<Vec<T3>>>)
     -> Rc<MutCell<Vec<(T1, T2, T3)>>> {
        Array_::map3(&Rc::from(move |x: &T1, y: &T2, z: &T3|
                                   (x.clone(), y.clone(), z.clone())),
                     source1, source2, source3)
    }
    pub fn chunkBySize<T: Clone +
                       'static>(chunkSize: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        if chunkSize.clone() <= 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputMustBePositive().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"size")) as Rc<str>);
        }
        {
            let len: i32 = source.len() as i32;
            let chunkCount: i32 = (len - 1i32) / chunkSize.clone() + 1i32;
            let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(&chunkCount);
            for i in 0i32..=chunkCount - 1i32 {
                let start: i32 = i * chunkSize.clone();
                let slice: Rc<MutCell<Vec<T>>> =
                    Array_::getSubArray(source, &start,
                                        &chunkSize.clone().min(len - start));
                res.get_mut().push(slice)
            }
            res
        }
    }
    pub fn splitAt<T: Clone +
                   'static>(index: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> (Rc<MutCell<Vec<T>>>, Rc<MutCell<Vec<T>>>) {
        if if index.clone() < 0i32 {
               true
           } else { index.clone() > source.len() as i32 } {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::indexOutOfBounds().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>);
        }
        (Array_::getSubArray(source, &0i32, index),
         Array_::getSubArray(source, index,
                             &(source.len() as i32 - index.clone())))
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone +
               'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        let acc: Rc<MutCell<T>> =
            Rc::from(MutCell::from(Native::getZero::<T>()));
        for i in 0i32..=source.len() as i32 - 1i32 {
            acc.set(acc.get().clone() + source[i].clone());
        }
        acc.get().clone()
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default +
                 Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          source: &Rc<MutCell<Vec<T>>>) -> U {
        let acc: Rc<MutCell<U>> =
            Rc::from(MutCell::from(Native::getZero::<U>()));
        for i in 0i32..=source.len() as i32 - 1i32 {
            acc.set(acc.get().clone() + projection(&source[i].clone()));
        }
        acc.get().clone()
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<MutCell<Vec<T>>>) -> T {
        Array_::reduce(&Rc::from({
                                     let projection = projection.clone();
                                     move |x: &T, y: &T|
                                         if projection(x) > projection(y) {
                                             x.clone()
                                         } else { y.clone() }
                                 }), xs)
    }
    pub fn max<T: PartialOrd + Clone + 'static>(xs: &Rc<MutCell<Vec<T>>>)
     -> T {
        Array_::reduce(&Rc::from(move |x: &T, y: &T|
                                     if x.clone() > y.clone() {
                                         x.clone()
                                     } else { y.clone() }), xs)
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<MutCell<Vec<T>>>) -> T {
        Array_::reduce(&Rc::from({
                                     let projection = projection.clone();
                                     move |x: &T, y: &T|
                                         if projection(x) < projection(y) {
                                             x.clone()
                                         } else { y.clone() }
                                 }), xs)
    }
    pub fn min<T: PartialOrd + Clone + 'static>(xs: &Rc<MutCell<Vec<T>>>)
     -> T {
        Array_::reduce(&Rc::from(move |x: &T, y: &T|
                                     if x.clone() < y.clone() {
                                         x.clone()
                                     } else { y.clone() }), xs)
    }
    pub fn average<T: core::ops::Add<Output = T> +
                   core::ops::Div<Output = T> + From<i32> + Default + Clone +
                   'static>(source: &Rc<MutCell<Vec<T>>>) -> T {
        if source.clone().is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
        }
        {
            let total: Rc<MutCell<T>> =
                Rc::from(MutCell::from(Native::getZero::<T>()));
            for i in 0i32..=source.len() as i32 - 1i32 {
                total.set(total.get().clone() + source[i].clone());
            }
            total.get().clone() / T::from(source.len() as i32)
        }
    }
    pub fn averageBy<T: Clone + 'static, U: core::ops::Add<Output = U> +
                     core::ops::Div<Output = U> + From<i32> + Default +
                     Clone +
                     'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                              source: &Rc<MutCell<Vec<T>>>) -> U {
        if source.is_empty() {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputArrayWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
        }
        {
            let total: Rc<MutCell<U>> =
                Rc::from(MutCell::from(Native::getZero::<U>()));
            for i in 0i32..=source.len() as i32 - 1i32 {
                total.set(total.get().clone() +
                              projection(&source[i].clone()));
            }
            total.get().clone() / U::from(source.len() as i32)
        }
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
        Array_::filter(predicate, source)
    }
    pub fn windowed<T: Clone +
                    'static>(windowSize: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        if windowSize.clone() <= 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputMustBePositive().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"size")) as Rc<str>);
        }
        {
            let len: i32 =
                0i32.max(source.len() as i32 - windowSize.clone() + 1i32);
            let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(&len);
            for i in 0i32..=len - 1i32 {
                let slice: Rc<MutCell<Vec<T>>> =
                    Array_::getSubArray(source, &i, windowSize);
                res.get_mut().push(slice)
            }
            res
        }
    }
    pub fn splitInto<T: Clone +
                     'static>(chunks: &i32, source: &Rc<MutCell<Vec<T>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        if chunks.clone() <= 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(Array_::SR::inputMustBePositive().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"chunks")) as Rc<str>);
        }
        if source.is_empty() {
            Native::arrayEmpty::<Rc<MutCell<Vec<T>>>>()
        } else {
            let res: Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> =
                Native::arrayWithCapacity::<Rc<MutCell<Vec<T>>>>(chunks);
            let chunks_1: i32 = chunks.clone().min(source.len() as i32);
            let minChunkSize: i32 = source.len() as i32 / chunks_1;
            let chunksWithExtraItem: i32 = source.len() as i32 % chunks_1;
            for i in 0i32..=chunks_1 - 1i32 {
                let chunkSize: i32 =
                    if i < chunksWithExtraItem {
                        minChunkSize + 1i32
                    } else { minChunkSize };
                let slice: Rc<MutCell<Vec<T>>> =
                    Array_::getSubArray(source,
                                        &(i * minChunkSize +
                                              chunksWithExtraItem.min(i)),
                                        &chunkSize);
                res.get_mut().push(slice)
            }
            res
        }
    }
    pub fn transpose<T: Clone +
                     'static>(arrays: &Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>>)
     -> Rc<MutCell<Vec<Rc<MutCell<Vec<T>>>>>> {
        if arrays.clone().is_empty() {
            Native::arrayEmpty::<Rc<MutCell<Vec<T>>>>()
        } else {
            let len: i32 = arrays.len() as i32;
            let firstArray: Rc<MutCell<Vec<T>>> = arrays[0i32].clone();
            let innerLen: i32 = firstArray.len() as i32;
            if !Array_::forAll(&Rc::from(move |a_2: &Rc<MutCell<Vec<T>>>|
                                             a_2.len() as i32 == innerLen),
                               arrays) {
                panic!("{}", Array_::SR::differentLengths());
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
                res
            }
        }
    }
}
