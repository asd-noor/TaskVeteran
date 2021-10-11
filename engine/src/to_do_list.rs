use crate::graph::NodeItem;
use crate::to_do_list_item::ToDoListItem;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::{Direction, Graph};

pub struct ToDoList {
    graph: Box<Graph<NodeItem, ()>>,
}

impl ToDoList {
    pub fn new() -> Self {
        let graph = Box::new(Graph::new());
        ToDoList::from_graph(graph)
    }

    pub fn from_graph(mut graph: Box<Graph<NodeItem, ()>>) -> Self {
        graph.add_node(NodeItem::Root);

        ToDoList { graph: graph }
    }

    pub fn add(&mut self, item: ToDoListItem, parent_: Option<usize>) -> Option<NodeIndex> {
        let child = self.graph.add_node(NodeItem::Item(item));

        let parent = if let Some(parent) = parent_ {
            parent
        } else {
            0
        };

        self.graph.add_edge(NodeIndex::new(parent), child, ());

        Some(child)
    }

    pub fn get(&self, id: usize) -> Option<&ToDoListItem> {
        match self.graph.node_weight(NodeIndex::new(id)) {
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

            self.graph.retain_nodes(|_, node| -> bool {
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
        let graph = &*self.graph;
        let mut dfs = Dfs::new(&graph, NodeIndex::new(id));
        let mut children = vec![];
        while let Some(item) = dfs.next(&graph) {
            children.push(item.index());
        }

        children
    }

    pub fn children(&self, index: usize) -> Vec<usize> {
        let mut items = vec![];
        for item in self
            .graph
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
