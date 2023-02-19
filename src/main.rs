use bevy::{
    prelude::{Component, App, Query, Commands, AssetServer, Res, Camera2dBundle, Transform, Vec3, Quat}, 
    DefaultPlugins, sprite::SpriteBundle, time::Time
};
use std::{fmt, f32::consts::PI};
use rand::Rng;

fn spawn_plane(commands: &mut Commands, asset_server: &Res<AssetServer>, Position { x, y }: Position, Movement { heading, speed }: Movement) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/plane_icon.png"),
            transform: Transform { 
                scale: Vec3{
                    x: 0.1,
                    y: 0.1,
                    z: 0.1
                },
                translation: Vec3 {
                    x,
                    y,
                    z: 0.
                },
                rotation: Quat::from_rotation_z(-heading*PI/180.)
            },
            ..SpriteBundle::default()
        },
        Plane,
        Movement {
            heading,
            speed,
        },
        Position{
            x, 
            y
        }));
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

impl Position {
    fn distance(&self, other: &Self) -> f32 {
        return ((self.x-other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Component)]
struct Movement {
    heading: f32,
    speed: f32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.02} E, {:.02} N", self.x, self.y)
    }
}

#[derive(Component)]
struct Plane;

#[derive(Component)]
struct GroundStation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        // .add_system(tell_positions)
        .run();    
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tower_handle = asset_server.load("sprites/tower_icon.png");
    let mut rng = rand::thread_rng();

    let mut random_plane = || {
        spawn_plane(&mut commands, &asset_server, Position { x: (rng.gen::<f32>()-0.5)*300., y: (rng.gen::<f32>()-0.5)*300. }, Movement { heading: rng.gen::<f32>()*360., speed: 20.+rng.gen::<f32>()*30. });
    };
    for _ in 0..10 {
        random_plane();
    }

    commands.spawn((
        SpriteBundle {
            texture: tower_handle,
            transform: Transform {
                scale: Vec3 {
                    x: 0.05,
                    y: 0.05,
                    z: 0.05
                },
                ..Transform::default()
            },
            ..SpriteBundle::default()
        },
        GroundStation,
        Position{x: 0.0, y: 0.0}));

    commands.spawn(Camera2dBundle::default());
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Position, &mut Transform, &Movement)>) {
    for (mut position, mut transform, movement) in &mut sprite_position {

        let xdiff = movement.speed * time.delta_seconds() * f32::sin(movement.heading*PI/180.);
        let ydiff = movement.speed * time.delta_seconds() * f32::cos(movement.heading*PI/180.);

        position.x += xdiff;
        position.y += ydiff;
        transform.translation.x += xdiff;
        transform.translation.y += ydiff;
    }
}
