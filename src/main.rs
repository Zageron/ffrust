use std::{
    convert::TryInto,
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

use bevy::prelude::*;

#[derive(Debug)]
struct Chart {
    notes: Vec<u32>,
    next_note: u32,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Note {
    spawned_at: u32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Lane {
    screen_position: Vec2,
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
        .add_resource(Lane {
            screen_position: Vec2::new(0.0, 0.0),
            direction: -1,
        })
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup)
        .add_startup_system(load_chart)
        .add_startup_system(initialize_lane)
        .add_system(animate_sprite_system)
        .add_system(spawn_notes_when_time)
        .add_system(animate_the_notes)
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
        next_note: 0,
    });
}

fn initialize_lane(windows: ResMut<Windows>, mut lane: ResMut<Lane>) {
    let wnd = windows.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let spawn_point: Vec2 = Vec2::new(0.0, size.y / 2.0);
    lane.screen_position.x = spawn_point.x;
    lane.screen_position.y = spawn_point.y;
}

fn animate_the_notes(mut query: Query<(&Note, &mut Transform)>) {
    for (note, mut transform) in query.iter_mut() {
        // This note is just the note. We also want the sprite position transoform.
        transform.translation.y += 1.0;
    }
}

fn spawn_notes_when_time(
    commands: &mut Commands,
    mut chart: ResMut<Chart>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if chart.next_note >= chart.notes.len() as u32 {
        return;
    }

    let mut note = chart.notes[chart.next_note as usize];

    while time.time_since_startup().as_millis() > note as u128 {
        // Do work with the chart, spawnt the notes and such. Note movement will be elsewhere.
        let texture_handle = asset_server.load("packed.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 8, 8);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..Default::default()
            })
            .with(Note { spawned_at: note });

        chart.next_note += 1;

        if chart.next_note >= chart.notes.len() as u32 {
            return;
        }

        note = chart.notes[chart.next_note as usize];
    }

    // DONE: I have a chart with 10 notes
    // it is single lane.
    // Roll the note up the screen from the bottom
    // Spawn it when the note is supposed to be hit for now.

    // Pick a position for the lane.
    // Figure out the bitmagic
    //

    // Set up a timer for deciding when to play the notes.

    // The timer needs to count up and
}

/// set up a simple 3D scene
fn setup(commands: &mut Commands) {
    commands
        // camera
        .spawn(Camera2dBundle::default());
}
