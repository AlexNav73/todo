
use yew::prelude::*;

use ::{Model, Msg};

pub struct ToDoItem {
    id: usize,
    value: String,
    checked: bool
}

pub enum ToDoItemMsg {
    CheckItem(usize)
}

impl ToDoItem {
    pub fn new(id: usize, value: String) -> Self {
        ToDoItem { checked: false, id, value }
    }

    pub fn handle(&mut self, msg: &ToDoItemMsg) {
        match msg {
            ToDoItemMsg::CheckItem(id) if *id == self.id => {
                self.checked = !self.checked;
            }
            _ => {}
        };
    }

    pub fn view<CTX: 'static>(&self) -> Html<CTX, Model> {
        let id = self.id;
        html! {
            <div>
                <input
                    type="checkbox",
                    checked=self.checked,
                    onclick=move |_| Msg::ToDoItem(ToDoItemMsg::CheckItem(id)),>
                </input>
                <span
                    class=if self.checked == true { "item-checked" } else { "" },>
                    {&self.value}
                </span>
            </div>
        }
    }
}
