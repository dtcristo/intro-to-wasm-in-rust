async function run() {
  const { _module, instance } = await WebAssembly.instantiateStreaming(
    fetch("/target/wasm32-unknown-unknown/release/add_rust.wasm"),
    {}
  );

  const result = instance.exports.add_rust(1, 1);
  console.log(result);
}

run();
