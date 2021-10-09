use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Direction;
use petgraph::Graph;

pub struct ToDoListItem {
    label: String,
}

impl ToDoListItem {
    pub fn new(label: &str) -> ToDoListItem {
        ToDoListItem {
            label: String::from(label),
        }
    }
}

// pub struct View {

// }

// pub enum NodeType {
//     ToDoListItem,
//     View,
// }

pub struct ToDoList {
    content: Graph<ToDoListItem, ()>,
}

impl ToDoList {
    pub fn new() -> ToDoList {
        let mut list = ToDoList {
            content: Graph::new(),
        };

        list.add(
            ToDoListItem {
                label: String::from("root"),
            },
            None,
        );

        list
    }

    pub fn add(&mut self, item: ToDoListItem, parent_: Option<usize>) -> Option<NodeIndex> {
        let child = self.content.add_node(item);

        if let Some(parent) = parent_ {
            self.content.add_edge(NodeIndex::new(parent), child, ());
        }

        Some(child)
    }

    pub fn get(&self, id: usize) -> Option<&ToDoListItem> {
        self.content.node_weight(NodeIndex::new(id))
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
mod tests {
    use super::*;
    #[test]
    fn gets_nothing() {
        let list = ToDoList::new();

        assert!(list.get(1).is_none());
    }
    #[test]
    fn root_exists() {
        let list = ToDoList::new();

        assert_eq!(1, list.content.node_count());
        assert_eq!(0, list.content.edge_count());
        assert!(list
            .content
            .node_indices()
            .find(|i| i.index() == 0)
            .is_some());

        assert_eq!(
            "root",
            match list.get(0) {
                Some(item) => item.label.as_str(),
                None => "",
            }
        );
    }
    #[test]
    fn gets_children() {
        let mut list = ToDoList::new();

        list.add(ToDoListItem::new("item1"), Some(0));
        list.add(ToDoListItem::new("item2"), Some(0));
        list.add(ToDoListItem::new("item3"), Some(0));

        let children = list.children(0);
        assert_eq!(3, children.len());
        assert_eq!("item1", list.get(children[0]).unwrap().label);
        assert_eq!("item2", list.get(children[1]).unwrap().label);
        assert_eq!("item3", list.get(children[2]).unwrap().label);
    }
    #[test]
    fn removes_node() {
        let mut list = ToDoList::new();

        list.add(ToDoListItem::new("item1"), Some(0));
        list.add(ToDoListItem::new("item2"), Some(0));
        list.add(ToDoListItem::new("item3"), Some(0));

        let removed_node_ = list.remove(2);
        assert!(removed_node_.is_some());
        assert_eq!(2, removed_node_.unwrap());
        assert_eq!(
            "item3",
            list.content.node_weight(NodeIndex::new(2)).unwrap().label
        );
        assert_eq!(
            "item1",
            list.content.node_weight(NodeIndex::new(1)).unwrap().label
        );
        assert_eq!(3, list.content.node_count());
    }
    #[test]
    fn ignores_root_removal() {
        let mut list = ToDoList::new();

        list.add(ToDoListItem::new("item1"), Some(0));
        list.add(ToDoListItem::new("item2"), Some(0));
        list.add(ToDoListItem::new("item3"), Some(0));

        assert!(list.remove(0).is_none());

        assert_eq!(list.content.node_count(), 4);
        assert_eq!(list.content.edge_count(), 3);
    }
    #[test]
    fn deep_remove_node() {
        let mut list = ToDoList::new();

        list.add(ToDoListItem::new("item1"), Some(0));
        list.add(ToDoListItem::new("item2"), Some(1));
        list.add(ToDoListItem::new("item3"), Some(2));

        assert!(list.remove(1).is_some());

        assert_eq!(list.content.node_count(), 1);
        assert_eq!(list.content.edge_count(), 0);
    }
    #[test]
    fn finds_deep_children() {
        let mut list = ToDoList::new();

        list.add(ToDoListItem::new("item1"), Some(0)); // 1
        list.add(ToDoListItem::new("item2"), Some(1)); // 2
        list.add(ToDoListItem::new("item3"), Some(2)); // 3
        list.add(ToDoListItem::new("item4"), Some(2)); // 4
        list.add(ToDoListItem::new("item5"), Some(3)); // 4

        assert_eq!(list.deep_children(2), vec![2, 3, 5, 4]);
    }
}
