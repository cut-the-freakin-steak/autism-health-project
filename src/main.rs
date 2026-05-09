use bevy::prelude::*;

fn main() -> AppExit {
    return App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
enum ArrowDirection {
    Left,
    Right,
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("sonic.webp")),
        ArrowDirection::Up,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
