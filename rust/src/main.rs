#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod nadk;

mod ui;
mod data;
mod tree;
mod equation;
mod editor;

use crate::editor::TextEditor;
use crate::equation::solve_equation;
use crate::ui::list::StringList;
use crate::tree::EquationTree;
use crate::ui::misc::show_result;

use indextree::NodeId;

use crate::nadk::display::{
    Color565, SCREEN_RECT, push_rect_uniform
};
use crate::nadk::keyboard::InputManager;
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
    /*draw_string(
        format!("Computer Geometry System v{}", env!("CARGO_PKG_VERSION")).as_str(),
        ScreenPoint::new(15, 15),
        false,
        COLOR_WHITE,
        COLOR_BLACK,
    );

    let mut tree = EquationTree::new();
    add_sample_data(&mut tree);

    let mut menu_list = StringList::new_with_max_row_count(15, 40);
    
    let mut current_node = tree.root;
    change_node(&mut menu_list, &mut tree, current_node);*/

    let mut input_man = InputManager::new();

    // this is here for testing purposes
    let mut textinput = TextEditor::new();
    loop {
        let input = textinput.start(&mut input_man);
        let res = solve_equation(&input, &mut input_man);
        show_result(res);
    }

    /*
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
                        solve_equation(data, &mut input_man);
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
    }*/
}

fn change_node(menu_list: &mut StringList, tree: &mut EquationTree, node: NodeId) {
    let items = tree.get_children(node);
    menu_list.clear();
    let _ = menu_list.select(0);
    menu_list.add_tree_items(&tree, items);
    menu_list.render();
}
