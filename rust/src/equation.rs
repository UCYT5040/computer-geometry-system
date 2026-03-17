use mathcore_nostd::{Expr, MathCore, MathError, engine::Engine};

#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, string::{String, ToString}, format, vec};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use crate::{nadk::{display::{COLOR_BLACK, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, push_rect_uniform}, keyboard::{InputManager, Key, wait_until_pressed_multiple}, time}, ui::{list::SCREEN_WIDTH, misc::{input_number_for, select_var}}};

pub trait IntoEquation {
    fn into_equation(&self) -> Option<Equation>;
}

#[derive(Clone)]
pub struct Equation {
    pub data: Expr
}

impl Equation {
    pub fn new(equation: &str) -> Result<Self, MathError> {
        match MathCore::parse(equation) {
            Ok(expr) => {
                Ok(Equation { data: expr })
            }
            Err(e) => Err(e)
        }
    }

    pub fn get_variables(&self) -> BTreeSet<String> {
        self.data.extract_variables()
    }
}

pub fn solve_equation(data: &impl IntoEquation, mut input_man: &mut InputManager) {
    if let Some(equation) = data.into_equation() {
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