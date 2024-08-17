enum Tile {
    Empty,
    Tree,
    Grass,
    Tent,
}

struct Side {
    max_tents: usize,
    cur_tents: usize,
}

struct Level {
    rows: Vec<Side>,
    cols: Vec<Side>,
    tiles: Vec<Vec<Tile>>,
}