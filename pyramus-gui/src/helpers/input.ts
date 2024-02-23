import {
    inputClick
  } from '@/../wasm/pkg/pyramus_wasm.js'
  
  export function handleClick(x: number, y: number) {
    return inputClick(x, y)
}
  