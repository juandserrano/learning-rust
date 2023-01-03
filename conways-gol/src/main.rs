use bevy::prelude::*;

#[derive(Component)]
struct Dot {
    position: Position,
    is_alive: bool,
    sprite: String,
}

impl Dot {
    fn new(x: i32, y: i32, is_alive: bool) -> Self {
        Self {
            position: Position {x, y},
            is_alive,
            sprite: if is_alive {
                "sprites/background_green.png".to_string()
            } else {"sprites/background_blue.png".to_string() }
        }
    }

    fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn(SpriteBundle {
            texture: asset_server.load(&self.sprite),
            transform: Transform::from_xyz(self.position.x as f32, self.position.y as f32, 1.0)
            .with_scale(Vec3::new(0.1, 0.1, 1.0)),
            ..default()

        });
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Resource)]
struct MyTimer(Timer);

fn setup(mut commands: Commands, query: Query<With<Dot>>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    for i in -500..500 {
        for j in -500..500 {
    commands.spawn(Dot::new(i, j, false));
    //commands.spawn(SpriteBundle {
    //    texture: asset_server.load("sprites/background_green.png"),
    //    transform: Transform::from_xyz(i as f32, j as f32, 1.).with_scale(Vec3::new(0.1, 0.1, 1.0)),
    //    ..default()
    //});
            
        }
        
    }
    for dot in query.iter() {
        dot.render(&mut commands, &asset_server);
    }
}

fn hello(mut commands: Commands, time: Res<Time>, mut timer: ResMut<MyTimer>,query: Query<&Dot>) {
    if timer.0.tick(time.delta()).just_finished() {
    commands.spawn(Dot::new(20, 30, true));

    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_startup_system(setup)
        .run()
}
