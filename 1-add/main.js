async function run() {
  // Fetch wasm binary and perform streaming instantiation
  const obj = await WebAssembly.instantiateStreaming(fetch("add.wasm"), {});

  // Call exported function
  const result = obj.instance.exports.add(1, 1);
  console.log(result);
}

run();
