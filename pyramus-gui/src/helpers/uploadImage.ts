import { uploadSvg, uploadImage, uploadText } from '@/../wasm/pkg/pyramus_wasm.js'

export async function createImage(name: string, parent: number, image: File) {
  const data: ArrayBuffer = await image.arrayBuffer()
  const array = new Uint8Array(data)
  return uploadImage(name, parent, array)
}

export function createSvg(name: string, parent: number, svg: string) {
  return uploadSvg(name, parent, svg)
}

export function createText(name: string, parent: number, text: string) {
  return uploadText(name, parent, text)
}
