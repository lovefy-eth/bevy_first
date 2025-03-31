use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Person;

#[derive(Component,Debug)]
struct Name(String);

#[derive(Component)]
struct Enemy;

#[derive(Component,Deref)]
struct Health(u32);

#[derive(Resource)]
struct BattleTimer(Timer);

#[derive(Resource)]
struct GreetTimer(Timer);


struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BattleTimer(Timer::new(Duration::from_secs(5), TimerMode::Repeating)));
        app.add_systems(Startup, (init_player,init_enemy));
        app.add_systems(Update,battle);
    }
}

fn init_player(mut cmd: Commands) {
    cmd.spawn((Person,Health(1000),Name("JianBo".to_string())));
}

fn init_enemy(mut cmd: Commands) {
    cmd.spawn((Enemy,Health(100),Name("fish".to_string())));
    cmd.spawn((Enemy,Health(200),Name("cat".to_string())));
    cmd.spawn((Enemy,Health(300),Name("dog".to_string())));
}


fn battle(
    mut timer: ResMut<BattleTimer>,
    time: Res<Time>,
    input:Res<ButtonInput<KeyCode>>,
    mut enemy: Query<(&mut Health,&Name),With<Enemy>>,
    mut player: Single<(&mut Health,&Name),Without<Enemy>>,) {

    if input.just_pressed(KeyCode::Space) {
        println!("player: {}", player.1.0);
        for (mut health,name) in &mut enemy.iter_mut() {
            health.0 = health.0.saturating_sub(1);
            println!("{} lose 1 health ",name.0);
        }
    }

    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (mut health,name) in &mut enemy.iter() {
            println!("Name:{} Health:{}", name.0, health.0);
        }
    }
}


const X_EXTENT: f32 = 900.;

fn add_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("FengYu".to_string())));
    commands.spawn((Person, Name("JianBo".to_string())));

    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Circle::new(50.)),
        meshes.add(CircularSector::new(50.0, 1.0)),
        meshes.add(Rectangle::new(50.0, 50.0)),

    ];

    let num_shapes = shapes.len();

    for (i,shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360.* i as f32 / num_shapes as f32,0.95,0.7);
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(-X_EXTENT/2.+i as f32 /(num_shapes-1) as f32 * X_EXTENT,0.0,0.0))
        );


    }


    commands.spawn((
        Text::new("Left Arrow Key: Animate Left Sprite\nRight Arrow Key: Animate Right Sprite"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

struct Entity(u64);

fn hello_world() {
    println!("hello world!");
}

struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0,TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, ((update_people, greet_people).chain()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BattlePlugin)
        .run();
}
