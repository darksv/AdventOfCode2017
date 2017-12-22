fn main() {
    let board = read_board();
    println!("1: {}", follow(&board).iter().collect::<String>());
}

fn read_board() -> Vec<Vec<char>> {
    use std::io::BufRead;
    let mut board: Vec<Vec<_>> = vec![];
    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let row = line.chars().collect();
        board.push(row);
    }
    board
}

enum Dir {
    Down,
    Right,
    Left,
    Up,
}

fn follow(board: &Vec<Vec<char>>) -> Vec<char> {
    let mut x = board[0].iter().position(|&x| x == '|').unwrap();
    let mut y = 0usize;
    let mut dir = Dir::Down;

    let mut letters = vec![];
//    let mut steps = 0;
    loop {
        match dir {
            Dir::Down => {
                loop {
                    match board[y][x] {
                        '+' => break,
                        ' ' => return letters,
                        letter @ 'A' ... 'Z' => {
                            letters.push(letter);
                        }
                        _ => ()
                    }
                    y += 1;
                }

                if board[y][x - 1] == '-' {
                    dir = Dir::Left;
                    x -= 1;
                } else if board[y][x + 1] == '-' {
                    dir = Dir::Right;
                    x += 1;
                }
            }
            Dir::Left => {
                loop {
                    match board[y][x] {
                        '+' => break,
                        ' ' => return letters,
                        letter @ 'A' ... 'Z' => {
                            letters.push(letter);
                        }
                        _ => ()
                    }
                    x -= 1;
                }

                if board[y - 1][x] == '|' {
                    dir = Dir::Up;
                    y -= 1;
                } else if board[y + 1][x] == '|' {
                    dir = Dir::Down;
                    y += 1;
                }
            }
            Dir::Up => {
                loop {
                    match board[y][x] {
                        '+' => break,
                        ' ' => return letters,
                        letter @ 'A' ... 'Z' => {
                            letters.push(letter);
                        }
                        _ => ()
                    }
                    y -= 1;
                }

                if board[y][x - 1] == '-' {
                    dir = Dir::Left;
                    x -= 1;
                } else if board[y][x + 1] == '-' {
                    dir = Dir::Right;
                    x += 1;
                }
            }
            Dir::Right => {
                loop {
                    match board[y][x] {
                        '+' => break,
                        ' ' => return letters,
                        letter @ 'A' ... 'Z' => {
                            letters.push(letter);
                        }
                        _ => ()
                    }
                    x += 1;
                }

                if board[y - 1][x] == '|' {
                    dir = Dir::Up;
                    y -= 1;
                } else if board[y + 1][x] == '|' {
                    dir = Dir::Down;
                    y += 1;
                }
            }
        }
    }
}
