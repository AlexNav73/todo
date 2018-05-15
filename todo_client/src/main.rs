extern crate yew;
extern crate todo_client;
extern crate stdweb;

use yew::prelude::*;
use todo_client::Model;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
