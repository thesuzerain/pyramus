import { testString } from "@/../wasm/pkg/pyramus_wasm.js";

export function test() {
	let f = testString();
	console.log(f);
	return f;
}

