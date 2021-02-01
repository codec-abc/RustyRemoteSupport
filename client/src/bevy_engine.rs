use std::{borrow::BorrowMut, ops::DerefMut, sync::{Arc, Mutex}};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use bevy::{prelude::*};
use bevy::{
    input::{keyboard::KeyCode, Input},
};

use bevy::render::camera::Camera;
use seed::{main, virtual_dom::Mailbox};
use crate::seed_ui::{Msg, BevyMsg};

/// This system prints 'A' key state
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    time: Res<Time>, 
    mut cam_query: Query<(&Camera, &mut Transform)>,
    mut seed_channel: ResMut<SeedMsgSender>
) 
{
    //info!("keyboard_input_system running!");
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");

        let delta_seconds = time.delta_seconds();

        for (_camera, mut transform) in cam_query.iter_mut() {
            let right = -transform.rotation * Vec3::unit_x();
            transform.translation += right * delta_seconds;
        }
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
        let mut seed_msg_sender = seed_channel.deref_mut().sender.lock().unwrap();
        seed_msg_sender.send(BevyMsg::Debug("A pushed".into()));
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

pub struct ExternalEvent {
    messages: Arc<Mutex<Vec<String>>>
}

impl ExternalEvent {
    pub fn new(messages: Arc<Mutex<Vec<String>>>) -> Self { 
        Self { 
            messages
        } 
    }
}

pub struct SeedMsgSender {
    pub sender: Mutex<Sender<BevyMsg>>
}

impl SeedMsgSender {
    pub fn new(sender: Sender<BevyMsg>) -> Self { 
        Self { 
            sender: Mutex::new(sender)
        } 
    }
}

pub fn create_seed_channel() -> (Sender<BevyMsg>, Receiver<BevyMsg>) {
    let (tx, rx): (Sender<BevyMsg>, Receiver<BevyMsg>) = mpsc::channel();
    (tx, rx)
}

fn external_event_loop_runner(mut external_event_loop: ResMut<ExternalEvent>) {
    let external_event_loop = external_event_loop.deref_mut();
    let mut messages = external_event_loop.messages.lock().unwrap();
    let messages = messages.deref_mut();

    for message in messages.iter() {
        info!("We have got a message {}", message);
    }

    messages.clear();
}

pub fn create_bevy_app(
    inner_external_event_loop: Arc<Mutex<Vec<String>>>,
    sender: Sender<BevyMsg>
) -> AppBuilder
{
    let mut app = bevy::app::App::build();

    app
        .add_resource(WindowDescriptor {
            width: 300.,
            height: 300.,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy_canvas".into()),
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_resource(ExternalEvent::new(inner_external_event_loop))
        .add_resource(SeedMsgSender::new(sender))
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_system(keyboard_input_system.system())
        .add_system(external_event_loop_runner.system())
        .add_startup_system(setup.system());

    app
}

//see https://github.com/smokku/bevy/blob/master/examples/wasm/winit_wasm.rs
//and https://github.com/bevyengine/bevy/blob/master/examples/app/custom_loop.rs
//and https://github.com/bevyengine/bevy/blob/841755aaf23acfd55b375c37390daeb302c5b30b/examples/wasm/assets_wasm.rs
