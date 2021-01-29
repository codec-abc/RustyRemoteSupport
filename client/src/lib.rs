use seed::{prelude::*, *};
use bevy::{prelude::*};
use bevy::{
    input::{keyboard::KeyCode, Input},
};

use bevy::render::camera::Camera;

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

fn view(model: &Model) -> seed::prelude::Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Decrement),],
    ]
}

///////////////////// BEVY ////////////////////

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, time: Res<Time>, mut cam_query: Query<(&Camera, &mut Transform)>) {
    //info!("keyboard_input_system running!");
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");

        let delta_seconds = time.delta_seconds();

        for (_camera, mut transform) in cam_query.iter_mut() {
            let right = transform.rotation * Vec3::unit_x();
            transform.translation += right * delta_seconds;
        }
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
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

//see https://github.com/smokku/bevy/blob/master/examples/wasm/winit_wasm.rs
//and https://github.com/bevyengine/bevy/blob/master/examples/app/custom_loop.rs
//and https://github.com/bevyengine/bevy/blob/841755aaf23acfd55b375c37390daeb302c5b30b/examples/wasm/assets_wasm.rs

#[wasm_bindgen(start)]
pub fn start() {
    seed::App::start("app", init, update, view);

    let mut app = bevy::app::App::build();

    app
        .add_resource(WindowDescriptor {
            width: 300.,
            height: 300.,
            canvas: Some("#bevy_canvas".into()),
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        //.add_system(hello_wasm_system.system())
        .add_system(keyboard_input_system.system())
        .add_startup_system(setup.system())
        //.set_runner(my_runner)
        .run();
}