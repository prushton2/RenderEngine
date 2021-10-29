mod position;
mod object;

fn main() {

    let square = object::Square::new(
        position::Vector3::new(0.0, 0.0, 0.0),
        position::Vector3::new(0.0, 1.0, 0.0),
        position::Vector3::new(1.0, 0.0, 0.0),
        position::Vector3::new(1.0, 1.0, 0.0),
    );
}