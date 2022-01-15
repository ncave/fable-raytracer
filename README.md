# fable-raytracer
Small ray tracer demo of the F# to Rust language transpiler in Fable 4.x

### Build and run:
- install Rust
- install `wasm-pack`
- run one of the performance tests:
  - `npm run test-js` (F# to JS, Fable 3.x, Node.js)
  - `npm run test-web` (F# to Rust to webasm, Browser, localhost:8080)
  - `npm run test-wasm` (F# to Rust to webasm, Node.js)
  - `npm run test-rust` (F# to Rust, running native)
  - `npm run test-dotnet` (F# running on .NET 6.0)