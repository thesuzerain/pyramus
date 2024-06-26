import init, { wasmMemory, switchEditorBase } from '@/../wasm/pkg/pyramus_wasm.js'

export type WasmRawInstance = WebAssembly.Memory

// `wasmImport` starts uninitialized because its initialization needs to occur asynchronously, and thus needs to occur by manually calling and awaiting `initWasm()`
let wasmImport: WebAssembly.Memory | undefined

// Should be called asynchronously before any other WASM functions are called
export async function initWasm() {
  // Skip if the WASM module is already initialized
  if (wasmImport !== undefined) return

  // Import the WASM module JS bindings
  // eslint-disable-next-line import/no-cycle
  await init()
  wasmImport = await wasmMemory()
}

export function getWasm(): WasmRawInstance {
  if (wasmImport === undefined) throw new Error('WASM module not initialized')
  return wasmImport
}

// TODO: may not be necessary anymore now that we are no longer editing the buffer directly
export function getRawBuffer(): ArrayBuffer {
  return getWasm().buffer
}

export function switchEditor(id : string) {
  return switchEditorBase(id)
}