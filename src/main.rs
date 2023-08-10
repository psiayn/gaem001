use bevy::{prelude::*, window::WindowResolution};

mod engine;
use engine::player::load_player_sprite_sheet;
use engine::animation::{keyboard_input, animate_sprite};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.17, 0.6, 0.39)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Gaem 001".to_string(),
                        resolution: WindowResolution::default(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, load_player_sprite_sheet)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    // commands.spawn(PixelCameraBundle::from_resolution(320, 240, true));
    // commands.spawn(Camera2dBundle::default());
    commands.spawn({
        let mut bundle = Camera2dBundle::default();
        bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical(200.0);
        bundle
    });
}
