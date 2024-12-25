import init, { MyStruct } from "../../wasm/bindgen"

let wasmMod = await init();
// let addition = add(1, 2);


export namespace WasmUtils {

	export const run = () => {
		
		let struct = MyStruct.new(1, "foo", [
			MyStruct.new(2, "bar", []),	
		]);
		console.assert(struct.b() === "folo")
		console.assert(struct.arr_len() ===  1)
		console.assert(struct.c()[0].b() === "bar")
		console.log('success')

	}

	// export const add = wasmMod.add;

}

