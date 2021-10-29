mod position;
mod object;

fn main() {

    let cube = object::Cube::new(
        position::Vector3::new(0.0, 0.0, 0.0),
        position::Vector3::new(1.0, 1.0, 1.0),
    );
}