/* tslint:disable */
/* eslint-disable */
/**
* @returns {number}
*/
export function get_buffer_offset(): number;
/**
* @returns {number}
*/
export function get_buffer_length(): number;
/**
* @param {number} x
* @param {number} y
* @param {number} w
* @param {number} h
* @param {number} angle
*/
export function render_scene(x: number, y: number, w: number, h: number, angle: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_buffer_offset: () => number;
  readonly get_buffer_length: () => number;
  readonly render_scene: (a: number, b: number, c: number, d: number, e: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
