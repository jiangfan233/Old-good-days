#![allow(non_snake_case)]
#[path = "./tetris/lib.rs"]
mod TetrisLib;

#[path = "./state/lib.rs"]
mod StateLib;

#[path = "./snake/lib.rs"]
mod SnakeLib;

#[path = "./dom/lib.rs"]
mod DomLib;


use DomLib::domtools::init_dom;
use SnakeLib::run_snake;
use TetrisLib::run_tetris;
use wasm_bindgen::prelude::wasm_bindgen;





#[wasm_bindgen(start)]
fn run() {
    let domElements = init_dom();

    // let _ = run_tetris(&domElements);
    run_snake(&domElements);
}