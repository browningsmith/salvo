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
 * This is the lib.rs file, it contains the bulk of the game logic. The main function, which is
 * called on execution, is located in main.rs. The main function makes extensive use of This
 * library file.
 **********************************************************************************************/

 //Dependencies
 use std::io;
 use std::io::Write;
 use std::process;

 //Global constants
 const BOARD_HEIGHT: u32 = 10;
 const BOARD_WIDTH: u32 = 10;

 /***********************************************************************************************
  * Enum Name: Difficulty
  *
  * Variants: Easy, Normal, hard
  *
  * Description: Variants of AI difficulty
  ***********************************************************************************************/

 enum Difficulty {
 
	Easy,
	Normal,
	Hard,
 }

 /***********************************************************************************************
  * Struct Name: GameBoard
  *
  * Attributes: Vec<Vec<char>>
  *
  * Description: Game board. Two dimensional vector of characters, containing either empty spaces,
  *              ship initials, 'X', or '~'
  ***********************************************************************************************/

 pub struct GameBoard {

	grid: Vec<Vec<char>>,
 }

 impl GameBoard {

	/**********************************************************************************************
	 * Function Name: new_board_empty
	 * 
	 * Input: None
	 * Output: GameBoard
	 *
	 * Description: Creates a new instance of an empty game board
	 **********************************************************************************************/
 
	pub fn new_board_empty() -> GameBoard {

		let mut new_grid: Vec<Vec<char>> = Vec::new(); //Create a new two dimensional vector

		for row in 0..BOARD_HEIGHT { //For each row from 0 to BOARD_HEIGHT-1
		
			let mut new_row: Vec<char> = vec![' '; BOARD_WIDTH as usize];  //Create a new vector of characters, length BOARD_WIDTH, filled with spaces

			new_grid.push(new_row); //Push row on to grid
		}

		return GameBoard { //Create and return game board
		
			grid: new_grid,
		}
	}

	/**********************************************************************************************
	 * Function Name: print_board
	 * 
	 * Input: &self
	 * Output: None
	 *
	 * Description: Displays the board to standard output. Behavior for a board larger than 10x10
	 *              is currently undefined.
	 **********************************************************************************************/

	 pub fn print_board(&self) {
	 
		println!("      1   2   3   4   5   6   7   8   9   10 "); //Print column numbers
		println!("  --|---|---|---|---|---|---|---|---|---|---|"); //Print top border

		for row in 0..self.grid.len() { //For each row in the grid
		
			print!(" {}  |", (row as u8 + 65) as char); //Print a space, the letter of the row (65 is ascii for capital A), a space and a vertical divider

			for col in 0..self.grid[row].len() { //For each column of the row
			
				//Print a space, the character at this location, a space, and vertical divider
				print!(" {} |", self.grid[row][col]);
			}

			//Print a new line
			println!("");
			println!("  --|---|---|---|---|---|---|---|---|---|---|"); //Print a horizontal divider
		}

		println!(""); //Print a new line to complete the board
	 }

	 /**********************************************************************************************
	 * Function Name: write_space
	 * 
	 * Input: &self, usize row, usize col, char c
	 * Output: None
	 *
	 * Description: Changes the element at row and col of board to c
	 **********************************************************************************************/

	 pub fn write_space(&mut self, row: usize, col: usize, c: char) {
	 
		self.grid[row][col] = c;
	 }

	 /**********************************************************************************************
	 * Function Name: get_space
	 * 
	 * Input: &self, usize row, usize col
	 * Output: char
	 *
	 * Description: Changes the element at row and col of board to c
	 **********************************************************************************************/

	 pub fn get_space(&self, row: usize, col: usize) -> char {
	 
		self.grid[row][col]
	 }

	 /**********************************************************************************************
	 * Function Name: clear_symbol
	 * 
	 * Input: &self, char c
	 * Output: None
	 *
	 * Description: searches the board and clears it of all instances of c
	 **********************************************************************************************/

	 pub fn clear_symbol(&mut self, c: char) {
	 
		for row in 0..self.grid.len() { //For each row in the grid

			for col in 0..self.grid[row].len() { //For each column of the row
			
				//If this space is equal to c
				if self.grid[row][col] == c {
				
					self.grid[row][col] = ' '; //Replace this location with a space
				}
			}
		}
	 }
 }

 

 /***********************************************************************************************
  * Struct Name: Player
  *
  * Attributes: String name, Fleet fleet, GameBoard board
  *
  * Description: Contains information about a player
  ***********************************************************************************************/

  struct Player {
  
	name: String,
	fleet: Fleet,
	board: GameBoard,
  }

  impl Player {
  
	/**********************************************************************************************
	 * Function Name: new_player
	 * 
	 * Input: &str
	 * Output: Player
	 *
	 * Description: Creates a new player with given name, standard Fleet and empty GameBoard
	 **********************************************************************************************/

	fn new_player(name: &str) -> Player {
	
		Player {
		
			name: String::from(name),
			fleet: Fleet::new_fleet_standard(),
			board: GameBoard::new_board_empty(),
		}
	}

	/**********************************************************************************************
	 * Function Name: arrange_fleet
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: Player's GameBoard and Fleet objects
	 *
	 * Description: Handles the process of a user arranging the ships in their fleet. Currently tailored
	 *              for a standard fleet only. To print a variable fleet will require significant
	 *              changes to print formatting
	 **********************************************************************************************/

	 fn arrange_fleet(&mut self) {
	 
		self.board.print_board(); //Print the board to Start

		print!("   Patrol Boat      [P]        2 Spaces     Status: "); //Show the Patrol Boat stats
		if self.fleet.ships[0].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Submarine        [S]        3 Spaces     Status: "); //Show the Patrol Boat stats
		if self.fleet.ships[1].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Destroyer        [D]        3 Spaces     Status: "); //Show the Patrol Boat stats
		if self.fleet.ships[2].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Battleship       [B]        4 Spaces     Status: "); //Show the Patrol Boat stats
		if self.fleet.ships[3].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Aircraft Carrier [C]        5 Spaces     Status: "); //Show the Patrol Boat stats
		if self.fleet.ships[4].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
	 }
  }

 /***********************************************************************************************
  * Struct Name: Fleet
  *
  * Attributes: Vec<Ship> ships
  *
  * Description: Collection of a fleet of ships. Contains a list of ship objects, and keeps track
  *              of how many of them are still afloat
  ***********************************************************************************************/

  struct Fleet {

	ships: Vec<Ship>,
  }

  impl Fleet {
  
	/**********************************************************************************************
	 * Function Name: new_fleet_standard
	 * 
	 * Input: None
	 * Output: Fleet
	 *
	 * Description: Creates and returns a standard Salvo fleet with the following five ships:
	 *
	 *              Patrol Boat      [P] length 2
	 *              Submarine        [S] length 3
	 *              Destroyer        [D] length 3
	 *              Battleship       [B] length 4
	 *              Aircraft Carrier [C] length 5
	 *
	 *              All ships are initialized as full health, unsunk and unplaced
	 *              
	 **********************************************************************************************/

	 fn new_fleet_standard() -> Fleet {
	 
		let mut fleet_vec: Vec<Ship> = Vec::new(); //Create new vector

		fleet_vec.push(Ship::new_ship("Patrol Boat", 'P', 2)); //Add Patrol Boat
		fleet_vec.push(Ship::new_ship("Submarine", 'S', 3)); //Add Patrol Boat
		fleet_vec.push(Ship::new_ship("Destroyer", 'D', 3)); //Add Patrol Boat
		fleet_vec.push(Ship::new_ship("Battleship", 'B', 4)); //Add Patrol Boat
		fleet_vec.push(Ship::new_ship("Aircraft Carrier", 'C', 5)); //Add Patrol Boat
		
		Fleet { //Create and return a new fleet
		
			ships: fleet_vec,
		}
	 }

	 /**********************************************************************************************
	 * Function Name: size
	 * 
	 * Input: &self
	 * Output: u32
	 *
	 * Description: Returns the total size of the fleet, both sunk and unsunk ships
	 **********************************************************************************************/

	 fn size(&self) -> u32 {
	 
		self.ships.len() as u32
	 }

	 /**********************************************************************************************
	 * Function Name: ships_remaining
	 * 
	 * Input: &self
	 * Output: u32
	 *
	 * Description: Returns the total number of ships that remain afloat in the fleet
	 **********************************************************************************************/

	 fn ships_remaining(&self) -> u32 {
	 
		let mut remain_count = 0; //Declare remain_count, start it at 0

		for ship in &(self.ships) {
		
			//For each ship not found sunk, increment remain_count;
			if ship.get_sunk() == false {
			
				remain_count += 1;
			}
		}

		remain_count
	 }

	 /**********************************************************************************************
	 * Function Name: get_deployed
	 * 
	 * Input: &self
	 * Output: u32
	 *
	 * Description: Returns how many ships have been deployed
	 **********************************************************************************************/

	 fn get_deployed(&self) -> u32 {
	 
		let mut deployed_count = 0; //Declare deployed_count, start it at 0

		for ship in &(self.ships) {
		
			//For each ship found as placed, increment deployed_count;
			if ship.get_placed() {
			
				deployed_count += 1;
			}
		}

		deployed_count
	 }
  }

  /***********************************************************************************************
  * Struct Name: Ship
  *
  * Attributes: String name, char letter, u32 length
  *             u32 health, bool placed
  *
  * Description: Ship object, contains identifying information and status of a ship's dimensions
  *              and health
  ***********************************************************************************************/

  struct Ship {
  
	name: String,
	letter: char,
	length: u32,
	health: u32,
	placed: bool,
  }

  impl Ship {
  
	/**********************************************************************************************
	 * Function Name: new_ship
	 * 
	 * Input: name: &str, letter: char, length: u32
	 * Output: Ship
	 *
	 * Description: Returns a new instance of ship with the given above parameters. By default,
	 *              ship is set to full health and set to not placed
	 **********************************************************************************************/

	fn new_ship(name: &str, letter: char, length: u32) -> Ship {
	
		//Return new Ship with given parameters as attributes
		Ship {

			name: String::from(name), //create new string object from passed slice
			letter,
			length,
			health: length, //Health should start as the same as length
			placed: false,  //Ship is not placed by default
		}
	}

	/**********************************************************************************************
	 * Function Name: get_name
	 * 
	 * Input: &self
	 * Output: &str
	 *
	 * Description: Returns string slice of the ship's name
	 **********************************************************************************************/

	fn get_name(&self) -> &str {
	
		&(self.name)
	}

	/**********************************************************************************************
	 * Function Name: get_letter
	 * 
	 * Input: &self
	 * Output: char
	 *
	 * Description: Returns the ships letter, which the board uses to represent it
	 **********************************************************************************************/

	fn get_letter(&self) -> char {
	
		self.letter
	}

	/**********************************************************************************************
	 * Function Name: get_length
	 * 
	 * Input: &self
	 * Output: u32
	 *
	 * Description: Returns ship length
	 **********************************************************************************************/

	fn get_length(&self) -> u32 {
	
		self.length
	}

	/**********************************************************************************************
	 * Function Name: get_health
	 * 
	 * Input: &self
	 * Output: u32
	 *
	 * Description: Returns value of ship's current health
	 **********************************************************************************************/

	fn get_health(&self) -> u32 {
	
		self.health
	}

	/**********************************************************************************************
	 * Function Name: get_sunk
	 * 
	 * Input: &self
	 * Output: bool
	 *
	 * Description: Returns true if the ship has been sunk, false otherwise. Ship is sunk if health
	 *              is 0.
	 **********************************************************************************************/

	fn get_sunk(&self) -> bool {
	
		match self.health {
		
			0 => true,
			_ => false,
		}
	}

	/**********************************************************************************************
	 * Function Name: get_placed
	 * 
	 * Input: &self
	 * Output: bool
	 *
	 * Description: Returns whether the ship has been placed or not
	 **********************************************************************************************/

	 fn get_placed(&self) -> bool {
	 
		self.placed
	 }

	 /**********************************************************************************************
	 * Function Name: place
	 * 
	 * Input: &self
	 * Output: None
	 *
	 * Description: Sets that the ship has been placed on the board. Does not actually write to game
	 *              board, only sets internal flag of ship
	 **********************************************************************************************/

	 fn place(&mut self) {
	 
		self.placed = true;
	 }

	 /**********************************************************************************************
	 * Function Name: remove
	 * 
	 * Input: &self
	 * Output: None
	 *
	 * Description: Sets that the ship has been removed from the board. Does not actually write to game
	 *              board, only sets internal flag of ship
	 **********************************************************************************************/

	 fn remove(&mut self) {
	 
		self.placed = false;
	 }

	 /**********************************************************************************************
	 * Function Name: damage
	 * 
	 * Input: &self
	 * Output: None
	 *
	 * Description: Decrements ship health by 1. If ships health is already at 0 it does nothing
	 **********************************************************************************************/

	 fn damage(&mut self) {
	 
		if self.health > 0 { //If health is greater than 0
		
			self.health -= 1; //remove one level of health
		}
	 }
  }

 /***********************************************************************************************
  * Struct Name: Salvo
  *
  * Attributes: Difficulty ai_difficulty, Player player1, Player player2
  *
  * Description: Instance of a game of Salvo. Contains attributes of the state of the game, and
  *              several methods that control game flow and logic, as well as user interface.
  ***********************************************************************************************/

 pub struct Salvo {

	ai_difficulty: Difficulty, //AI dificulty
	player1: Player,
	player2: Player,
 }

 impl Salvo {
 
	/**********************************************************************************************
	 * Function Name: new_game
	 * 
	 * Input: None
	 * Output: Salvo
	 *
	 * Description: returns a new instance of Salvo. By default, ai_difficulty is set to Normal.
	 *              2 Players are created, User and CPU, and each is given a standard unplaced
	 *              fleet and an empty game board
	 **********************************************************************************************/

	pub fn new_game() -> Salvo {
	
		Salvo {
		
			ai_difficulty: Difficulty::Normal, //Set AI difficulty to Normal
			player1: Player::new_player("User"),
			player2: Player::new_player("CPU"),
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
	 *
	 *              Currently handles game flow of 1 player game against the CPU. Minor refactoring
	 *              will be needed to handle a two player game
	 **********************************************************************************************/

	pub fn run_game(&mut self) {
	
		println!("\n\n\nGreetings, Admiral! Welcome to the Naval Combat Simulation SALVO.\n"); //Greet the user

		//Infinite loop, begin game flow. Will continue to start new games until the user exits
		/*loop {
	
			self.select_difficulty(); //Have the user select difficulty for new game

			//Have the user arrange their ships before the game Begins
			println!("Admiral, it is time to deploy the fleet! Arrange your ships on the board below:\n");

			self.player1.arrange_fleet(); //Have the user arrange their fleet manually
		}*/
		
		let mut input = get_input_or_exit("Type in George, John, Ringo, or Paul");

		if find(&input, &["george"], &[1]) == 1 {println!("George Found!");} else {println!("George Not Found!");}
		if find(&input, &["john"], &[1]) == 1 {println!("John Found!");} else {println!("John Not Found!");}
		if find(&input, &["ringo"], &[1]) == 1 {println!("Ringo Found!");} else {println!("Ringo Not Found!");}
		if find(&input, &["paul"], &[1]) == 1 {println!("Paul Found!");} else {println!("Paul Not Found!");}
	}

	/**********************************************************************************************
	 * Function Name: select_difficulty
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: self.ai_difficulty
	 *
	 * Description: Handles asking the user to choose an AI difficulty. Since it is also the first method
	 *              executed by run_game(), it displays a brief description of how to interface with the
	 *              game, and will display instructions if the user asks.
	 **********************************************************************************************/
	 pub fn select_difficulty(&mut self) {
	 
		let mut selection_made = false; //selection_made is false to start. Will keep asking the user for difficulty
		                                //until this is true

		while selection_made == false {

			let difficulty_selection = prompt("Select difficulty of opponent by typing 'easy', 'normal', or 'hard'\n\nFor instructions on how to play the game, type 'instructions' or 'ins'\n\nType 'end' at any time to exit the program",
			                                  &["eas", "nor", "har", "ins"],&[1,2,3,4]);

			if difficulty_selection == 1 { //User picked easy
			
				//Tell the user they have selected "easy", and prompt for a confirmation
				if prompt_yn("You have selected an easy opponent.\n\nWould you like to continue? Type 'yes' or 'no'.") { //If user says yes
				
					selection_made = true; //Set selection made to true
					self.ai_difficulty = Difficulty::Easy; //Set game difficulty to easy
				}
				else { //If user says no
				
					selection_made = false; //Set selection made to false, so the user will be prompted again.
				}
			}
			else if difficulty_selection == 2 { //User picked normal
			
				//Tell the user they have selected "normal", and prompt for a confirmation
				if prompt_yn("You have selected a normal opponent.\n\nWould you like to continue? Type 'yes' or 'no'.") { //If user says yes
				
					selection_made = true; //Set selection made to true
					self.ai_difficulty = Difficulty::Normal; //Set game difficulty to normal
				}
				else { //If user says no
				
					selection_made = false; //Set selection made to false, so the user will be prompted again.
				}
			}
			else if difficulty_selection == 3 { //User picked hard
			
				//Tell the user they have selected "hard", and prompt for a confirmation
				if prompt_yn("You have selected a hard opponent.\n\nWould you like to continue? Type 'yes' or 'no'.") { //If user says yes
				
					selection_made = true; //Set selection made to true
					self.ai_difficulty = Difficulty::Hard; //Set game difficulty to hard
				}
				else { //If user says no
				
					selection_made = false; //Set selection made to false, so the user will be prompted again.
				}
			}
			else { //User wants to see instructions
			
				selection_made = false; //Set selection_made to false, since the user did not pick anything
				display_instructions(); //Display instructions
				pause_for_enter(); //Pause for the user to read and hit enter
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

		let input = get_input_or_exit(text).to_uppercase(); //prompt and get user input, make it uppercase, and put it into variable input

		let mut n = 0; //Initialize n as 0, this will be the index of the option we are comparing

		for option in options.iter() { //For each option in the options array

			if input.contains(&option.to_uppercase()) { //If the input string contains the option

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
* Function Name: prompt_yn
* 
* Input: &str text
* Output: bool
*
* Description: This is similar to prompt, however it only looks to see if the user has entered
*              yes or no. If the user enters invalid input, it will restate the prompt and get more
*              user input. Will not return until valid input is entered.
**********************************************************************************************/

pub fn prompt_yn(text: &str) -> bool {

	let result = prompt(text, &["YES","NO"], &[1,2]); //Set result to 1 or 2, based on whether yes or no is entered

	return match result {
	
		1 => true,  //Return true if yes
		2 => false, //Return false if no
		_ => false, //To make compiler happy, this calling of promt can only return 1 or 2
	}
}

/**********************************************************************************************
* Function Name: find
* 
* Input: &str text, &[&str] options, &[u32] results
* Output: i32 result
*
* Description: Similar method to how prompt interprets input, but rather than calling get_input_or_exit
*              itself and repeating the prompt on invalid input, it interprets a string of text input that is passed to it.
*              only once. It can be used in combination with other calls to find() to interpret combinations of words.
*
*              The first argument is a string reference to interpret. The second argument is an array
*              of strings, which is a list of valid inputs that the function will look for within
*              the first argument. The third argument is an array of equal size, containing which
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
*              Returns -1 if no valid input is found
*
*              Behavior is undefined if the second and third arguments are not equally
*              sized arrays.
**********************************************************************************************/

pub fn find(input: &str, options: &[&str], results: &[u32]) -> i32 {

	let mut result: i32 = -1; //declare result. This will be the return value of the Function. -1 indicates invalid input

	let input = input.to_uppercase(); //Convert input to uppercase

	let mut n = 0; //Initialize n as 0, this will be the index of the option we are comparing

	for option in options.iter() { //For each option in the options array

		if input.contains(&option.to_uppercase()) { //If the input string contains the option

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

	//If result is -1 at this point, input was invalid, otherwise it will be something from options array

	return result; //Return the result
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
	
		print!("{}\n--> ", text); //Print the prompt text and the "--> " prompt arrow
		io::stdout().flush()
			.expect("Error flushing stdout from \"prompt\""); //Rust appears to buffer stdout by line. This insures the whole
															//Previous line is printed before getting user input.

		input = String::new(); //Reset input string

		io::stdin().read_line(&mut input)
			.expect("Error reading user input");  //Read user input into input

		println!(""); //Print a new line

		let input_caps = input.to_uppercase(); //Create copy of input that is all caps, for comparison

		//If the user entered text that contains "end", confirm whether they wish to leave the game
		if input_caps.contains("END") {

			print!("Are you sure you want to exit the game? Type yes or no.\n--> "); //Ask the user if they really want to leave, and prompt again for input
			io::stdout().flush()
				.expect("Error flushing stdout from \"prompt\""); //Rust appears to buffer stdout by line. This insures the whole
																//Previous line is printed before getting user input.
		
			input = String::new(); //Reset input string

			io::stdin().read_line(&mut input)
				.expect("Error reading user input");  //Read user input into input

			println!(""); //Print a new line

			let input_caps = input.to_uppercase(); //Create copy of input that is all caps, for comparison

			//If the user entered text that contains "no", repeat original prompt
			if input_caps.contains("NO") {
			
				prompting = true; //Set prompting to true, so the prompt will repeat
			}

			//If the user entered text that contains "yes", exit the game
			else if input_caps.contains("YES") {
			
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

/**********************************************************************************************
* Function Name: get_coordinates
* 
* Input: &str text
* Output: (u32, u32) coordinates
*
* Description: This method prompts the user to enter coordinates, with custom prompt text. It is
*              actually quite robust in detecting coordinates of any variation of the letters A-J
*              for row letters, and numbers 1-10 or one-ten for column numbers.
*
*              Column and row can be entered in any order, for the most part
*
*              It is able to detect upper or lower case row numbers
*
*              It is able to detect column numbers whether they are numerals or words
*
*              Not case sensitive
*
*              Is a tad bit glitchy on reading user input that contain E F G H and I specifically, may detect The
*              wrong coordinates, especially if the column is provided as a word rather than a numeral,
*              so make sure the game asks the user to confirm after entering coordinates.
**********************************************************************************************/

pub fn get_coordinates(text: &str) {

	let mut row: i32 = 0; //declare row. This will be part of the the return value of the function. 0 indicates invalid input
	let mut col: i32 = 0; //declare column. This will be part of the return value of the function. 0 indicates invalid input

	while (row == 0) || (col == 0) { //As long as input is invalid

		//reset row and col to 0, invalid inputs
		row = 0;
		col = 0;

		let input = get_input_or_exit(text).to_uppercase(); //prompt and get user input, capitalize it, and assign it to input variable

		//First, attempt to parse a column number

		//Start by searching for the name of a number
		let mut col_option = 1; //Let col_option be 1, this is the column option we are comparing
		let mut col_name_found = false; //Let col_name_found be false. This flag says we have detected a name rather than numeral

		for col_name in ["ON","TWO","THR","FOU","FIV","SIX","SEV","EIGHT","NIN","TEN"].iter() {  //Iterate over strings of what could be the names of numbers 1 through 10
		
			if input.contains(col_name) { //If input string contains the numeral name

				col_name_found = true; //Set col_name_found to true
			
				//Check to see that no other columns have been found
				if col == 0 {
				
					col = col_option; //Set col to the col_option we just compared
				}

				//Else, since a column was already found, check to see if it was a different column
				else if col != col_option {
				
					col = 0; //Reset col to invalid input
					break; //break out of the for loop
				}
			}

			col_option = col_option + 1; //Increment col_option
		}

		//Next, search for numerals
		
		//Check for 1
		if input.contains("1") {

			//Check to see if it is actually 10
			if input.contains("10") {
			
				//Check to see that no other rows have been found
				if col == 0 {
				
					col = 10; //set col to 10 if no other rows were found
				}

				//Else, since a row was already found, check to see if it was a different row
				else if col != 10 {
				
					col = 0; //Reset col to invalid input if a different row was already found
				}
			}
			else { //Else, input just contains 1
			
				//Check to see that no other rows have been found
				if col == 0 {
				
					col = 1; //set col to 10 if no other rows were found
				}

				//Else, since a row was already found, check to see if it was a different row
				else if col != 1 {
				
					col = 0; //Reset col to invalid input if a different row was already found
				}
			}
		}

		//Check for 2 through 9
		col_option = 2; //Set col_option to 2

		for col_name in ["2","3","4","5","6","7","8","9"].iter() {
		
			if input.contains(col_name) { //If input string contains the numeral name
			
				//Check to see that no other rows have been found
				if col == 0 {
				
					col = col_option; //Set col to the col_option we just compared
				}

				//Else, since a row was already found, check to see if it was a different row
				else if col != col_option {
				
					col = 0; //Reset col to invalid input
					break; //break out of the for loop
				}
			}

			col_option = col_option + 1; //Increment col_option
		}

		if col == 0 { //If column is still invalid at this point, we should go to next iteration and not search for a row
		
			println!("Input is invalid\n"); //Let the user know the input is invalid
			continue;
		}

		//Second, attempt to parse a row Letter

		//A B C D E F G H I J
		//one two three four five six seven eight nine ten
		//numeral names contain: E F I G H, so check that these exist with spaces on the end
		//E with a space on the end may actually need to be ignored, since it is the only one of those letters that any number ends with

		//Check if a numeral name was found, so we know to be careful about E F G H and I, and make sure they have spaces
		if col_name_found {

			let mut row_option = 1; //Let row_option be 1, this is the row option we are comparing
		
			for row_name in ["A","B","C","D","E ","F ","G ","H ", "I ", "J"].iter() {
		
				if input.contains(row_name) { //If input string contains one of the above letters, some with spaces after them
			
					//Check to see that no other rows have been found, or that 5 was found, since "E " may be read at the end of a number
					if (row == 0) || (row == 5) {
				
						row = row_option; //Set row to the row_option we just compared
					}

					//Else, since a row was already found:
					//If the row was different, and this is not 5 meaning we didn't just find an E
					else if (row != row_option) && (row_option != 5) {
				
						row = 0; //Reset row to invalid input
						break; //break out of the for loop
					}
				}

				row_option = row_option + 1; //Increment row_option
			}
		}
		else { //Column name not found, so we need not be careful about letters

			let mut row_option = 1; //Let row_option be 1, this is the row option we are comparing
		
			for row_name in ["A","B","C","D","E","F","G","H","I","J"].iter() {
		
				if input.contains(row_name) { //If input string contains the numeral name
			
					//Check to see that no other rows have been found
					if row == 0 {
				
						row = row_option; //Set row to the row_option we just compared
					}

					//Else, since a row was already found, check to see if it was a different row, other than "E", because "E " may have been read off the end of a number
					else if row != row_option {
				
						row = 0; //Reset row to invalid input
						break; //break out of the for loop
					}
				}

				row_option = row_option + 1; //Increment row_option
			}
		}

		if row == 0 { //If row is 0 at this point, input was invalid

			println!("Input is invalid\n"); //Let the user know the input is invalid
		}
	} //If input was invalid this loop repeats

	println!("Coordinates interpreted: {} {}", row, col);
}

/**********************************************************************************************
* Function Name: display_instructions
* 
* Input: None
* Output: None
*
* Description: Displays the instructions of Salvo
**********************************************************************************************/

fn display_instructions() {

	println!("Salvo is a classical naval combat simulation.");
	println!("Each player is allocated their own 10x10 grid of open sea, where they can place their ships.");
	println!("There are 5 classes of ships, each taking up a certain number of spaces on the game board.");
	println!("Both players must deploy all 5 ships before the simulation can begin.");
	println!("");
	println!("     Patrol Boat       [P]      Length: 2 Spaces");
	println!("     Submarine         [S]      Length: 3 Spaces");
	println!("     Destroyer         [D]      Length: 3 Spaces");
	println!("     Battleship        [B]      Length: 4 Spaces");
	println!("     Aircraft Carrier  [C]      Length: 5 Spaces");
	println!("");
	println!("The first player will select a spot on the enemy's board to attack.");
	println!("The player cannot see the placement of the enemy's ships on the enemy's grid. He or she must try to guess where the ships are.");
	println!("The player selects a location to attack by typing in the coordinates, denoted by the letters and numbers on the edges of the game board.");
	println!("Letter denotes row, and number denotes column.");
	println!("If part of the enemy ship takes up the attacked square, it is a HIT, and an 'X' will be placed there.");
	println!("If the attack hits a square that does not contain a ship, it is a MISS, and a '~' symbol will be placed there.");
	println!("If the player does get a HIT, they get to make another attack. Player continues to attack until they get a MISS.");
	println!("Once the player missed, it is the next player's turn to attack.");
	println!("A ship is considered SUNK if all of it's spaces are hit by the enemy");
	println!("The game ends when all of one of the player's ships are sunk. The player with some ships still afloat is declared the winner!");
	println!("");
}