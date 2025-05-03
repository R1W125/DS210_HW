fn main() {
    let size: usize = 24;
    let mut first = Board::create(size);

    first.board[0][2] = 1;
    first.board[1][2] = 1;
    first.board[2][0] = 1;
    first.board[2][1] = 1;
    first.board[2][2] = 1;

    first.print();
    println!();

    for _ in 0..4{

        let mut next = Board::create(size);

        for i in 0..first.board.len(){
            for j in 0..first.board.len(){
                first.next_iteration(&mut next, i, j);
            }
        }

        next.print();
        println!();

        first = next;
    }
}

#[derive(PartialEq)]
struct Board{
    size: usize,
    board: Vec<Vec<usize>>,
}

impl Board{
    fn create(x:usize) -> Self{
        Board{
            size:x,
            board:vec![vec![0;x];x]
        }
    }
    fn num_neighbors(&self, x:usize, y:usize) -> usize{
        let diff = [-1,0,1];
        let mut sum:usize = 0;
        for xdiff in diff{
            for ydiff in diff{
                if xdiff == 0 && ydiff == 0{
                    continue;
                }
                let nx = (x as isize + xdiff + self.size as isize) % self.size as isize;
                let ny = (y as isize + ydiff + self.size as isize) % self.size as isize;
                if nx != x as isize || ny != y as isize{
                    if self.board[nx as usize][ny as usize] == 1{
                        sum += 1;
                    }
                }
            }
        }
        return sum
    }
    fn next_iteration(&self, other: &mut Board, x:usize, y:usize){
        let neighbors = self.num_neighbors(x,y);
        match neighbors{
            3 => other.board[x][y] = 1,
            2 => other.board[x][y] = self.board[x][y],
            _ => other.board[x][y] = 0,
        }
    }
    fn print(&self){
        for i in &self.board{
            for j in i{
                print!("{}", if *j == 1 {"🐷"} else {"🟩"});
            }
            println!();
        }
    } 
}


//Tests 
// make a small version of the game using 3x3 grid
// 3x3 grid: all other points will be neighbors
// make your own 3x3 grid and use for asserteq
// source - Zach Gentile
#[test]
fn lonely_pig_dies(){
    let size: usize = 3;
    let mut first = Board::create(size);
    first.board[0][0] = 1;
    for _ in 0..1{
        let mut next = Board::create(size);
        for i in 0..first.board.len(){
            for j in 0..first.board.len(){
                first.next_iteration(&mut next, i, j);
            }
        }
        first = next;
    }
    let correct = Board::create(size);
    assert_eq!(first,correct,"FAIL!!!");
}

#[test]
fn square_of_pigs(){
    let size: usize = 4;
    let mut first = Board::create(size);
    first.board[1][1] = 1;
    first.board[1][2] = 1;
    first.board[2][1] = 1;
    for _ in 0..5{
        let mut next = Board::create(size);
        for i in 0..first.board.len(){
            for j in 0..first.board.len(){
                first.next_iteration(&mut next, i, j);
            }
        }
        next.print();
        println!();
        first = next;
    }
    let mut correct = Board::create(size);
    correct.board[1][1] = 1;
    correct.board[1][2] = 1;
    correct.board[2][1] = 1;
    correct.board[2][2] = 1;
    assert_eq!(first,correct,"FAIL!!!");
}