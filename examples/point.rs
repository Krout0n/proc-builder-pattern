use proc_builder_pattern::Builder;

#[derive(Default, Debug, Builder)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    // Point { x: 1, y: 2 }
    dbg!(Point::default().x(1).y(2));
}
