#if !FABLE_COMPILER_RUST
module RayTracer

#else //FABLE_COMPILER_RUST
[<Fable.Core.Rust.OuterAttr("cfg", [|"target_arch = \"wasm32\""|])>]
module RayTracer

open Fable.Core
open Fable.Core.Rust

let imports() =
    import "wasm_bindgen::prelude::*" ""
    ()

let maxWidth = 2048
let maxHeight = 2048
let bufferLength = maxWidth * maxHeight * 4
let buffer = Array.zeroCreate<byte> bufferLength

[<Emit("*const u8")>]
type BufferType = interface end

[<Emit("$0.as_ptr()")>]
let ptrOf x: BufferType = nativeOnly

[<OuterAttr("wasm_bindgen")>]
let get_buffer_length () = bufferLength

[<OuterAttr("wasm_bindgen")>]
let get_buffer_offset () = ptrOf buffer

[<OuterAttr("wasm_bindgen")>]
let render_scene(x, y, w, h, angle) =
    RayTracerDemo.renderScene(buffer, x, y, w, h, angle)

#endif //FABLE_COMPILER_RUST
