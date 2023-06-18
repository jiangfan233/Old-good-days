pub mod food;
pub mod gameboard;
pub mod pos;
pub mod snake;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, Document, Element, KeyboardEvent};

use crate::{
    DomLib::domtools::{create_div, DomElements},
    StateLib::state::{use_state, State},
};

use self::{
    gameboard::{Direction, GameBoard},
    pos::Pos,
};

struct NodePos {
    node: Element,
    pos: Pos,
}

pub fn run_snake(domElements: &DomElements) {
    let DomElements {
        window,
        document,
        body,
    } = domElements;
    let gameboard = use_state(|| GameBoard::new(30, 30));
    let blocks = use_state(|| append_blocks(&document, &gameboard.value()));

    let container = create_div(&document, "");
    container.set_id("snake");
    container.set_class_name("snake-container");

    let _ = container.append_with_node(
        &blocks
            .value()
            .iter()
            .map(|nodePos| nodePos.node.clone())
            .collect(),
    );

    body.prepend_with_node_1(&container).unwrap();
    add_keydown_listener(&document, &gameboard, &blocks);
}

fn add_keydown_listener(
    document: &Document,
    gameboard: &State<GameBoard<'static>>,
    blocks: &State<Vec<NodePos>>,
) {
    let mut gameboard = gameboard.clone();
    let blocks = blocks.clone();

    let dyn_handle_keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        match e.code().as_str() {
            "ArrowLeft" => {
                gameboard.set(|mut t| {
                    t.try_move(Direction::Left);
                    t
                });
            }
            "ArrowRight" => {
                gameboard.set(|mut t| {
                    t.try_move(Direction::Right);
                    t
                });
            }
            "ArrowUp" => {
                gameboard.set(|mut t| {
                    t.try_move(Direction::Up);
                    t
                });
            }
            "ArrowDown" => {
                // js_sys::eval("console.log('dsadasd')").expect("err");
                gameboard.set(|mut t| {
                    t.try_move(Direction::Down);
                    t
                });
            }
            _ => {
                // js_sys::eval("console.log('HHHHHHHHHHHHHH')").expect("UNCH");
                // console::log_1(&"Hello using web-sys".into());
                console::log_1(&e.code().into());
            }
        }
        update_dom(&blocks, &gameboard);
    });
    document.set_onkeydown(Some(dyn_handle_keydown.as_ref().unchecked_ref()));

    // prevent this closure being dropped!!!
    dyn_handle_keydown.forget();
}

fn update_dom(blocks: &State<Vec<NodePos>>, gameboard: &State<GameBoard<'_>>) {
    blocks.value().iter().for_each(move |nodePos| {
        if nodePos.pos == gameboard.value().food.pos {
            nodePos
                .node
                .set_inner_html(gameboard.value().food.food_color);
        } else if gameboard.value().snake.positions.contains(&nodePos.pos) {
            nodePos.node.set_inner_html(gameboard.value().snake.color);
        } else {
            nodePos.node.set_inner_html("");
        }
    })
}

fn append_blocks(document: &Document, gameboard: &GameBoard) -> Vec<NodePos> {
    gameboard
        .iter_positions()
        .map(|pos| {
            let node = create_div(document, gameboard.get_position(&pos));
            node.set_attribute("key", pos.key().as_str()).unwrap();
            NodePos { node, pos }
        })
        .collect::<Vec<NodePos>>()
}
