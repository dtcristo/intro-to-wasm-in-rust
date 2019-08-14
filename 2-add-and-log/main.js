async function run() {
  // Prepare imports for WASM module
  const importObj = {
    imports: { log: arg => console.log(arg) }
  };

  // Provide imports for streaming instantiation
  const { _module, instance } = await WebAssembly.instantiateStreaming(
    fetch("add_and_log.wasm"),
    importObj
  );

  // Call exported fucntion
  instance.exports.addAndLog(1, 1);
}

run();
