use mathcore_nostd::{Expr, MathCore, MathError, engine::Engine};

#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, string::{String, ToString}, format, vec};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use crate::{nadk::keyboard::{InputManager}, ui::{misc::{input_number_for, select_var}}};

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

pub fn solve_equation(data: &impl IntoEquation, mut input_man: &mut InputManager) -> String {
    let mut res = "Error".to_string();
    if let Some(equation) = data.into_equation() {
        let math = MathCore::new();

        let mut vars = equation.get_variables();
        // remove automatically set consts
        vars.remove("e");
        vars.remove("pi");
        vars.remove("tau");

        let solve_res;
        let prefix;

        if vars.is_empty() {
            solve_res = math.evaluate(&equation.data.to_string()).and_then(|r| Ok(vec![r]));
            prefix = String::new();
        } else {
            let selres = select_var(&vars, &mut input_man);
            if selres.is_none() { return "Failed to prompt user to select variable".to_string(); }
            let selres = selres.unwrap();

            let engine = Engine::new();
            vars.remove(&selres);
            let mut expr = equation.data;
            for var in &vars {
                let res = input_number_for(var, &mut input_man, &math);
                expr = engine.substitute(&expr, var, &res).unwrap_or(expr);
            }

            solve_res = MathCore::solve(expr.to_string().as_str(), &selres);
            prefix = format!("{} = ", selres);
        }

        match solve_res {
            Ok(r) => {
                if r.is_empty() {
                    res = "No results".to_string();
                } else if r.len() == 1 {
                    if let Expr::Number(n) = r[0] {
                        res = format!("{}{:.10}", prefix, n)
                    } else {
                        res = format!("{}{}", prefix, r[0])
                    }
                } else {
                    res = format!("{}{:?}", prefix, r);
                }
            },
            Err(e) => {
                res = e.to_string();
            }
        }
    }
    return res;
}
