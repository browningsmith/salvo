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
 * This is the lib.rs file, it contains the bulk of the game logic. The main function, which is
 * called on execution, is located in main.rs. The main function makes extensive use of This
 * library file.
 **********************************************************************************************/

 //Dependencies
 use std::io;
 use std::io::Write;


/**********************************************************************************************
 * Function Name: prompt
 * 
 * Input: &str prompt, &[&str] options 
 * Output: i32 result
 *
 * Description: Handles input from the user. First argument is a string of text to prompt the user with.
 *              Second argument is a list of possible valid responses. (IMPORTANT: array of inputs 
 *              must be uppercase strings!) Function returns the index of the result that was
 *              found in the user input. Repeats the given prompt until valid input is given.
 *
 *              This method prints the prompt given as the first argument, then on the next line prints
 *              "--> ", to indicate to the user that they should type something.
 *
 *              If the user enters more than one possible valid input on the same line, The
 *              method returns the lowest index of of the matching options. For example: If
 *              options is ["YES", "NO"], and the user types "no yes", the method returns 0, not 1.
 **********************************************************************************************/

 

 pub fn prompt(text: &str, options: &[&str]) -> i32 {

	print!("{}\n--> ", text); //Print the prompt, new line, and "--> " to indicate that the user should type something.
	io::stdout().flush()
	    .expect("Error flushing stdout from \"prompt\""); //Rust appears to buffer stdout by line. This insures the whole
		                                                  //Previous line is printed before getting user input.

	let mut input = String::new(); //Create a new string and assign it to input

	io::stdin().read_line(&mut input)
	    .expect("Error reading user input");  //Read user input into input

	println!("You typed: {}", input.to_uppercase()); //Display what the user input was

	//Declare result, init to -1, this will be the index of the option that was found in the Input
	let mut result = -1;

	//Declare n, init to -1, this will be the index of the option we are currently comparing
	let mut n = -1;

	for option in options.iter() { //For each option in the list of options
		
		n = n+1; //Increment result

		if input.to_uppercase().contains(option) { //If the user input contains the option
		
			result = n; //Set result to n
			return result; //Return the result
		}
	}

	return result; //Return the result, which is negative 1 at this point if no valid input was entered
}
