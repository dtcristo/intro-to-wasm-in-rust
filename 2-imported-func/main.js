async function run() {
  // Prepare imports for WASM module
  const importObj = {
    imports: { imported_func: arg => console.log(arg) }
  };

  // Provide imports for streaming instantiation
  const obj = await WebAssembly.instantiateStreaming(
    fetch("imported_func.wasm"),
    importObj
  );

  // Call exported fucntion
  obj.instance.exports.exported_func();
}

run();
