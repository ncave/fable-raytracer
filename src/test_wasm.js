import { performance } from "perf_hooks";
import { render_scene } from "../pkg/fable_raytracer.js";

function measureTime(f) {
    const t0 = performance.now();
    let res = f();
    const t1 = performance.now();
    return [res, t1 - t0];
}

async function run() {
    const [x, y, w, h] = [0, 0, 2048, 2048];
    console.log("Raytracer running...");
    const [_, elapsed] = measureTime (() => render_scene(x, y, w, h));
    console.log("Ray tracing done:");
    console.log(` - rendered image size: (${w}x${h})\n - elapsed: ${elapsed} ms`);
}

run();
