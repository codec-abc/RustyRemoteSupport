use seed::{prelude::*, *};
use shared;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    shared::ANSWER_TO_EVERYTHING
}

type Model = i32;

enum Msg {
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Decrement => *model -= 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Decrement),],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}