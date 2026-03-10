use mathcore_nostd::{Expr, MathCore, MathError};

#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, string::String};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Equation {
    data: Expr
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