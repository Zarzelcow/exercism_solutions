use std::collections::HashMap;

struct MinesweeperBoard {
    width: usize,
    height: usize,
    positions: HashMap<(i32, i32), BoardPosition>,
}

#[derive(Debug, Clone, Copy)]
struct BoardPosition {
    number: u32,
    is_mine: bool,
}

impl BoardPosition {
    fn new() -> Self {
        Self { number: 0, is_mine: false }
    }
    fn to_char(&self) -> char {
        return if self.is_mine { '*' } else { char::from_digit(self.number, 10).unwrap() };
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() { return vec![]; }
    if minefield[0].len() == 0 { return vec![String::new()]; }
    let mut board = MinesweeperBoard {
        width: minefield[0].len(),
        height: minefield.len(),
        positions: HashMap::new(),
    };
    for (y, line) in minefield.iter().enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            match ch {
                42 /* '*' */ => {
                    for pos in get_3x3(x, y) {
                        let entry = board.positions.entry(pos).or_insert(BoardPosition::new());
                        entry.number += 1;
                    }
                    board.positions.insert((x as i32, y as i32), BoardPosition { is_mine: true, number: 0 });
                }
                _ => {}
            }
        }
    }
    return board_to_strings(&board);
}

fn board_to_strings(board: &MinesweeperBoard) -> Vec<String> {
    let mut result = Vec::new();
    for y in 0..board.height {
        result.push((0..board.width)
            .map(|x| board.positions.get(&(x as i32, y as i32)))
            .map(|o| o.map_or_else(|| ' ', |p| p.to_char(),))
            .collect::<String>()
        );
    }
    return result;
}

fn get_3x3(x: usize, y: usize) -> Vec<(i32, i32)> {
    let matrix = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 0), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    return matrix
        .iter()
        .map(|(off_y, off_x)|
            (x as i32 + off_x, y as i32 + off_y)
        )
        .collect::<Vec<(i32, i32)>>();
}