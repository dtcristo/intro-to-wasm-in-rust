// Prepare imports for WASM module
const importObj = {
  imports: { imported_func: arg => console.log(arg) }
};
WebAssembly.instantiateStreaming(fetch("imported_func.wasm"), importObj).then(
  obj => {
    obj.instance.exports.exported_func();
  }
);
