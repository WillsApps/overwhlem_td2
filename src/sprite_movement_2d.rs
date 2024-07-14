

//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;


pub struct SpriteMovement;
impl Plugin for SpriteMovement {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement);
    }
}
//
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(Update, sprite_movement)
//         .run();
// }

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_pixel_dark.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
