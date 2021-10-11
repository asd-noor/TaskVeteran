use crate::to_do_list_item::ToDoListItem;
use crate::view::View;

#[derive(PartialEq)]
pub enum NodeItem {
    Item(ToDoListItem),
    View(View),
    Root,
}
