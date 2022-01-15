module TestApp
open type System.Console

[<EntryPoint>]
let main _args =
    let x, y, w, h = (0, 0, 1024, 1024)
    let len = w * h * 4
    let angle = 0.0
    let data = Array.create len 0uy
    WriteLine("{0}", "Raytracer running...")
    let _, elapsed = Platform.measureTime (fun () -> RayTracerDemo.renderScene (data, x, y, w, h, angle))
    WriteLine("Ray tracing:\n - rendered image size: ({0}x{1})\n - elapsed: {2} ms", w, h, elapsed)
    0
