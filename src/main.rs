use std::{
    fs::File,
    io::{BufReader, Read},
};

use bevy::prelude::*;

#[derive(Default)]
struct NoteskinAtlas {
    texture_atlas_handle: Handle<TextureAtlas>,
}

#[derive(Debug)]
struct Chart {
    notes: Vec<u32>,
    next_note: u32,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Note {
    pub spawned_at: u128,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Lane {
    screen_position: Vec2,
    direction: i8,
}

fn main() {
    let mut app = App::build();
    app.init_resource::<NoteskinAtlas>()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
        .add_system(process_note_lifetime)
        .add_system(keyboard_input_update)
        .add_resource(Timer::from_seconds(2.0, false))
        .run();
}

fn keyboard_input_update(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(Entity, &Note)>,
    time: Res<Time>,
) {
    let current_time = time.time_since_startup().as_millis();

    for (_entity, note) in query.iter() {
        // Probably need the lane? Lane info based on how long it's going to take the note to get to the top of the lane.
        // Lane logic needs to be not ass.
        let target_lesser = note.spawned_at + 1250;
        let target_greater = note.spawned_at - 1250;
        let pressed = keyboard_input.just_pressed(KeyCode::Space);
        if pressed {
            println!("Space was pressed!");
            if current_time > target_greater && current_time < target_lesser {
                println!("Hit the note?");
            }
        }
    }

    // If we hit and there is a note that is within "some varialbe" of the "expected state" then ok!

    // Time spawned - expected time to reach target. (Need to calculate this expectation.)
    // if we are whithin a certain number of milliseconds of this, then destroy the note! And print a yay.
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

fn initialize_lane(
    commands: &mut Commands,
    windows: ResMut<Windows>,
    mut lane: ResMut<Lane>,
    noteskin_atlas: Res<NoteskinAtlas>,
) {
    let wnd = windows.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let spawn_point: Vec2 = Vec2::new(0.0, size.y / 2.0);
    lane.screen_position.x = spawn_point.x;
    lane.screen_position.y = spawn_point.y;

    // Also place some entities for the appropriate receptor.
    commands.spawn(SpriteSheetBundle {
        texture_atlas: noteskin_atlas.texture_atlas_handle.clone_weak(),
        transform: Transform {
            translation: Vec3::new(0.0, lane.screen_position.y - 96.0, 0.0),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(36 as u32),
        ..Default::default()
    });
}

fn process_note_lifetime(commands: &mut Commands, query: Query<(Entity, &Note)>, time: Res<Time>) {
    let current_time = time.time_since_startup().as_millis();

    for (entity, note) in query.iter() {
        if current_time > note.spawned_at + 5000 {
            println!("You missed so long ago.");
            commands.despawn(entity);
        }
    }
}

fn animate_the_notes(mut query: Query<(&Note, &mut Transform)>) {
    for (_note, mut transform) in query.iter_mut() {
        // This note is just the note. We also want the sprite position transoform.
        transform.translation.y += 0.1;
    }
}

fn spawn_notes_when_time(
    commands: &mut Commands,
    mut chart: ResMut<Chart>,
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    noteskin_atlas: Res<NoteskinAtlas>,
) {
    if !timer.finished() {
        timer.tick(time.delta().as_secs_f32());
        return;
    }

    if chart.next_note >= chart.notes.len() as u32 {
        return;
    }

    let mut note = chart.notes[chart.next_note as usize];

    let current_time = time.time_since_startup().as_millis();
    while current_time > note as u128 {
        // Do work with the chart, spawnt the notes and such. Note movement will be elsewhere.

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: noteskin_atlas.texture_atlas_handle.clone_weak(),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..Default::default()
            })
            .with(Note {
                spawned_at: current_time,
            });

        chart.next_note += 1;

        if chart.next_note >= chart.notes.len() as u32 {
            return;
        }

        note = chart.notes[chart.next_note as usize];
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut noteskin_atlas: ResMut<NoteskinAtlas>,
) {
    let texture_handle = asset_server.load("packed.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 8, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    noteskin_atlas.texture_atlas_handle = texture_atlas_handle;

    commands.spawn(Camera2dBundle::default());
}
