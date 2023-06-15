#![allow(non_snake_case)]
mod pos;
mod shape;
mod tetris;

use js_sys::Function;
use pos::Pos;
use shape::Shape;
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

struct ElementData {
    node: Element,
    pos: Pos,
}

enum Behavior {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let body = document.body().unwrap();

    let tetris = Tetris::default();

    let container = create_div(&document, "");

    let blocks = append_blocks(&document, &tetris);

    container.set_class_name("container");

    container.append_with_node(&blocks).unwrap();
    body.prepend_with_node_1(&container).unwrap();

    add_keydown_listener(&document, &tetris, &blocks);
    let timer = ticker(&window, &tetris, &blocks);

    Ok(())
}

fn ticker(window: &Window, tetris: &Tetris, blocks: &js_sys::Array) -> Timer {
    let mut tetris = tetris.clone();
    let mut blocks = blocks.clone();
    let t = Closure::<dyn FnMut()>::new(move || {
        if let Some(prev_shape) = tetris.tick() {
            update_dom(
                &blocks,
                &prev_shape,
                &tetris.current_shape,
                Behavior::MoveDown,
            );
        }
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

fn add_keydown_listener(document: &Document, tetris: &Tetris, blocks: &js_sys::Array) {
    let mut tetris = tetris.clone();
    let blocks = blocks.clone();

    let dyn_handle_keydown =
        Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| match e.code().as_str() {
            "ArrowLeft" => {
                if let Some(shape) = tetris.shift(Direction::Left) {
                    update_dom(&blocks, &shape, &tetris.current_shape, Behavior::MoveLeft);
                }
            }
            "ArrowRight" => {
                if let Some(shape) = tetris.shift(Direction::Right) {
                    update_dom(&blocks, &shape, &tetris.current_shape, Behavior::MoveRight);
                }
            }
            "ArrowUp" => {
                if let Some(shape) = tetris.rotate() {
                    update_dom(&blocks, &shape, &tetris.current_shape, Behavior::Rotate);
                }
            }
            "ArrowDown" => {
                js_sys::eval("console.log('dsadasd')").expect("err");
            }
            _ => {
                js_sys::eval("console.log('HHHHHHHHHHHHHH')").expect("UNCH");
            }
        });

    document.set_onkeydown(Some(dyn_handle_keydown.as_ref().unchecked_ref()));

    // prevent this closure being dropped!!!
    dyn_handle_keydown.forget();

}

fn update_dom(blocks: &Array, prev_shape: &Shape, current_shape: &Shape, behavior: Behavior) {
    let nodes = blocks.iter().map(Element::from);

    nodes.for_each(|node| {
        if prev_shape
            .positions
            .iter()
            .any(|pos| node.get_attribute("key").unwrap_or("null".to_string()) == pos.key())
        {
            node.set_inner_html("");
        }
        if current_shape
            .positions
            .iter()
            .any(|pos| node.get_attribute("key").unwrap_or("null".to_string()) == pos.key())
        {
            node.set_inner_html(current_shape.typ);
        }
    })
}
