use mathcore_nostd::{Expr, MathCore, MathError, engine::Engine};

#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, string::{String, ToString}, format, vec};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use crate::{nadk::keyboard::{InputManager}, ui::{misc::{input_number_for, select_var}}};

pub trait IntoEquation {
    #[allow(clippy::wrong_self_convention)]
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

pub fn solve_equation(data: &impl IntoEquation, input_man: &mut InputManager) -> String {
    let mut res;
    if let Some(equation) = data.into_equation() {
        let math = MathCore::new();

        let mut vars = equation.get_variables();
        // remove automatically set consts
        vars.remove("e");
        vars.remove("pi");
        vars.remove("tau");

        let solve_res;

        if vars.is_empty() {
            solve_res = math.evaluate(&equation.data.to_string()).map(|r| vec![r]);
            res = String::new();
        } else {
            let selres = select_var(&vars, input_man);
            if selres.is_none() { return "Failed to prompt user to select variable".to_string(); }
            let selres = selres.unwrap();

            let engine = Engine::new();
            vars.remove(&selres);
            let mut expr = equation.data;
            for var in &vars {
                let res = input_number_for(var, input_man, &math);
                expr = engine.substitute(&expr, var, &res).unwrap_or(expr);
            }

            solve_res = MathCore::solve(expr.to_string().as_str(), &selres);
            res = format!("{} =\n", selres);
        }

        match solve_res {
            Ok(r) => {
                if r.is_empty() {
                    res = "No results".to_string();
                } else {
                    for (i, one_res) in r.iter().enumerate() {
                        res.push_str(&format!("{:?}{}", one_res, if i < r.len() - 1 { "\n" } else { "" }));
                    }
                }
            },
            Err(e) => {
                res = e.to_string();
            }
        }
    } else {
        res = "Error parsing expression".to_string();
    }
    res
}
