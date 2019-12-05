/***********************************************************************************************
 *                Program Name: salvo
 *
 *                      Author: Browning Keith Smith <browningsmith@email.arizona.edu>
 *                        Date: December 2, 2019
 *                Last Updated: December 4, 2019
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
 use std::process;

 /***********************************************************************************************
  * Enum Name: Difficulty
  *
  * Variants: Easy, Normal, hard
  *
  * Description: Variants of AI difficulty
  ***********************************************************************************************/

 pub enum Difficulty {
 
	Easy,
	Normal,
	Hard,
 }

 /***********************************************************************************************
  * Struct Name: Salvo
  *
  * Attributes: Difficulty ai_difficulty
  *
  * Description: Instance of a game of Salvo. Contains attributes of the state of the game, and
  *              several methods that control game flow and logic, as well as user interface.
  ***********************************************************************************************/

 pub struct Salvo {

	ai_difficulty: Difficulty, //AI dificulty
 }

 impl Salvo {
 
	/**********************************************************************************************
	 * Function Name: new_game
	 * 
	 * Input: None
	 * Output: Salvo
	 *
	 * Description: returns a new instance of Salvo. By default, ai_difficulty is set to Normal.
	 **********************************************************************************************/

	pub fn new_game() -> Salvo {
	
		Salvo {
		
			ai_difficulty: Difficulty::Normal, //Set AI difficulty to Normal
		}
	}

	/**********************************************************************************************
	 * Function Name: run_game
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: self.ai_difficulty
	 *
	 * Description: Begins running the game. Greets the user, has the user select difficulty, and then
	 *              executes the game with the desired difficulty.
	 **********************************************************************************************/

	pub fn run_game(&mut self) {
	
		println!("\n\n\nGreetings, Admiral! Welcome to the Naval Combat Simulation SALVO.\n"); //Greet the user

		//Infinite loop, begin game flow. Will continue to start new games until the user exits
		loop {
	
			self.select_difficulty(); //Have the user select difficulty

			println!("BEGIN GAME HERE");
			pause_for_enter();
		}
	}

	/**********************************************************************************************
	 * Function Name: run_game
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: self.ai_difficulty
	 *
	 * Description: Gives brief description on how to interface with the game.
	 *              Handles the user selecting game difficulty. Also displays instructions if the User
	 *              wants to see them. 
	 **********************************************************************************************/
	 pub fn select_difficulty(&mut self) {
	 
		let mut difficulty_selection_made = false; //Initialize difficulty_selection_made. As long as this is false, continue to prompt user for difficulty setting

		while !difficulty_selection_made {

			//Prompt the user to select a game difficulty
			let difficulty = prompt("Type 'easy', 'normal', or 'hard' to select difficulty of opponent\n\nFor instructions on how to play the game, type 'ins'.\n\nType 'end' at any time to end the simulation.",
								&["eas","nor","har","ins"],&[1,2,3,4]);

			if difficulty == 1 {
		
				println!("You have selected an easy opponent!\n"); //Tell the user Easy was selected
				self.ai_difficulty = Difficulty::Easy; //set difficulty to easy
				difficulty_selection_made = true;
			}
			else if difficulty == 2 {
		
				println!("You have selected a normal opponent!\n"); //tell the user Normal was selected
				self.ai_difficulty = Difficulty::Normal; //set difficulty to normal
				difficulty_selection_made = true;
			}
			else if difficulty == 3 {
		
				println!("You have selected a hard opponent!\n"); //tell the user Hard was selected
				self.ai_difficulty = Difficulty::Hard; //set difficulty to hard
				difficulty_selection_made = true;
			}
			else if difficulty == 4 { //User has selected instructions
		
				println!("User wants to see the instructions"); //Display instructions

				// difficulty_selecttion_made is still false, so user will be prompted again
			}
		}
	 }
}

/**********************************************************************************************
* Function Name: prompt
* 
* Input: &str text, &[&str] options, &[u32] results
* Output: u32 result
*
* Description: General method for interpreting user input. First argument is a string of text
*              that we wish to ask the user. After displaying this, the function calls get_input_or_exit
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

		let input = get_input_or_exit(text); //prompt and get user input, put it into input

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
* Function Name: get_input_or_exit
* 
* Input: &str text
* Output: String result
*
* Description: This function handles prompting the user with given text, and providing an
*              opportunity to respond. It also checks to see if the user entered 'end', and
*              will ask the user to confirm if they want to exit the game, or otherwise repeat
*              the given prompt.
**********************************************************************************************/

pub fn get_input_or_exit(text: &str) -> String {

	let mut prompting = true; //Declare prompting and set it to true, means we are still looking for input

	let mut input = String::new(); //Initialize a new String, assign it to input

	while prompting {
	
		print!("{}\n\n--> ", text); //Print the prompt text and the "--> " prompt arrow
		io::stdout().flush()
			.expect("Error flushing stdout from \"prompt\""); //Rust appears to buffer stdout by line. This insures the whole
															//Previous line is printed before getting user input.

		input = String::new(); //Reset input string

		io::stdin().read_line(&mut input)
			.expect("Error reading user input");  //Read user input into input

		println!(""); //Print a new line

		//If the user entered text that contains "end", confirm whether they wish to leave the game
		if input.to_uppercase().contains("END") {

			print!("Are you sure you want to exit the game? Type yes or no.\n\n--> "); //Ask the user if they really want to leave, and prompt again for input
			io::stdout().flush()
				.expect("Error flushing stdout from \"prompt\""); //Rust appears to buffer stdout by line. This insures the whole
																//Previous line is printed before getting user input.
		
			input = String::new(); //Reset input string

			io::stdin().read_line(&mut input)
				.expect("Error reading user input");  //Read user input into input

			println!(""); //Print a new line

			//If the user entered text that contains "no", repeat original prompt
			if input.to_uppercase().contains("NO") {
			
				prompting = true; //Set prompting to true, so the prompt will repeat
			}

			//If the user entered text that contains "yes", exit the game
			else if input.to_uppercase().contains("YES") {
			
				println!("Naval Combat Simulation SALVO terminated. Have a nice day, Admiral!\n"); //Print a goodbye message

				process::exit(0); //Exit the program
			}
			else { //If neither was entered, repeat original prompt
			
				prompting = true; //Set prompting to true, so the prompt will repeat.
			}
		}
		else {
		
			prompting = false; //Set prompting to false, since we have the input
		}
	}

	return input; //Return user input
}

/**********************************************************************************************
* Function Name: pause_for_enter
* 
* Input: &str text
* Output: None
*
* Description: This function is useful for pausing output and waiting for the user to hit 'ENTER'
*              It calls the get_input_or_exit funtion like prompt does, so user can still exit The
*              game at this point.
**********************************************************************************************/

pub fn pause_for_enter() {

	get_input_or_exit("Press 'ENTER' to continue");
}