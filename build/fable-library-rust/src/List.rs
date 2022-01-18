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
use crate::import_c6216f2::*;
use std::rc::Rc;
pub mod List {
    use super::*;
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Node_1<T: Clone + 'static> {
        pub head: T,
        pub tail: MutCell<Option<Rc<List::Node_1<T>>>>,
    }
    pub mod SR {
        use super::*;
        pub fn indexOutOfBounds() -> Rc<str> {
            Native::string(&"The index was outside the range of elements in the list.")
        }
        pub fn inputListWasEmpty() -> Rc<str> {
            Native::string(&"List was empty")
        }
        pub fn inputMustBeNonNegative() -> Rc<str> {
            Native::string(&"The input must be non-negative.")
        }
        pub fn inputSequenceEmpty() -> Rc<str> {
            Native::string(&"The input sequence was empty.")
        }
        pub fn inputSequenceTooLong() -> Rc<str> {
            Native::string(&"The input sequence contains more than one element.")
        }
        pub fn keyNotFoundAlt() -> Rc<str> {
            Native::string(&"An index satisfying the predicate was not found in the collection.")
        }
        pub fn differentLengths() -> Rc<str> {
            Native::string(&"The lists had different lengths.")
        }
        pub fn notEnoughElements() -> Rc<str> {
            Native::string(&"The input sequence has an insufficient number of elements.")
        }
    }
    pub fn indexNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", List::SR::keyNotFoundAlt())
    }
    pub fn setConsTail<T: Clone +
                       'static>(t: &Option<Rc<List::Node_1<T>>>,
                                xs: &Option<Rc<List::Node_1<T>>>) {
        match xs {
            None => (),
            Some(xs_0_0) => {
                let node: Rc<List::Node_1<T>> = xs_0_0.clone();
                node.tail.set(t.clone())
            }
        };
    }
    pub fn emptyRoot<T: Clone + 'static>() -> Option<Rc<List::Node_1<T>>> {
        Some(Rc::from(List::Node_1::<T>{head: Native::defaultOf::<T>(),
                                        tail:
                                            MutCell::from(None::<Rc<List::Node_1<T>>>),}))
    }
    pub fn appendConsNoTail<T: Clone +
                            'static>(x: &T, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        {
            let t: Option<Rc<List::Node_1<T>>> =
                Some(Rc::from(List::Node_1::<T>{head: x.clone(),
                                                tail:
                                                    MutCell::from(None::<Rc<List::Node_1<T>>>),}));
            List::setConsTail(&t, xs);
            t.clone()
        }.clone()
    }
    pub fn empty<T: Clone + 'static>() -> Option<Rc<List::Node_1<T>>> {
        None::<Rc<List::Node_1<T>>>
    }
    pub fn cons<T: Clone + 'static>(x: &T, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        Some(Rc::from(List::Node_1::<T>{head: x.clone(),
                                        tail: MutCell::from(xs.clone()),}))
    }
    pub fn singleton<T: Clone + 'static>(x: &T)
     -> Option<Rc<List::Node_1<T>>> {
        List::cons(x, &List::empty())
    }
    pub fn isEmpty<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> bool {
        xs.is_none()
    }
    pub fn head<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>) -> T {
        match xs {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(List::SR::inputListWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list")) as Rc<str>),
            Some(xs_0_0) => xs_0_0.head.clone(),
        }
    }
    pub fn tryHead<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<T> {
        match xs {
            None => None::<T>,
            Some(xs_0_0) => Some(xs_0_0.head.clone()),
        }
    }
    pub fn tail<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        match xs {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(List::SR::inputListWasEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list")) as Rc<str>),
            Some(xs_0_0) =>
            {
                let node: Rc<List::Node_1<T>> = xs_0_0.clone();
                node.tail.get().clone()
            }.clone(),
        }
    }
    pub fn length<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> i32 {
        fn inner_loop<a_: Clone +
                      'static>(i: &i32, xs_1: &Option<Rc<List::Node_1<a_>>>)
         -> i32 {
            match xs_1 {
                Some(xs_1_0_0) => {
                    let node: Rc<List::Node_1<a_>> = xs_1_0_0.clone();
                    inner_loop(&(i.clone() + 1i32), &node.tail.get())
                }
                _ => i.clone(),
            }
        }
        inner_loop(&0i32, xs)
    }
    pub fn tryLast<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<T> {
        match xs {
            Some(xs_0_0) =>
            {
                let node: Rc<List::Node_1<T>> = xs_0_0.clone();
                if List::isEmpty(&node.tail.get()) {
                    Some(node.head.clone())
                } else { List::tryLast(&node.tail.get()) }
            }.clone(),
            _ => None::<T>,
        }
    }
    pub fn last<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            let matchValue: Option<T> = List::tryLast(xs);
            match &matchValue {
                None => panic!("{}", List::SR::inputListWasEmpty()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn ofOption<T: Clone + 'static>(opt: &Option<T>)
     -> Option<Rc<List::Node_1<T>>> {
        match opt {
            None => List::empty(),
            Some(opt_0_0) => List::singleton(opt_0_0),
        }
    }
    pub fn toArray<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> =
                Native::arrayWithCapacity::<T>(&List::length(xs));
            let xs_1: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs.clone()));
            while !List::isEmpty(&xs_1.get()) {
                res.get_mut().push(List::head(&xs_1.get()));
                xs_1.set(List::tail(&xs_1.get()))
            }
            Native::arrayCopy(&res)
        }.clone()
    }
    pub fn fold<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, xs: &Option<Rc<List::Node_1<T>>>)
     -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            let xs_1: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs.clone()));
            while !List::isEmpty(&xs_1.get()) {
                acc.set(folder(&acc.get(), &List::head(&xs_1.get())));
                xs_1.set(List::tail(&xs_1.get()))
            }
            acc.get().clone()
        }.clone()
    }
    pub fn reverse<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::fold(&Rc::from(move |acc: &Option<Rc<List::Node_1<T>>>, x: &T|
                                 List::cons(x, acc)), &List::empty(), xs)
    }
    pub fn foldBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>,
                             xs: &Option<Rc<List::Node_1<T>>>, state: &State)
     -> State {
        List::fold(&Rc::from({
                                 let folder = folder.clone();
                                 move |acc: &State, x: &T| folder(x, acc)
                             }), state, &List::reverse(xs))
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone +
                 'static>(folder:
                              &Rc<impl Fn(&State, &T1, &T2) -> (State) +
                                  'static>, state: &State,
                          xs: &Option<Rc<List::Node_1<T1>>>,
                          ys: &Option<Rc<List::Node_1<T2>>>) -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            let xs_1: Rc<MutCell<Option<Rc<List::Node_1<T1>>>>> =
                Rc::from(MutCell::from(xs.clone()));
            let ys_1: Rc<MutCell<Option<Rc<List::Node_1<T2>>>>> =
                Rc::from(MutCell::from(ys.clone()));
            while if !List::isEmpty(&xs_1.get()) {
                      !List::isEmpty(&ys_1.get())
                  } else { false } {
                acc.set(folder(&acc.get(), &List::head(&xs_1.get()),
                               &List::head(&ys_1.get())));
                xs_1.set(List::tail(&xs_1.get()));
                ys_1.set(List::tail(&ys_1.get()))
            }
            acc.get().clone()
        }.clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone +
                     'static>(folder:
                                  &Rc<impl Fn(&T1, &T2, &State) -> (State) +
                                      'static>,
                              xs: &Option<Rc<List::Node_1<T1>>>,
                              ys: &Option<Rc<List::Node_1<T2>>>,
                              state: &State) -> State {
        List::fold2(&Rc::from({
                                  let folder = folder.clone();
                                  move |acc: &State, x: &T1, y: &T2|
                                      folder(x, y, acc)
                              }), state, &List::reverse(xs),
                    &List::reverse(ys))
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Option<Rc<List::Node_1<T>>>) -> bool {
        if List::isEmpty(xs) {
            true
        } else {
            if predicate(&List::head(xs)) {
                List::forAll(predicate, &List::tail(xs))
            } else { false }
        }
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Option<Rc<List::Node_1<T1>>>,
                            ys: &Option<Rc<List::Node_1<T2>>>) -> bool {
        let matchValue: Rc<(bool, bool)> =
            Rc::from((List::isEmpty(xs), List::isEmpty(ys)));
        if matchValue.0.clone() {
            if matchValue.1.clone() {
                true
            } else {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::differentLengths().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list2")) as Rc<str>)
            }
        } else {
            if matchValue.1.clone() {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::differentLengths().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list2")) as Rc<str>)
            } else {
                if predicate(&List::head(xs), &List::head(ys)) {
                    List::forAll2(predicate, &List::tail(xs), &List::tail(ys))
                } else { false }
            }
        }
    }
    pub fn unfold<State: Clone + 'static, T: Clone +
                  'static>(gen:
                               &Rc<impl Fn(&State)
                                   -> (Option<Rc<(T, State)>>) + 'static>,
                           state: &State) -> Option<Rc<List::Node_1<T>>> {
        {
            fn inner_loop<a_: Clone + 'static, T: Clone +
                          'static>(gen_1:
                                       &Rc<impl Fn(&a_)
                                           -> (Option<Rc<(T, a_)>>) +
                                           'static>, acc: &a_,
                                   root: &Option<Rc<List::Node_1<T>>>,
                                   xs: &Option<Rc<List::Node_1<T>>>)
             -> Option<Rc<List::Node_1<T>>> {
                {
                    let matchValue: Option<Rc<(T, a_)>> = gen_1(acc);
                    match &matchValue {
                        Some(matchValue_0_0) =>
                        {
                            let acc_1: a_ = matchValue_0_0.1.clone();
                            let t: Option<Rc<List::Node_1<T>>> =
                                List::appendConsNoTail(&matchValue_0_0.0.clone(),
                                                       xs);
                            inner_loop(gen_1, &acc_1,
                                       &if List::isEmpty(root) {
                                            t.clone()
                                        } else { root.clone() }, &t)
                        }.clone(),
                        _ => root.clone(),
                    }
                }.clone()
            }
            inner_loop(gen, state, &List::empty(), &List::empty())
        }.clone()
    }
    pub fn iterate<T: Clone +
                   'static>(action: &Rc<impl Fn(&T) + 'static>,
                            xs: &Option<Rc<List::Node_1<T>>>) {
        List::fold(&Rc::from({
                                 let action = action.clone();
                                 move |unitVar0: &(), x: &T| action(x)
                             }), &(), xs);
    }
    pub fn iterate2<T1: Clone + 'static, T2: Clone +
                    'static>(action: &Rc<impl Fn(&T1, &T2) + 'static>,
                             xs: &Option<Rc<List::Node_1<T1>>>,
                             ys: &Option<Rc<List::Node_1<T2>>>) {
        List::fold2(&Rc::from({
                                  let action = action.clone();
                                  move |unitVar0: &(), x: &T1, y: &T2|
                                      action(x, y)
                              }), &(), xs, ys);
    }
    pub fn iterateIndexed<T: Clone +
                          'static>(action: &Rc<impl Fn(&i32, &T) + 'static>,
                                   xs: &Option<Rc<List::Node_1<T>>>) {
        Util::ignore(&List::fold(&Rc::from({
                                               let action = action.clone();
                                               move |i: &i32, x: &T|
                                                   {
                                                       action(i, x);
                                                       i.clone() + 1i32
                                                   }
                                           }), &0i32, xs));
    }
    pub fn iterateIndexed2<T1: Clone + 'static, T2: Clone +
                           'static>(action:
                                        &Rc<impl Fn(&i32, &T1, &T2) +
                                            'static>,
                                    xs: &Option<Rc<List::Node_1<T1>>>,
                                    ys: &Option<Rc<List::Node_1<T2>>>) {
        Util::ignore(&List::fold2(&Rc::from({
                                                let action = action.clone();
                                                move |i: &i32, x: &T1, y: &T2|
                                                    {
                                                        action(i, x, y);
                                                        i.clone() + 1i32
                                                    }
                                            }), &0i32, xs, ys));
    }
    pub fn ofArrayWithTail<T: Clone +
                           'static>(xs: &Rc<MutCell<Vec<T>>>,
                                    tail_1: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        {
            let res: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(tail_1.clone()));
            let len: i32 = xs.clone().len() as i32;
            for i in (0i32..=len - 1i32).rev() {
                res.set(List::cons(&xs[i].clone(), &res.get()));
            }
            res.get().clone()
        }.clone()
    }
    pub fn ofArray<T: Clone + 'static>(xs: &Rc<MutCell<Vec<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::ofArrayWithTail(xs, &List::empty())
    }
    pub fn append<T: Clone +
                  'static>(xs: &Option<Rc<List::Node_1<T>>>,
                           ys: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::fold(&Rc::from(move |acc: &Option<Rc<List::Node_1<T>>>, x: &T|
                                 List::cons(x, acc)), ys, &List::reverse(xs))
    }
    pub fn choose<T: Clone + 'static, U: Clone +
                  'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                           xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<U>>> {
        {
            let root: Rc<MutCell<Option<Rc<List::Node_1<U>>>>> =
                Rc::from(MutCell::from(List::empty()));
            let node: Rc<MutCell<Option<Rc<List::Node_1<U>>>>> =
                Rc::from(MutCell::from(root.get().clone()));
            let xs_1: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs.clone()));
            while !List::isEmpty(&xs_1.get()) {
                {
                    let matchValue: Option<U> =
                        chooser(&List::head(&xs_1.get()));
                    match &matchValue {
                        None => (),
                        Some(matchValue_0_0) => {
                            let x: U = matchValue_0_0.clone();
                            node.set(List::appendConsNoTail(&x, &node.get()));
                            if List::isEmpty(&root.get()) {
                                root.set(node.get().clone());
                            }
                        }
                    }
                }
                xs_1.set(List::tail(&xs_1.get()))
            }
            root.get().clone()
        }.clone()
    }
    pub fn concat<T: Clone +
                  'static>(sources:
                               &Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>>)
     -> Option<Rc<List::Node_1<T>>> {
        {
            let root: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(List::empty()));
            let node: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(root.get().clone()));
            let xs:
                    Rc<MutCell<Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>>>> =
                Rc::from(MutCell::from(sources.clone()));
            let ys: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(List::empty()));
            while !List::isEmpty(&xs.get()) {
                ys.set(List::head(&xs.get()));
                while !List::isEmpty(&ys.get()) {
                    node.set({
                                 let xs_1: Option<Rc<List::Node_1<T>>> =
                                     node.get().clone();
                                 List::appendConsNoTail(&List::head(&ys.get()),
                                                        &xs_1)
                             }.clone());
                    if List::isEmpty(&root.get()) {
                        root.set(node.get().clone());
                    }
                    ys.set(List::tail(&ys.get()))
                }
                xs.set(List::tail(&xs.get()))
            }
            root.get().clone()
        }.clone()
    }
    pub fn compareWith<T: Clone +
                       'static>(comparer:
                                    &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                                xs: &Option<Rc<List::Node_1<T>>>,
                                ys: &Option<Rc<List::Node_1<T>>>) -> i32 {
        let matchValue: Rc<(bool, bool)> =
            Rc::from((List::isEmpty(xs), List::isEmpty(ys)));
        if matchValue.0.clone() {
            if matchValue.1.clone() { 0i32 } else { -1i32 }
        } else {
            if matchValue.1.clone() {
                1i32
            } else {
                let c: i32 = comparer(&List::head(xs), &List::head(ys));
                if c == 0i32 {
                    List::compareWith(comparer, &List::tail(xs),
                                      &List::tail(ys))
                } else { c }
            }
        }
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Option<Rc<List::Node_1<T>>>) -> bool {
        if List::isEmpty(xs) {
            false
        } else {
            if predicate(&List::head(xs)) {
                true
            } else { List::exists(predicate, &List::tail(xs)) }
        }
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Option<Rc<List::Node_1<T1>>>,
                            ys: &Option<Rc<List::Node_1<T2>>>) -> bool {
        let matchValue: Rc<(bool, bool)> =
            Rc::from((List::isEmpty(xs), List::isEmpty(ys)));
        if matchValue.0.clone() {
            if matchValue.1.clone() {
                false
            } else {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::differentLengths().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list2")) as Rc<str>)
            }
        } else {
            if matchValue.1.clone() {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::differentLengths().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list2")) as Rc<str>)
            } else {
                if predicate(&List::head(xs), &List::head(ys)) {
                    true
                } else {
                    List::exists2(predicate, &List::tail(xs), &List::tail(ys))
                }
            }
        }
    }
    pub fn contains<T: PartialEq + Clone +
                    'static>(value: &T, xs: &Option<Rc<List::Node_1<T>>>)
     -> bool {
        List::exists(&Rc::from({
                                   let value = value.clone();
                                   move |x: &T| x.clone() == value.clone()
                               }), xs)
    }
    pub fn filter<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::choose(&Rc::from({
                                   let predicate = predicate.clone();
                                   move |x: &T|
                                       if predicate(x) {
                                           Some(x.clone())
                                       } else { None::<T> }
                               }), xs)
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<U>>> {
        List::unfold(&Rc::from({
                                   let mapping = mapping.clone();
                                   move |xs_1: &Option<Rc<List::Node_1<T>>>|
                                       if List::isEmpty(xs_1) {
                                           None::<Rc<(U,
                                                      Option<Rc<List::Node_1<T>>>)>>
                                       } else {
                                           Some(Rc::from((mapping(&List::head(xs_1)),
                                                          List::tail(xs_1))))
                                       }
                               }), xs)
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone +
                      'static>(mapping:
                                   &Rc<impl Fn(&i32, &T) -> (U) + 'static>,
                               xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<U>>> {
        List::unfold(&Rc::from({
                                   let mapping = mapping.clone();
                                   move
                                       |tupledArg:
                                            &Rc<(i32,
                                                 Option<Rc<List::Node_1<T>>>)>|
                                       {
                                           let i: i32 = tupledArg.0.clone();
                                           let xs_1:
                                                   Option<Rc<List::Node_1<T>>> =
                                               tupledArg.1.clone();
                                           if List::isEmpty(&xs_1) {
                                               None::<Rc<(U,
                                                          Rc<(i32,
                                                              Option<Rc<List::Node_1<T>>>)>)>>
                                           } else {
                                               Some(Rc::from((mapping(&i,
                                                                      &List::head(&xs_1)),
                                                              Rc::from((i +
                                                                            1i32,
                                                                        List::tail(&xs_1))))))
                                           }
                                       }.clone()
                               }), &Rc::from((0i32, xs.clone())))
    }
    pub fn collect<T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&T)
                                    -> (Option<Rc<List::Node_1<U>>>) +
                                    'static>,
                            xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<U>>> {
        List::concat(&List::map(mapping, xs))
    }
    pub fn indexed<a_: Clone + 'static>(xs: &Option<Rc<List::Node_1<a_>>>)
     -> Option<Rc<List::Node_1<Rc<(i32, a_)>>>> {
        List::mapIndexed(&Rc::from(move |i: &i32, x: &a_|
                                       Rc::from((i.clone(), x.clone()))), xs)
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         xs: &Option<Rc<List::Node_1<T1>>>,
                         ys: &Option<Rc<List::Node_1<T2>>>)
     -> Option<Rc<List::Node_1<U>>> {
        List::unfold(&Rc::from({
                                   let mapping = mapping.clone();
                                   move
                                       |tupledArg:
                                            &Rc<(Option<Rc<List::Node_1<T1>>>,
                                                 Option<Rc<List::Node_1<T2>>>)>|
                                       {
                                           let xs_1:
                                                   Option<Rc<List::Node_1<T1>>> =
                                               tupledArg.0.clone();
                                           let ys_1:
                                                   Option<Rc<List::Node_1<T2>>> =
                                               tupledArg.1.clone();
                                           if if List::isEmpty(&xs_1) {
                                                  true
                                              } else { List::isEmpty(&ys_1) }
                                              {
                                               None::<Rc<(U,
                                                          Rc<(Option<Rc<List::Node_1<T1>>>,
                                                              Option<Rc<List::Node_1<T2>>>)>)>>
                                           } else {
                                               Some(Rc::from((mapping(&List::head(&xs_1),
                                                                      &List::head(&ys_1)),
                                                              Rc::from((List::tail(&xs_1),
                                                                        List::tail(&ys_1))))))
                                           }
                                       }.clone()
                               }), &Rc::from((xs.clone(), ys.clone())))
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&i32, &T1, &T2) -> (U) +
                                        'static>,
                                xs: &Option<Rc<List::Node_1<T1>>>,
                                ys: &Option<Rc<List::Node_1<T2>>>)
     -> Option<Rc<List::Node_1<U>>> {
        List::unfold(&Rc::from({
                                   let mapping = mapping.clone();
                                   move
                                       |tupledArg:
                                            &Rc<(i32,
                                                 Option<Rc<List::Node_1<T1>>>,
                                                 Option<Rc<List::Node_1<T2>>>)>|
                                       {
                                           let i: i32 = tupledArg.0.clone();
                                           let xs_1:
                                                   Option<Rc<List::Node_1<T1>>> =
                                               tupledArg.1.clone();
                                           let ys_1:
                                                   Option<Rc<List::Node_1<T2>>> =
                                               tupledArg.2.clone();
                                           if if List::isEmpty(&xs_1) {
                                                  true
                                              } else { List::isEmpty(&ys_1) }
                                              {
                                               None::<Rc<(U,
                                                          Rc<(i32,
                                                              Option<Rc<List::Node_1<T1>>>,
                                                              Option<Rc<List::Node_1<T2>>>)>)>>
                                           } else {
                                               Some(Rc::from((mapping(&i,
                                                                      &List::head(&xs_1),
                                                                      &List::head(&ys_1)),
                                                              Rc::from((i +
                                                                            1i32,
                                                                        List::tail(&xs_1),
                                                                        List::tail(&ys_1))))))
                                           }
                                       }.clone()
                               }), &Rc::from((0i32, xs.clone(), ys.clone())))
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         xs: &Option<Rc<List::Node_1<T1>>>,
                         ys: &Option<Rc<List::Node_1<T2>>>,
                         zs: &Option<Rc<List::Node_1<T3>>>)
     -> Option<Rc<List::Node_1<U>>> {
        List::unfold(&Rc::from({
                                   let mapping = mapping.clone();
                                   move
                                       |tupledArg:
                                            &Rc<(Option<Rc<List::Node_1<T1>>>,
                                                 Option<Rc<List::Node_1<T2>>>,
                                                 Option<Rc<List::Node_1<T3>>>)>|
                                       {
                                           let xs_1:
                                                   Option<Rc<List::Node_1<T1>>> =
                                               tupledArg.0.clone();
                                           let ys_1:
                                                   Option<Rc<List::Node_1<T2>>> =
                                               tupledArg.1.clone();
                                           let zs_1:
                                                   Option<Rc<List::Node_1<T3>>> =
                                               tupledArg.2.clone();
                                           if if if List::isEmpty(&xs_1) {
                                                     true
                                                 } else {
                                                     List::isEmpty(&ys_1)
                                                 } {
                                                  true
                                              } else { List::isEmpty(&zs_1) }
                                              {
                                               None::<Rc<(U,
                                                          Rc<(Option<Rc<List::Node_1<T1>>>,
                                                              Option<Rc<List::Node_1<T2>>>,
                                                              Option<Rc<List::Node_1<T3>>>)>)>>
                                           } else {
                                               Some(Rc::from((mapping(&List::head(&xs_1),
                                                                      &List::head(&ys_1),
                                                                      &List::head(&zs_1)),
                                                              Rc::from((List::tail(&xs_1),
                                                                        List::tail(&ys_1),
                                                                        List::tail(&zs_1))))))
                                           }
                                       }.clone()
                               }),
                     &Rc::from((xs.clone(), ys.clone(), zs.clone())))
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&State, &T) -> (Rc<(U, State)>) +
                                    'static>, state: &State,
                            xs: &Option<Rc<List::Node_1<T>>>)
     -> Rc<(Option<Rc<List::Node_1<U>>>, State)> {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            Rc::from((List::unfold(&Rc::from({
                                                 let acc = acc.clone();
                                                 let mapping =
                                                     mapping.clone();
                                                 move
                                                     |xs_1:
                                                          &Option<Rc<List::Node_1<T>>>|
                                                     if List::isEmpty(xs_1) {
                                                         None::<Rc<(U,
                                                                    Option<Rc<List::Node_1<T>>>)>>
                                                     } else {
                                                         {
                                                             let m:
                                                                     Rc<(U,
                                                                         State)> =
                                                                 mapping(&acc.get(),
                                                                         &List::head(xs_1));
                                                             acc.set(m.1.clone());
                                                             Some(Rc::from((m.0.clone(),
                                                                            List::tail(xs_1))))
                                                         }.clone()
                                                     }
                                             }), xs), acc.get().clone()))
        }.clone()
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&T, &State)
                                        -> (Rc<(U, State)>) + 'static>,
                                xs: &Option<Rc<List::Node_1<T>>>,
                                state: &State)
     -> Rc<(Option<Rc<List::Node_1<U>>>, State)> {
        {
            let ys: Rc<MutCell<Option<Rc<List::Node_1<U>>>>> =
                Rc::from(MutCell::from(List::empty()));
            let st: State =
                List::fold(&Rc::from({
                                         let mapping = mapping.clone();
                                         let ys = ys.clone();
                                         move |acc: &State, x: &T|
                                             {
                                                 let m: Rc<(U, State)> =
                                                     mapping(x, acc);
                                                 ys.set(List::cons(&m.0.clone(),
                                                                   &ys.get()));
                                                 m.1.clone()
                                             }.clone()
                                     }), state, &List::reverse(xs));
            Rc::from((ys.get().clone(), st))
        }.clone()
    }
    pub fn tryPick<T: Clone + 'static, U: Clone +
                   'static>(chooser:
                                &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                            xs: &Option<Rc<List::Node_1<T>>>) -> Option<U> {
        {
            fn inner_loop<T: Clone + 'static, U: Clone +
                          'static>(chooser_1:
                                       &Rc<impl Fn(&T) -> (Option<U>) +
                                           'static>,
                                   xs_1: &Option<Rc<List::Node_1<T>>>)
             -> Option<U> {
                if List::isEmpty(xs_1) {
                    None::<U>
                } else {
                    {
                        let matchValue: Option<U> =
                            chooser_1(&List::head(xs_1));
                        match &matchValue {
                            None => inner_loop(chooser_1, &List::tail(xs_1)),
                            _ => matchValue.clone(),
                        }
                    }.clone()
                }
            }
            inner_loop(chooser, xs)
        }.clone()
    }
    pub fn pick<T: Clone + 'static, U: Clone +
                'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                         xs: &Option<Rc<List::Node_1<T>>>) -> U {
        {
            let matchValue: Option<U> = List::tryPick(chooser, xs);
            match &matchValue {
                None => panic!("{}", List::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFind<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Option<Rc<List::Node_1<T>>>) -> Option<T> {
        List::tryPick(&Rc::from({
                                    let predicate = predicate.clone();
                                    move |x: &T|
                                        if predicate(x) {
                                            Some(x.clone())
                                        } else { None::<T> }
                                }), xs)
    }
    pub fn find<T: Clone +
                'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                         xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            let matchValue: Option<T> = List::tryFind(predicate, xs);
            match &matchValue {
                None => panic!("{}", List::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFindBack<T: Clone +
                       'static>(predicate:
                                    &Rc<impl Fn(&T) -> (bool) + 'static>,
                                xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<T> {
        Array::tryFindBack(predicate, &List::toArray(xs))
    }
    pub fn findBack<T: Clone +
                    'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                             xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            let matchValue: Option<T> = List::tryFindBack(predicate, xs);
            match &matchValue {
                None => panic!("{}", List::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFindIndex<T: Clone +
                        'static>(predicate:
                                     &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<i32> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   predicate_1:
                                       &Rc<impl Fn(&T) -> (bool) + 'static>,
                                   xs_1: &Option<Rc<List::Node_1<T>>>)
             -> Option<i32> {
                if List::isEmpty(xs_1) {
                    None::<i32>
                } else {
                    if predicate_1(&List::head(xs_1)) {
                        Some(i.clone())
                    } else {
                        inner_loop(&(i.clone() + 1i32), predicate_1,
                                   &List::tail(xs_1))
                    }
                }
            }
            inner_loop(&0i32, predicate, xs)
        }.clone()
    }
    pub fn findIndex<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List::Node_1<T>>>) -> i32 {
        let matchValue: Option<i32> = List::tryFindIndex(predicate, xs);
        match &matchValue {
            None => panic!("{}", List::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndexBack<T: Clone +
                            'static>(predicate:
                                         &Rc<impl Fn(&T) -> (bool) + 'static>,
                                     xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<i32> {
        Array::tryFindIndexBack(predicate, &List::toArray(xs))
    }
    pub fn findIndexBack<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  xs: &Option<Rc<List::Node_1<T>>>) -> i32 {
        let matchValue: Option<i32> = List::tryFindIndexBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", List::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryItem<T: Clone +
                   'static>(index: &i32, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<T> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   xs_1: &Option<Rc<List::Node_1<T>>>)
             -> Option<T> {
                if List::isEmpty(xs_1) {
                    None::<T>
                } else {
                    if i.clone() == 0i32 {
                        Some(List::head(xs_1))
                    } else {
                        if i.clone() < 0i32 {
                            None::<T>
                        } else {
                            inner_loop(&(i.clone() - 1i32), &List::tail(xs_1))
                        }
                    }
                }
            }
            inner_loop(index, xs)
        }.clone()
    }
    pub fn item<T: Clone +
                'static>(index: &i32, xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            let matchValue: Option<T> = List::tryItem(index, xs);
            match &matchValue {
                Some(matchValue_0_0) => matchValue_0_0.clone(),
                _ =>
                panic!("{}",
                       Rc::from((Rc::from(List::SR::indexOutOfBounds().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>),
            }
        }.clone()
    }
    pub fn initialize<T: Clone +
                      'static>(count: &i32,
                               initializer:
                                   &Rc<impl Fn(&i32) -> (T) + 'static>)
     -> Option<Rc<List::Node_1<T>>> {
        List::unfold(&Rc::from({
                                   let count = count.clone();
                                   let initializer = initializer.clone();
                                   move |i: &i32|
                                       if i.clone() < count {
                                           Some(Rc::from((initializer(i),
                                                          i.clone() + 1i32)))
                                       } else { None::<Rc<(T, i32)>> }
                               }), &0i32)
    }
    pub fn pairwise<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<Rc<(T, T)>>>> {
        List::ofArray(&Array::pairwise(&List::toArray(xs)))
    }
    pub fn partition<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List::Node_1<T>>>)
     -> Rc<(Option<Rc<List::Node_1<T>>>, Option<Rc<List::Node_1<T>>>)> {
        {
            let root1: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(List::empty()));
            let root2: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(List::empty()));
            let node1: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(root1.get().clone()));
            let node2: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(root2.get().clone()));
            let xs_1: Rc<MutCell<Option<Rc<List::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs.clone()));
            while !List::isEmpty(&xs_1.get()) {
                let x: T = List::head(&xs_1.get());
                if predicate(&x) {
                    node1.set(List::appendConsNoTail(&x, &node1.get()));
                    if List::isEmpty(&root1.get()) {
                        root1.set(node1.get().clone());
                    }
                } else {
                    node2.set(List::appendConsNoTail(&x, &node2.get()));
                    if List::isEmpty(&root2.get()) {
                        root2.set(node2.get().clone());
                    }
                }
                xs_1.set(List::tail(&xs_1.get()))
            }
            Rc::from((root1.get().clone(), root2.get().clone()))
        }.clone()
    }
    pub fn reduce<T: Clone +
                  'static>(reduction: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                           xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            if List::isEmpty(xs) {
                panic!("{}", List::SR::inputListWasEmpty());
            }
            List::fold(reduction, &List::head(xs), &List::tail(xs))
        }.clone()
    }
    pub fn reduceBack<T: Clone +
                      'static>(reduction:
                                   &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                               xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            if List::isEmpty(xs) {
                panic!("{}", List::SR::inputListWasEmpty());
            }
            List::foldBack(reduction, &List::tail(xs), &List::head(xs))
        }.clone()
    }
    pub fn replicate<a_: Clone + 'static>(count: &i32, initial: &a_)
     -> Option<Rc<List::Node_1<a_>>> {
        List::initialize(count,
                         &Rc::from({
                                       let initial = initial.clone();
                                       move |_arg1: &i32| initial.clone()
                                   }))
    }
    pub fn unzip<a_: Clone + 'static, b_: Clone +
                 'static>(xs: &Option<Rc<List::Node_1<Rc<(a_, b_)>>>>)
     -> Rc<(Option<Rc<List::Node_1<a_>>>, Option<Rc<List::Node_1<b_>>>)> {
        List::foldBack(&Rc::from(move
                                     |tupledArg: &Rc<(a_, b_)>,
                                      tupledArg_1:
                                          &Rc<(Option<Rc<List::Node_1<a_>>>,
                                               Option<Rc<List::Node_1<b_>>>)>|
                                     Rc::from((List::cons(&tupledArg.0.clone(),
                                                          &tupledArg_1.0.clone()),
                                               List::cons(&tupledArg.1.clone(),
                                                          &tupledArg_1.1.clone())))),
                       xs, &Rc::from((List::empty(), List::empty())))
    }
    pub fn unzip3<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
                  'static>(xs: &Option<Rc<List::Node_1<Rc<(a_, b_, c_)>>>>)
     ->
         Rc<(Option<Rc<List::Node_1<a_>>>, Option<Rc<List::Node_1<b_>>>,
             Option<Rc<List::Node_1<c_>>>)> {
        List::foldBack(&Rc::from(move
                                     |tupledArg: &Rc<(a_, b_, c_)>,
                                      tupledArg_1:
                                          &Rc<(Option<Rc<List::Node_1<a_>>>,
                                               Option<Rc<List::Node_1<b_>>>,
                                               Option<Rc<List::Node_1<c_>>>)>|
                                     Rc::from((List::cons(&tupledArg.0.clone(),
                                                          &tupledArg_1.0.clone()),
                                               List::cons(&tupledArg.1.clone(),
                                                          &tupledArg_1.1.clone()),
                                               List::cons(&tupledArg.2.clone(),
                                                          &tupledArg_1.2.clone())))),
                       xs,
                       &Rc::from((List::empty(), List::empty(),
                                  List::empty())))
    }
    pub fn zip<a_: Clone + 'static, b_: Clone +
               'static>(xs: &Option<Rc<List::Node_1<a_>>>,
                        ys: &Option<Rc<List::Node_1<b_>>>)
     -> Option<Rc<List::Node_1<Rc<(a_, b_)>>>> {
        List::map2(&Rc::from(move |x: &a_, y: &b_|
                                 Rc::from((x.clone(), y.clone()))), xs, ys)
    }
    pub fn zip3<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
                'static>(xs: &Option<Rc<List::Node_1<a_>>>,
                         ys: &Option<Rc<List::Node_1<b_>>>,
                         zs: &Option<Rc<List::Node_1<c_>>>)
     -> Option<Rc<List::Node_1<Rc<(a_, b_, c_)>>>> {
        List::map3(&Rc::from(move |x: &a_, y: &b_, z: &c_|
                                 Rc::from((x.clone(), y.clone(), z.clone()))),
                   xs, ys, zs)
    }
    pub fn sortWith<T: Clone +
                    'static>(comparer:
                                 &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                             xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        {
            let arr: Rc<MutCell<Vec<T>>> = List::toArray(xs);
            Array::sortInPlaceWith(comparer, &arr);
            List::ofArray(&arr)
        }.clone()
    }
    pub fn sort<T: PartialOrd + Clone +
                'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::sortWith(&Rc::from(move |e1: &T, e2: &T| Util::compare(e1, e2)),
                       xs)
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone +
                  'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                           xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::sortWith(&Rc::from({
                                     let projection = projection.clone();
                                     move |x: &T, y: &T|
                                         Util::compare(&projection(x),
                                                       &projection(y))
                                 }), xs)
    }
    pub fn sortDescending<T: PartialOrd + Clone +
                          'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::sortWith(&Rc::from(move |x: &T, y: &T|
                                     Util::compare(x, y) * -1i32), xs)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone +
                            'static>(projection:
                                         &Rc<impl Fn(&T) -> (U) + 'static>,
                                     xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::sortWith(&Rc::from({
                                     let projection = projection.clone();
                                     move |x: &T, y: &T|
                                         Util::compare(&projection(x),
                                                       &projection(y)) * -1i32
                                 }), xs)
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone +
               'static>(xs: &Option<Rc<List::Node_1<T>>>) -> T {
        List::fold(&Rc::from(move |acc: &T, x: &T| acc.clone() + x.clone()),
                   &Native::getZero::<T>(), xs)
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default +
                 Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Option<Rc<List::Node_1<T>>>) -> U {
        List::fold(&Rc::from({
                                 let projection = projection.clone();
                                 move |acc: &U, x: &T|
                                     acc.clone() + projection(x)
                             }), &Native::getZero::<U>(), xs)
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Option<Rc<List::Node_1<T>>>) -> T {
        List::reduce(&Rc::from({
                                   let projection = projection.clone();
                                   move |x: &T, y: &T|
                                       if projection(x) > projection(y) {
                                           x.clone()
                                       } else { y.clone() }
                               }), xs)
    }
    pub fn max<T: PartialOrd + Clone +
               'static>(xs: &Option<Rc<List::Node_1<T>>>) -> T {
        List::reduce(&Rc::from(move |x: &T, y: &T|
                                   if x.clone() > y.clone() {
                                       x.clone()
                                   } else { y.clone() }), xs)
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Option<Rc<List::Node_1<T>>>) -> T {
        List::reduce(&Rc::from({
                                   let projection = projection.clone();
                                   move |x: &T, y: &T|
                                       if projection(x) < projection(y) {
                                           x.clone()
                                       } else { y.clone() }
                               }), xs)
    }
    pub fn min<T: PartialOrd + Clone +
               'static>(xs: &Option<Rc<List::Node_1<T>>>) -> T {
        List::reduce(&Rc::from(move |x: &T, y: &T|
                                   if x.clone() < y.clone() {
                                       x.clone()
                                   } else { y.clone() }), xs)
    }
    pub fn average<T: core::ops::Add<Output = T> +
                   core::ops::Div<Output = T> + From<i32> + Default + Clone +
                   'static>(xs: &Option<Rc<List::Node_1<T>>>) -> T {
        {
            if List::isEmpty(xs) {
                panic!("{}", List::SR::inputListWasEmpty());
            }
            {
                let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
                List::fold(&Rc::from({
                                         let count = count.clone();
                                         move |acc: &T, x: &T|
                                             {
                                                 count.set(count.get() +
                                                               1i32);
                                                 acc.clone() + x.clone()
                                             }.clone()
                                     }), &Native::getZero::<T>(), xs) /
                    T::from(count.get()).clone()
            }.clone()
        }.clone()
    }
    pub fn averageBy<T: Clone + 'static, U: core::ops::Add<Output = U> +
                     core::ops::Div<Output = U> + From<i32> + Default +
                     Clone +
                     'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                              xs: &Option<Rc<List::Node_1<T>>>) -> U {
        {
            if List::isEmpty(xs) {
                panic!("{}", List::SR::inputListWasEmpty());
            }
            {
                let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
                List::fold(&Rc::from({
                                         let count = count.clone();
                                         let projection = projection.clone();
                                         move |acc: &U, x: &T|
                                             {
                                                 count.set(count.get() +
                                                               1i32);
                                                 acc.clone() + projection(x)
                                             }.clone()
                                     }), &Native::getZero::<U>(), xs) /
                    U::from(count.get()).clone()
            }.clone()
        }.clone()
    }
    pub fn permute<T: Clone +
                   'static>(indexMap: &Rc<impl Fn(&i32) -> (i32) + 'static>,
                            xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::ofArray(&Array::permute(indexMap, &List::toArray(xs)))
    }
    pub fn chunkBySize<T: Clone +
                       'static>(chunkSize: &i32,
                                xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>> {
        List::ofArray(&Array::map(&Rc::from(move |xs_2: &Rc<MutCell<Vec<T>>>|
                                                List::ofArray(xs_2)),
                                  &Array::chunkBySize(chunkSize,
                                                      &List::toArray(xs))))
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone +
                    'static>(xs: &Option<Rc<List::Node_1<T1>>>,
                             ys: &Option<Rc<List::Node_1<T2>>>)
     -> Option<Rc<List::Node_1<Rc<(T1, T2)>>>> {
        {
            let root: Rc<MutCell<Option<Rc<List::Node_1<Rc<(T1, T2)>>>>>> =
                Rc::from(MutCell::from(List::empty()));
            let node: Rc<MutCell<Option<Rc<List::Node_1<Rc<(T1, T2)>>>>>> =
                Rc::from(MutCell::from(root.get().clone()));
            List::iterate(&Rc::from({
                                        let node = node.clone();
                                        let root = root.clone();
                                        let ys = ys.clone();
                                        move |x: &T1|
                                            List::iterate(&Rc::from({
                                                                        let node
                                                                            =
                                                                            node.clone();
                                                                        let root
                                                                            =
                                                                            root.clone();
                                                                        let x
                                                                            =
                                                                            x.clone();
                                                                        move
                                                                            |y:
                                                                                 &T2|
                                                                            {
                                                                                node.set(List::appendConsNoTail(&Rc::from((x.clone(),
                                                                                                                           y.clone())),
                                                                                                                &node.get()));
                                                                                if List::isEmpty(&root.get())
                                                                                   {
                                                                                    root.set(node.get().clone());
                                                                                }
                                                                            }
                                                                    }), &ys)
                                    }), xs);
            root.get().clone()
        }.clone()
    }
    pub fn scan<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<State>>> {
        List::ofArray(&Array::scan(folder, state, &List::toArray(xs)))
    }
    pub fn scanBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>,
                             xs: &Option<Rc<List::Node_1<T>>>, state: &State)
     -> Option<Rc<List::Node_1<State>>> {
        List::ofArray(&Array::scanBack(folder, &List::toArray(xs), state))
    }
    pub fn skip<T: Clone +
                'static>(count: &i32, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        if count.clone() <= 0i32 {
            xs.clone()
        } else {
            {
                if List::isEmpty(xs) {
                    panic!("{}",
                           Rc::from((Rc::from(List::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list")) as Rc<str>);
                }
                List::skip(&(count.clone() - 1i32), &List::tail(xs))
            }.clone()
        }
    }
    pub fn skipWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        if List::isEmpty(xs) {
            xs.clone()
        } else {
            if !predicate(&List::head(xs)) {
                xs.clone()
            } else { List::skipWhile(predicate, &List::tail(xs)) }
        }
    }
    pub fn take<T: Clone +
                'static>(count: &i32, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        {
            if count.clone() < 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::inputMustBeNonNegative().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"count")) as Rc<str>);
            }
            List::unfold(&Rc::from(move
                                       |tupledArg:
                                            &Rc<(i32,
                                                 Option<Rc<List::Node_1<T>>>)>|
                                       {
                                           let i: i32 = tupledArg.0.clone();
                                           let xs_1:
                                                   Option<Rc<List::Node_1<T>>> =
                                               tupledArg.1.clone();
                                           if i > 0i32 {
                                               {
                                                   if List::isEmpty(&xs_1) {
                                                       panic!("{}",
                                                              Rc::from((Rc::from(List::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list")) as Rc<str>);
                                                   }
                                                   Some(Rc::from((List::head(&xs_1),
                                                                  Rc::from((i
                                                                                -
                                                                                1i32,
                                                                            List::tail(&xs_1))))))
                                               }.clone()
                                           } else {
                                               None::<Rc<(T,
                                                          Rc<(i32,
                                                              Option<Rc<List::Node_1<T>>>)>)>>
                                           }
                                       }.clone()),
                         &Rc::from((count.clone(), xs.clone())))
        }.clone()
    }
    pub fn takeWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::unfold(&Rc::from({
                                   let predicate = predicate.clone();
                                   move |xs_1: &Option<Rc<List::Node_1<T>>>|
                                       if if !List::isEmpty(xs_1) {
                                              predicate(&List::head(xs_1))
                                          } else { false } {
                                           Some(Rc::from((List::head(xs_1),
                                                          List::tail(xs_1))))
                                       } else {
                                           None::<Rc<(T,
                                                      Option<Rc<List::Node_1<T>>>)>>
                                       }
                               }), xs)
    }
    pub fn truncate<T: Clone +
                    'static>(count: &i32, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::unfold(&Rc::from(move
                                   |tupledArg:
                                        &Rc<(i32,
                                             Option<Rc<List::Node_1<T>>>)>|
                                   {
                                       let i: i32 = tupledArg.0.clone();
                                       let xs_1: Option<Rc<List::Node_1<T>>> =
                                           tupledArg.1.clone();
                                       if if i > 0i32 {
                                              !List::isEmpty(&xs_1)
                                          } else { false } {
                                           Some(Rc::from((List::head(&xs_1),
                                                          Rc::from((i - 1i32,
                                                                    List::tail(&xs_1))))))
                                       } else {
                                           None::<Rc<(T,
                                                      Rc<(i32,
                                                          Option<Rc<List::Node_1<T>>>)>)>>
                                       }
                                   }.clone()),
                     &Rc::from((count.clone(), xs.clone())))
    }
    pub fn splitAt<T: Clone +
                   'static>(index: &i32, xs: &Option<Rc<List::Node_1<T>>>)
     -> Rc<(Option<Rc<List::Node_1<T>>>, Option<Rc<List::Node_1<T>>>)> {
        {
            if index.clone() < 0i32 {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::inputMustBeNonNegative().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>);
            }
            if index.clone() > List::length(xs) {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>);
            }
            Rc::from((List::take(index, xs), List::skip(index, xs)))
        }.clone()
    }
    pub fn exactlyOne<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> T {
        if List::isEmpty(xs) {
            panic!("{}",
                   Rc::from((Rc::from(List::SR::inputSequenceEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list")) as Rc<str>)
        } else {
            if List::isEmpty(&List::tail(xs)) {
                List::head(xs)
            } else {
                panic!("{}",
                       Rc::from((Rc::from(List::SR::inputSequenceTooLong().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"list")) as Rc<str>)
            }
        }
    }
    pub fn tryExactlyOne<T: Clone + 'static>(xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<T> {
        if if !List::isEmpty(xs) {
               List::isEmpty(&List::tail(xs))
           } else { false } {
            Some(List::head(xs))
        } else { None::<T> }
    }
    pub fn r#where<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<T>>> {
        List::filter(predicate, xs)
    }
    pub fn windowed<T: Clone +
                    'static>(windowSize: &i32,
                             xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>> {
        List::ofArray(&Array::map(&Rc::from(move |xs_2: &Rc<MutCell<Vec<T>>>|
                                                List::ofArray(xs_2)),
                                  &Array::windowed(windowSize,
                                                   &List::toArray(xs))))
    }
    pub fn splitInto<T: Clone +
                     'static>(chunks: &i32, xs: &Option<Rc<List::Node_1<T>>>)
     -> Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>> {
        List::ofArray(&Array::map(&Rc::from(move |xs_2: &Rc<MutCell<Vec<T>>>|
                                                List::ofArray(xs_2)),
                                  &Array::splitInto(chunks,
                                                    &List::toArray(xs))))
    }
    pub fn transpose<T: Clone +
                     'static>(lists:
                                  &Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>>)
     -> Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>> {
        if List::isEmpty(lists) {
            List::empty()
        } else {
            {
                let roots:
                        Option<Rc<List::Node_1<Option<Rc<List::Node_1<T>>>>>> =
                    List::map(&Rc::from(move |x: &T| List::singleton(x)),
                              &List::head(lists));
                let nodes: Rc<MutCell<Vec<Option<Rc<List::Node_1<T>>>>>> =
                    List::toArray(&roots);
                List::iterate(&Rc::from({
                                            let nodes = nodes.clone();
                                            move
                                                |xs_2:
                                                     &Option<Rc<List::Node_1<T>>>|
                                                List::iterateIndexed(&Rc::from({
                                                                                   let nodes
                                                                                       =
                                                                                       nodes.clone();
                                                                                   move
                                                                                       |i:
                                                                                            &i32,
                                                                                        x_1:
                                                                                            &T|
                                                                                       {
                                                                                           if i.clone()
                                                                                                  >=
                                                                                                  nodes.len()
                                                                                                      as
                                                                                                      i32
                                                                                              {
                                                                                               panic!("{}",
                                                                                                      Rc::from((Rc::from(List::SR::differentLengths().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"lists")) as Rc<str>);
                                                                                           }
                                                                                           nodes.get_mut()[i.clone()
                                                                                                               as
                                                                                                               usize]
                                                                                               =
                                                                                               List::appendConsNoTail(x_1,
                                                                                                                      &nodes[i.clone()].clone())
                                                                                       }
                                                                               }),
                                                                     xs_2)
                                        }), &List::tail(lists));
                roots.clone()
            }.clone()
        }
    }
}
