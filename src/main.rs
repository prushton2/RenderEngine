mod position;

fn main() {
    let pos1 = position::Vector3::new(1.0, 2.0, 3.0);
    let pos2 = position::Vector3::new(1.0, 2.0, 3.0);

    let angle = position::Angle::new(10.0);

    let rot = position::Rotation::new(
        position::Angle::new(70.0),
        position::Angle::new(80.0),
        position::Angle::new(90.0),
    );

    let sum = pos1.add(pos2);
    dbg!(&sum.x);
    dbg!(&sum.y);
    dbg!(&sum.z);

    dbg!(&angle.angle);

    dbg!(&rot.x.angle);
    dbg!(&rot.y.angle);
    dbg!(&rot.z.angle);

}