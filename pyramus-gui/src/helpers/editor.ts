import { imageBufferPointer, generate_checker_board } from "@/../wasm/pkg/pyramus_wasm.js";
import {  getRawBuffer } from "@/../src/helpers/state";

export function getImageBufferPointer() {
	return imageBufferPointer();
}

export function clearCanvas(canvas : HTMLCanvasElement) {
	const canvasContext = canvas.getContext("2d");
	if (!canvasContext) {
		return;
	}
	canvasContext.clearRect(0, 0, canvas.width, canvas.height);
}


export function generateCheckerBoard() {
	return generate_checker_board(
		getDarkValue(),
		getDarkValue(),
		getDarkValue(),
		getLightValue(),
		getLightValue(),
		getLightValue()
	  );
}

const getDarkValue = () => {
    return Math.floor(Math.random() * 100);
  };

  const getLightValue = () => {
    return Math.floor(Math.random() * 127) + 127;
  };

  export async function drawCheckerBoard(canvas : HTMLCanvasElement) {
	const canvasContext = canvas.getContext("2d");
	const canvasImageData = canvasContext?.createImageData(
		canvas.width,
		canvas.height
	);
	if (!canvasContext || !canvasImageData) {
		return;
	}
  
	  const checkerBoardSize = 20;
  
	  // Generate a new checkboard in wasm
	  generateCheckerBoard();
  
	  // Pull out the RGBA values from Wasm memory
	  // Starting at the memory index of out output buffer (given by our pointer)
	  // 20 * 20 * 4 = checkboard max X * checkerboard max Y * number of pixel properties (R,G.B,A)
	  const outputPointer = getImageBufferPointer();
	  const buffer = new Uint8Array(getRawBuffer());
	  const imageDataArray = buffer.slice(
		outputPointer,
		outputPointer + checkerBoardSize * checkerBoardSize * 4
	  );
  
	  // Set the values to the canvas image data
	  canvasImageData?.data.set(imageDataArray);
  
	  // Clear the canvas
	  canvasContext?.clearRect(0, 0, canvas.width, canvas.height);
  
	  // Place the new generated checkerboard onto the canvas
	  canvasContext?.putImageData(canvasImageData, 0, 0);
	};
  
