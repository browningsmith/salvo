/***********************************************************************************************
 *                Program Name: salvo
 *
 *                      Author: Browning Keith Smith <browningsmith@email.arizona.edu>
 *                        Date: December 2, 2019
 *                Last Updated: December 6, 2019
 *
 *                  Assignment: Project: Learn a New (to You!) Programming Language
 *                      Part 3: Custom Program, Salvo (Battleship-like game)
 *                    Due Date: December 9, 2019
 *                       Class: CSc 372
 *                  Instructor: L. McCann
 *                         TAs: Tito Ferra, Josh Xiong
 *
 *                Dependencies: See Cargo.toml
 *
 *                 Description: This program allows the user to play a Battleship-like game against
 *                              the CPU, or against a human opponent. The game board is displayed
 *                              in the console using lines and dashes to make the classic 10x10
 *                              board. Ships are denoted by capital letters. A miss is denoted
 *                              by the '~' symbol (it looks like a wave, doesn't it!). A hit is
 *                              denoted by a 'X' symbol. The user interfaces with the game by typing
 *                              commands into the console when prompted.
 *
 *                       Input: None
 *                      Output: None
 *
 * Example Execution (Windows): .\salvo.exe
 * 
 * This is the main.rs file, and it's only purpose is to create a new Salvo game and call run_game()
 * from lib.rs. lib.rs contains the bulk of the game logic and terminal interfacing.
 **********************************************************************************************/

/**********************************************************************************************
 * Function Name: main
 * 
 * Input: None
 * Output: None
 *
 * Description: Initializes the game.
 **********************************************************************************************/
use salvo;

fn main() {

    //salvo::Salvo::new_game().run_game(); //Create a new Salvo game instance and then run it.

	let mut board = salvo::GameBoard::new_board_empty();

	board.print_board();

	println!("Adding A's");

	board.write_space(1,1,'A');
	board.write_space(2,7,'A');
	board.write_space(0,4,'A');
	board.write_space(2,9,'A');
	board.write_space(5,5,'A');

	board.print_board();

	println!("Adding B's");

	board.write_space(7,1,'B');
	board.write_space(9,7,'B');
	board.write_space(8,4,'B');
	board.write_space(0,9,'B');
	board.write_space(3,5,'B');

	board.print_board();

	println!("Removing A's");

	board.clear_symbol('A');

	board.print_board();
	
}
