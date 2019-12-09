/***********************************************************************************************
 *                   Program Name: salvo
 *
 *                         Author: Browning Keith Smith <browningsmith@email.arizona.edu>
 *                           Date: December 2, 2019
 *                   Last Updated: December 9, 2019
 *
 *                     Assignment: Project: Learn a New (to You!) Programming Language
 *                         Part 3: Custom Program, Salvo (Battleship-like game)
 *                       Due Date: December 9, 2019
 *                          Class: CSc 372
 *                     Instructor: L. McCann
 *                            TAs: Tito Ferra, Josh Xiong
 *
 *                   Dependencies: See salvo_Cargo.toml
 *
 *                    Description: This program allows the user to play a Battleship-like game against
 *                                 the CPU, or against a human opponent. The game board is displayed
 *                                 in the console using lines and dashes to make the classic 10x10
 *                                 board. Ships are denoted by capital letters. A miss is denoted
 *                                 by the '~' symbol (it looks like a wave, doesn't it!). A hit is
 *                                 denoted by a 'X' symbol. The user interfaces with the game by typing
 *                                 commands into the console when prompted.
 *
 *                          Input: None
 *                         Output: None
 *
 * Rust Language Study Google Doc: https://docs.google.com/document/d/1iInK2BWCybQMu_Oqyt0Vk5t5cwM1oJUSXZOpX7Y-5Jw/edit?usp=sharing_eip&ts=5ddc4adc
 *
 *    Example Execution (Windows): .\salvo.exe
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

    salvo::Salvo::new_game().run_game(); //Create a new Salvo game instance and then run it.
}
