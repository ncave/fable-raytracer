module TestApp

open Platform
open type System.Console

let render() =
    let x, y, w, h = (0, 0, 1024, 1024)
    let len = w * h * 4
    let angle = 0.0
    let data = Array.create len 0uy
    WriteLine("Raytracer running...")
    let _, elapsed = measureTime (fun () -> RayTracerDemo.renderScene (data, x, y, w, h, angle))
    WriteLine($"Ray tracing done:\n - rendered image size: ({w}x{h})\n - elapsed: {elapsed} ms")

[<EntryPoint>]
let main _args =
    render()
    0
