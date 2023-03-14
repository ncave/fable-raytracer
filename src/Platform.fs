module Platform

#if FABLE_COMPILER_RUST

open Fable.Core

module Performance =
    [<Erase; Emit("std::time::Duration")>]
    type Duration =
        abstract as_millis: unit -> uint64 // actually u128
        abstract as_secs_f64: unit -> float

    [<Erase; Emit("std::time::Instant")>]
    type Instant =
        abstract duration_since: Instant -> Duration
        abstract elapsed: unit -> Duration

    [<Emit("std::time::Instant::now()")>]
    let now(): Instant = nativeOnly

let measureTime (f: unit -> 'T): 'T * float =
    let t0 = Performance.now()
    let res = f ()
    let t1 = Performance.now()
    let duration = t1.duration_since(t0)
    let elapsed = duration.as_secs_f64()
    res, elapsed * 1000.0

#endif

#if FABLE_COMPILER_PYTHON

open Fable.Core
open System.Diagnostics

let measureTime (f: unit -> 'T): 'T * float =
    let freq = double System.Diagnostics.Stopwatch.Frequency
    let t0 = Stopwatch.GetTimestamp()
    let res = f ()
    let t1 = Stopwatch.GetTimestamp()
    let elapsed = double (t1 - t0) / freq
    res, elapsed * 1000.0

#endif


// #if FABLE_COMPILER_JAVASCRIPT
#if FABLE_COMPILER && !FABLE_COMPILER_RUST && !FABLE_COMPILER_PYTHON

open Fable.Core.JsInterop

type private IPerformance =
    abstract now: unit -> float

let private performance: IPerformance = importMember "perf_hooks"

let measureTime (f: unit -> 'T): 'T * float =
    let t0 = performance.now()
    let res = f ()
    let t1 = performance.now()
    res, (t1 - t0)

#endif

#if !FABLE_COMPILER // .NET

let measureTime (f: unit -> 'T): 'T * float =
    let sw = System.Diagnostics.Stopwatch.StartNew()
    let res = f ()
    sw.Stop()
    res, sw.Elapsed.TotalMilliseconds

#endif
