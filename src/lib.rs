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
 * Input: &str text, &[&str] options, &[u32] results
 * Output: u32 result
 *
 * Description: General method for handling user input. First argument is a string of text
 *              that we wish to ask the user. After displaying this, the function calls get_input_arrow
 *              to print the "--> " cursor and grab user input. The second argument is an array
 *              of strings, which is a list of valid inputs that the function will look for within
 *              the string that the user input. The third argument is an array of equal size, containing which
 *              unsigned integer is to be returned if the corresponding string from the second argument
 *              is found. These must be greater than or equal to 0!
 *
 *              If the user enters two or more valid inputs on the same line, the input is considered invalid.
 *
 *              However, if you would like there to be more than one option that return the same result, place
 *              the same unsigned integer in the corresponding locations of the third argument for those options.
 *              In this case the function will not consider it invalid if two different options that Return
 *              the same result are entered on one line. Otherwise, have a unique unsigned integer for each option.
 *
 *              If user enters invalid input, it will restate the prompt and get more user input. Will
 *              not return until valid input is entered
 *
 *              Behavior is undefined if the second and third arguments are not equally
 *              sized arrays.
 **********************************************************************************************/

 pub fn prompt(text: &str, options: &[&str], results: &[u32]) -> u32 {

	let mut result: i32 = -1; //declare result. This will be the return value of the Function. -1 indicates invalid input

	while result == -1 { //As long as input is invalid
	
		println!("{}", text); //Print the prompt

		let input = get_input_arrow(); //Display arrow and get user input, put it into input

		let mut n = 0; //Initialize n as 0, this will be the index of the option we are comparing

		for option in options.iter() { //For each option in the options array

			if input.to_uppercase().contains(&option.to_uppercase()) { //If the input string contains the option

				//Check to see that no other results have been found yet
				if result == -1 {
					result = results[n] as i32; //Set result to the proper result from results array
				}

				//Else, since a result was already found, check to see if it was a different result
				else if result != results[n] as i32 {
				
					result = -1; //Reset result to -1, invalid input
					break; //Break out of the for loop
				}
			}

			n = n + 1; //Increment n, and repeat for loop to check next option
		}

		if result == -1 { //If result is -1 at this point, input was invalid

			println!("Input is invalid\n"); //Let the user know the input is invalid
		}
	} //If input is -1 (invalid), this loop repeats

	return result as u32; //Return the result
}

/**********************************************************************************************
 * Function Name: get_input_arrow
 * 
 * Input: None 
 * Output: String result
 *
 * Description: This method is what creates the "--> " prompt cursor that is seen in the game.
 *              When called, it displays the "-- >", and allows the user to enter some text.
 *              After the user hits the 'ENTER' key, the string that the user typed is returned
 **********************************************************************************************/

 pub fn get_input_arrow() -> String {
 
	print!("--> "); //Display the "--> " prompt arrow
	io::stdout().flush()
	    .expect("Error flushing stdout from \"prompt\""); //Rust appears to buffer stdout by line. This insures the whole
		                                                  //Previous line is printed before getting user input.

	let mut input = String::new(); //Initialize a new String, assign it to input

	io::stdin().read_line(&mut input)
	    .expect("Error reading user input");  //Read user input into input

	println!(""); //Print a new line

	return input;
 }