use alloc::string::String;
use indextree::{Arena, NodeId};

pub struct EqTree {
    arena: Arena<String>,
    root: NodeId
}

impl EqTree {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node("root".into());
        EqTree { arena, root }
    }

    pub fn add_child(&mut self, parent: NodeId, data: impl Into<String>) -> NodeId {
        let child = self.arena.new_node(data.into());
        parent.append(child, &mut self.arena);
        return child;
    }
}
