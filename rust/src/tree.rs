#![allow(unused)]

#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

use indextree::{Arena, NodeId};

use crate::equation::Equation;

pub enum ItemType {
    Category,
    Equation,
    Root,
}

#[derive(Default)]
pub struct ItemData {
    data_category: Option<String>,
    data_equation: Option<Equation>,
}

pub struct TreeItem {
    pub name: String,
    pub item_type: ItemType,
    data: ItemData,
}

pub struct EquationTree {
    arena: Arena<TreeItem>,
    pub root: NodeId,
}

impl EquationTree {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let root_item = TreeItem {
            name: "root".into(),
            item_type: ItemType::Root,
            data: ItemData::default(),
        };
        let root = arena.new_node(root_item);
        EquationTree { arena, root }
    }

    pub fn add_child(&mut self, parent: NodeId, data: TreeItem) -> NodeId {
        let child = self.arena.new_node(data);
        parent.append(child, &mut self.arena);
        child
    }

    pub fn get_children(&mut self, parent: NodeId) -> Vec<NodeId> {
        parent.children(&mut self.arena).collect()
    }

    pub fn get_data(&self, node: NodeId) -> Option<&TreeItem> {
        if let Some(item) = self.arena.get(node) {
            return Some(item.get());
        }
        None
    }

    pub fn get_parent(&self, node: NodeId) -> Option<NodeId> {
        if let Some(item) = self.arena.get(node) {
            return item.parent()
        }
        None
    }
}

impl TreeItem {
    pub fn new_category_with_name(name: impl Into<String>) -> Self {
        TreeItem {
            name: name.into(),
            item_type: ItemType::Category,
            data: ItemData::default(),
        }
    }

    pub fn new_equation_with_name(name: impl Into<String>) -> Self {
        TreeItem {
            name: name.into(),
            item_type: ItemType::Equation,
            data: ItemData::default(),
        }
    }
}
