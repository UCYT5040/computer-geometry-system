#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, format, string::{String, ToString}, vec};
use mathcore_nostd::{Expr, MathCore};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use crate::{editor::ROW_HEIGHT, nadk::{display::{COLOR_BLACK, COLOR_RED, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, pull_rect, push_rect, push_rect_uniform_bordered}, keyboard::{InputManager, Key, wait_until_pressed, wait_until_pressed_multiple}, time}, ui::list::{SCREEN_HEIGHT, SCREEN_WIDTH, StringList}};

pub fn select_var(vars: &BTreeSet<String>, input_man: &mut InputManager) -> Option<String> {
    if vars.is_empty() { return None; }
    show_text_box(&["Select desired output:".to_string()]);
    let mut list = StringList::new_with_width(55, 55 + 15, SCREEN_WIDTH - 110, 4);
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

pub fn input_number_for(var: &str, input_man: &mut InputManager, math: &MathCore) -> Expr {
    let mut text_arr: [String; 2] = [format!("Input variable for {}:", var), String::new()];
    show_text_box(&text_arr);
    loop {
        input_man.scan();
        if let Some(last_pressed) = input_man.get_last_pressed() {
            match last_pressed.get_matching_char(false, false) {
                Some(ch) => {
                    text_arr[1].push(ch);
                },
                None => {
                    match last_pressed {
                        Key::Backspace => { text_arr[1].pop(); },
                        Key::Ok => break,
                        _ => {}
                    }
                }
            }
            show_text_box(&text_arr);
        }
        time::wait_milliseconds(20);
    }
    let resm = math.evaluate(&text_arr[1]);
    match resm {
        Ok(expr) => {
            return expr;
        }
        Err(e) => {
            show_alert(e.to_string());
            return input_number_for(var, input_man, math);
        }
    }
}

pub fn show_result(res: String) {
    show_text_box(&[res]);
    time::wait_milliseconds(500);
    wait_until_pressed_multiple(vec![Key::Ok, Key::Back]);
}

pub fn show_text_box(lines: &[String]) {
    push_rect_uniform_bordered(ScreenRect::new(50, 50, SCREEN_WIDTH - 100, SCREEN_HEIGHT - 100), COLOR_BLACK, COLOR_WHITE);
    for (i, line) in lines.iter().enumerate() {
        draw_string(&line, ScreenPoint::new(55, 55 + (i * ROW_HEIGHT) as u16), false, COLOR_WHITE, COLOR_BLACK);
    }
}

pub fn show_alert(content: String) {
    let alert_rect = ScreenRect::new(80, 80, SCREEN_WIDTH - 160, SCREEN_HEIGHT - 160);

    let saved_rect = pull_rect(alert_rect);
    push_rect_uniform_bordered(alert_rect, COLOR_BLACK, COLOR_RED);
    draw_string(&content, ScreenPoint::new(alert_rect.x + 5, alert_rect.y + 5), false, COLOR_WHITE, COLOR_BLACK);
    draw_string("Press Exe to dismiss", ScreenPoint::new(alert_rect.x + 20, alert_rect.y + 20), false, COLOR_WHITE, COLOR_BLACK);
    time::wait_milliseconds(500);
    wait_until_pressed(Key::Exe);

    push_rect(alert_rect, &saved_rect);
}
