enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::West;
    steer(dir);
}

fn steer(dir: Direction) {
    match dir {
        Direction::North => println!("North Direction"),
        Direction::South => println!("South Direction"),
        _ => print!("Horizontal direction"),
    }
}
