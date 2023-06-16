#![allow(non_snake_case)]
mod persist;
mod pos;
mod ref_container;
mod shape;
mod state;
mod tetris;

use js_sys::Function;
use pos::Pos;
use shape::Shape;
use state::{use_state, State};
use std::convert::AsRef;
use std::fmt;
use tetris::{Direction, Tetris};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use js_sys::{Array, Date};
use web_sys::{console, Document, Element, HtmlElement, KeyboardEvent, Window};

// js_sys::eval("document.onkeydown = console.log");

struct Timer {
    timer: i32,
    clear: Closure<dyn Fn(Window)>,
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let body = document.body().unwrap();

    let tetris = use_state(|| Tetris::default());

    let container = create_div(&document, "");

    let blocks = use_state(|| append_blocks(&document, &tetris.value()));

    container.set_class_name("container");

    container.append_with_node(&blocks.value()).unwrap();
    body.prepend_with_node_1(&container).unwrap();

    add_keydown_listener(&document, &tetris, &blocks);
    let timer = ticker(&window, &tetris, &blocks);

    Ok(())
}

fn ticker(window: &Window, tetris: &State<Tetris>, blocks: &State<js_sys::Array>) -> Timer {
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

fn append_blocks(document: &Document, tetris: &Tetris) -> js_sys::Array {
    tetris
        .iter_positions()
        .map(|pos| {
            let node = create_div(document, tetris.getPosition(pos));
            node.set_attribute("key", pos.key().as_str()).unwrap();
            node
        })
        .collect::<Array>()
}

fn create_div(document: &Document, text: &str) -> Element {
    let val = document.create_element("div").unwrap();
    val.set_text_content(Some(text));
    val
}

fn add_keydown_listener(
    document: &Document,
    tetris: &State<Tetris>,
    blocks: &State<js_sys::Array>,
) {
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

fn update_dom(blocks: &State<Array>, tetris: &State<Tetris>) {
    let current_shape = &tetris.value().current_shape;
    let fixed_shapes = &tetris.value().fixed_shapes;

    blocks.value().iter().map(Element::from).for_each(|node| {
        node.set_inner_html("");

        if current_shape
            .positions
            .iter()
            .any(|pos| node.get_attribute("key").unwrap_or("null".to_string()) == pos.key())
        {
            node.set_inner_html(current_shape.typ);
        }

        fixed_shapes.iter().for_each(|shape| {
            if shape
                .positions
                .iter()
                .any(|pos| {
                    node.get_attribute("key").unwrap_or("null".to_string()) == pos.key()
                })
            {
                node.set_inner_html(shape.typ);
            }
        });

    })
}

mod test {
    use crate::{state::use_state, tetris::Tetris};

    #[test]
    fn test() {
        let mut tetris = use_state(|| Tetris::default());
        tetris.set(|mut t: Tetris| {
            t.tick();
            t.tick();
            t.tick();
            print!("{:#?}", t.current_shape);
            if let Some(old_shape) = t.rotate() {
                // println!("{:#?}", old_shape);
            } else {
                println!("can not rotate");
            }
            print!("{:#?}", t.current_shape);
            t
        });

    }
}
