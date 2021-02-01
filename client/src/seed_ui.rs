use std::{borrow::BorrowMut, sync::{Arc, Mutex}};
use std::sync::mpsc::{Sender, Receiver};
use bevy::prelude::AppBuilder;
use enclose::enc;

use seed::{*, app::OrdersContainer, prelude::*};

use crate::bevy_engine::create_seed_channel;
use crate::bevy_engine::create_bevy_app;

use shared;

//#[derive(Debug)]
struct Model {
    counter: i32,
    bevy_engine: Option<SeedBevyIntegration>
    // 
    //tx: Sender<BevyMsg>
}

struct SeedBevyIntegration {
    bevy_event_loop: Arc<Mutex<Vec<String>>>,
    rx: Receiver<BevyMsg>,
    //bevy_app: AppBuilder,
}

#[derive(Debug)]
pub enum BevyMsg {
    Debug(String),
}

pub enum Msg {
    Decrement,
    JsUpdate,
    JsReady(bool),
}

fn my_runner(mut app: bevy::app::App) {
    log!("my_runner");
    //app.update();
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Decrement => {
            model.counter -= 1;
            // let mut mutex_guard = model.bevy_event_loop.lock().unwrap();
            // let bevy_event_loop = mutex_guard.borrow_mut();
            // bevy_event_loop.push(format!("new value to set in bevy {}", model.counter));
        }

        Msg::JsReady(ready) => {
            if ready {
                log!("Js is ready.");

                let bevy_inner_loop = Arc::new(Mutex::new(vec!()));
                let cloned_inner_loop_ref = bevy_inner_loop.clone();
                let (tx, rx) = create_seed_channel();

                let mut bevy_app = create_bevy_app(
                    bevy_inner_loop.clone(),
                    tx
                );

                //bevy_app.set_runner(my_runner).run();

                // bevy_app.set_runner(|bevy_application| {
                //     log!("my_runner");
                //     bevy_application.run();
                // });

                bevy_app.run();

                model.bevy_engine = Some(SeedBevyIntegration {
                    bevy_event_loop: bevy_inner_loop,
                    rx: rx,
                    //bevy_app: bevy_app
                });

                //model.bevy_engine.as_mut().unwrap().bevy_app.app.initialize();

                orders.perform_cmd(async { enableTick() });

                log!("Js ready is done.");
            }
        }

        Msg::JsUpdate => {
            //log!("update from js");

            //model.bevy_engine.as_mut().unwrap().bevy_app.app.update();

            // let received_result = model.rx.try_recv();
            
            // match received_result {
            //     Ok(msg) => {
            //         log!("bevy update {:?}", msg);
            //     }
            //     Err(e) => {
            //         //log!("error {:?}", e);
            //     }
            // }
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

#[wasm_bindgen]
pub fn start() -> Box<[JsValue]> {

    let init: Box<dyn FnOnce(Url, &mut OrdersContainer<Msg, Model, Node<Msg>>) -> Model> = Box::new(move |_, _| {
        Model {
            counter: shared::ANSWER_TO_EVERYTHING,
            bevy_engine: None
        }
    });

    let seed_app = seed::App::start("app", init, update, view);
    let closures = create_closures_for_js(&seed_app);
    closures
}

fn create_closures_for_js(app: &App<Msg, Model, Node<Msg>>) -> Box<[JsValue]> {
    let js_tick = wrap_in_permanent_closure(enc!((app) move |_: i32| {
        app.update(Msg::JsUpdate)
    }));

    let js_ready = wrap_in_permanent_closure(enc!((app) move |ready: bool| {
        app.update(Msg::JsReady(ready))
    }));

    vec![js_ready, js_tick].into_boxed_slice()
}

fn wrap_in_permanent_closure<T>(f: impl FnMut(T) + 'static) -> JsValue
where
    T: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    // `Closure::new` isn't in `stable` Rust (yet) - it's a custom implementation from Seed.
    // If you need more flexibility, use `Closure::wrap`.
    let closure = Closure::new(f);
    let closure_as_js_value = closure.as_ref().clone();
    // `forget` leaks `Closure` - we should use it only when
    // we want to call given `Closure` more than once.
    closure.forget();
    closure_as_js_value
}

#[wasm_bindgen]
extern "C" {
    fn enableTick();
}