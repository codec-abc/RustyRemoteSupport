use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

use seed::{*, app::OrdersContainer, prelude::*};
use crate::bevy_engine::create_bevy_app;
use shared;


#[derive(Clone, Default)]
struct Model {
    bevy_event_loop: Arc<Mutex<Vec<String>>>,
    counter: i32,
}

enum Msg {
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Decrement => {
            model.counter -= 1;
            let mut mutex_guard = model.bevy_event_loop.lock().unwrap();
            let bevy_event_loop = mutex_guard.borrow_mut();
            bevy_event_loop.push(format!("new value in bevy {}", model.counter));
        }
    }
}

fn view(model: &Model) -> seed::prelude::Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Decrement),],
    ]
}


#[wasm_bindgen(start)]
pub fn start() {
    let bevy_inner_loop = Arc::new(Mutex::new(vec!()));

    let cloned_inner_loop_ref = bevy_inner_loop.clone();

    let init: Box<dyn FnOnce(Url, &mut OrdersContainer<Msg, Model, Node<Msg>>) -> Model> = Box::new(move |_, _| {
        Model {
            bevy_event_loop: cloned_inner_loop_ref,
            counter: shared::ANSWER_TO_EVERYTHING,
        }
    });

    seed::App::start("app", init, update, view);
    
    let mut bevy_app = create_bevy_app(bevy_inner_loop.clone());
    bevy_app.run();
}