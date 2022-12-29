# fable-raytracer
Small ray tracer demo of the F# to Rust language transpiler in Fable 4.x

### Online demo:
https://ncave.github.io/fable-raytracer/

### Build and run:
- install [.NET](https://dotnet.microsoft.com/en-us/download) and [Node.js](https://nodejs.org/en/)
- install [Rust](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- run `dotnet tool restore` to get [Fable](https://github.com/fable-compiler/Fable) installed locally
- run one of the performance tests:
  - `npm run test-js` (F# to JavaScript, running in Node.js)
  - `npm run test-web` (F# to Rust to WebAssembly, in Browser, localhost:8080)
  - `npm run test-wasm` (F# to Rust to WebAssembly, running in Node.js)
  - `npm run test-rust` (F# to Rust, running as native binary)
    - `npm run test-rust-target-cpu` (as above, but allow Rust to target your CPU and use newer instructions)
  - `npm run test-dotnet` (F# running on .NET as managed code)
  - `npm run test-native` (F# running on .NET as native binary)
  - `npm run test-python` (F# to Python, running as Python)
