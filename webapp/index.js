import { greet, test, wasm_init } from '../raytracer/Cargo.toml';

wasm_init();
console.log(test());
