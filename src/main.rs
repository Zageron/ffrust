use std::{
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

use bevy::{prelude::*, window::WindowId};

#[derive(Debug)]
struct Chart {
    notes: Vec<u32>,
}

#[derive(Debug)]
struct Lane {
    screen_position: f32,
    direction: i8,
}

fn main() {
    let mut app = App::build();
    app.add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "FFR Rust Edition".to_string(),
            width: 800,
            height: 600,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup)
        .add_startup_system(load_chart)
        .add_system(animate_sprite_system)
        .add_system(animate_the_lane)
        .run();
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn load_file() -> std::io::Result<Vec<u32>> {
    let file = File::open("assets/chart01")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let data: Vec<u32> = contents
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    Ok(data)
}

fn load_chart(commands: &mut Commands) {
    let value = load_file();
    commands.insert_resource(Chart {
        notes: value.unwrap(),
    });
}

fn animate_the_lane(chart: Res<Chart>, time: Res<Time>, windows: Res<Windows>) {
    let delta_time: Duration = time.delta();
    // delta_time.as_millis()
    // Get the size of the window
    let wnd = windows.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    println!("{:?}", size);

    // DONE: I have a chart with 10 notes
    // it is single lane.
    // Queue the notes in an MS queue. sure
    // Roll the note up the screen from the bottom
    // Spawn it when the note is supposed to be hit for now.

    // Pick a position for the lane.
    // Figure out the bitmagic
    //

    // Set up a timer for deciding when to play the notes.

    // The timer needs to count up and
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("packed.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 8, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        // camera
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true));
}
