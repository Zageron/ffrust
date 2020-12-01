#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

fn create_note() -> Vec<Vertex> {
    let mut notes = Vec::new();

    for i in 0..10 {
        let angle: f32 = ((i as f32) / 10.0 * 360.0).to_radians();
        let x = 100.0 * angle.cos();
        let y = 100.0 * angle.sin();
        notes.push(Vertex {
            position: [x, y, 0.0],
            color: [0.5, 0.0, 0.5],
        });
    }

    notes
}

fn build_indicies_from_note(note: Vec<Vertex>) -> Vec<u16> {
    let mut indicies = Vec::new();

    for i in 0..note.len() {
        indicies.push(i as u16);
    }

    indicies
}
