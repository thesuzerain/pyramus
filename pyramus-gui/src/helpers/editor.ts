import {
  imageBufferPointer,
  testRenderStringResvg,
  getStage,
  removeObject,
  editTransform,
  renameObject,
  selectObjects,
  type FrontendStage,
} from '@/../wasm/pkg/pyramus_wasm.js'

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
export function selectItems(items: number[]) {
  const itemsUint32 = new Uint32Array(items);
  selectObjects(itemsUint32)
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

// TODO: Can we attach this to the stage object?
export function editItemName(item: number, name: string) {
  renameObject(item, name)
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
