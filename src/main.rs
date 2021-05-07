use bevy::prelude::*;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn main() {
 App::build()
 .add_startup_system(setup.system())
 .add_system(snake_movement.system())
 .add_plugins(DefaultPlugins)
 .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system())) // <--
 .run();
}



fn spawn_snake( mut commands: Commands, materials: Res<Materials>){
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
    })
    .insert(SnakeHead);
}

fn setup(mut commands: Commands,mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7,0.7,0.7).into()),
    });
    
}

fn snake_movement(
    mut commands: Commands, materials: Res<Materials>,
    keyboard_input: Res<Input<KeyCode>>,
    mut head_posistions: Query<(With<SnakeHead>, &mut Transform)>){
    for (_head, mut transform) in head_posistions.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            commands
    .spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(3.0, 3.0)),
        ..Default::default()
    })
    .insert(SnakeHead);
        }
    }
}


#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
struct SnakeHead;
struct bullet;
struct Materials {
    head_material: Handle<ColorMaterial>,
}