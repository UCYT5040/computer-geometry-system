#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod nadk;

mod list;
mod data;
mod tree;
mod equation;
mod editor;

use crate::editor::TextEditor;
use crate::list::{SCREEN_WIDTH, StringList};
use crate::tree::{EquationTree, ItemType, TreeItem};

#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, format, string::{String, ToString}, vec};
#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use indextree::NodeId;
use mathcore_nostd::{Expr, MathCore};
use mathcore_nostd::engine::Engine;

use crate::data::add_sample_data;
use crate::nadk::display::{
    COLOR_BLACK, COLOR_WHITE, Color565, SCREEN_RECT, ScreenPoint, ScreenRect, draw_string, push_rect_uniform
};
use crate::nadk::keyboard::{InputManager, Key, wait_until_pressed_multiple};
use crate::nadk::time;
use crate::nadk::utils::wait_ok_released;

// The app name must be a C string and the app name size must include the end line NULL character
configure_app!(b"CGS\0", 4, "../target/icon.nwi", 745);

// Setup the heap allocator if you need one
setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    init_heap!();
    wait_ok_released();

    push_rect_uniform(SCREEN_RECT, Color565::from_rgb888(0, 0, 0));
    draw_string(
        format!("Computer Geometry System v{}", env!("CARGO_PKG_VERSION")).as_str(),
        ScreenPoint::new(15, 15),
        false,
        COLOR_WHITE,
        COLOR_BLACK,
    );

    let mut tree = EquationTree::new();
    add_sample_data(&mut tree);

    let mut menu_list = StringList::new_with_max_row_count(15, 40);
    let mut input_man = InputManager::new();

    let mut current_node = tree.root;
    change_node(&mut menu_list, &mut tree, current_node);

    // this is here for testing purposes
    let mut textinput = TextEditor::new();
    let _content = textinput.start(&mut input_man);

    loop {
        input_man.scan();
        if input_man.is_just_pressed(Key::Down) {
            menu_list.next();
            menu_list.render();
        } else if input_man.is_just_pressed(Key::Up) {
            menu_list.previous();
            menu_list.render();
        } else if input_man.is_just_pressed(Key::Ok) {
            if let Some(item) = menu_list.get_selected()
                && let Some(item_id) = item.id
                && let Some(data) = tree.get_data(item_id)
            {
                match data.item_type {
                    ItemType::Category => {
                        current_node = item_id;
                        change_node(&mut menu_list, &mut tree, current_node);
                    }
                    ItemType::Equation => {
                        handle_equation(data, &mut input_man);
                        menu_list.render();
                    }
                    _ => {}
                }
            }
        } else if input_man.is_just_pressed(Key::Back) {
            if current_node != tree.root && let Some(data) = tree.get_data(current_node) {
                match data.item_type {
                    ItemType::Category => {
                        current_node = tree.get_parent(current_node).unwrap_or(tree.root);
                        change_node(&mut menu_list, &mut tree, current_node);
                    }
                    _ => {}
                }
            }
        } else if input_man.is_just_pressed(Key::Home) {
            break;
        }
        time::wait_milliseconds(50);
    }
}

fn handle_equation(data: &TreeItem, mut input_man: &mut InputManager) {
    if let Some(equation) = data.data.get_equation() {
        let mut vars = equation.get_variables();
        // remove automatically set consts
        vars.remove("e");
        vars.remove("pi");
        vars.remove("tau");
        if let Some(out) = select_var(&vars, &mut input_man) {
            let math = MathCore::new();
            let engine = Engine::new();
            vars.remove(&out);
            let mut expr = equation.data;
            for var in &vars {
                let res = input_number_for(var, &mut input_man, &math);
                expr = engine.substitute(&expr, var, &res).unwrap_or(expr);
            }

            let res: String;
            match MathCore::solve(expr.to_string().as_str(), &out) {
                Ok(r) => {
                    if r.is_empty() {
                        res = "No results".to_string();
                    } else if r.len() == 1 {
                        if let Expr::Number(n) = r[0] {
                            res = format!("{} = {:.10}", out, n)
                        } else {
                            res = format!("{} = {}", out, r[0])
                        }
                    } else {
                        res = format!("{} = {:?}", out, r);
                    }
                },
                Err(e) => {
                    res = e.to_string();
                }
            }

            push_rect_uniform(ScreenRect::new(15, 200, SCREEN_WIDTH - 15, 15), COLOR_BLACK);
            draw_string(res.as_str(), ScreenPoint::new(15, 200), false, COLOR_WHITE, COLOR_BLACK);
            time::wait_milliseconds(500);
            wait_until_pressed_multiple(vec![Key::Ok, Key::Back]);
            input_man.scan();
        }
    }
}

fn select_var(vars: &BTreeSet<String>, input_man: &mut InputManager) -> Option<String> {
    if vars.is_empty() { return None; }
    draw_string("Select desired output", ScreenPoint::new(15, 40), false, COLOR_WHITE, COLOR_BLACK);
    let mut list = StringList::new_with_max_row_count(15, 55);
    list.clear_screen();
    for var in vars {
        list.add(var);
    }
    list.render();
    loop {
        input_man.scan();
        if input_man.is_just_pressed(Key::Down) {
            list.next();
            list.render();
        } else if input_man.is_just_pressed(Key::Up) {
            list.previous();
            list.render();
        } else if input_man.is_just_pressed(Key::Ok) {
            list.clear_screen();
            return list.get_selected().and_then(|i| Some(i.name));
        }
        time::wait_milliseconds(50);
    }
}

fn input_number_for(var: &str, input_man: &mut InputManager, math: &MathCore) -> Expr {
    push_rect_uniform(ScreenRect::new(15, 40, SCREEN_WIDTH - 15, 15 * 2), COLOR_BLACK);
    draw_string(format!("Input variable for {}:", var).as_str(), ScreenPoint::new(15, 40), false, COLOR_WHITE, COLOR_BLACK);
    let mut res = String::new();
    loop {
        input_man.scan();
        if let Some(last_pressed) = input_man.get_last_pressed() {
            match last_pressed.get_matching_char(false, false) {
                Some(ch) => {
                    res.push(ch);
                },
                None => {
                    match last_pressed {
                        Key::Backspace => { res.pop(); },
                        Key::Ok => break,
                        _ => {}
                    }
                }
            }
            push_rect_uniform(ScreenRect::new(15, 55, SCREEN_WIDTH - 15, 15), COLOR_BLACK);
            draw_string(res.as_str(), ScreenPoint::new(15, 55), false, COLOR_WHITE, COLOR_BLACK);
        }
        time::wait_milliseconds(20);
    }
    let resm = math.evaluate(&res);
    match resm {
        Ok(expr) => {
            return expr;
        }
        Err(e) => {
            push_rect_uniform(ScreenRect::new(15, 200, SCREEN_WIDTH - 15, 15), COLOR_BLACK);
            draw_string(format!("{}", e).as_str(), ScreenPoint::new(15, 200), false, COLOR_WHITE, COLOR_BLACK);
            return input_number_for(var, input_man, math);
        }
    }
}

fn change_node(menu_list: &mut StringList, tree: &mut EquationTree, node: NodeId) {
    let items = tree.get_children(node);
    menu_list.clear();
    let _ = menu_list.select(0);
    menu_list.add_tree_items(&tree, items);
    menu_list.render();
}
