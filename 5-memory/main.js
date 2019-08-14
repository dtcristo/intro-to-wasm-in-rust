import init, { makeString } from "./pkg/memory.js";

async function run() {
  const { memory } = await init();

  // Build a string in rust and get the raw parts
  const rawString = makeString();
  const { pointer, length } = rawString;
  rawString.free();
  console.log(pointer, length);

  // Extract raw data of string
  const data = new Uint8Array(memory.buffer, pointer, length);
  console.log(data);

  // Decode string and print
  let decoder = new TextDecoder();
  const string = decoder.decode(data);
  console.log(string);
}

run();
