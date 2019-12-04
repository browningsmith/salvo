/***********************************************************************************************
 *                Program Name: salvo
 *
 *                      Author: Browning Keith Smith <browningsmith@email.arizona.edu>
 *                        Date: December 2, 2019
 *                Last Updated: December 2, 2019
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
 * This is the main.rs file, it contains the main function, which handles top-level game flow.
 * The bulk of game logic is located in the lib.rs file.
 **********************************************************************************************/

/**********************************************************************************************
 * Function Name: main
 * 
 * Input: None
 * Output: None
 *
 * Description: Handles top-level game flow
 *              Upon startup, welcomes the user.
 *              Asks the user if they would prefer a one-player battle or a two-player battle.
 *              After the battle has concluded, asks if the user would like to play again.
 **********************************************************************************************/
use salvo;

fn main() {
    println!("\n\n\nGreetings, Admiral! Welcome to the Naval Combat Simulation SALVO.\n"); //Greet the user


	
	let result = salvo::prompt("Please type in one, two, or three", &["1","one","2","two","3","three"], &[1,1,2,2,3,3]);

	if result == 1 {
	
		println!("You typed in one!");
	}
	else if result == 2 {
	
		println!("You typed in two!");
	}
	else if result == 3 {
	
		println!("You typed in three!");
	}
	else {
	
		println!("You didn't enter anything valid!");
	}
}
