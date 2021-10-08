use petgraph::graph::NodeIndex;
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

    pub fn add(&mut self, item: ToDoListItem, parent_: Option<NodeIndex>) -> Option<NodeIndex> {
        let child = self.content.add_node(item);
        match parent_ {
            Some(parent) => {
                self.content.add_edge(parent, child, ());
            }
            None => {}
        }

        Some(child)
    }

    pub fn get(&self, id: usize) -> Option<&ToDoListItem> {
        self.content.node_weight(NodeIndex::new(id))
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
        let root = NodeIndex::from(0);

        list.add(ToDoListItem::new("item1"), Some(root));
        list.add(ToDoListItem::new("item2"), Some(root));
        list.add(ToDoListItem::new("item3"), Some(root));

        let children = list.children(0);
        assert_eq!(3, children.len());
        assert_eq!("item1", list.get(children[0]).unwrap().label);
        assert_eq!("item2", list.get(children[1]).unwrap().label);
        assert_eq!("item3", list.get(children[2]).unwrap().label);
    }
}
