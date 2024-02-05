import { testString, wasmMemory } from '@/../wasm/pkg/pyramus_wasm.js'

export function testHelloFromRust() {
  return testString()
}

export async function getBuffer() {
  let wasmImport = await wasmMemory()
  return wasmImport.buffer
}
