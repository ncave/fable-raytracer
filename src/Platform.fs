module Platform

#if FABLE_COMPILER_RUST

open Fable.Core.JsInterop

module Performance =
    type Duration =
        // abstract as_millis: unit -> uint64 // actually u128
        abstract as_secs_f64: unit -> float

    type Instant =
        // abstract duration_since: Instant -> Duration
        abstract elapsed: unit -> Duration

    // let inline now(): Instant = importMember "std::time::Instant"
    let inline internal now(): obj = emitJsExpr () "std::time::Instant::now"

let measureTime (f: unit -> 'T): 'T * float =
    let t0 = Performance.now()
    let res = f ()
    let t1 = Performance.now()
    // let elapsed = t1.duration_since(t0).as_secs_f64()
    let elapsed = (t0 :?> Performance.Instant).elapsed().as_secs_f64()
    res, elapsed * 1000.0

#endif

// #if FABLE_COMPILER_JAVASCRIPT
#if FABLE_COMPILER && !FABLE_COMPILER_RUST

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
