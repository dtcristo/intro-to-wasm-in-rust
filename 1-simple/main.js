// Fetch wasm binary and perform streaming instantiation
WebAssembly.instantiateStreaming(fetch("simple.wasm"), {}).then(obj => {
  // Call exported function
  const result = obj.instance.exports.add(1, 1);
  console.log(result);
});
