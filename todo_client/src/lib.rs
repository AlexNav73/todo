#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

mod todo_item;

use yew::prelude::*;

use todo_item::{ToDoItem, ToDoItemMsg};

pub struct Model {
    value: String,
    items: Vec<ToDoItem>
}

pub enum Msg {
    OnTextInput(String),
    AddNewItem,
    ToDoItem(ToDoItemMsg)
}

impl Model {
    fn render_list<CTX>(&self) -> Html<CTX, Self>
        where CTX: 'static
    {
        html! {
            <ul>{
                for self.items.iter().map(|item| item.view())
            }</ul>
        }
    }
}

impl<CTX> Component<CTX> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            value: "".into(),
            items: vec![]
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::OnTextInput(new_value) => {
                self.value = new_value;
            }
            Msg::AddNewItem => {
                let idx = self.items.len();
                let new_item = self.value.clone();
                self.value = "".into();
                self.items.push(ToDoItem::new(idx, new_item));
            }
            Msg::ToDoItem(inner) => self.items.iter_mut().for_each(|x| x.handle(&inner))
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <div>
                    <input
                        value=&self.value,
                        oninput=|e: InputData| Msg::OnTextInput(e.value),
                        placeholder="placeholder",>
                    </input>
                    <button onclick=|_| Msg::AddNewItem,>{ "Add Item" }</button>
                </div>
                <div>{ self.render_list() }</div>
            </div>
        }
    }
}
