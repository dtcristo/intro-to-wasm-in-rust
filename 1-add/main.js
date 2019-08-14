async function run() {
  // Fetch wasm binary and perform streaming instantiation
  const { _module, instance } = await WebAssembly.instantiateStreaming(
    fetch("add.wasm"),
    {}
  );

  // Call exported function
  const result = instance.exports.add(1, 1);
  console.log(result);
}

run();
