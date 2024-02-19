import {
  imageBufferPointer,
  testRenderResvg,
  testRenderStringResvg,
  getStage,
  removeObject,
  editTransform,
  type FrontendStage,
} from '@/../wasm/pkg/pyramus_wasm.js'

export function testRender(canvas: HTMLCanvasElement) {
  testRenderResvg(canvas)
}

export function testRenderString() {
  return testRenderStringResvg() // TODO: should this take an argument?
}

export function getStageObject(): FrontendStage {
  return getStage()
}

// TODO: Can we attach this to the stage object?
export function deleteItem(item: number) {
  removeObject(item)
}

// TODO: Can we attach this to the stage object?
export function editItemTransform(
  item: number,
  position: [number, number],
  rotation: number,
  scale: [number, number]
) {
  editTransform(item, position[0], position[1], rotation, scale[0], scale[1])
}

export function getImageBufferPointer() {
  return imageBufferPointer()
}

export function clearCanvas(canvas: HTMLCanvasElement) {
  const canvasContext = canvas.getContext('2d')
  if (!canvasContext) {
    return
  }
  canvasContext.clearRect(0, 0, canvas.width, canvas.height)
}
