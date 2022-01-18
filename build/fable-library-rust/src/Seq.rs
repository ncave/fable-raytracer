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
use crate::import_df4a7900::*;
use crate::import_ec6ee4e9::*;
use crate::import_c6216f2::*;
use crate::import_f222008f::*;
use std::rc::Rc;
pub mod Seq {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn enumerationAlreadyFinished() -> Rc<str> {
            Native::string(&"Enumeration already finished.")
        }
        pub fn enumerationNotStarted() -> Rc<str> {
            Native::string(&"Enumeration has not started. Call MoveNext.")
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
        pub fn notEnoughElements() -> Rc<str> {
            Native::string(&"The input sequence has an insufficient number of elements.")
        }
        pub fn resetNotSupported() -> Rc<str> {
            Native::string(&"Reset is not supported on this enumerator.")
        }
    }
    pub fn indexNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", Seq::SR::keyNotFoundAlt())
    }
    pub mod Enumerable {
        use super::*;
        pub fn noReset<a_: Clone + 'static>() -> a_ {
            panic!("{}", Seq::SR::resetNotSupported())
        }
        pub fn notStarted<a_: Clone + 'static>() -> a_ {
            panic!("{}", Seq::SR::enumerationNotStarted())
        }
        pub fn alreadyFinished<a_: Clone + 'static>() -> a_ {
            panic!("{}", Seq::SR::enumerationAlreadyFinished())
        }
        pub trait IEnumerator_1<T: Clone + 'static> {
            fn MoveNext(&self)
            -> bool;
            fn Current(&self)
            -> T;
            fn Dispose(&self);
        }
        pub trait IEnumerable_1<T: Clone + 'static> {
            fn GetEnumerator(&self)
            -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>>;
        }
        #[derive(Clone)]
        pub struct Enumerable_1<T: Clone + 'static> {
            f: Rc<dyn Fn() -> (Rc<dyn Seq::Enumerable::IEnumerator_1<T>>) +
                  'static>,
        }
        impl <T: Clone + 'static> Seq::Enumerable::Enumerable_1<T> {
            pub fn new(f:
                           &Rc<impl Fn()
                               ->
                                   (Rc<dyn Seq::Enumerable::IEnumerator_1<T>>) +
                               'static>)
             -> Rc<Seq::Enumerable::Enumerable_1<T>> {
                {
                    let f_1;
                    ();
                    f_1 = f.clone();
                    ();
                    Rc::from(Seq::Enumerable::Enumerable_1::<T>{f:
                                                                    f_1.clone(),})
                }.clone()
            }
        }
        impl <T: Clone + 'static> Seq::Enumerable::IEnumerable_1<T> for
         Enumerable_1<T> {
            fn GetEnumerator(&self)
             -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
                (self.f)()
            }
        }
        #[derive(Clone)]
        pub struct Enumerator<T: Clone + 'static> {
            next: Rc<dyn Fn() -> (Option<T>) + 'static>,
            curr: MutCell<Option<T>>,
        }
        impl <T: Clone + 'static> Seq::Enumerable::Enumerator<T> {
            pub fn new(next: &Rc<impl Fn() -> (Option<T>) + 'static>)
             -> Rc<Seq::Enumerable::Enumerator<T>> {
                {
                    let next_1;
                    let curr: Option<T>;
                    ();
                    next_1 = next.clone();
                    curr = None::<T>;
                    ();
                    Rc::from(Seq::Enumerable::Enumerator::<T>{next:
                                                                  next_1.clone(),
                                                              curr:
                                                                  MutCell::from(curr.clone()),})
                }.clone()
            }
        }
        impl <T: Clone + 'static> Seq::Enumerable::IEnumerator_1<T> for
         Enumerator<T> {
            fn Current(&self) -> T {
                Option_::getValue(&self.curr.get()).clone()
            }
            fn MoveNext(&self) -> bool {
                self.curr.set((self.next)());
                self.curr.get().is_some()
            }
            fn Dispose(&self) { (); }
        }
        pub fn fromFunction<T: Clone +
                            'static>(next:
                                         &Rc<impl Fn() -> (Option<T>) +
                                             'static>)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            (Seq::Enumerable::Enumerator::new(next).clone() as
                 Rc<dyn Seq::Enumerable::IEnumerator_1<T>>).clone()
        }
        pub fn empty<T: Clone + 'static>()
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            Seq::Enumerable::fromFunction(&Rc::from(move || None::<T>))
        }
        pub fn singleton<T: Clone + 'static>(x: &T)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            {
                let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let i = i.clone();
                                                            let x = x.clone();
                                                            move ||
                                                                if i.get() <
                                                                       1i32 {
                                                                    {
                                                                        i.set(i.get()
                                                                                  +
                                                                                  1i32);
                                                                        Some(x.clone())
                                                                    }.clone()
                                                                } else {
                                                                    None::<T>
                                                                }
                                                        }))
            }.clone()
        }
        pub fn ofArray<T: Clone + 'static>(arr: &Rc<MutCell<Vec<T>>>)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            {
                let len: i32 = arr.clone().len() as i32;
                let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let arr =
                                                                arr.clone();
                                                            let i = i.clone();
                                                            move ||
                                                                if i.get() <
                                                                       len {
                                                                    {
                                                                        i.set(i.get()
                                                                                  +
                                                                                  1i32);
                                                                        Some(arr[i.get()
                                                                                     -
                                                                                     1i32].clone())
                                                                    }.clone()
                                                                } else {
                                                                    None::<T>
                                                                }
                                                        }))
            }.clone()
        }
        pub fn ofList<T: Clone + 'static>(xs: &List_1<T>)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            {
                let it: Rc<MutCell<List_1<T>>> =
                    Rc::from(MutCell::from(xs.clone()));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let it =
                                                                it.clone();
                                                            move ||
                                                                if !List::isEmpty(&it.get())
                                                                   {
                                                                    {
                                                                        let tail_1:
                                                                                List_1<T> =
                                                                            List::tail(&it.get()).clone();
                                                                        let head_1:
                                                                                T =
                                                                            List::head(&it.get()).clone();
                                                                        it.set(tail_1);
                                                                        Some(head_1)
                                                                    }.clone()
                                                                } else {
                                                                    None::<T>
                                                                }
                                                        }))
            }.clone()
        }
        pub fn append<T: Clone +
                      'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>,
                               ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            {
                let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(-1i32));
                let innerOpt:
                        Rc<MutCell<Option<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>>> =
                    Rc::from(MutCell::from(None::<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>));
                let finished: Rc<MutCell<bool>> =
                    Rc::from(MutCell::from(false));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let finished =
                                                                finished.clone();
                                                            let i = i.clone();
                                                            let innerOpt =
                                                                innerOpt.clone();
                                                            let xs =
                                                                xs.clone();
                                                            let ys =
                                                                ys.clone();
                                                            move ||
                                                                {
                                                                    let moveNext:
                                                                            Rc<MutCell<bool>> =
                                                                        Rc::from(MutCell::from(false));
                                                                    while if !finished.get()
                                                                             {
                                                                              !moveNext.get()
                                                                          } else {
                                                                              false
                                                                          } {
                                                                        match &innerOpt.get()
                                                                            {
                                                                            None
                                                                            =>
                                                                            if i.get()
                                                                                   <
                                                                                   1i32
                                                                               {
                                                                                i.set(i.get()
                                                                                          +
                                                                                          1i32);
                                                                                {
                                                                                    let it:
                                                                                            Rc<dyn Seq::Enumerable::IEnumerable_1<T>> =
                                                                                        if i.get()
                                                                                               ==
                                                                                               0i32
                                                                                           {
                                                                                            xs.clone()
                                                                                        } else {
                                                                                            ys.clone()
                                                                                        };
                                                                                    innerOpt.set(Some(it.GetEnumerator()))
                                                                                }
                                                                            } else {
                                                                                finished.set(true)
                                                                            },
                                                                            Some(innerOpt_0_0)
                                                                            =>
                                                                            {
                                                                                let inner:
                                                                                        Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                                                                                    innerOpt_0_0.clone();
                                                                                if inner.MoveNext()
                                                                                   {
                                                                                    moveNext.set(true)
                                                                                } else {
                                                                                    innerOpt.set(None::<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>)
                                                                                }
                                                                            }
                                                                        };
                                                                    }
                                                                    if if !finished.get()
                                                                          {
                                                                           moveNext.get()
                                                                       } else {
                                                                           false
                                                                       } {
                                                                        Some(Option_::getValue(&innerOpt.get()).Current().clone())
                                                                    } else {
                                                                        None::<T>
                                                                    }
                                                                }.clone()
                                                        }))
            }.clone()
        }
        pub fn concat<T: Clone +
                      'static>(sources:
                                   &Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<dyn Seq::Enumerable::IEnumerable_1<T>>>>)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            {
                let outerOpt:
                        Rc<MutCell<Option<Rc<dyn Seq::Enumerable::IEnumerator_1<Rc<dyn Seq::Enumerable::IEnumerable_1<T>>>>>>> =
                    Rc::from(MutCell::from(None::<Rc<dyn Seq::Enumerable::IEnumerator_1<Rc<dyn Seq::Enumerable::IEnumerable_1<T>>>>>));
                let innerOpt:
                        Rc<MutCell<Option<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>>> =
                    Rc::from(MutCell::from(None::<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>));
                let finished: Rc<MutCell<bool>> =
                    Rc::from(MutCell::from(false));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let finished =
                                                                finished.clone();
                                                            let innerOpt =
                                                                innerOpt.clone();
                                                            let outerOpt =
                                                                outerOpt.clone();
                                                            let sources =
                                                                sources.clone();
                                                            move ||
                                                                {
                                                                    let moveNext:
                                                                            Rc<MutCell<bool>> =
                                                                        Rc::from(MutCell::from(false));
                                                                    while if !finished.get()
                                                                             {
                                                                              !moveNext.get()
                                                                          } else {
                                                                              false
                                                                          } {
                                                                        match &outerOpt.get()
                                                                            {
                                                                            None
                                                                            =>
                                                                            outerOpt.set(Some(sources.GetEnumerator())),
                                                                            Some(outerOpt_0_0)
                                                                            =>
                                                                            {
                                                                                let outer:
                                                                                        Rc<dyn Seq::Enumerable::IEnumerator_1<Rc<dyn Seq::Enumerable::IEnumerable_1<T>>>> =
                                                                                    outerOpt_0_0.clone();
                                                                                match &innerOpt.get()
                                                                                    {
                                                                                    None
                                                                                    =>
                                                                                    if outer.MoveNext()
                                                                                       {
                                                                                        let it:
                                                                                                Rc<dyn Seq::Enumerable::IEnumerable_1<T>> =
                                                                                            outer.Current().clone();
                                                                                        innerOpt.set(Some(it.GetEnumerator()))
                                                                                    } else {
                                                                                        finished.set(true)
                                                                                    },
                                                                                    Some(innerOpt_0_0)
                                                                                    =>
                                                                                    {
                                                                                        let inner:
                                                                                                Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                                                                                            innerOpt_0_0.clone();
                                                                                        if inner.MoveNext()
                                                                                           {
                                                                                            moveNext.set(true)
                                                                                        } else {
                                                                                            innerOpt.set(None::<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        };
                                                                    }
                                                                    if if !finished.get()
                                                                          {
                                                                           moveNext.get()
                                                                       } else {
                                                                           false
                                                                       } {
                                                                        Some(Option_::getValue(&innerOpt.get()).Current().clone())
                                                                    } else {
                                                                        None::<T>
                                                                    }
                                                                }.clone()
                                                        }))
            }.clone()
        }
        pub fn generateWhileSome<T: Clone + 'static, U: Clone +
                                 'static>(openf:
                                              &Rc<impl Fn() -> (T) + 'static>,
                                          compute:
                                              &Rc<impl Fn(&T) -> (Option<U>) +
                                                  'static>,
                                          closef: &Rc<impl Fn(&T) + 'static>)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<U>> {
            {
                let finished: Rc<MutCell<bool>> =
                    Rc::from(MutCell::from(false));
                let state: Rc<MutCell<Option<T>>> =
                    Rc::from(MutCell::from(None::<T>));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let closef =
                                                                closef.clone();
                                                            let compute =
                                                                compute.clone();
                                                            let finished =
                                                                finished.clone();
                                                            let openf =
                                                                openf.clone();
                                                            let state =
                                                                state.clone();
                                                            move ||
                                                                if finished.get()
                                                                   {
                                                                    None::<U>
                                                                } else {
                                                                    {
                                                                        if state.get().is_none()
                                                                           {
                                                                            state.set(Some(openf()));
                                                                        }
                                                                        {
                                                                            let res:
                                                                                    Option<U> =
                                                                                compute(&Option_::getValue(&state.get()));
                                                                            if res.is_none()
                                                                               {
                                                                                finished.set(true);
                                                                                match &state.get()
                                                                                    {
                                                                                    Some(state_0_0)
                                                                                    =>
                                                                                    {
                                                                                        let x:
                                                                                                T =
                                                                                            state_0_0.clone();
                                                                                        {
                                                                                            let try_result =
                                                                                                closef(&x);
                                                                                            state.set(None::<T>);
                                                                                            try_result
                                                                                        }
                                                                                    }
                                                                                    _
                                                                                    =>
                                                                                    (),
                                                                                }
                                                                            }
                                                                            res.clone()
                                                                        }.clone()
                                                                    }.clone()
                                                                }
                                                        }))
            }.clone()
        }
        pub fn unfold<State: Clone + 'static, T: Clone +
                      'static>(f:
                                   &Rc<impl Fn(&State)
                                       -> (Option<Rc<(T, State)>>) + 'static>,
                               state: &State)
         -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
            {
                let acc: Rc<MutCell<State>> =
                    Rc::from(MutCell::from(state.clone()));
                Seq::Enumerable::fromFunction(&Rc::from({
                                                            let acc =
                                                                acc.clone();
                                                            let f = f.clone();
                                                            move ||
                                                                {
                                                                    let matchValue:
                                                                            Option<Rc<(T,
                                                                                       State)>> =
                                                                        f(&acc.get());
                                                                    match &matchValue
                                                                        {
                                                                        None
                                                                        =>
                                                                        None::<T>,
                                                                        Some(matchValue_0_0)
                                                                        =>
                                                                        {
                                                                            let x:
                                                                                    T =
                                                                                matchValue_0_0.0.clone();
                                                                            let st:
                                                                                    State =
                                                                                matchValue_0_0.1.clone();
                                                                            acc.set(st);
                                                                            Some(x)
                                                                        }.clone(),
                                                                    }
                                                                }.clone()
                                                        }))
            }.clone()
        }
    }
    pub fn mkSeq<T: Clone +
                 'static>(f:
                              &Rc<impl Fn()
                                  ->
                                      (Rc<dyn Seq::Enumerable::IEnumerator_1<T>>) +
                                  'static>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        (Seq::Enumerable::Enumerable_1::new(f).clone() as
             Rc<dyn Seq::Enumerable::IEnumerable_1<T>>).clone()
    }
    pub fn ofSeq<T: Clone +
                 'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerator_1<T>> {
        xs.clone().GetEnumerator()
    }
    pub fn delay<T: Clone +
                 'static>(generator:
                              &Rc<impl Fn()
                                  ->
                                      (Rc<dyn Seq::Enumerable::IEnumerable_1<T>>) +
                                  'static>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let generator = generator.clone();
                                 move || generator().GetEnumerator()
                             }))
    }
    pub fn concat<T: Clone +
                  'static>(sources:
                               &Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<dyn Seq::Enumerable::IEnumerable_1<T>>>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let sources = sources.clone();
                                 move || Seq::Enumerable::concat(&sources)
                             }))
    }
    pub fn unfold<State: Clone + 'static, T: Clone +
                  'static>(generator:
                               &Rc<impl Fn(&State)
                                   -> (Option<Rc<(T, State)>>) + 'static>,
                           state: &State)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let generator = generator.clone();
                                 let state = state.clone();
                                 move ||
                                     Seq::Enumerable::unfold(&generator,
                                                             &state)
                             }))
    }
    pub fn empty<a_: Clone + 'static>()
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<a_>> {
        Seq::mkSeq(&Rc::from(move || Seq::Enumerable::empty()))
    }
    pub fn singleton<T: Clone + 'static>(x: &T)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let x = x.clone();
                                 move || Seq::Enumerable::singleton(&x)
                             }))
    }
    pub fn ofArray<T: Clone + 'static>(arr: &Rc<MutCell<Vec<T>>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let arr = arr.clone();
                                 move || Seq::Enumerable::ofArray(&arr)
                             }))
    }
    pub fn ofList<T: Clone + 'static>(xs: &List_1<T>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let xs = xs.clone();
                                 move || Seq::Enumerable::ofList(&xs)
                             }))
    }
    pub fn generate<a_: Clone + 'static, b_: Clone +
                    'static>(create: &Rc<impl Fn() -> (a_) + 'static>,
                             compute:
                                 &Rc<impl Fn(&a_) -> (Option<b_>) + 'static>,
                             dispose: &Rc<impl Fn(&a_) + 'static>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<b_>> {
        Seq::mkSeq(&Rc::from({
                                 let compute = compute.clone();
                                 let create = create.clone();
                                 let dispose = dispose.clone();
                                 move ||
                                     Seq::Enumerable::generateWhileSome(&create,
                                                                        &compute,
                                                                        &dispose)
                             }))
    }
    pub fn generateIndexed<a_: Clone + 'static, b_: Clone +
                           'static>(create: &Rc<impl Fn() -> (a_) + 'static>,
                                    compute:
                                        &Rc<impl Fn(&i32, &a_)
                                            -> (Option<b_>) + 'static>,
                                    dispose: &Rc<impl Fn(&a_) + 'static>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<b_>> {
        Seq::mkSeq(&Rc::from({
                                 let compute = compute.clone();
                                 let create = create.clone();
                                 let dispose = dispose.clone();
                                 move ||
                                     {
                                         let i: Rc<MutCell<i32>> =
                                             Rc::from(MutCell::from(-1i32));
                                         Seq::Enumerable::generateWhileSome(&create,
                                                                            &Rc::from({
                                                                                          let compute
                                                                                              =
                                                                                              compute.clone();
                                                                                          let i
                                                                                              =
                                                                                              i.clone();
                                                                                          move
                                                                                              |x:
                                                                                                   &a_|
                                                                                              {
                                                                                                  i.set(i.get()
                                                                                                            +
                                                                                                            1i32);
                                                                                                  compute(&i.get(),
                                                                                                          x)
                                                                                              }.clone()
                                                                                      }),
                                                                            &dispose)
                                     }.clone()
                             }))
    }
    pub fn append<T: Clone +
                  'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>,
                           ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let xs = xs.clone();
                                 let ys = ys.clone();
                                 move || Seq::Enumerable::append(&xs, &ys)
                             }))
    }
    pub fn choose<T: Clone + 'static, U: Clone +
                  'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                           xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    move || Seq::ofSeq(&xs)
                                }),
                      &Rc::from({
                                    let chooser = chooser.clone();
                                    move
                                        |e:
                                             &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                        {
                                            let curr: Rc<MutCell<Option<U>>> =
                                                Rc::from(MutCell::from(None::<U>));
                                            while if curr.get().is_none() {
                                                      e.clone().MoveNext()
                                                  } else { false } {
                                                curr.set(chooser(&e.Current()));
                                            }
                                            curr.get().clone()
                                        }.clone()
                                }),
                      &Rc::from(move
                                    |e_1:
                                         &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                    e_1.clone().Dispose()))
    }
    pub fn compareWith<T: Clone +
                       'static>(comparer:
                                    &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                                xs:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>,
                                ys:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> i32 {
        let e1: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let e2: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                        Seq::ofSeq(ys);
                    {
                        let try_result_1 =
                            {
                                let c: Rc<MutCell<i32>> =
                                    Rc::from(MutCell::from(0i32));
                                let b1: Rc<MutCell<bool>> =
                                    Rc::from(MutCell::from(e1.MoveNext()));
                                let b2: Rc<MutCell<bool>> =
                                    Rc::from(MutCell::from(e2.MoveNext()));
                                while if if c.get() == 0i32 {
                                             b1.get()
                                         } else { false } {
                                          b2.get()
                                      } else { false } {
                                    c.set(comparer(&e1.Current(),
                                                   &e2.Current()));
                                    if c.get() == 0i32 {
                                        b1.set(e1.MoveNext());
                                        b2.set(e2.MoveNext())
                                    }
                                }
                                if c.get() != 0i32 {
                                    c.get()
                                } else {
                                    if b1.get() {
                                        1i32
                                    } else {
                                        if b2.get() { -1i32 } else { 0i32 }
                                    }
                                }
                            };
                        if false { e2.Dispose(); }
                        try_result_1
                    }
                };
            if false { e1.Dispose(); }
            try_result
        }
    }
    pub fn exactlyOne<T: Clone +
                      'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    if e.MoveNext() {
                        {
                            let v: T = e.Current().clone();
                            if e.MoveNext() {
                                panic!("{}",
                                       Rc::from((Rc::from(Seq::SR::inputSequenceTooLong().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>)
                            } else { v }
                        }.clone()
                    } else {
                        panic!("{}",
                               Rc::from((Rc::from(Seq::SR::inputSequenceEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>)
                    };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn tryExactlyOne<T: Clone +
                         'static>(xs:
                                      &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<T> {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    if e.MoveNext() {
                        {
                            let v: T = e.Current().clone();
                            if e.MoveNext() { None::<T> } else { Some(v) }
                        }.clone()
                    } else { None::<T> };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> bool {
        let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let found: Rc<MutCell<bool>> =
                        Rc::from(MutCell::from(false));
                    while if !found.get() { e.MoveNext() } else { false } {
                        found.set(predicate(&e.Current()));
                    }
                    found.get()
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                            ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> bool {
        let e1: Rc<dyn Seq::Enumerable::IEnumerator_1<T1>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let e2: Rc<dyn Seq::Enumerable::IEnumerator_1<T2>> =
                        Seq::ofSeq(ys);
                    {
                        let try_result_1 =
                            {
                                let found: Rc<MutCell<bool>> =
                                    Rc::from(MutCell::from(false));
                                while if if !found.get() {
                                             e1.MoveNext()
                                         } else { false } {
                                          e2.MoveNext()
                                      } else { false } {
                                    found.set(predicate(&e1.Current(),
                                                        &e2.Current()));
                                }
                                found.get()
                            };
                        if false { e2.Dispose(); }
                        try_result_1
                    }
                };
            if false { e1.Dispose(); }
            try_result
        }
    }
    pub fn contains<T: PartialEq + Clone +
                    'static>(value: &T,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> bool {
        Seq::exists(&Rc::from({
                                  let value = value.clone();
                                  move |x: &T| x.clone() == value.clone()
                              }), xs)
    }
    pub fn filter<T: Clone +
                  'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::choose(&Rc::from({
                                  let f = f.clone();
                                  move |x: &T|
                                      if f(x) {
                                          Some(x.clone())
                                      } else { None::<T> }
                              }), xs)
    }
    pub fn tryFind<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<T> {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    {
                        let res: Rc<MutCell<Option<T>>> =
                            Rc::from(MutCell::from(None::<T>));
                        while if res.get().is_none() {
                                  e.MoveNext()
                              } else { false } {
                            let c: T = e.Current().clone();
                            if predicate(&c) { res.set(Some(c.clone())); }
                        }
                        res.get().clone()
                    };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn find<T: Clone +
                'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let matchValue: Option<T> = Seq::tryFind(predicate, xs);
            match &matchValue {
                None => panic!("{}", Seq::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFindIndex<T: Clone +
                        'static>(predicate:
                                     &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 xs:
                                     &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<i32> {
        {
            fn inner_loop<T: Clone +
                          'static>(i: &i32,
                                   predicate_1:
                                       &Rc<impl Fn(&T) -> (bool) + 'static>,
                                   e:
                                       &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>)
             -> Option<i32> {
                if e.clone().MoveNext() {
                    if predicate_1(&e.Current()) {
                        Some(i.clone())
                    } else { inner_loop(&(i.clone() + 1i32), predicate_1, e) }
                } else { None::<i32> }
            }
            let e_1: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                Seq::ofSeq(xs);
            {
                let try_result = inner_loop(&0i32, predicate, &e_1);
                if false { e_1.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn findIndex<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> i32 {
        let matchValue: Option<i32> = Seq::tryFindIndex(predicate, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn fold<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> State {
        {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            {
                let enumerator: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                    xs.clone().GetEnumerator();
                {
                    let try_result =
                        while enumerator.MoveNext() {
                            let x: T = enumerator.Current().clone();
                            acc.set(folder(&acc.get(), &x))
                        };
                    if false { enumerator.Dispose(); }
                    try_result
                }
            }
            acc.get().clone()
        }.clone()
    }
    pub fn toArray<T: Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<MutCell<Vec<T>>> {
        {
            let res: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
            {
                let enumerator: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                    xs.clone().GetEnumerator();
                {
                    let try_result =
                        while enumerator.MoveNext() {
                            let x: T = enumerator.Current().clone();
                            res.get_mut().push(x)
                        };
                    if false { enumerator.Dispose(); }
                    try_result
                }
            }
            Native::arrayCopy(&res)
        }.clone()
    }
    pub fn toList<T: Clone +
                  'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> List_1<T> {
        {
            let acc: Rc<MutCell<List_1<T>>> =
                Rc::from(MutCell::from(List::empty::<T>()));
            {
                let enumerator: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                    xs.clone().GetEnumerator();
                {
                    let try_result =
                        while enumerator.MoveNext() {
                            let x: T = enumerator.Current().clone();
                            acc.set(List::cons(&x, &acc.get()))
                        };
                    if false { enumerator.Dispose(); }
                    try_result
                }
            }
            List::reverse(&acc.get())
        }.clone()
    }
    pub fn foldBack<T: Clone + 'static, a_: Clone +
                    'static>(folder: &Rc<impl Fn(&T, &a_) -> (a_) + 'static>,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>,
                             state: &a_) -> a_ {
        Array::foldBack(folder, &Seq::toArray(xs), state)
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone +
                 'static>(folder:
                              &Rc<impl Fn(&State, &T1, &T2) -> (State) +
                                  'static>, state: &State,
                          xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                          ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> State {
        {
            let e1: Rc<dyn Seq::Enumerable::IEnumerator_1<T1>> =
                Seq::ofSeq(xs);
            {
                let try_result =
                    {
                        let e2: Rc<dyn Seq::Enumerable::IEnumerator_1<T2>> =
                            Seq::ofSeq(ys);
                        {
                            let try_result_1 =
                                {
                                    let acc: Rc<MutCell<State>> =
                                        Rc::from(MutCell::from(state.clone()));
                                    while if e1.MoveNext() {
                                              e2.MoveNext()
                                          } else { false } {
                                        acc.set(folder(&acc.get(),
                                                       &e1.Current(),
                                                       &e2.Current()));
                                    }
                                    acc.get().clone()
                                };
                            if false { e2.Dispose(); }
                            try_result_1
                        }.clone()
                    };
                if false { e1.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone +
                     'static>(folder:
                                  &Rc<impl Fn(&T1, &T2, &State) -> (State) +
                                      'static>,
                              xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                              ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>,
                              state: &State) -> State {
        Array::foldBack2(folder, &Seq::toArray(xs), &Seq::toArray(ys), state)
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> bool {
        !Seq::exists(&Rc::from({
                                   let predicate = predicate.clone();
                                   move |x: &T| !predicate(x)
                               }), xs)
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                            ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> bool {
        !Seq::exists2(&Rc::from({
                                    let predicate = predicate.clone();
                                    move |x: &T1, y: &T2| !predicate(x, y)
                                }), xs, ys)
    }
    pub fn tryFindBack<T: Clone +
                       'static>(predicate:
                                    &Rc<impl Fn(&T) -> (bool) + 'static>,
                                xs:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<T> {
        Array::tryFindBack(predicate, &Seq::toArray(xs))
    }
    pub fn findBack<T: Clone +
                    'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let matchValue: Option<T> = Seq::tryFindBack(predicate, xs);
            match &matchValue {
                None => panic!("{}", Seq::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn tryFindIndexBack<T: Clone +
                            'static>(predicate:
                                         &Rc<impl Fn(&T) -> (bool) + 'static>,
                                     xs:
                                         &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<i32> {
        Array::tryFindIndexBack(predicate, &Seq::toArray(xs))
    }
    pub fn findIndexBack<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  xs:
                                      &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> i32 {
        let matchValue: Option<i32> = Seq::tryFindIndexBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryHead<T: Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<T> {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    if e.MoveNext() {
                        Some(e.Current().clone())
                    } else { None::<T> };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn head<T: Clone +
                'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let matchValue: Option<T> = Seq::tryHead(xs);
            match &matchValue {
                None =>
                panic!("{}",
                       Rc::from((Rc::from(Seq::SR::inputSequenceEmpty().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn initialize<a_: Clone +
                      'static>(count: &i32,
                               f: &Rc<impl Fn(&i32) -> (a_) + 'static>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<a_>> {
        Seq::unfold(&Rc::from({
                                  let count = count.clone();
                                  let f = f.clone();
                                  move |i: &i32|
                                      if i.clone() < count {
                                          Some(Rc::from((f(i),
                                                         i.clone() + 1i32)))
                                      } else { None::<Rc<(a_, i32)>> }
                              }), &0i32)
    }
    pub fn initializeInfinite<a_: Clone +
                              'static>(f:
                                           &Rc<impl Fn(&i32) -> (a_) +
                                               'static>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<a_>> {
        Seq::initialize(&i32::MAX, f)
    }
    pub fn isEmpty<T: Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> bool {
        let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result = !e.MoveNext();
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn tryItem<T: Clone +
                   'static>(index: &i32,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<T> {
        {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(index.clone()));
            if i.get() < 0i32 {
                None::<T>
            } else {
                {
                    let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                        Seq::ofSeq(xs);
                    {
                        let try_result =
                            {
                                while if i.get() >= 0i32 {
                                          e.MoveNext()
                                      } else { false } {
                                    i.set(i.get() - 1i32);
                                }
                                if i.get() >= 0i32 {
                                    None::<T>
                                } else { Some(e.Current().clone()) }
                            };
                        if false { e.Dispose(); }
                        try_result
                    }.clone()
                }.clone()
            }
        }.clone()
    }
    pub fn item<T: Clone +
                'static>(index: &i32,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let matchValue: Option<T> = Seq::tryItem(index, xs);
            match &matchValue {
                None =>
                panic!("{}",
                       Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"index")) as Rc<str>),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn iterate<T: Clone +
                   'static>(action: &Rc<impl Fn(&T) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>) {
        Seq::fold(&Rc::from({
                                let action = action.clone();
                                move |unitVar0: &(), x: &T| action(x)
                            }), &(), xs);
    }
    pub fn iterate2<T1: Clone + 'static, T2: Clone +
                    'static>(action: &Rc<impl Fn(&T1, &T2) + 'static>,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                             ys:
                                 &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>) {
        Seq::fold2(&Rc::from({
                                 let action = action.clone();
                                 move |unitVar0: &(), x: &T1, y: &T2|
                                     action(x, y)
                             }), &(), xs, ys);
    }
    pub fn iterateIndexed<T: Clone +
                          'static>(action: &Rc<impl Fn(&i32, &T) + 'static>,
                                   xs:
                                       &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>) {
        Util::ignore(&Seq::fold(&Rc::from({
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
                                    xs:
                                        &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                                    ys:
                                        &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>) {
        Util::ignore(&Seq::fold2(&Rc::from({
                                               let action = action.clone();
                                               move |i: &i32, x: &T1, y: &T2|
                                                   {
                                                       action(i, x, y);
                                                       i.clone() + 1i32
                                                   }
                                           }), &0i32, xs, ys));
    }
    pub fn tryLast<T: Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<T> {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    if e.MoveNext() {
                        {
                            let acc: Rc<MutCell<T>> =
                                Rc::from(MutCell::from(e.Current().clone()));
                            while e.MoveNext() {
                                acc.set(e.Current().clone());
                            }
                            Some(acc.get().clone())
                        }.clone()
                    } else { None::<T> };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn last<T: Clone +
                'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let matchValue: Option<T> = Seq::tryLast(xs);
            match &matchValue {
                None =>
                panic!("{}",
                       Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn length<T: Clone +
                  'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> i32 {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    while e.MoveNext() { count.set(count.get() + 1i32); }
                    count.get()
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    move || Seq::ofSeq(&xs)
                                }),
                      &Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |e:
                                             &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                        if e.clone().MoveNext() {
                                            Some(mapping(&e.Current()))
                                        } else { None::<U> }
                                }),
                      &Rc::from(move
                                    |e_1:
                                         &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                    e_1.clone().Dispose()))
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone +
                      'static>(mapping:
                                   &Rc<impl Fn(&i32, &T) -> (U) + 'static>,
                               xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           move || Seq::ofSeq(&xs)
                                       }),
                             &Rc::from({
                                           let mapping = mapping.clone();
                                           move
                                               |i: &i32,
                                                e:
                                                    &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                               if e.clone().MoveNext() {
                                                   Some(mapping(i,
                                                                &e.Current()))
                                               } else { None::<U> }
                                       }),
                             &Rc::from(move
                                           |e_1:
                                                &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                           e_1.clone().Dispose()))
    }
    pub fn indexed<T: Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<(i32, T)>>> {
        Seq::mapIndexed(&Rc::from(move |i: &i32, x: &T|
                                      Rc::from((i.clone(), x.clone()))), xs)
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                         ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    let ys = ys.clone();
                                    move ||
                                        Rc::from((Seq::ofSeq(&xs),
                                                  Seq::ofSeq(&ys)))
                                }),
                      &Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &Rc<(Rc<dyn Seq::Enumerable::IEnumerator_1<T1>>,
                                                  Rc<dyn Seq::Enumerable::IEnumerator_1<T2>>)>|
                                        {
                                            let e1:
                                                    Rc<dyn Seq::Enumerable::IEnumerator_1<T1>> =
                                                tupledArg.0.clone();
                                            let e2:
                                                    Rc<dyn Seq::Enumerable::IEnumerator_1<T2>> =
                                                tupledArg.1.clone();
                                            if if e1.MoveNext() {
                                                   e2.MoveNext()
                                               } else { false } {
                                                Some(mapping(&e1.Current(),
                                                             &e2.Current()))
                                            } else { None::<U> }
                                        }.clone()
                                }),
                      &Rc::from(move
                                    |tupledArg_1:
                                         &Rc<(Rc<dyn Seq::Enumerable::IEnumerator_1<T1>>,
                                              Rc<dyn Seq::Enumerable::IEnumerator_1<T2>>)>|
                                    {
                                        let try_result =
                                            tupledArg_1.0.clone().Dispose();
                                        tupledArg_1.1.clone().Dispose();
                                        try_result
                                    }))
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&i32, &T1, &T2) -> (U) +
                                        'static>,
                                xs:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                                ys:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           let ys = ys.clone();
                                           move ||
                                               Rc::from((Seq::ofSeq(&xs),
                                                         Seq::ofSeq(&ys)))
                                       }),
                             &Rc::from({
                                           let mapping = mapping.clone();
                                           move
                                               |i: &i32,
                                                tupledArg:
                                                    &Rc<(Rc<dyn Seq::Enumerable::IEnumerator_1<T1>>,
                                                         Rc<dyn Seq::Enumerable::IEnumerator_1<T2>>)>|
                                               {
                                                   let e1:
                                                           Rc<dyn Seq::Enumerable::IEnumerator_1<T1>> =
                                                       tupledArg.0.clone();
                                                   let e2:
                                                           Rc<dyn Seq::Enumerable::IEnumerator_1<T2>> =
                                                       tupledArg.1.clone();
                                                   if if e1.MoveNext() {
                                                          e2.MoveNext()
                                                      } else { false } {
                                                       Some(mapping(i,
                                                                    &e1.Current(),
                                                                    &e2.Current()))
                                                   } else { None::<U> }
                                               }.clone()
                                       }),
                             &Rc::from(move
                                           |tupledArg_1:
                                                &Rc<(Rc<dyn Seq::Enumerable::IEnumerator_1<T1>>,
                                                     Rc<dyn Seq::Enumerable::IEnumerator_1<T2>>)>|
                                           {
                                               let try_result =
                                                   tupledArg_1.0.clone().Dispose();
                                               tupledArg_1.1.clone().Dispose();
                                               try_result
                                           }))
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                         ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>,
                         zs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T3>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    let ys = ys.clone();
                                    let zs = zs.clone();
                                    move ||
                                        Rc::from((Seq::ofSeq(&xs),
                                                  Seq::ofSeq(&ys),
                                                  Seq::ofSeq(&zs)))
                                }),
                      &Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &Rc<(Rc<dyn Seq::Enumerable::IEnumerator_1<T1>>,
                                                  Rc<dyn Seq::Enumerable::IEnumerator_1<T2>>,
                                                  Rc<dyn Seq::Enumerable::IEnumerator_1<T3>>)>|
                                        {
                                            let e1:
                                                    Rc<dyn Seq::Enumerable::IEnumerator_1<T1>> =
                                                tupledArg.0.clone();
                                            let e2:
                                                    Rc<dyn Seq::Enumerable::IEnumerator_1<T2>> =
                                                tupledArg.1.clone();
                                            let e3:
                                                    Rc<dyn Seq::Enumerable::IEnumerator_1<T3>> =
                                                tupledArg.2.clone();
                                            if if if e1.MoveNext() {
                                                      e2.MoveNext()
                                                  } else { false } {
                                                   e3.MoveNext()
                                               } else { false } {
                                                Some(mapping(&e1.Current(),
                                                             &e2.Current(),
                                                             &e3.Current()))
                                            } else { None::<U> }
                                        }.clone()
                                }),
                      &Rc::from(move
                                    |tupledArg_1:
                                         &Rc<(Rc<dyn Seq::Enumerable::IEnumerator_1<T1>>,
                                              Rc<dyn Seq::Enumerable::IEnumerator_1<T2>>,
                                              Rc<dyn Seq::Enumerable::IEnumerator_1<T3>>)>|
                                    {
                                        let try_result =
                                            tupledArg_1.0.clone().Dispose();
                                        {
                                            let try_result_1 =
                                                tupledArg_1.1.clone().Dispose();
                                            tupledArg_1.2.clone().Dispose();
                                            try_result_1
                                        }
                                        try_result
                                    }))
    }
    pub fn readOnly<T: Clone +
                    'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::map(&Rc::from(move |x: &T| x.clone()), xs)
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&State, &T) -> (Rc<(U, State)>) +
                                    'static>, state: &State,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<(Rc<dyn Seq::Enumerable::IEnumerable_1<U>>, State)> {
        {
            let patternInput: Rc<(Rc<MutCell<Vec<U>>>, State)> =
                Array::mapFold(mapping, state, &Seq::toArray(xs));
            Rc::from((Seq::readOnly(&Seq::ofArray(&patternInput.0.clone())),
                      patternInput.1.clone()))
        }.clone()
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&T, &State)
                                        -> (Rc<(U, State)>) + 'static>,
                                xs:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>,
                                state: &State)
     -> Rc<(Rc<dyn Seq::Enumerable::IEnumerable_1<U>>, State)> {
        {
            let patternInput: Rc<(Rc<MutCell<Vec<U>>>, State)> =
                Array::mapFoldBack(mapping, &Seq::toArray(xs), state);
            Rc::from((Seq::readOnly(&Seq::ofArray(&patternInput.0.clone())),
                      patternInput.1.clone()))
        }.clone()
    }
    pub fn collect<T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&T)
                                    ->
                                        (Rc<dyn Seq::Enumerable::IEnumerable_1<U>>) +
                                    'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<U>> {
        Seq::delay(&Rc::from({
                                 let mapping = mapping.clone();
                                 let xs = xs.clone();
                                 move || Seq::concat(&Seq::map(&mapping, &xs))
                             }))
    }
    pub fn cache<T: Clone +
                 'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        {
            let prefix: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
            let enumOpt:
                    Rc<MutCell<Option<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>>> =
                Rc::from(MutCell::from(None::<Rc<dyn Seq::Enumerable::IEnumerator_1<T>>>));
            let finished: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
            Seq::unfold(&Rc::from({
                                      let enumOpt = enumOpt.clone();
                                      let finished = finished.clone();
                                      let prefix = prefix.clone();
                                      let xs = xs.clone();
                                      move |i: &i32|
                                          if i.clone() < prefix.len() as i32 {
                                              Some(Rc::from((prefix[i.clone()].clone(),
                                                             i.clone() +
                                                                 1i32)))
                                          } else {
                                              {
                                                  if enumOpt.get().is_none() {
                                                      enumOpt.set(Some(xs.GetEnumerator()));
                                                  }
                                                  if enumOpt.get().is_some() {
                                                      if {
                                                             let e:
                                                                     Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                                                                 Option_::getValue(&enumOpt.get()).clone();
                                                             !finished.get()
                                                         } {
                                                          let e_1:
                                                                  Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                                                              Option_::getValue(&enumOpt.get()).clone();
                                                          if e_1.MoveNext() {
                                                              {
                                                                  prefix.get_mut().push(e_1.Current());
                                                                  Some(Rc::from((e_1.Current().clone(),
                                                                                 i.clone()
                                                                                     +
                                                                                     1i32)))
                                                              }.clone()
                                                          } else {
                                                              {
                                                                  finished.set(true);
                                                                  None::<Rc<(T,
                                                                             i32)>>
                                                              }.clone()
                                                          }
                                                      } else {
                                                          None::<Rc<(T, i32)>>
                                                      }
                                                  } else {
                                                      None::<Rc<(T, i32)>>
                                                  }
                                              }.clone()
                                          }
                                  }), &0i32)
        }.clone()
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone +
                    'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                             ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<(T1, T2)>>> {
        {
            let ysCache: Rc<dyn Seq::Enumerable::IEnumerable_1<T2>> =
                Seq::cache(ys);
            Seq::delay(&Rc::from({
                                     let xs = xs.clone();
                                     let ysCache = ysCache.clone();
                                     move ||
                                         Seq::concat(&Seq::map(&Rc::from({
                                                                             let ysCache
                                                                                 =
                                                                                 ysCache.clone();
                                                                             move
                                                                                 |x:
                                                                                      &T1|
                                                                                 Seq::map(&Rc::from({
                                                                                                        let x
                                                                                                            =
                                                                                                            x.clone();
                                                                                                        move
                                                                                                            |y:
                                                                                                                 &T2|
                                                                                                            Rc::from((x.clone(),
                                                                                                                      y.clone()))
                                                                                                    }),
                                                                                          &ysCache)
                                                                         }),
                                                               &xs))
                                 }))
        }.clone()
    }
    pub fn tryPick<T: Clone + 'static, a_: Clone +
                   'static>(chooser:
                                &Rc<impl Fn(&T) -> (Option<a_>) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Option<a_> {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    {
                        let res: Rc<MutCell<Option<a_>>> =
                            Rc::from(MutCell::from(None::<a_>));
                        while if res.get().is_none() {
                                  e.MoveNext()
                              } else { false } {
                            res.set(chooser(&e.Current()));
                        }
                        res.get().clone()
                    };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn pick<T: Clone + 'static, a_: Clone +
                'static>(chooser: &Rc<impl Fn(&T) -> (Option<a_>) + 'static>,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> a_ {
        {
            let matchValue: Option<a_> = Seq::tryPick(chooser, xs);
            match &matchValue {
                None => panic!("{}", Seq::SR::keyNotFoundAlt()),
                Some(matchValue_0_0) => matchValue_0_0.clone(),
            }
        }.clone()
    }
    pub fn reduce<T: Clone +
                  'static>(folder: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                           xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let e: Rc<dyn Seq::Enumerable::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    if e.MoveNext() {
                        {
                            let acc: Rc<MutCell<T>> =
                                Rc::from(MutCell::from(e.Current().clone()));
                            while e.MoveNext() {
                                acc.set(folder(&acc.get(), &e.Current()));
                            }
                            acc.get().clone()
                        }.clone()
                    } else { panic!("{}", Seq::SR::inputSequenceEmpty()) };
                if false { e.Dispose(); }
                try_result
            }.clone()
        }.clone()
    }
    pub fn reduceBack<T: Clone +
                      'static>(folder: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                               xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let arr: Rc<MutCell<Vec<T>>> = Seq::toArray(xs);
            if arr.len() as i32 > 0i32 {
                Array::reduceBack(folder, &arr)
            } else { panic!("{}", Seq::SR::inputSequenceEmpty()) }
        }.clone()
    }
    pub fn replicate<a_: Clone + 'static>(n: &i32, x: &a_)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<a_>> {
        Seq::initialize(n,
                        &Rc::from({
                                      let x = x.clone();
                                      move |_arg1: &i32| x.clone()
                                  }))
    }
    pub fn reverse<T: Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::reverse(&Seq::toArray(&xs)))
                             }))
    }
    pub fn scan<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<State>> {
        Seq::delay(&Rc::from({
                                 let folder = folder.clone();
                                 let state = state.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let acc: Rc<MutCell<State>> =
                                             Rc::from(MutCell::from(state.clone()));
                                         Seq::append(&Seq::singleton(&state),
                                                     &Seq::map(&Rc::from({
                                                                             let acc
                                                                                 =
                                                                                 acc.clone();
                                                                             let folder
                                                                                 =
                                                                                 folder.clone();
                                                                             move
                                                                                 |x:
                                                                                      &T|
                                                                                 {
                                                                                     acc.set(folder(&acc.get(),
                                                                                                    x));
                                                                                     acc.get().clone()
                                                                                 }.clone()
                                                                         }),
                                                               &xs))
                                     }.clone()
                             }))
    }
    pub fn scanBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>,
                             state: &State)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<State>> {
        Seq::delay(&Rc::from({
                                 let folder = folder.clone();
                                 let state = state.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::scanBack(&folder,
                                                                   &Seq::toArray(&xs),
                                                                   &state))
                             }))
    }
    pub fn skip<T: Clone +
                'static>(count: &i32,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let count = count.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let e:
                                                 Rc<dyn Seq::Enumerable::IEnumerator_1<T>> =
                                             Seq::ofSeq(&xs);
                                         for i in 1i32..=count {
                                             if !e.MoveNext() {
                                                 panic!("{}",
                                                        Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
                                             };
                                         }
                                         e.clone()
                                     }.clone()
                             }))
    }
    pub fn skipWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let predicate = predicate.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let skipped: Rc<MutCell<bool>> =
                                             Rc::from(MutCell::from(true));
                                         Seq::filter(&Rc::from({
                                                                   let predicate
                                                                       =
                                                                       predicate.clone();
                                                                   let skipped
                                                                       =
                                                                       skipped.clone();
                                                                   move
                                                                       |x: &T|
                                                                       {
                                                                           if skipped.get()
                                                                              {
                                                                               skipped.set(predicate(x));
                                                                           }
                                                                           !skipped.get()
                                                                       }
                                                               }), &xs)
                                     }.clone()
                             }))
    }
    pub fn tail<T: Clone +
                'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::skip(&1i32, xs)
    }
    pub fn take<T: Clone +
                'static>(count: &i32,
                         xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           move || Seq::ofSeq(&xs)
                                       }),
                             &Rc::from({
                                           let count = count.clone();
                                           move
                                               |i: &i32,
                                                e:
                                                    &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                               if i.clone() < count {
                                                   {
                                                       if !e.clone().MoveNext()
                                                          {
                                                           panic!("{}",
                                                                  Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &Native::string(&"\nParameter name: ")) as
              Rc<str>).to_string() + &Native::string(&"source")) as Rc<str>);
                                                       }
                                                       Some(e.Current().clone())
                                                   }.clone()
                                               } else { None::<T> }
                                       }),
                             &Rc::from(move
                                           |e_1:
                                                &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                           e_1.clone().Dispose()))
    }
    pub fn takeWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    move || Seq::ofSeq(&xs)
                                }),
                      &Rc::from({
                                    let predicate = predicate.clone();
                                    move
                                        |e:
                                             &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                        if if e.clone().MoveNext() {
                                               predicate(&e.Current())
                                           } else { false } {
                                            Some(e.Current().clone())
                                        } else { None::<T> }
                                }),
                      &Rc::from(move
                                    |e_1:
                                         &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                    e_1.clone().Dispose()))
    }
    pub fn truncate<T: Clone +
                    'static>(count: &i32,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           move || Seq::ofSeq(&xs)
                                       }),
                             &Rc::from({
                                           let count = count.clone();
                                           move
                                               |i: &i32,
                                                e:
                                                    &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                               if if i.clone() < count {
                                                      e.clone().MoveNext()
                                                  } else { false } {
                                                   Some(e.Current().clone())
                                               } else { None::<T> }
                                       }),
                             &Rc::from(move
                                           |e_1:
                                                &Rc<dyn Seq::Enumerable::IEnumerator_1<T>>|
                                           e_1.clone().Dispose()))
    }
    pub fn zip<T1: Clone + 'static, T2: Clone +
               'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                        ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<(T1, T2)>>> {
        Seq::map2(&Rc::from(move |x: &T1, y: &T2|
                                Rc::from((x.clone(), y.clone()))), xs, ys)
    }
    pub fn zip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T1>>,
                         ys: &Rc<dyn Seq::Enumerable::IEnumerable_1<T2>>,
                         zs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T3>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<(T1, T2, T3)>>> {
        Seq::map3(&Rc::from(move |x: &T1, y: &T2, z: &T3|
                                Rc::from((x.clone(), y.clone(), z.clone()))),
                  xs, ys, zs)
    }
    pub fn pairwise<T: Clone +
                    'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<(T, T)>>> {
        Seq::delay(&Rc::from({
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::pairwise(&Seq::toArray(&xs)))
                             }))
    }
    pub fn splitInto<T: Clone +
                     'static>(chunks: &i32,
                              xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<MutCell<Vec<T>>>>> {
        Seq::delay(&Rc::from({
                                 let chunks = chunks.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::splitInto(&chunks,
                                                                    &Seq::toArray(&xs)))
                             }))
    }
    pub fn r#where<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::filter(predicate, xs)
    }
    pub fn windowed<T: Clone +
                    'static>(windowSize: &i32,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<MutCell<Vec<T>>>>> {
        Seq::delay(&Rc::from({
                                 let windowSize = windowSize.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::windowed(&windowSize,
                                                                   &Seq::toArray(&xs)))
                             }))
    }
    pub fn sortWith<T: Clone +
                    'static>(comparer:
                                 &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                             xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let comparer = comparer.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let arr: Rc<MutCell<Vec<T>>> =
                                             Seq::toArray(&xs);
                                         Array::sortInPlaceWith(&comparer,
                                                                &arr);
                                         Seq::ofArray(&arr)
                                     }.clone()
                             }))
    }
    pub fn sort<T: PartialOrd + Clone +
                'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from(move |e1: &T, e2: &T| Util::compare(e1, e2)),
                      xs)
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone +
                  'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                           xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        Util::compare(&projection(x),
                                                      &projection(y))
                                }), xs)
    }
    pub fn sortDescending<T: PartialOrd + Clone +
                          'static>(xs:
                                       &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from(move |x: &T, y: &T|
                                    Util::compare(x, y) * -1i32), xs)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone +
                            'static>(projection:
                                         &Rc<impl Fn(&T) -> (U) + 'static>,
                                     xs:
                                         &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        Util::compare(&projection(x),
                                                      &projection(y)) * -1i32
                                }), xs)
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone +
               'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>) -> T {
        Seq::fold(&Rc::from(move |acc: &T, x: &T| acc.clone() + x.clone()),
                  &Native::getZero::<T>(), xs)
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default +
                 Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> U {
        Seq::fold(&Rc::from({
                                let projection = projection.clone();
                                move |acc: &U, x: &T|
                                    acc.clone() + projection(x)
                            }), &Native::getZero::<U>(), xs)
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        Seq::reduce(&Rc::from({
                                  let projection = projection.clone();
                                  move |x: &T, y: &T|
                                      if projection(x) > projection(y) {
                                          x.clone()
                                      } else { y.clone() }
                              }), xs)
    }
    pub fn max<T: PartialOrd + Clone +
               'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>) -> T {
        Seq::reduce(&Rc::from(move |x: &T, y: &T|
                                  if x.clone() > y.clone() {
                                      x.clone()
                                  } else { y.clone() }), xs)
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        Seq::reduce(&Rc::from({
                                  let projection = projection.clone();
                                  move |x: &T, y: &T|
                                      if projection(x) < projection(y) {
                                          x.clone()
                                      } else { y.clone() }
                              }), xs)
    }
    pub fn min<T: PartialOrd + Clone +
               'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>) -> T {
        Seq::reduce(&Rc::from(move |x: &T, y: &T|
                                  if x.clone() < y.clone() {
                                      x.clone()
                                  } else { y.clone() }), xs)
    }
    pub fn average<T: core::ops::Add<Output = T> +
                   core::ops::Div<Output = T> + From<i32> + Default + Clone +
                   'static>(xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> T {
        {
            let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            let total: T =
                Seq::fold(&Rc::from({
                                        let count = count.clone();
                                        move |acc: &T, x: &T|
                                            {
                                                count.set(count.get() + 1i32);
                                                acc.clone() + x.clone()
                                            }.clone()
                                    }), &Native::getZero::<T>(), xs);
            if count.get() == 0i32 {
                panic!("{}", Seq::SR::inputSequenceEmpty());
            }
            total / T::from(count.get()).clone()
        }.clone()
    }
    pub fn averageBy<T: Clone + 'static, U: core::ops::Add<Output = U> +
                     core::ops::Div<Output = U> + From<i32> + Default +
                     Clone +
                     'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                              xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> U {
        {
            let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            let total: U =
                Seq::fold(&Rc::from({
                                        let count = count.clone();
                                        let projection = projection.clone();
                                        move |acc: &U, x: &T|
                                            {
                                                count.set(count.get() + 1i32);
                                                acc.clone() + projection(x)
                                            }.clone()
                                    }), &Native::getZero::<U>(), xs);
            if count.get() == 0i32 {
                panic!("{}", Seq::SR::inputSequenceEmpty());
            }
            total / U::from(count.get()).clone()
        }.clone()
    }
    pub fn permute<T: Clone +
                   'static>(f: &Rc<impl Fn(&i32) -> (i32) + 'static>,
                            xs: &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let f = f.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::permute(&f,
                                                                  &Seq::toArray(&xs)))
                             }))
    }
    pub fn chunkBySize<T: Clone +
                       'static>(chunkSize: &i32,
                                xs:
                                    &Rc<dyn Seq::Enumerable::IEnumerable_1<T>>)
     -> Rc<dyn Seq::Enumerable::IEnumerable_1<Rc<MutCell<Vec<T>>>>> {
        Seq::delay(&Rc::from({
                                 let chunkSize = chunkSize.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array::chunkBySize(&chunkSize,
                                                                      &Seq::toArray(&xs)))
                             }))
    }
}
