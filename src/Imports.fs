module Imports

open Fable.Core

// these import helpers are only temporary

[<Emit("()")>]
let emitUnit args: unit = nativeOnly

#if FABLE_COMPILER_RUST

let inline internal import selector path =
    emitUnit (Rust.import selector path)

#else

let inline internal import selector path = ()

#endif
