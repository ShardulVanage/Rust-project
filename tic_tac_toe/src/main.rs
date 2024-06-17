use std::{env::current_exe, io};

const PLAYER_X:char ='X';
const PLAYER_O:char ='O';

const BOARD_SIZE:usize =3;//2D ARRAY -3X3
type Board = [[char;BOARD_SIZE];BOARD_SIZE];

fn initialize_board()->Board{
    return [[' ';BOARD_SIZE];BOARD_SIZE];
}
fn print_board(board:&Board){
    for row in board{
        for cell in row  {
            print!("{}",cell);
        }
        println!();
    }
}

fn get_player_move(current_player:char,board:&Board)->(usize,usize){
    loop {
        println!("PLayer {} input (row,col)",current_player);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");//we are converting string to intiger ("0 1"->"0","1"-> 0 1)
    println!("input:{}",input);

 let coordinate:Vec<usize> =input
    .split_whitespace()// it will covert 0 and 1 to sub string like "0","1"
    .flat_map(str::parse::<usize>)// coverting this values in usize
    .collect();// this will make it in vector size array 
 if coordinate.len()==2 {
    let (row , col) = (coordinate[0],coordinate[1]);
    if row < BOARD_SIZE&& col<BOARD_SIZE && board[row][col]==' '{
       return (row,col); 
    }
}
println!("Invaild Input ");
    }
}

fn check_winner(current_player:char,board:&Board)->bool{
    for row in 0..BOARD_SIZE{
        if board[row][0]== current_player && board[row][1]== current_player && board[row][2]== current_player{
         return true;
        }
    }
    for col in 0..BOARD_SIZE{
        if board[0][col]== current_player && board[1][col]== current_player && board[2][col]== current_player{
            return true;
        }
    }
    if (board[0][0]== current_player && board[1][1]== current_player && board[2][2]== current_player)
     ||
    (board[0][2]== current_player && board[1][1]== current_player && board[2][0]== current_player)
    {
        return true;    
    }
    return false;
}
fn check_draw(board:&Board)->bool{
    for row in board{
        for cell in row{
            if *cell==' '{ //derefrence opertor *
                return false;
            }
        }
    }
    return true;
}

fn play_game(){
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;
     loop {
        println!("Current Board");
        print_board(&board);

        let(row,cell)=get_player_move(current_player,&board);
        board[row][cell] = current_player;
        if check_winner(current_player,&board){
            print_board(&board);
            println!("Player {} is winnner",current_player);
            break;
        }

        if check_draw(&board) {
            print_board(&board);
            println!("Game is Draw");
            break;
        }

        current_player = if current_player==PLAYER_X{
            PLAYER_O
        }else{
            PLAYER_X

        }
     }
     
}

fn main() {
    println!("Welcome to Tic Tac Toe Game ");
    play_game();
}
