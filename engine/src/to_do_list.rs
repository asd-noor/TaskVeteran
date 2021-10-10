use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Direction;
use petgraph::Graph;

#[derive(PartialEq)]
pub struct ToDoListItem {
    label: String,
    completed: bool,
}

impl ToDoListItem {
    pub fn new(label: &str) -> Self {
        ToDoListItem {
            label: String::from(label),
            completed: false,
        }
    }
}

#[derive(PartialEq)]
pub struct View {
    name: String,
}

impl View {
    pub fn new(name: String) -> Self {
        View { name: name }
    }
}

#[derive(PartialEq)]
pub enum NodeItem {
    Item(ToDoListItem),
    View(View),
    Root,
}

pub struct ToDoList {
    content: Graph<NodeItem, ()>,
}

impl ToDoList {
    pub fn new() -> ToDoList {
        let mut list = ToDoList {
            content: Graph::new(),
        };

        list.content.add_node(NodeItem::Root);

        list
    }

    pub fn add(&mut self, item: ToDoListItem, parent_: Option<usize>) -> Option<NodeIndex> {
        let child = self.content.add_node(NodeItem::Item(item));

        let parent = if let Some(parent) = parent_ {
            parent
        } else {
            0
        };

        self.content.add_edge(NodeIndex::new(parent), child, ());

        Some(child)
    }

    pub fn get(&self, id: usize) -> Option<&ToDoListItem> {
        match self.content.node_weight(NodeIndex::new(id)) {
            Some(NodeItem::Item(item)) => Some(item),
            _ => None,
        }
    }

    pub fn remove(&mut self, id: usize) -> Option<usize> {
        // Important: Since the index of the last node is going to change to `id` after this operation, any stored references to the last node by its index should be updated accordingly

        if id == 0 {
            // Prevent deletion of the root
            None
        } else {
            let children = self.deep_children(id);

            self.content.retain_nodes(|_, node| -> bool {
                children
                    .iter()
                    .position(|&i| -> bool {
                        let index = node.index();
                        i == index || id == index
                    })
                    .is_none()
            });

            if let Some(child) = children.get(0) {
                Some(*child)
            } else {
                None
            }
        }
    }

    // includes the parent
    pub fn deep_children(&self, id: usize) -> Vec<usize> {
        let mut dfs = Dfs::new(&self.content, NodeIndex::new(id));
        let mut children = vec![];
        while let Some(item) = dfs.next(&self.content) {
            children.push(item.index());
        }

        children
    }

    pub fn children(&self, index: usize) -> Vec<usize> {
        let mut items = vec![];
        for item in self
            .content
            .neighbors_directed(NodeIndex::new(index), Direction::Outgoing)
        {
            items.push(item.index());
        }
        items.reverse();
        items
    }
}

#[cfg(test)]
mod tests;
