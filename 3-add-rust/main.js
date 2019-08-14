async function run() {
  const obj = await WebAssembly.instantiateStreaming(
    fetch("/target/wasm32-unknown-unknown/release/add_rust.wasm"),
    {}
  );

  const result = obj.instance.exports.add_rust(1, 1);
  console.log(result);
}

run();
