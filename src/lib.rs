#![allow(non_snake_case)]
#[path = "./tetris/lib.rs"]
mod TetrisLib;

#[path = "./state/lib.rs"]
mod StateLib;

use TetrisLib::run_tetris;
use wasm_bindgen::prelude::wasm_bindgen;





#[wasm_bindgen(start)]
fn run() {
    let _ = run_tetris();
}