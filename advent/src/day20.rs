use itertools::Itertools;
use std::collections::HashMap;

type Edge = String;
type Grid = Vec<Vec<char>>;
type Image = Vec<Vec<Grid>>;

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    edges: Vec<Edge>,
    chars: Grid,
    rotation: usize,
    flipped: bool,
}

impl Tile {
    pub fn reorient(&mut self) {
        let len = self.chars.len();
        let mut new = vec![vec![' '; len]; len];

        for i in 0..len {
            for j in 0..len {
                if self.flipped && self.rotation == 0 {
                    new[i][j] = self.chars[len - j - 1][len - i - 1];
                } else {
                    new[i][j] = self.chars[j][len - i - 1];
                }
            }
        }
        self.chars = new;
        self.rotation = (self.rotation + 90) % 360;
        if self.rotation == 0 {
            self.flipped = !self.flipped;
        }
    }

    pub fn new(id: usize, edges: Vec<Edge>, chars: Grid) -> Tile {
        Tile {
            id,
            edges,
            chars,
            rotation: 0,
            flipped: false,
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let tiles = parse_input(input);
    let tile_edges = get_tile_edges(&tiles.values().collect_vec());

    tile_edges
        .into_iter()
        .filter(|(_, value)| value.len() == 4)
        .map(|(id, _)| id)
        .product()
}

pub fn part_2(input: &str) -> usize {
    let mut grid = assemble_image(input);
    let mut sea_monsters = 0;
    while sea_monsters == 0 {
        sea_monsters = count_sea_monsters(&grid.chars);
        grid.reorient();
    }
    grid.chars
        .into_iter()
        .flatten()
        .filter(|c| *c == '#')
        .count()
        - (sea_monsters * SEA_MONSTER.len())
}

// Not as much of a monster as the entirety of this file
const SEA_MONSTER: &[(usize, usize)] = &[
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

fn count_sea_monsters(chars: &Grid) -> usize {
    let height = SEA_MONSTER.iter().map(|(x, _)| x).max().unwrap() + 1;
    let width = SEA_MONSTER.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut count = 0;
    for i in 0..chars.len() - height {
        for j in 0..chars[0].len() - width {
            if SEA_MONSTER.iter().all(|(x, y)| chars[x + i][y + j] == '#') {
                count += 1;
            }
        }
    }
    count
}

fn assemble_image(input: &str) -> Tile {
    let mut tiles = parse_input(input);
    let size = (tiles.len() as f64).sqrt() as usize;
    let mut tile_edges = get_tile_edges(&tiles.values().collect_vec());

    // Start from an arbitrary corner and find the correct orientation via trial and error
    let (first_id, _) = tile_edges
        .iter()
        .find(|(_, value)| value.len() == 4)
        .unwrap();
    let first_corner = tiles.get_mut(&first_id).unwrap();

    tile_edges.remove(&first_corner.id);

    let mut image = None;
    while image.is_none() {
        first_corner.reorient();
        image = assemble_tiles(
            first_corner.chars.clone(),
            size,
            parse_input(input),
            tile_edges.clone(),
        );
    }

    let image = image.unwrap();
    let tile_width = image[0][0].len();
    let grid_width = image.len();

    let mut full_image =
        vec![vec!['.'; grid_width * (tile_width - 2)]; grid_width * (tile_width - 2)];
    for i in 0..grid_width {
        for j in 0..grid_width {
            for x in 1..tile_width - 1 {
                for y in 1..tile_width - 1 {
                    if image[i][j][x][y] == '#' {
                        full_image[i * (tile_width - 2) + x - 1][j * (tile_width - 2) + y - 1] =
                            '#';
                    }
                }
            }
        }
    }
    Tile::new(1, Vec::new(), full_image)
}

fn assemble_tiles(
    start_corner: Grid,
    size: usize,
    mut tiles: HashMap<usize, Tile>,
    mut tile_edges: HashMap<usize, Vec<Edge>>,
) -> Option<Image> {
    let mut image = vec![vec![Vec::new(); size]; size];
    image[0][0] = start_corner;

    for i in 0..size {
        for j in 0..size {
            if i == 0 && j == 0 {
                continue;
            }

            let mut top_edge = "".into();
            let mut left_edge = "".into();
            if i != 0 {
                top_edge = bottom_row(&image[i - 1][j]);
            }
            if j != 0 {
                left_edge = right_column(&image[i][j - 1]);
            }

            // Each edge is unique so finding one edge is enough to know where the tile goes on the grid
            // Try all orientations for the unique tile until finding the one matching the edge
            let has_neighbor = tile_edges
                .iter()
                .find(|(_, edges)| has_edges(edges, &top_edge, &left_edge));

            has_neighbor?;

            let neighbor = tiles.get_mut(&has_neighbor.unwrap().0).unwrap();
            orient_tile(neighbor, &top_edge, &left_edge);

            tile_edges.remove(&neighbor.id);
            image[i][j] = neighbor.chars.clone();
        }
    }
    debug_assert!(tile_edges.is_empty());
    Some(image)
}

fn orient_tile(tile: &mut Tile, top_edge: &str, left_edge: &str) {
    loop {
        let top = &top_row(&tile.chars);
        let left = &left_column(&tile.chars);
        if (top_edge.is_empty() || top_edge == top) && (left_edge.is_empty() || left_edge == left) {
            break;
        } else {
            tile.reorient()
        }
    }
}

fn get_tile_edges(tiles: &[&Tile]) -> HashMap<usize, Vec<Edge>> {
    let mut edge_count = HashMap::new();
    for tile in tiles {
        for edge in &tile.edges {
            *edge_count.entry(edge).or_insert(0) += 1;
        }
    }

    let mut tile_edges = HashMap::new();
    for tile in tiles {
        for edge in &tile.edges {
            let possible_edge = *edge_count.get(&edge).unwrap() == 2;
            if possible_edge {
                tile_edges
                    .entry(tile.id)
                    .or_insert_with(Vec::new)
                    .push(edge.clone());
            }
        }
    }

    tile_edges
}

fn parse_input(input: &str) -> HashMap<usize, Tile> {
    let mut tiles = HashMap::new();
    for section in input.split_terminator("\n\n") {
        let id_line = section.lines().next().unwrap();
        let id = id_line[5..9].parse::<usize>().unwrap();

        let chars = section
            .lines()
            .skip(1)
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut edges = vec![
            top_row(&chars),
            bottom_row(&chars),
            left_column(&chars),
            right_column(&chars),
        ];
        let reversed_edges = edges
            .iter()
            .map(|edge| edge.chars().rev().join(""))
            .collect_vec();
        edges.extend(reversed_edges);

        tiles.insert(id, Tile::new(id, edges, chars));
    }
    tiles
}

fn has_edges(edges: &[String], top_edge: &Edge, left_edge: &Edge) -> bool {
    (left_edge.is_empty() || edges.contains(left_edge))
        && (top_edge.is_empty() || edges.contains(top_edge))
}

fn top_row(chars: &Grid) -> Edge {
    chars[0].iter().copied().collect()
}

fn bottom_row(chars: &Grid) -> Edge {
    chars[chars.len() - 1].iter().copied().collect()
}

fn left_column(chars: &Grid) -> Edge {
    (0..chars.len()).map(|idx| chars[idx][0]).join("")
}

fn right_column(chars: &Grid) -> Edge {
    (0..chars.len())
        .map(|idx| chars[idx][chars.len() - 1])
        .join("")
}

#[test]
fn test_solve() {
    let input = crate::read_file("D:/D/Rust/Advent/input/day20_test.txt");
    assert_eq!(part_1(&input), 20899048083289);
    assert_eq!(part_2(&input), 273);
}

#[test]
fn test_reorient() {
    let chars = vec![vec!['a', 'b'], vec!['c', 'd']];
    let mut rotations: Vec<Grid> = Vec::new();
    let mut tile = Tile::new(4, Vec::new(), chars);
    for _ in 0..8 {
        tile.reorient();
        rotations.push(tile.chars.clone());
    }
    assert_eq!(rotations.len(), 8);
    assert_eq!(rotations.iter().unique().count(), 8);
}

#[test]
fn test_assemble_image() {
    let input = crate::read_file("D:/D/Rust/Advent/input/day20_test.txt");
    let expected = ".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###";

    let chars = expected
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut image = assemble_image(&input);

    for _ in 0..9 {
        if chars == image.chars {
            return;
        }
        image.reorient();
    }
    panic!("There should be an orientation that matches the test grid");
}
