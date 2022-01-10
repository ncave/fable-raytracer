module TestApp
open type System.Console

[<EntryPoint>]
let main _args =
    // WriteLine("{0}", "Raytracer sample")
    RayTracerDemo.renderScene()
    0
