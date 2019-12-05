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
 * This is the main.rs file, and it's only purpose is to call run_game() from lib.rs. lib.rs contains
 * the bulk of the game logic and terminal interfacing.
 **********************************************************************************************/

/**********************************************************************************************
 * Function Name: main
 * 
 * Input: None
 * Output: None
 *
 * Description: Initializes the battleship game.
 **********************************************************************************************/
use salvo;

fn main() {

    println!("\n\n\nGreetings, Admiral! Welcome to the Naval Combat Simulation SALVO.\n"); //Greet the user

	//Infinite loop, begin game flow. Will continue to start new games until the user exits
	loop {
	
		//Prompt the user to select a game difficulty
		let difficulty = salvo::prompt("Type 'easy', 'normal', or 'hard' to select difficulty of opponent\n\nFor instructions on how to play the game, type 'ins'.\n\nType 'end' at any time to end the simulation.\n",
		                     &["eas","nor","har","ins"],&[1,2,3,4]);

		if difficulty == 1 {
		
			println!("User wants an easy game");
		}
		else if difficulty == 2 {
		
			println!("User wants a normal game");
		}
		else if difficulty == 3 {
		
			println!("User wants a hard game");
		}
		else if difficulty == 4 {
		
			println!("User wants to see the instructions");
		}
	}
}
