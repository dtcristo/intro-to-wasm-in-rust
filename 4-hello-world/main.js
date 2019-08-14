import init, { greet } from "./pkg/hello_world.js";

async function run() {
  await init();
  greet("Rust meetup");
}

run();
