#![allow(non_snake_case)]
mod shape;
mod pos;
mod tetris;


use pos::Pos;
use crate::DomLib::domtools::{create_div, DomElements};
use crate::StateLib::state::{use_state, State};
use std::convert::AsRef;
use tetris::{Direction, Tetris};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use web_sys::{console, Document, Element, KeyboardEvent, Window};

// js_sys::eval("document.onkeydown = console.log");

struct Timer {
    timer: i32,
    clear: Closure<dyn Fn(Window)>,
}

struct NodePos {
    node: Element,
    pos: Pos,
}

pub fn run_tetris(domElements: &DomElements) -> Result<(), JsValue> {
    let DomElements{
        window,
        document,
        body,
     } = domElements;

    let tetris = use_state(Tetris::default);
    let blocks = use_state(|| append_blocks(&document, &tetris.value()));

    let container = create_div(&document, "");
    container.set_id("tetris");
    container.set_class_name("tetris-container");
    

    let _ = container.append_with_node(
        &blocks
            .value()
            .iter()
            .map(|nodePos| nodePos.node.clone())
            .collect(),
    );

    // container.append_with_node(&blocks.value()).unwrap();
    body.prepend_with_node_1(&container).unwrap();

    add_keydown_listener(&document, &tetris, &blocks);
    let _ = ticker(&window, &tetris, &blocks);

    Ok(())
}

fn ticker(window: &Window, tetris: &State<Tetris>, blocks: &State<Vec<NodePos>>) -> Timer {
    let mut tetris = tetris.clone();
    let blocks = blocks.clone();
    let t = Closure::<dyn FnMut()>::new(move || {
        tetris.set(|mut t| {
            t.tick();
            t
        });
        update_dom(&blocks, &tetris);
    });

    let timer = window
        .set_interval_with_callback_and_timeout_and_arguments_0(t.as_ref().unchecked_ref(), 1000)
        .unwrap();
    let clear = Closure::<dyn Fn(Window)>::new(move |window: Window| {
        window.clear_interval_with_handle(timer)
    });

    t.forget();

    Timer { timer, clear }
}

fn append_blocks(document: &Document, tetris: &Tetris) -> Vec<NodePos> {
    tetris
        .iter_positions()
        .map(|pos| {
            let node = create_div(document, tetris.getPosition(pos));
            node.set_attribute("key", pos.key().as_str()).unwrap();
            NodePos { node, pos }
        })
        .collect::<Vec<NodePos>>()
}


fn add_keydown_listener(document: &Document, tetris: &State<Tetris>, blocks: &State<Vec<NodePos>>) {
    let mut tetris = tetris.clone();
    let blocks = blocks.clone();

    let dyn_handle_keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        match e.code().as_str() {
            "ArrowLeft" => {
                tetris.set(|mut t| {
                    t.shift(Direction::Left);
                    t
                });
            }
            "ArrowRight" => {
                tetris.set(|mut t| {
                    t.shift(Direction::Right);
                    t
                });
            }
            "ArrowUp" => {
                tetris.set(|mut t| {
                    t.rotate();
                    t
                });
            }
            "ArrowDown" => {
                // js_sys::eval("console.log('dsadasd')").expect("err");
                tetris.set(|mut t| {
                    t.tick();
                    t
                });
            }
            _ => {
                // js_sys::eval("console.log('HHHHHHHHHHHHHH')").expect("UNCH");
                // console::log_1(&"Hello using web-sys".into());
                console::log_1(&e.code().into());
            }
        }
        update_dom(&blocks, &tetris);
    });

    document.set_onkeydown(Some(dyn_handle_keydown.as_ref().unchecked_ref()));

    // prevent this closure being dropped!!!
    dyn_handle_keydown.forget();
}

fn update_dom(blocks: &State<Vec<NodePos>>, tetris: &State<Tetris>) {
    let current_shape = &tetris.value().current_shape;
    let fixed_shapes = &tetris.value().fixed_shapes;

    blocks.value().iter().for_each(|NodePos { node, pos }| {
        node.set_inner_html("");
        if current_shape
            .positions
            .iter()
            .any(|p| p.0 == pos.0 && p.1 == pos.1)
        {
            node.set_inner_html(current_shape.typ);
        }

        fixed_shapes.iter().for_each(|shape| {
            if shape.positions.iter().any(|p| p.0 == pos.0 && p.1 == pos.1) {
                node.set_inner_html(shape.typ);
            }
        });
    })
}

mod test {
    use crate::StateLib::state::{use_state, State};
    use crate::TetrisLib::Tetris;

    #[test]
    fn test() {
        let mut tetris = use_state(Tetris::default);
        tetris.set(|mut t: Tetris| {
            t.tick();
            t.tick();
            t.tick();
            print!("{:#?}", t.current_shape);
            if let Some(old_shape) = t.rotate() {
                println!("{:#?}", old_shape);
            } else {
                println!("can not rotate");
            }
            print!("{:#?}", t.current_shape);
            t
        });
    }
}
