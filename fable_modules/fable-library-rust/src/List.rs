#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_3bd9ae6a::*;
use crate::import_eae1ac5e::*;
use crate::import_8d7d6be8::*;
use crate::import_f222008f::*;
use crate::import_c6216f2::*;
pub mod List_ {
    use super::*;
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Hash, Eq, Ord)]
    pub struct Node_1<T: Clone + 'static> {
        pub head: T,
        pub tail: MutCell<Option<Rc<List_::Node_1<T>>>>,
    }
    pub mod SR {
        use super::*;
        pub fn indexOutOfBounds() -> Rc<str> {
            thread_local! {
                static indexOutOfBounds: Rc<str> =
    String_::string(&"The index was outside the range of elements in the list.");
            };
            indexOutOfBounds.with(|value| value.clone())
        }
        pub fn inputListWasEmpty() -> Rc<str> {
            thread_local! {
                static inputListWasEmpty: Rc<str> = String_::string(&"List was empty");
            };
            inputListWasEmpty.with(|value| value.clone())
        }
        pub fn inputMustBeNonNegative() -> Rc<str> {
            thread_local! {
                static inputMustBeNonNegative: Rc<str> =
    String_::string(&"The input must be non-negative.");
            };
            inputMustBeNonNegative.with(|value| value.clone())
        }
        pub fn inputSequenceEmpty() -> Rc<str> {
            thread_local! {
                static inputSequenceEmpty: Rc<str> =
    String_::string(&"The input sequence was empty.");
            };
            inputSequenceEmpty.with(|value| value.clone())
        }
        pub fn inputSequenceTooLong() -> Rc<str> {
            thread_local! {
                static inputSequenceTooLong: Rc<str> =
    String_::string(&"The input sequence contains more than one element.");
            };
            inputSequenceTooLong.with(|value| value.clone())
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
    String_::string(&"The lists had different lengths.");
            };
            differentLengths.with(|value| value.clone())
        }
        pub fn notEnoughElements() -> Rc<str> {
            thread_local! {
                static notEnoughElements: Rc<str> =
    String_::string(&"The input sequence has an insufficient number of elements.");
            };
            notEnoughElements.with(|value| value.clone())
        }
    }
    pub fn indexNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", List_::SR::keyNotFoundAlt())
    }
    pub fn setConsTail<T: Clone +
                       'static>(t: &Option<Rc<List_::Node_1<T>>>,
                                xs: &Option<Rc<List_::Node_1<T>>>) {
        match xs {
            None => (),
            Some(xs_0_0) => {
                let node: Rc<List_::Node_1<T>> = xs_0_0.clone();
                node.tail.set(t.clone())
            }
        };
    }
    pub fn emptyRoot<T: Clone + 'static>() -> Option<Rc<List_::Node_1<T>>> {
        Some(Rc::from(List_::Node_1::<T>{head: Native::defaultOf::<T>(),
                                         tail:
                                             MutCell::from(None::<Rc<List_::Node_1<T>>>),}))
    }
    pub fn appendConsNoTail<T: Clone +
                            'static>(x: &T, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        let t: Option<Rc<List_::Node_1<T>>> =
            Some(Rc::from(List_::Node_1::<T>{head: x.clone(),
                                             tail:
                                                 MutCell::from(None::<Rc<List_::Node_1<T>>>),}));
        List_::setConsTail(&t, xs);
        t.clone()
    }
    pub fn empty<T: Clone + 'static>() -> Option<Rc<List_::Node_1<T>>> {
        None::<Rc<List_::Node_1<T>>>
    }
    pub fn cons<T: Clone + 'static>(x: &T, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        Some(Rc::from(List_::Node_1::<T>{head: x.clone(),
                                         tail: MutCell::from(xs.clone()),}))
    }
    pub fn singleton<T: Clone + 'static>(x: &T)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::cons(x, &List_::empty())
    }
    pub fn isEmpty<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> bool {
        xs.is_none()
    }
    pub fn head<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        match xs {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::inputListWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list")) as Rc<str>),
            Some(xs_0_0) => xs_0_0.head.clone(),
        }
    }
    pub fn tryHead<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<T> {
        match xs {
            None => None::<T>,
            Some(xs_0_0) => Some(xs_0_0.head.clone()),
        }
    }
    pub fn tail<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        match xs {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::inputListWasEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list")) as Rc<str>),
            Some(xs_0_0) => {
                let node: Rc<List_::Node_1<T>> = xs_0_0.clone();
                node.tail.get().clone()
            }
        }
    }
    pub fn length<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> i32 {
        fn inner_loop<a_: Clone +
                      'static>(i: &i32, xs_1: &Option<Rc<List_::Node_1<a_>>>)
         -> i32 {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<a_>>>>> =
                Rc::from(MutCell::from(xs_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (match &xs_1.get() {
                             Some(xs_1_0_0) => {
                                 let node: Rc<List_::Node_1<a_>> =
                                     xs_1_0_0.clone();
                                 {
                                     let i_temp: i32 = i.get() + 1i32;
                                     let xs_1_temp:
                                             Option<Rc<List_::Node_1<a_>>> =
                                         node.tail.get().clone();
                                     i.set(i_temp);
                                     xs_1.set(xs_1_temp);
                                     continue '_inner_loop
                                 }
                             }
                             _ => i.get(),
                         }) ;
                }
        }
        inner_loop(&0i32, xs)
    }
    pub fn tryLast<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<T> {
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        '_tryLast:
            loop  {
                break '_tryLast
                    (match &xs.get() {
                         Some(xs_0_0) => {
                             let node: Rc<List_::Node_1<T>> = xs_0_0.clone();
                             if List_::isEmpty(&node.tail.get()) {
                                 Some(node.head.clone())
                             } else {
                                 let xs_temp: Option<Rc<List_::Node_1<T>>> =
                                     node.tail.get().clone();
                                 xs.set(xs_temp);
                                 continue '_tryLast
                             }
                         }
                         _ => None::<T>,
                     }) ;
            }
    }
    pub fn last<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        let matchValue: Option<T> = List_::tryLast(xs);
        match &matchValue {
            None => panic!("{}", List_::SR::inputListWasEmpty()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn ofOption<T: Clone + 'static>(opt: &Option<T>)
     -> Option<Rc<List_::Node_1<T>>> {
        match opt {
            None => List_::empty(),
            Some(opt_0_0) => List_::singleton(opt_0_0),
        }
    }
    pub fn toArray<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> =
            Native::arrayWithCapacity::<T>(&List_::length(xs));
        let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        while !List_::isEmpty(&xs_1.get()) {
            res.get_mut().push(List_::head(&xs_1.get()));
            xs_1.set(List_::tail(&xs_1.get()))
        }
        res
    }
    pub fn fold<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, xs: &Option<Rc<List_::Node_1<T>>>)
     -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        while !List_::isEmpty(&xs_1.get()) {
            acc.set(folder(&acc.get(), &List_::head(&xs_1.get())));
            xs_1.set(List_::tail(&xs_1.get()))
        }
        acc.get().clone()
    }
    pub fn reverse<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::fold(&Rc::from(move |acc: &Option<Rc<List_::Node_1<T>>>, x: &T|
                                  List_::cons(x, acc)), &List_::empty(), xs)
    }
    pub fn foldBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>,
                             xs: &Option<Rc<List_::Node_1<T>>>, state: &State)
     -> State {
        List_::fold(&Rc::from({
                                  let folder = folder.clone();
                                  move |acc: &State, x: &T| folder(x, acc)
                              }), state, &List_::reverse(xs))
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone +
                 'static>(folder:
                              &Rc<impl Fn(&State, &T1, &T2) -> (State) +
                                  'static>, state: &State,
                          xs: &Option<Rc<List_::Node_1<T1>>>,
                          ys: &Option<Rc<List_::Node_1<T2>>>) -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T1>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        let ys_1: Rc<MutCell<Option<Rc<List_::Node_1<T2>>>>> =
            Rc::from(MutCell::from(ys.clone()));
        while if !List_::isEmpty(&xs_1.get()) {
                  !List_::isEmpty(&ys_1.get())
              } else { false } {
            acc.set(folder(&acc.get(), &List_::head(&xs_1.get()),
                           &List_::head(&ys_1.get())));
            xs_1.set(List_::tail(&xs_1.get()));
            ys_1.set(List_::tail(&ys_1.get()))
        }
        acc.get().clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone +
                     'static>(folder:
                                  &Rc<impl Fn(&T1, &T2, &State) -> (State) +
                                      'static>,
                              xs: &Option<Rc<List_::Node_1<T1>>>,
                              ys: &Option<Rc<List_::Node_1<T2>>>,
                              state: &State) -> State {
        List_::fold2(&Rc::from({
                                   let folder = folder.clone();
                                   move |acc: &State, x: &T1, y: &T2|
                                       folder(x, y, acc)
                               }), state, &List_::reverse(xs),
                     &List_::reverse(ys))
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Option<Rc<List_::Node_1<T>>>) -> bool {
        let predicate = Rc::from(MutCell::from(predicate.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        '_forAll:
            loop  {
                break '_forAll
                    (if List_::isEmpty(&xs.get()) {
                         true
                     } else {
                         if predicate.get()(&List_::head(&xs.get())) {
                             let predicate_temp = predicate.get();
                             let xs_temp: Option<Rc<List_::Node_1<T>>> =
                                 List_::tail(&xs.get());
                             predicate.set(predicate_temp);
                             xs.set(xs_temp);
                             continue '_forAll
                         } else { false }
                     }) ;
            }
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T1>>>,
                            ys: &Option<Rc<List_::Node_1<T2>>>) -> bool {
        let predicate = Rc::from(MutCell::from(predicate.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T1>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        let ys: Rc<MutCell<Option<Rc<List_::Node_1<T2>>>>> =
            Rc::from(MutCell::from(ys.clone()));
        '_forAll2:
            loop  {
                break '_forAll2
                    ({
                         let matchValue: (bool, bool) =
                             (List_::isEmpty(&xs.get()),
                              List_::isEmpty(&ys.get()));
                         if matchValue.0.clone() {
                             if matchValue.1.clone() {
                                 true
                             } else {
                                 panic!("{}",
                                        Rc::from((Rc::from(List_::SR::differentLengths().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list2")) as Rc<str>)
                             }
                         } else {
                             if matchValue.1.clone() {
                                 panic!("{}",
                                        Rc::from((Rc::from(List_::SR::differentLengths().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list2")) as Rc<str>)
                             } else {
                                 if predicate.get()(&List_::head(&xs.get()),
                                                    &List_::head(&ys.get())) {
                                     let predicate_temp = predicate.get();
                                     let xs_temp:
                                             Option<Rc<List_::Node_1<T1>>> =
                                         List_::tail(&xs.get());
                                     let ys_temp:
                                             Option<Rc<List_::Node_1<T2>>> =
                                         List_::tail(&ys.get());
                                     predicate.set(predicate_temp);
                                     xs.set(xs_temp);
                                     ys.set(ys_temp);
                                     continue '_forAll2
                                 } else { false }
                             }
                         }
                     }) ;
            }
    }
    pub fn unfold<State: Clone + 'static, T: Clone +
                  'static>(gen:
                               &Rc<impl Fn(&State) -> (Option<(T, State)>) +
                                   'static>, state: &State)
     -> Option<Rc<List_::Node_1<T>>> {
        let root: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let node: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(root.get().clone()));
        let acc: Rc<MutCell<Option<(T, State)>>> =
            Rc::from(MutCell::from(gen(state)));
        while acc.get().is_some() {
            let patternInput: (T, State) = Option_::getValue(&acc.get());
            node.set(List_::appendConsNoTail(&patternInput.0.clone(),
                                             &node.get()));
            if List_::isEmpty(&root.get()) { root.set(node.get().clone()); }
            acc.set(gen(&patternInput.1.clone()))
        }
        root.get().clone()
    }
    pub fn iterate<T: Clone +
                   'static>(action: &Rc<impl Fn(&T) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T>>>) {
        List_::fold(&Rc::from({
                                  let action = action.clone();
                                  move |unitVar0: &(), x: &T| action(x)
                              }), &(), xs);
    }
    pub fn iterate2<T1: Clone + 'static, T2: Clone +
                    'static>(action: &Rc<impl Fn(&T1, &T2) + 'static>,
                             xs: &Option<Rc<List_::Node_1<T1>>>,
                             ys: &Option<Rc<List_::Node_1<T2>>>) {
        List_::fold2(&Rc::from({
                                   let action = action.clone();
                                   move |unitVar0: &(), x: &T1, y: &T2|
                                       action(x, y)
                               }), &(), xs, ys);
    }
    pub fn iterateIndexed<T: Clone +
                          'static>(action: &Rc<impl Fn(&i32, &T) + 'static>,
                                   xs: &Option<Rc<List_::Node_1<T>>>) {
        Util::ignore(&List_::fold(&Rc::from({
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
                                    xs: &Option<Rc<List_::Node_1<T1>>>,
                                    ys: &Option<Rc<List_::Node_1<T2>>>) {
        Util::ignore(&List_::fold2(&Rc::from({
                                                 let action = action.clone();
                                                 move
                                                     |i: &i32, x: &T1, y: &T2|
                                                     {
                                                         action(i, x, y);
                                                         i.clone() + 1i32
                                                     }
                                             }), &0i32, xs, ys));
    }
    pub fn ofArrayWithTail<T: Clone +
                           'static>(xs: &Rc<MutCell<Vec<T>>>,
                                    tail_1: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        let res: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(tail_1.clone()));
        let len: i32 = xs.len() as i32;
        for i in (0i32..=len - 1i32).rev() {
            res.set(List_::cons(&xs[i].clone(), &res.get()));
        }
        res.get().clone()
    }
    pub fn ofArray<T: Clone + 'static>(xs: &Rc<MutCell<Vec<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::ofArrayWithTail(xs, &List_::empty())
    }
    pub fn append<T: Clone +
                  'static>(xs: &Option<Rc<List_::Node_1<T>>>,
                           ys: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::fold(&Rc::from(move |acc: &Option<Rc<List_::Node_1<T>>>, x: &T|
                                  List_::cons(x, acc)), ys,
                    &List_::reverse(xs))
    }
    pub fn choose<T: Clone + 'static, U: Clone +
                  'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                           xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        let root: Rc<MutCell<Option<Rc<List_::Node_1<U>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let node: Rc<MutCell<Option<Rc<List_::Node_1<U>>>>> =
            Rc::from(MutCell::from(root.get().clone()));
        let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        while !List_::isEmpty(&xs_1.get()) {
            {
                let matchValue: Option<U> =
                    chooser(&List_::head(&xs_1.get()));
                match &matchValue {
                    None => (),
                    Some(matchValue_0_0) => {
                        let x: U = matchValue_0_0.clone();
                        node.set(List_::appendConsNoTail(&x, &node.get()));
                        if List_::isEmpty(&root.get()) {
                            root.set(node.get().clone());
                        }
                    }
                }
            }
            xs_1.set(List_::tail(&xs_1.get()))
        }
        root.get().clone()
    }
    pub fn concat<T: Clone +
                  'static>(sources:
                               &Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        let root: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let node: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(root.get().clone()));
        let xs:
                Rc<MutCell<Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>>>> =
            Rc::from(MutCell::from(sources.clone()));
        let ys: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        while !List_::isEmpty(&xs.get()) {
            ys.set(List_::head(&xs.get()));
            while !List_::isEmpty(&ys.get()) {
                node.set({
                             let xs_1: Option<Rc<List_::Node_1<T>>> =
                                 node.get().clone();
                             List_::appendConsNoTail(&List_::head(&ys.get()),
                                                     &xs_1)
                         });
                if List_::isEmpty(&root.get()) {
                    root.set(node.get().clone());
                }
                ys.set(List_::tail(&ys.get()))
            }
            xs.set(List_::tail(&xs.get()))
        }
        root.get().clone()
    }
    pub fn compareWith<T: Clone +
                       'static>(comparer:
                                    &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                                xs: &Option<Rc<List_::Node_1<T>>>,
                                ys: &Option<Rc<List_::Node_1<T>>>) -> i32 {
        let comparer = Rc::from(MutCell::from(comparer.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        let ys: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(ys.clone()));
        '_compareWith:
            loop  {
                break '_compareWith
                    ({
                         let matchValue: (bool, bool) =
                             (List_::isEmpty(&xs.get()),
                              List_::isEmpty(&ys.get()));
                         if matchValue.0.clone() {
                             if matchValue.1.clone() { 0i32 } else { -1i32 }
                         } else {
                             if matchValue.1.clone() {
                                 1i32
                             } else {
                                 let c: i32 =
                                     comparer.get()(&List_::head(&xs.get()),
                                                    &List_::head(&ys.get()));
                                 if c == 0i32 {
                                     let comparer_temp = comparer.get();
                                     let xs_temp:
                                             Option<Rc<List_::Node_1<T>>> =
                                         List_::tail(&xs.get());
                                     let ys_temp:
                                             Option<Rc<List_::Node_1<T>>> =
                                         List_::tail(&ys.get());
                                     comparer.set(comparer_temp);
                                     xs.set(xs_temp);
                                     ys.set(ys_temp);
                                     continue '_compareWith
                                 } else { c }
                             }
                         }
                     }) ;
            }
    }
    pub fn compareTo<T: PartialOrd + Clone +
                     'static>(xs: &Option<Rc<List_::Node_1<T>>>,
                              ys: &Option<Rc<List_::Node_1<T>>>) -> i32 {
        Util::compare(xs, ys)
    }
    pub fn equalsTo<T: Eq + core::hash::Hash + Clone +
                    'static>(xs: &Option<Rc<List_::Node_1<T>>>,
                             ys: &Option<Rc<List_::Node_1<T>>>) -> bool {
        xs.clone() == ys.clone()
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Option<Rc<List_::Node_1<T>>>) -> bool {
        let predicate = Rc::from(MutCell::from(predicate.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        '_exists:
            loop  {
                break '_exists
                    (if List_::isEmpty(&xs.get()) {
                         false
                     } else {
                         if predicate.get()(&List_::head(&xs.get())) {
                             true
                         } else {
                             let predicate_temp = predicate.get();
                             let xs_temp: Option<Rc<List_::Node_1<T>>> =
                                 List_::tail(&xs.get());
                             predicate.set(predicate_temp);
                             xs.set(xs_temp);
                             continue '_exists
                         }
                     }) ;
            }
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T1>>>,
                            ys: &Option<Rc<List_::Node_1<T2>>>) -> bool {
        let predicate = Rc::from(MutCell::from(predicate.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T1>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        let ys: Rc<MutCell<Option<Rc<List_::Node_1<T2>>>>> =
            Rc::from(MutCell::from(ys.clone()));
        '_exists2:
            loop  {
                break '_exists2
                    ({
                         let matchValue: (bool, bool) =
                             (List_::isEmpty(&xs.get()),
                              List_::isEmpty(&ys.get()));
                         if matchValue.0.clone() {
                             if matchValue.1.clone() {
                                 false
                             } else {
                                 panic!("{}",
                                        Rc::from((Rc::from(List_::SR::differentLengths().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list2")) as Rc<str>)
                             }
                         } else {
                             if matchValue.1.clone() {
                                 panic!("{}",
                                        Rc::from((Rc::from(List_::SR::differentLengths().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list2")) as Rc<str>)
                             } else {
                                 if predicate.get()(&List_::head(&xs.get()),
                                                    &List_::head(&ys.get())) {
                                     true
                                 } else {
                                     let predicate_temp = predicate.get();
                                     let xs_temp:
                                             Option<Rc<List_::Node_1<T1>>> =
                                         List_::tail(&xs.get());
                                     let ys_temp:
                                             Option<Rc<List_::Node_1<T2>>> =
                                         List_::tail(&ys.get());
                                     predicate.set(predicate_temp);
                                     xs.set(xs_temp);
                                     ys.set(ys_temp);
                                     continue '_exists2
                                 }
                             }
                         }
                     }) ;
            }
    }
    pub fn contains<T: Eq + core::hash::Hash + Clone +
                    'static>(value: &T, xs: &Option<Rc<List_::Node_1<T>>>)
     -> bool {
        List_::exists(&Rc::from({
                                    let value = value.clone();
                                    move |x: &T| x.clone() == value.clone()
                                }), xs)
    }
    pub fn filter<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::choose(&Rc::from({
                                    let predicate = predicate.clone();
                                    move |x: &T|
                                        if predicate(x) {
                                            Some(x.clone())
                                        } else { None::<T> }
                                }), xs)
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        List_::unfold(&Rc::from({
                                    let mapping = mapping.clone();
                                    move |xs_1: &Option<Rc<List_::Node_1<T>>>|
                                        if List_::isEmpty(xs_1) {
                                            None::<(U,
                                                    Option<Rc<List_::Node_1<T>>>)>
                                        } else {
                                            Some((mapping(&List_::head(xs_1)),
                                                  List_::tail(xs_1)))
                                        }
                                }), xs)
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone +
                      'static>(mapping:
                                   &Rc<impl Fn(&i32, &T) -> (U) + 'static>,
                               xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        List_::unfold(&Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &(i32,
                                               Option<Rc<List_::Node_1<T>>>)|
                                        {
                                            let i: i32 = tupledArg.0.clone();
                                            let xs_1:
                                                    Option<Rc<List_::Node_1<T>>> =
                                                tupledArg.1.clone();
                                            if List_::isEmpty(&xs_1) {
                                                None::<(U,
                                                        (i32,
                                                         Option<Rc<List_::Node_1<T>>>))>
                                            } else {
                                                Some((mapping(&i,
                                                              &List_::head(&xs_1)),
                                                      (i + 1i32,
                                                       List_::tail(&xs_1))))
                                            }
                                        }
                                }), &(0i32, xs.clone()))
    }
    pub fn collect<T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&T)
                                    -> (Option<Rc<List_::Node_1<U>>>) +
                                    'static>,
                            xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        let root: Rc<MutCell<Option<Rc<List_::Node_1<U>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let node: Rc<MutCell<Option<Rc<List_::Node_1<U>>>>> =
            Rc::from(MutCell::from(root.get().clone()));
        let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        let ys: Rc<MutCell<Option<Rc<List_::Node_1<U>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        while !List_::isEmpty(&xs_1.get()) {
            ys.set(mapping(&List_::head(&xs_1.get())));
            while !List_::isEmpty(&ys.get()) {
                node.set({
                             let xs_2: Option<Rc<List_::Node_1<U>>> =
                                 node.get().clone();
                             List_::appendConsNoTail(&List_::head(&ys.get()),
                                                     &xs_2)
                         });
                if List_::isEmpty(&root.get()) {
                    root.set(node.get().clone());
                }
                ys.set(List_::tail(&ys.get()))
            }
            xs_1.set(List_::tail(&xs_1.get()))
        }
        root.get().clone()
    }
    pub fn indexed<a_: Clone + 'static>(xs: &Option<Rc<List_::Node_1<a_>>>)
     -> Option<Rc<List_::Node_1<(i32, a_)>>> {
        List_::mapIndexed(&Rc::from(move |i: &i32, x: &a_|
                                        (i.clone(), x.clone())), xs)
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         xs: &Option<Rc<List_::Node_1<T1>>>,
                         ys: &Option<Rc<List_::Node_1<T2>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        List_::unfold(&Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &(Option<Rc<List_::Node_1<T1>>>,
                                               Option<Rc<List_::Node_1<T2>>>)|
                                        {
                                            let xs_1:
                                                    Option<Rc<List_::Node_1<T1>>> =
                                                tupledArg.0.clone();
                                            let ys_1:
                                                    Option<Rc<List_::Node_1<T2>>> =
                                                tupledArg.1.clone();
                                            if if List_::isEmpty(&xs_1) {
                                                   true
                                               } else {
                                                   List_::isEmpty(&ys_1)
                                               } {
                                                None::<(U,
                                                        (Option<Rc<List_::Node_1<T1>>>,
                                                         Option<Rc<List_::Node_1<T2>>>))>
                                            } else {
                                                Some((mapping(&List_::head(&xs_1),
                                                              &List_::head(&ys_1)),
                                                      (List_::tail(&xs_1),
                                                       List_::tail(&ys_1))))
                                            }
                                        }
                                }), &(xs.clone(), ys.clone()))
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&i32, &T1, &T2) -> (U) +
                                        'static>,
                                xs: &Option<Rc<List_::Node_1<T1>>>,
                                ys: &Option<Rc<List_::Node_1<T2>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        List_::unfold(&Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &(i32,
                                               Option<Rc<List_::Node_1<T1>>>,
                                               Option<Rc<List_::Node_1<T2>>>)|
                                        {
                                            let i: i32 = tupledArg.0.clone();
                                            let xs_1:
                                                    Option<Rc<List_::Node_1<T1>>> =
                                                tupledArg.1.clone();
                                            let ys_1:
                                                    Option<Rc<List_::Node_1<T2>>> =
                                                tupledArg.2.clone();
                                            if if List_::isEmpty(&xs_1) {
                                                   true
                                               } else {
                                                   List_::isEmpty(&ys_1)
                                               } {
                                                None::<(U,
                                                        (i32,
                                                         Option<Rc<List_::Node_1<T1>>>,
                                                         Option<Rc<List_::Node_1<T2>>>))>
                                            } else {
                                                Some((mapping(&i,
                                                              &List_::head(&xs_1),
                                                              &List_::head(&ys_1)),
                                                      (i + 1i32,
                                                       List_::tail(&xs_1),
                                                       List_::tail(&ys_1))))
                                            }
                                        }
                                }), &(0i32, xs.clone(), ys.clone()))
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         xs: &Option<Rc<List_::Node_1<T1>>>,
                         ys: &Option<Rc<List_::Node_1<T2>>>,
                         zs: &Option<Rc<List_::Node_1<T3>>>)
     -> Option<Rc<List_::Node_1<U>>> {
        List_::unfold(&Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &(Option<Rc<List_::Node_1<T1>>>,
                                               Option<Rc<List_::Node_1<T2>>>,
                                               Option<Rc<List_::Node_1<T3>>>)|
                                        {
                                            let xs_1:
                                                    Option<Rc<List_::Node_1<T1>>> =
                                                tupledArg.0.clone();
                                            let ys_1:
                                                    Option<Rc<List_::Node_1<T2>>> =
                                                tupledArg.1.clone();
                                            let zs_1:
                                                    Option<Rc<List_::Node_1<T3>>> =
                                                tupledArg.2.clone();
                                            if if if List_::isEmpty(&xs_1) {
                                                      true
                                                  } else {
                                                      List_::isEmpty(&ys_1)
                                                  } {
                                                   true
                                               } else {
                                                   List_::isEmpty(&zs_1)
                                               } {
                                                None::<(U,
                                                        (Option<Rc<List_::Node_1<T1>>>,
                                                         Option<Rc<List_::Node_1<T2>>>,
                                                         Option<Rc<List_::Node_1<T3>>>))>
                                            } else {
                                                Some((mapping(&List_::head(&xs_1),
                                                              &List_::head(&ys_1),
                                                              &List_::head(&zs_1)),
                                                      (List_::tail(&xs_1),
                                                       List_::tail(&ys_1),
                                                       List_::tail(&zs_1))))
                                            }
                                        }
                                }), &(xs.clone(), ys.clone(), zs.clone()))
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&State, &T) -> ((U, State)) +
                                    'static>, state: &State,
                            xs: &Option<Rc<List_::Node_1<T>>>)
     -> (Option<Rc<List_::Node_1<U>>>, State) {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        (List_::unfold(&Rc::from({
                                     let acc = acc.clone();
                                     let mapping = mapping.clone();
                                     move
                                         |xs_1: &Option<Rc<List_::Node_1<T>>>|
                                         if List_::isEmpty(xs_1) {
                                             None::<(U,
                                                     Option<Rc<List_::Node_1<T>>>)>
                                         } else {
                                             let m: (U, State) =
                                                 mapping(&acc.get(),
                                                         &List_::head(xs_1));
                                             acc.set(m.1.clone());
                                             Some((m.0.clone(),
                                                   List_::tail(xs_1)))
                                         }
                                 }), xs), acc.get().clone())
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&T, &State) -> ((U, State)) +
                                        'static>,
                                xs: &Option<Rc<List_::Node_1<T>>>,
                                state: &State)
     -> (Option<Rc<List_::Node_1<U>>>, State) {
        let ys: Rc<MutCell<Option<Rc<List_::Node_1<U>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let st: State =
            List_::fold(&Rc::from({
                                      let mapping = mapping.clone();
                                      let ys = ys.clone();
                                      move |acc: &State, x: &T|
                                          {
                                              let m: (U, State) =
                                                  mapping(x, acc);
                                              ys.set(List_::cons(&m.0.clone(),
                                                                 &ys.get()));
                                              m.1.clone()
                                          }
                                  }), state, &List_::reverse(xs));
        (ys.get().clone(), st)
    }
    pub fn tryPick<T: Clone + 'static, U: Clone +
                   'static>(chooser:
                                &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T>>>) -> Option<U> {
        fn inner_loop<T: Clone + 'static, U: Clone +
                      'static>(chooser_1:
                                   &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                               xs_1: &Option<Rc<List_::Node_1<T>>>)
         -> Option<U> {
            let chooser_1 = Rc::from(MutCell::from(chooser_1.clone()));
            let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if List_::isEmpty(&xs_1.get()) {
                             None::<U>
                         } else {
                             let matchValue: Option<U> =
                                 chooser_1.get()(&List_::head(&xs_1.get()));
                             match &matchValue {
                                 None => {
                                     let chooser_1_temp = chooser_1.get();
                                     let xs_1_temp:
                                             Option<Rc<List_::Node_1<T>>> =
                                         List_::tail(&xs_1.get());
                                     chooser_1.set(chooser_1_temp);
                                     xs_1.set(xs_1_temp);
                                     continue '_inner_loop
                                 }
                                 _ => matchValue.clone(),
                             }
                         }) ;
                }
        }
        inner_loop(chooser, xs)
    }
    pub fn pick<T: Clone + 'static, U: Clone +
                'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                         xs: &Option<Rc<List_::Node_1<T>>>) -> U {
        let matchValue: Option<U> = List_::tryPick(chooser, xs);
        match &matchValue {
            None => panic!("{}", List_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFind<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T>>>) -> Option<T> {
        List_::tryPick(&Rc::from({
                                     let predicate = predicate.clone();
                                     move |x: &T|
                                         if predicate(x) {
                                             Some(x.clone())
                                         } else { None::<T> }
                                 }), xs)
    }
    pub fn find<T: Clone +
                'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                         xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        let matchValue: Option<T> = List_::tryFind(predicate, xs);
        match &matchValue {
            None => panic!("{}", List_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindBack<T: Clone +
                       'static>(predicate:
                                    &Rc<impl Fn(&T) -> (bool) + 'static>,
                                xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<T> {
        Array_::tryFindBack(predicate, &List_::toArray(xs))
    }
    pub fn findBack<T: Clone +
                    'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                             xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        let matchValue: Option<T> = List_::tryFindBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", List_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndex<T: Clone +
                        'static>(predicate:
                                     &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<i32> {
        fn inner_loop<T: Clone +
                      'static>(i: &i32,
                               predicate_1:
                                   &Rc<impl Fn(&T) -> (bool) + 'static>,
                               xs_1: &Option<Rc<List_::Node_1<T>>>)
         -> Option<i32> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let predicate_1 = Rc::from(MutCell::from(predicate_1.clone()));
            let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if List_::isEmpty(&xs_1.get()) {
                             None::<i32>
                         } else {
                             if predicate_1.get()(&List_::head(&xs_1.get())) {
                                 Some(i.get())
                             } else {
                                 let i_temp: i32 = i.get() + 1i32;
                                 let predicate_1_temp = predicate_1.get();
                                 let xs_1_temp: Option<Rc<List_::Node_1<T>>> =
                                     List_::tail(&xs_1.get());
                                 i.set(i_temp);
                                 predicate_1.set(predicate_1_temp);
                                 xs_1.set(xs_1_temp);
                                 continue '_inner_loop
                             }
                         }) ;
                }
        }
        inner_loop(&0i32, predicate, xs)
    }
    pub fn findIndex<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List_::Node_1<T>>>) -> i32 {
        let matchValue: Option<i32> = List_::tryFindIndex(predicate, xs);
        match &matchValue {
            None => panic!("{}", List_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndexBack<T: Clone +
                            'static>(predicate:
                                         &Rc<impl Fn(&T) -> (bool) + 'static>,
                                     xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<i32> {
        Array_::tryFindIndexBack(predicate, &List_::toArray(xs))
    }
    pub fn findIndexBack<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  xs: &Option<Rc<List_::Node_1<T>>>) -> i32 {
        let matchValue: Option<i32> = List_::tryFindIndexBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", List_::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryItem<T: Clone +
                   'static>(index: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<T> {
        fn inner_loop<T: Clone +
                      'static>(i: &i32, xs_1: &Option<Rc<List_::Node_1<T>>>)
         -> Option<T> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
                Rc::from(MutCell::from(xs_1.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if List_::isEmpty(&xs_1.get()) {
                             None::<T>
                         } else {
                             if i.get() == 0i32 {
                                 Some(List_::head(&xs_1.get()))
                             } else {
                                 if i.get() < 0i32 {
                                     None::<T>
                                 } else {
                                     let i_temp: i32 = i.get() - 1i32;
                                     let xs_1_temp:
                                             Option<Rc<List_::Node_1<T>>> =
                                         List_::tail(&xs_1.get());
                                     i.set(i_temp);
                                     xs_1.set(xs_1_temp);
                                     continue '_inner_loop
                                 }
                             }
                         }) ;
                }
        }
        inner_loop(index, xs)
    }
    pub fn item<T: Clone +
                'static>(index: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> T {
        let matchValue: Option<T> = List_::tryItem(index, xs);
        match &matchValue {
            Some(matchValue_0_0) => matchValue_0_0.clone(),
            _ =>
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::indexOutOfBounds().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>),
        }
    }
    pub fn initialize<T: Clone +
                      'static>(count: &i32,
                               initializer:
                                   &Rc<impl Fn(&i32) -> (T) + 'static>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::unfold(&Rc::from({
                                    let count = count.clone();
                                    let initializer = initializer.clone();
                                    move |i: &i32|
                                        if i.clone() < count {
                                            Some((initializer(i),
                                                  i.clone() + 1i32))
                                        } else { None::<(T, i32)> }
                                }), &0i32)
    }
    pub fn pairwise<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<(T, T)>>> {
        List_::ofArray(&Array_::pairwise(&List_::toArray(xs)))
    }
    pub fn partition<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List_::Node_1<T>>>)
     -> (Option<Rc<List_::Node_1<T>>>, Option<Rc<List_::Node_1<T>>>) {
        let root1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let root2: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let node1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(root1.get().clone()));
        let node2: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(root2.get().clone()));
        let xs_1: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        while !List_::isEmpty(&xs_1.get()) {
            let x: T = List_::head(&xs_1.get());
            if predicate(&x) {
                node1.set(List_::appendConsNoTail(&x, &node1.get()));
                if List_::isEmpty(&root1.get()) {
                    root1.set(node1.get().clone());
                }
            } else {
                node2.set(List_::appendConsNoTail(&x, &node2.get()));
                if List_::isEmpty(&root2.get()) {
                    root2.set(node2.get().clone());
                }
            }
            xs_1.set(List_::tail(&xs_1.get()))
        }
        (root1.get().clone(), root2.get().clone())
    }
    pub fn reduce<T: Clone +
                  'static>(reduction: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                           xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        if List_::isEmpty(xs) {
            panic!("{}", List_::SR::inputListWasEmpty());
        }
        List_::fold(reduction, &List_::head(xs), &List_::tail(xs))
    }
    pub fn reduceBack<T: Clone +
                      'static>(reduction:
                                   &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                               xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        if List_::isEmpty(xs) {
            panic!("{}", List_::SR::inputListWasEmpty());
        }
        List_::foldBack(reduction, &List_::tail(xs), &List_::head(xs))
    }
    pub fn replicate<a_: Clone + 'static>(count: &i32, initial: &a_)
     -> Option<Rc<List_::Node_1<a_>>> {
        List_::initialize(count,
                          &Rc::from({
                                        let initial = initial.clone();
                                        move |_arg1: &i32| initial.clone()
                                    }))
    }
    pub fn unzip<a_: Clone + 'static, b_: Clone +
                 'static>(xs: &Option<Rc<List_::Node_1<(a_, b_)>>>)
     -> (Option<Rc<List_::Node_1<a_>>>, Option<Rc<List_::Node_1<b_>>>) {
        List_::foldBack(&Rc::from(move
                                      |tupledArg: &(a_, b_),
                                       tupledArg_1:
                                           &(Option<Rc<List_::Node_1<a_>>>,
                                             Option<Rc<List_::Node_1<b_>>>)|
                                      (List_::cons(&tupledArg.0.clone(),
                                                   &tupledArg_1.0.clone()),
                                       List_::cons(&tupledArg.1.clone(),
                                                   &tupledArg_1.1.clone()))),
                        xs, &(List_::empty(), List_::empty()))
    }
    pub fn unzip3<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
                  'static>(xs: &Option<Rc<List_::Node_1<(a_, b_, c_)>>>)
     ->
         (Option<Rc<List_::Node_1<a_>>>, Option<Rc<List_::Node_1<b_>>>,
          Option<Rc<List_::Node_1<c_>>>) {
        List_::foldBack(&Rc::from(move
                                      |tupledArg: &(a_, b_, c_),
                                       tupledArg_1:
                                           &(Option<Rc<List_::Node_1<a_>>>,
                                             Option<Rc<List_::Node_1<b_>>>,
                                             Option<Rc<List_::Node_1<c_>>>)|
                                      (List_::cons(&tupledArg.0.clone(),
                                                   &tupledArg_1.0.clone()),
                                       List_::cons(&tupledArg.1.clone(),
                                                   &tupledArg_1.1.clone()),
                                       List_::cons(&tupledArg.2.clone(),
                                                   &tupledArg_1.2.clone()))),
                        xs, &(List_::empty(), List_::empty(), List_::empty()))
    }
    pub fn zip<a_: Clone + 'static, b_: Clone +
               'static>(xs: &Option<Rc<List_::Node_1<a_>>>,
                        ys: &Option<Rc<List_::Node_1<b_>>>)
     -> Option<Rc<List_::Node_1<(a_, b_)>>> {
        List_::map2(&Rc::from(move |x: &a_, y: &b_| (x.clone(), y.clone())),
                    xs, ys)
    }
    pub fn zip3<a_: Clone + 'static, b_: Clone + 'static, c_: Clone +
                'static>(xs: &Option<Rc<List_::Node_1<a_>>>,
                         ys: &Option<Rc<List_::Node_1<b_>>>,
                         zs: &Option<Rc<List_::Node_1<c_>>>)
     -> Option<Rc<List_::Node_1<(a_, b_, c_)>>> {
        List_::map3(&Rc::from(move |x: &a_, y: &b_, z: &c_|
                                  (x.clone(), y.clone(), z.clone())), xs, ys,
                    zs)
    }
    pub fn sortWith<T: Clone +
                    'static>(comparer:
                                 &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                             xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        let arr: Rc<MutCell<Vec<T>>> = List_::toArray(xs);
        Array_::sortInPlaceWith(comparer, &arr);
        List_::ofArray(&arr)
    }
    pub fn sort<T: PartialOrd + Clone +
                'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::sortWith(&Rc::from(move |e1: &T, e2: &T|
                                      Util::compare(e1, e2)), xs)
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone +
                  'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                           xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::sortWith(&Rc::from({
                                      let projection = projection.clone();
                                      move |x: &T, y: &T|
                                          Util::compare(&projection(x),
                                                        &projection(y))
                                  }), xs)
    }
    pub fn sortDescending<T: PartialOrd + Clone +
                          'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::sortWith(&Rc::from(move |x: &T, y: &T|
                                      Util::compare(x, y) * -1i32), xs)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone +
                            'static>(projection:
                                         &Rc<impl Fn(&T) -> (U) + 'static>,
                                     xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::sortWith(&Rc::from({
                                      let projection = projection.clone();
                                      move |x: &T, y: &T|
                                          Util::compare(&projection(x),
                                                        &projection(y)) *
                                              -1i32
                                  }), xs)
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone +
               'static>(xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        List_::fold(&Rc::from(move |acc: &T, x: &T| acc.clone() + x.clone()),
                    &Native::getZero::<T>(), xs)
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default +
                 Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Option<Rc<List_::Node_1<T>>>) -> U {
        List_::fold(&Rc::from({
                                  let projection = projection.clone();
                                  move |acc: &U, x: &T|
                                      acc.clone() + projection(x)
                              }), &Native::getZero::<U>(), xs)
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        List_::reduce(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        if projection(x) > projection(y) {
                                            x.clone()
                                        } else { y.clone() }
                                }), xs)
    }
    pub fn max<T: PartialOrd + Clone +
               'static>(xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        List_::reduce(&Rc::from(move |x: &T, y: &T|
                                    if x.clone() > y.clone() {
                                        x.clone()
                                    } else { y.clone() }), xs)
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        List_::reduce(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        if projection(x) < projection(y) {
                                            x.clone()
                                        } else { y.clone() }
                                }), xs)
    }
    pub fn min<T: PartialOrd + Clone +
               'static>(xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        List_::reduce(&Rc::from(move |x: &T, y: &T|
                                    if x.clone() < y.clone() {
                                        x.clone()
                                    } else { y.clone() }), xs)
    }
    pub fn average<T: core::ops::Add<Output = T> +
                   core::ops::Div<Output = T> + From<i32> + Default + Clone +
                   'static>(xs: &Option<Rc<List_::Node_1<T>>>) -> T {
        if List_::isEmpty(xs) {
            panic!("{}", List_::SR::inputListWasEmpty());
        }
        {
            let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            List_::fold(&Rc::from({
                                      let count = count.clone();
                                      move |acc: &T, x: &T|
                                          {
                                              count.set(count.get() + 1i32);
                                              acc.clone() + x.clone()
                                          }
                                  }), &Native::getZero::<T>(), xs) /
                T::from(count.get())
        }
    }
    pub fn averageBy<T: Clone + 'static, U: core::ops::Add<Output = U> +
                     core::ops::Div<Output = U> + From<i32> + Default +
                     Clone +
                     'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                              xs: &Option<Rc<List_::Node_1<T>>>) -> U {
        if List_::isEmpty(xs) {
            panic!("{}", List_::SR::inputListWasEmpty());
        }
        {
            let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            List_::fold(&Rc::from({
                                      let count = count.clone();
                                      let projection = projection.clone();
                                      move |acc: &U, x: &T|
                                          {
                                              count.set(count.get() + 1i32);
                                              acc.clone() + projection(x)
                                          }
                                  }), &Native::getZero::<U>(), xs) /
                U::from(count.get())
        }
    }
    pub fn permute<T: Clone +
                   'static>(indexMap: &Rc<impl Fn(&i32) -> (i32) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::ofArray(&Array_::permute(indexMap, &List_::toArray(xs)))
    }
    pub fn chunkBySize<T: Clone +
                       'static>(chunkSize: &i32,
                                xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>> {
        List_::ofArray(&Array_::map(&Rc::from(move
                                                  |xs_2: &Rc<MutCell<Vec<T>>>|
                                                  List_::ofArray(xs_2)),
                                    &Array_::chunkBySize(chunkSize,
                                                         &List_::toArray(xs))))
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone +
                    'static>(xs: &Option<Rc<List_::Node_1<T1>>>,
                             ys: &Option<Rc<List_::Node_1<T2>>>)
     -> Option<Rc<List_::Node_1<(T1, T2)>>> {
        let root: Rc<MutCell<Option<Rc<List_::Node_1<(T1, T2)>>>>> =
            Rc::from(MutCell::from(List_::empty()));
        let node: Rc<MutCell<Option<Rc<List_::Node_1<(T1, T2)>>>>> =
            Rc::from(MutCell::from(root.get().clone()));
        List_::iterate(&Rc::from({
                                     let node = node.clone();
                                     let root = root.clone();
                                     let ys = ys.clone();
                                     move |x: &T1|
                                         List_::iterate(&Rc::from({
                                                                      let node
                                                                          =
                                                                          node.clone();
                                                                      let root
                                                                          =
                                                                          root.clone();
                                                                      let x =
                                                                          x.clone();
                                                                      move
                                                                          |y:
                                                                               &T2|
                                                                          {
                                                                              node.set(List_::appendConsNoTail(&(x.clone(),
                                                                                                                 y.clone()),
                                                                                                               &node.get()));
                                                                              if List_::isEmpty(&root.get())
                                                                                 {
                                                                                  root.set(node.get().clone());
                                                                              }
                                                                          }
                                                                  }), &ys)
                                 }), xs);
        root.get().clone()
    }
    pub fn scan<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<State>>> {
        List_::ofArray(&Array_::scan(folder, state, &List_::toArray(xs)))
    }
    pub fn scanBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>,
                             xs: &Option<Rc<List_::Node_1<T>>>, state: &State)
     -> Option<Rc<List_::Node_1<State>>> {
        List_::ofArray(&Array_::scanBack(folder, &List_::toArray(xs), state))
    }
    pub fn skip<T: Clone +
                'static>(count: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(count.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        '_skip:
            loop  {
                break '_skip
                    (if count.get() <= 0i32 {
                         xs.get()
                     } else {
                         if List_::isEmpty(&xs.get()) {
                             panic!("{}",
                                    Rc::from((Rc::from(List_::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list")) as Rc<str>);
                         }
                         {
                             let count_temp: i32 = count.get() - 1i32;
                             let xs_temp: Option<Rc<List_::Node_1<T>>> =
                                 List_::tail(&xs.get());
                             count.set(count_temp);
                             xs.set(xs_temp);
                             continue '_skip
                         }
                     }) ;
            }
    }
    pub fn skipWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        let predicate = Rc::from(MutCell::from(predicate.clone()));
        let xs: Rc<MutCell<Option<Rc<List_::Node_1<T>>>>> =
            Rc::from(MutCell::from(xs.clone()));
        '_skipWhile:
            loop  {
                break '_skipWhile
                    (if List_::isEmpty(&xs.get()) {
                         xs.get()
                     } else {
                         if !predicate.get()(&List_::head(&xs.get())) {
                             xs.get()
                         } else {
                             let predicate_temp = predicate.get();
                             let xs_temp: Option<Rc<List_::Node_1<T>>> =
                                 List_::tail(&xs.get());
                             predicate.set(predicate_temp);
                             xs.set(xs_temp);
                             continue '_skipWhile
                         }
                     }) ;
            }
    }
    pub fn take<T: Clone +
                'static>(count: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        if count.clone() < 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::inputMustBeNonNegative().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"count")) as Rc<str>);
        }
        List_::unfold(&Rc::from(move
                                    |tupledArg:
                                         &(i32, Option<Rc<List_::Node_1<T>>>)|
                                    {
                                        let i: i32 = tupledArg.0.clone();
                                        let xs_1:
                                                Option<Rc<List_::Node_1<T>>> =
                                            tupledArg.1.clone();
                                        if i > 0i32 {
                                            if List_::isEmpty(&xs_1) {
                                                panic!("{}",
                                                       Rc::from((Rc::from(List_::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list")) as Rc<str>);
                                            }
                                            Some((List_::head(&xs_1),
                                                  (i - 1i32,
                                                   List_::tail(&xs_1))))
                                        } else {
                                            None::<(T,
                                                    (i32,
                                                     Option<Rc<List_::Node_1<T>>>))>
                                        }
                                    }), &(count.clone(), xs.clone()))
    }
    pub fn takeWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::unfold(&Rc::from({
                                    let predicate = predicate.clone();
                                    move |xs_1: &Option<Rc<List_::Node_1<T>>>|
                                        if if !List_::isEmpty(xs_1) {
                                               predicate(&List_::head(xs_1))
                                           } else { false } {
                                            Some((List_::head(xs_1),
                                                  List_::tail(xs_1)))
                                        } else {
                                            None::<(T,
                                                    Option<Rc<List_::Node_1<T>>>)>
                                        }
                                }), xs)
    }
    pub fn truncate<T: Clone +
                    'static>(count: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::unfold(&Rc::from(move
                                    |tupledArg:
                                         &(i32, Option<Rc<List_::Node_1<T>>>)|
                                    {
                                        let i: i32 = tupledArg.0.clone();
                                        let xs_1:
                                                Option<Rc<List_::Node_1<T>>> =
                                            tupledArg.1.clone();
                                        if if i > 0i32 {
                                               !List_::isEmpty(&xs_1)
                                           } else { false } {
                                            Some((List_::head(&xs_1),
                                                  (i - 1i32,
                                                   List_::tail(&xs_1))))
                                        } else {
                                            None::<(T,
                                                    (i32,
                                                     Option<Rc<List_::Node_1<T>>>))>
                                        }
                                    }), &(count.clone(), xs.clone()))
    }
    pub fn splitAt<T: Clone +
                   'static>(index: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> (Option<Rc<List_::Node_1<T>>>, Option<Rc<List_::Node_1<T>>>) {
        if index.clone() < 0i32 {
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::inputMustBeNonNegative().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>);
        }
        if index.clone() > List_::length(xs) {
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>);
        }
        (List_::take(index, xs), List_::skip(index, xs))
    }
    pub fn exactlyOne<T: Clone + 'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> T {
        if List_::isEmpty(xs) {
            panic!("{}",
                   Rc::from((Rc::from(List_::SR::inputSequenceEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list")) as Rc<str>)
        } else {
            if List_::isEmpty(&List_::tail(xs)) {
                List_::head(xs)
            } else {
                panic!("{}",
                       Rc::from((Rc::from(List_::SR::inputSequenceTooLong().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"list")) as Rc<str>)
            }
        }
    }
    pub fn tryExactlyOne<T: Clone +
                         'static>(xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<T> {
        if if !List_::isEmpty(xs) {
               List_::isEmpty(&List_::tail(xs))
           } else { false } {
            Some(List_::head(xs))
        } else { None::<T> }
    }
    pub fn r#where<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<T>>> {
        List_::filter(predicate, xs)
    }
    pub fn windowed<T: Clone +
                    'static>(windowSize: &i32,
                             xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>> {
        List_::ofArray(&Array_::map(&Rc::from(move
                                                  |xs_2: &Rc<MutCell<Vec<T>>>|
                                                  List_::ofArray(xs_2)),
                                    &Array_::windowed(windowSize,
                                                      &List_::toArray(xs))))
    }
    pub fn splitInto<T: Clone +
                     'static>(chunks: &i32, xs: &Option<Rc<List_::Node_1<T>>>)
     -> Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>> {
        List_::ofArray(&Array_::map(&Rc::from(move
                                                  |xs_2: &Rc<MutCell<Vec<T>>>|
                                                  List_::ofArray(xs_2)),
                                    &Array_::splitInto(chunks,
                                                       &List_::toArray(xs))))
    }
    pub fn transpose<T: Clone +
                     'static>(lists:
                                  &Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>>)
     -> Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>> {
        if List_::isEmpty(lists) {
            List_::empty()
        } else {
            let roots:
                    Option<Rc<List_::Node_1<Option<Rc<List_::Node_1<T>>>>>> =
                List_::map(&Rc::from(move |x: &T| List_::singleton(x)),
                           &List_::head(lists));
            let nodes: Rc<MutCell<Vec<Option<Rc<List_::Node_1<T>>>>>> =
                List_::toArray(&roots);
            List_::iterate(&Rc::from({
                                         let nodes = nodes.clone();
                                         move
                                             |xs_2:
                                                  &Option<Rc<List_::Node_1<T>>>|
                                             List_::iterateIndexed(&Rc::from({
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
                                                                                                    Rc::from((Rc::from(List_::SR::differentLengths().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"lists")) as Rc<str>);
                                                                                         }
                                                                                         nodes.get_mut()[i.clone()
                                                                                                             as
                                                                                                             usize]
                                                                                             =
                                                                                             List_::appendConsNoTail(x_1,
                                                                                                                     &nodes[i.clone()].clone())
                                                                                     }
                                                                             }),
                                                                   xs_2)
                                     }), &List_::tail(lists));
            roots.clone()
        }
    }
}
