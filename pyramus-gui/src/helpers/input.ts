import { inputMouseMove, inputMouseDown, inputMouseUp } from '@/../wasm/pkg/pyramus_wasm.js'

export function handleMouseDown(x: number, y: number) {
  return inputMouseDown(x, y)
}

export function handleMouseUp() {
  return inputMouseUp()
}

export function handleMouseMove(x: number, y: number) {
  return inputMouseMove(x, y)
}
