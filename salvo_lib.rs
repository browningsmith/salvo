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
 *                                 the CPU. The game board is displayed
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
 * This is the lib.rs file, it contains the bulk of the game logic. The main function, which is
 * called on execution, is located in main.rs. The main function makes extensive use of This
 * library file.
 **********************************************************************************************/

 //Dependencies
 use std::io;
 use std::io::Write;
 use std::process;
 use rand::Rng;

 //Global constants
 const BOARD_HEIGHT: usize = 10;
 const BOARD_WIDTH: usize = 10;

 /***********************************************************************************************
  * Enum Name: Difficulty
  *
  * Variants: Easy, Normal, hard
  *
  * Description: Variants of AI difficulty
  ***********************************************************************************************/

 #[derive(PartialEq)]
 enum Difficulty {
 
	Easy,
	Normal,
	Hard,
 }

 /***********************************************************************************************
  * Enum Name: Orientation
  *
  * Variants: Up, Right, Left, Down
  *
  * Description: Variants of how a ship can be placed
  ***********************************************************************************************/

 enum Orientation {
 
	Up,
	Right,
	Left,
	Down,
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

		for _row in 0..BOARD_HEIGHT { //For each row from 0 to BOARD_HEIGHT-1
		
			let new_row: Vec<char> = vec![' '; BOARD_WIDTH as usize];  //Create a new vector of characters, length BOARD_WIDTH, filled with spaces

			new_grid.push(new_row); //Push row on to grid
		}

		return GameBoard { //Create and return game board
		
			grid: new_grid,
		}
	}

	/**********************************************************************************************
	 * Function Name: get_height
	 * 
	 * Input: &self
	 * Output: usize
	 *
	 * Description: Returns the height of the board, ie. number of rows
	 **********************************************************************************************/

	pub fn  get_height(&self) -> usize {
	
		self.grid.len() //Return the height of first dimension of board, or number of rows
	}

	/**********************************************************************************************
	 * Function Name: get_width
	 * 
	 * Input: &self
	 * Output: usize
	 *
	 * Description: Returns the width of the board, ie. number of columns
	 **********************************************************************************************/

	pub fn  get_width(&self) -> usize {
	
		self.grid[0].len() //Return the height of second dimension of board, or number of columns
	}

	 /**********************************************************************************************
	 * Function Name: print_board
	 * 
	 * Input: &self, masked: bool, usize dot_row, usize dot_col
	 * Output: None
	 *
	 * Description: Displays the board to standard output. Behavior for a board larger than 10x10
	 *              is currently undefined. Prints a '*' in place of any existing symbol at location
	 *              dot_row, dot_col, if they are within bounds
	 *
	 *              If masked is set to true, hides all ship designations
	 **********************************************************************************************/

	 pub fn print_board(&self, masked: bool, dot_row: usize, dot_col: usize) {
	 
		println!("      1   2   3   4   5   6   7   8   9   10 "); //Print column numbers
		println!("    |---|---|---|---|---|---|---|---|---|---|"); //Print top border

		for row in 0..self.grid.len() { //For each row in the grid
		
			print!(" {}  |", (row as u8 + 65) as char); //Print a space, the letter of the row (65 is ascii for capital A), a space and a vertical divider

			for col in 0..self.grid[row].len() { //For each column of the row
			
				//If row and column match dot_row and dot_col
				if (row == dot_row) && (col == dot_col) {
				
					print!(" * |"); //Print a space, a '*', a space, and a vertical divider
				}
				else {

					if masked {
				
						//Print a space, the character at this location, a space, and vertical divider
						print!(" {} |", match self.grid[row][col] {
					
							'~' => '~', //If it is a miss character, okay to print
							'X' => 'X', //If it is a hit character, okay to print
							_ => ' ', //anything else hide with a space
						});
					}
					else {
				
						//Print a space, the character at this location, a space, and vertical divider
						print!(" {} |", self.grid[row][col]);
					}
				}
			}

			//Print a new line
			println!("");
			println!("    |---|---|---|---|---|---|---|---|---|---|"); //Print a horizontal divider
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
	 * Description: Returns the character of the space at the given coordinates
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
  * Attributes: String name, Fleet fleet, GameBoard board, update_string: String
  *
  * Description: Contains information about a player
  ***********************************************************************************************/

  struct Player {
  
	//name: String,
	fleet: Fleet,
	board: GameBoard,
	update_string: String,
	honed: bool,
	hit_origin_row: usize,
	hit_origin_col: usize,
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

	fn new_player() -> Player {
	
		Player {
		
			//name: String::from(name),
			fleet: Fleet::new_fleet_standard(),
			board: GameBoard::new_board_empty(),
			update_string: String::new(),
			honed: false,
			hit_origin_row: 0,
			hit_origin_col: 0,
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

		let mut arranging = true; //Set arranging flag to true, as long as this is true, we are continuing to arrange the fleet

		//Print current arrangement stats
		self.arrangement_stats(11,11);
	 
		//as long as we are still arranging
		while arranging {

			let mut invalid_command = true; //Set invalid_command flag to true. As long as this is true, repeat next loop

			while invalid_command {

				//Reset invalid_command to false
				invalid_command = false;

				//Display arrangement instructions
				display_arrangement_instructions();

				//Get user input to parse
				let mut input = get_input_or_exit("What would you like to do, Admiral?");

				//Parse a command

				//none is -1, so assume place or move
				//double entry is -2, so invalid command
				//random is 1
				//remove or clear is 2
				let command = find(&input, &["ran", "rem", "cle", "lear"], &[1,2,2,2]);

				//Parse for the word "all"

				//if all is found, 1, otherwise -1
				let all = find(&input, &["all"], &[1]);

				//Parse for a ship name
				//none is -1, so valid for a remove all, clear all, or random command
				//double entry is -2, so invalid on remove, clear, or move command
				//Patrol Boat is 0
				//Submarine is 1
				//Destroyer is 2
				//Battleship is 3
				//Aircraft Carrier is 4
				let ship_selection = find(&input, &["pat", "boa", "sub", "mar", "des", "roy", "bat","air","cra","car","rier"], &[0,0,1,1,2,2,3,4,4,4,4]);

				//If the command is 1, do a random arrangement
				if command == 1 {

					//Randomize the Fleet
					self.randomize_fleet();
				
					//Print current arrangement stats
					self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

					//Explain to the user that this feature has not been implemented yet
					println!("Ships placed in random arrangement\n");
				}

				//If the command is 2, and all is 1, ask the user if they are sure they want to clear the board
				else if (command == 2) && (all == 1) {
				
					if prompt_yn("Are you sure you want to remove all your ships? Type 'yes' or 'no'") { //Ask if they are sure
					
						//Clear the board
						self.clear_board();

						//Print current arrangement stats
						self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

						//Tell the user that the board has been cleared
						println!("Board cleared\n");
					}
					else { //If they are not sure
					
						//Print current arrangement stats, but don't clear the board
						self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);
					}
				}

				//If the command is 2, and all is not 1, meaning the user wants to remove only one Ship
				else if command == 2 {
				
					//If no valid ship was selected
					if (ship_selection == -1) || (ship_selection == -2) {
					
						//Print current arrangement stats
						self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

						//Explain that the user entered an invalid command
						println!("Invalid command.\n");

						invalid_command = true; //Set invalid command to true
					}
					else {
					
						self.remove_ship(ship_selection as usize); //remove the selected ship from the board

						//Print current arrangement stats
						self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

						//Tell the user the ship wasremoved
						println!("Your {} was removed from the board.\n", self.fleet.ships[ship_selection as usize].get_name());
					}

				}

				//If the command is a double entry, input is invalid
				else if command == -2 {
				
					//Print current arrangement stats
					self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

					//Explain that the user entered an invalid command
					println!("Invalid command.\n");

					invalid_command = true; //Set invalid command to true
				}

				//If no command was entered, assume user wants to move or place a ship
				else {
				
					//If no valid ship name was entered
					if (ship_selection == -1) || (ship_selection == -2) {
					
						//Print current arrangement stats
						self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

						//Explain that the user entered an invalid command
						println!("Invalid command.\n");

						invalid_command = true; //Set invalid command to true
					}
					else {

						//Start trying to place the Ship
						let mut placing = true;

						while placing {
					
							//Ask the user for coordinates to place the ship At

							input = get_input_or_exit(&(format!("Enter the coordinates at which you wish to place your {}", self.fleet.ships[ship_selection as usize].get_name())));

							//If the word cancel is detected, abort placement
							if find(&input, &["can"], &[1]) == 1 {

								//Print current arrangement stats
								self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);
						
								println!("Placement cancelled\n");
								placing = false; //Set placing to false, so we stop trying to place the ship.
							}
							else {
						
								//Attempt to parse coordinates
								let (row, col) = find_coordinates(&input);

								//If coordinates valid
								if row != -1 { //col will also be -1 according to find_coordinates API
							
									//If the symbol is not empty, and does not equal this ship's letter
									if (self.board.get_space(row as usize, col as usize) != ' ') && (self.board.get_space(row as usize, col as usize) != self.fleet.ships[ship_selection as usize].get_letter()) {
									
										//Print current arrangement stats
										self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

										println!("Cannot place ship there. There is an obstruction.\n");
									}
									else {

										self.remove_ship(ship_selection as usize); //remove the selected ship from the board
									
										//Print current arrangement stats, with a dot showing where we are trying to place
										self.arrangement_stats(row as usize, col as usize);

										//Start trying to orient the Ship
										let mut orienting = true;

										while orienting {
										
											//Ask the user for an orientation
											input = get_input_or_exit(&(format!("Would you like your {} to be facing 'up', 'down', 'left' or 'right' from that space?\nIt is {} spaces long.", 
											                                    self.fleet.ships[ship_selection as usize].get_name(),
																				self.fleet.ships[ship_selection as usize].get_length())));

											//If the word cancel is detected, abort placement
											if find(&input, &["can"], &[1]) == 1 {
											
												//Print current arrangement stats
												self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);
						
												println!("Placement cancelled\n");
												orienting = false; //Set orienting to false, so we stop trying to place the ship.
												placing = false; //Set placing to false, so we stop trying to place the ship.
											}
											else {
											
												//Attempt to parse an Orientation
												//-2 is invalid
												//-1 is Invalid
												//1 is Up
												//2 is Down
												//3 is Left
												//4 is right
												let orientation = find(&input, &["up","dow","lef","rig"], &[1,2,3,4]);

												//If no valid orientation was given
												if orientation < 0 {
												
													//Print current arrangement stats
													self.arrangement_stats(row as usize, col as usize);

													println!("Invalid command.\n");
												}
												else {
												
													//Set orientation to match the desired direction
													let orientation = match orientation {
													
														1 => Orientation::Up,
														2 => Orientation::Down,
														3 => Orientation::Left,
														4 => Orientation::Right,
														_ => Orientation::Up, //This should not happen, but compiler needs pleasing
													};

													//Attempt to place the ship. If it doesn't work have the loop repeat
													if self.place_ship(ship_selection as usize, row as usize, col as usize, orientation) != true {
													
														//Print current arrangement stats
														self.arrangement_stats(row as usize, col as usize);

														println!("Cannot place ship there. There is an obstruction.\n");
													}
													else { //If it worked
													
														//Print current arrangement stats
														self.arrangement_stats(BOARD_WIDTH, BOARD_HEIGHT);

														println!("{} placed successfully at {}{}.\n", self.fleet.ships[ship_selection as usize].get_name(), (row as u8 + 65) as char, col);

														orienting = false; //Set orienting to false, so we stop trying to place the ship.
														placing = false; //Set placing to false, so we stop trying to place the ship.
													}
												}
											}
										}
									}
								}
								else { //If coordinates are invalid
								
									//Print current arrangement stats
									self.arrangement_stats(BOARD_WIDTH,BOARD_HEIGHT);

									println!("Invalid coordinates.\n");
								}
							}
						}
					}
				}
			}

			//If all the ships have been Placed
			if self.fleet.get_deployed() == self.fleet.size() {
			
				//Ask the user if they are done arranging their ships
				if prompt_yn("Are you finished arranging all of your ships, Admiral? Type 'yes' or 'no'.") {
				
					arranging = false; //Set arranging to false
				}
				else {
				
					arranging = true; //Set arranging to true
				}
			}
			else { //If not all ships are placed
			
				arranging = true; //set arranging to true
			}
		}
	 }

	 /**********************************************************************************************
	 * Function Name: randomize_fleet
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: Player's GameBoard and Fleet objects
	 *
	 * Description: Handles the process of the CPU arranging ships randomly. Is also called By
	 *              arrange_fleet if the user wishes to randomly arrange their own fleet.
	 *
	 *              Currently only designed to work with a standard Salvo fleet
	 **********************************************************************************************/

	 fn randomize_fleet(&mut self) {
	 
		self.clear_board(); //Clear the board

		//For each ship in the Salvo standard fleet
		for ship_no in 0..self.fleet.size() {
		
			let mut placing = true; //Set placing to true. As long as this is true, continue generating random placement for this ship
			while placing {

				let row = rand::thread_rng().gen_range(0,10);
				let col = rand::thread_rng().gen_range(0,10);
				let ori_select = rand::thread_rng().gen_range(1,5);

				let orientation = match ori_select {
			
					1 => Orientation::Up,
					2 => Orientation::Down,
					3 => Orientation::Left,
					4 => Orientation::Right,
					_ => Orientation::Up, //This should not happen, but compiler needs pleasing
				};

				/*println!("Attempting to place {} at {} {} facing {}.",
						 self.fleet.ships[ship_no as usize].get_name(),
						 (row as u8 + 65) as char,
						 col,
						 match orientation {
					 
							Orientation::Up => "up",
							Orientation::Down => "down",
							Orientation::Left => "left",
							Orientation::Right => "right",
						 }
				);*/

				//Attempt to place the ship, and set placing to false if it is successfull
				if self.place_ship(ship_no as usize, row as usize, col as usize, orientation) {
				
					/*println!("Placement of {} at {} {} successful.",
							self.fleet.ships[ship_no as usize].get_name(),
							(row as u8 + 65) as char,
							col);*/

					placing = false;
				}
				else {
				
					/*println!("Placement of {} at {} {} unsuccessful.",
						 self.fleet.ships[ship_no as usize].get_name(),
						 (row as u8 + 65) as char,
						 col);*/

					placing = true;
				}
			}
		}
	 }

	 /**********************************************************************************************
	 * Function Name: arrangement_stats
	 * 
	 * Input: &self, dot_row: usize, dot_col: usize
	 * Output: None
	 *
	 * Description: Prints the gameboard to show current arrangement, and other arrangement stats.
	 *              Prints a '*' on the board if dot_row and dot_col are within bounds
	 **********************************************************************************************/

	 fn arrangement_stats(&self, dot_row: usize, dot_col: usize) {
	 
		self.board.print_board(false,dot_row,dot_col); //Print the board, unmasked

		print!("   Patrol Boat      [P]        2 Spaces     Status: "); //Show the Patrol Boat stats
		if self.fleet.ships[0].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Submarine        [S]        3 Spaces     Status: "); //Show the Sub stats
		if self.fleet.ships[1].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Destroyer        [D]        3 Spaces     Status: "); //Show the Destroyer stats
		if self.fleet.ships[2].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Battleship       [B]        4 Spaces     Status: "); //Show the Battleship stats
		if self.fleet.ships[3].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		print!("   Aircraft Carrier [C]        5 Spaces     Status: "); //Show the Aircraft Carrier stats
		if self.fleet.ships[4].get_placed() { println!("Placed"); } else { println!("Not Placed"); }
		println!("");
	 }

	 /**********************************************************************************************
	 * Function Name: place_ship
	 * 
	 * Input: &mut self, ship_no: usize, row: usize, col: usize, orientation: u32
	 * Output: bool
	 *
	 * Changes: Player's GameBoard and Fleet objects
	 *
	 * Description: Attempts to place a ship on the board at the given coordinates with the given
	 *              Orientation. If able to place ship, it writes the ship's letter to the appropriate
	 *              spaces on the board, changes the ship object to placed, and returns true. If
	 *              it is unable to place the ship due to the area being out of bounds, or if There
	 *              is another ship in the way, it does nothing, and returns false.
	 **********************************************************************************************/

	 fn place_ship(&mut self, ship_no: usize, row: usize, col: usize, orientation: Orientation) -> bool {

		let ship = &mut self.fleet.ships[ship_no];
	 
		for n in 0..ship.get_length() { //n will be used to scan a section of spaces to make sure a ship can be placed
		
			let mut r = row as i32; //Create copies of row and col
			let mut c = col as i32;

			//Depending on orientation, change r and c to the next square to look at
			match orientation {
			
				Orientation::Up => r -= n as i32,
				Orientation::Down => r += n as i32,
				Orientation::Left => c -= n as i32,
				Orientation::Right => c += n as i32,
			}

			//If the squre is not inbounds
			if (r < 0) || (r >= self.board.get_width() as i32) || (c < 0) || (c >= self.board.get_height() as i32) {
			
				//println!("Unable to place ship! {} {} is out of bounds!", r, c);
				return false; //Return false, without doing anything
			}

			//If the square is anything other than a space
			else if self.board.get_space(r as usize, c as usize) != ' ' {
			
				//println!("Unable to place ship! {} {} is not empty!", r, c);
				return false; //Return false, without doing anything
			}
		}

		//At this point, it is clear to place the ship

		for n in 0..ship.get_length() {

			let mut r = row as i32; //Create copies of row and col
			let mut c = col as i32;

			//Depending on orientation, change r and c to the next square to look at
			match orientation {
			
				Orientation::Up => r -= n as i32,
				Orientation::Down => r += n as i32,
				Orientation::Left => c -= n as i32,
				Orientation::Right => c += n as i32,
			}

			//Write the ships letter to the board
			self.board.write_space(r as usize, c as usize, ship.get_letter());
		}

		//Set ship placed to true
		ship.place();

		//Return true
		return true;
	 }

	 /**********************************************************************************************
	 * Function Name: remove_ship
	 * 
	 * Input: &mut self, ship_no: usize
	 * Output: None
	 *
	 * Changes: Player's GameBoard and Fleet objects
	 *
	 * Description: Removes a ship from the board, if it was placed. Does nothing if ship was Not
	 *              placed
	 **********************************************************************************************/

	 fn remove_ship(&mut self, ship_no: usize) {
	 
		let ship = &mut self.fleet.ships[ship_no];

		//Remove this ship's symbol from the board
		self.board.clear_symbol(ship.get_letter());

		//Set ship to not placed
		ship.remove();
	 }

	 /**********************************************************************************************
	 * Function Name: clear_board
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: Player's GameBoard and Fleet objects
	 *
	 * Description: Replaces all characters on the board with spaces. Changes all ships in the fleet
	 *              to not placed
	 **********************************************************************************************/

	 fn clear_board(&mut self) {
	 
		self.board = GameBoard::new_board_empty(); //Set this player's game board to a new empty board

		for n in 0..self.fleet.size() { //For each ship in the fleet
		
			self.fleet.ships[n as usize].remove(); //Set it to not placed
		}
	 }

	 /**********************************************************************************************
	 * Function Name: attack
	 * 
	 * Input: &mut self, row: usize, col: usize
	 * Output: (hit: bool, sunk: bool, letter: char)
	 *
	 * Changes: Player's GameBoard and Fleet objects
	 *
	 * Description: Attacks a space on the player's board. If it is a miss, it sets that space to
	 *              a '~'. If it is a hit, it sets that space to a 'X'.
	 *
	 *              Returns (false, false, ' ') if it is a miss
	 *              Returns (false, false, '~') if the attacked space was already a miss
	 *              Returns (false, false, 'X') if the attacked space was already a HIT
	 *              Returns (true, false, char) if the attack is a hit, and returns the character that was in the space
	 *              Returns (true, true, char) if the attack is a hit, and if the attacked ship was sunk,
	 *              and returns the character of the ship that was sunk.
	 **********************************************************************************************/

	 fn attack(&mut self, row: usize, col: usize) -> (bool, bool, char) {
	 
		let space = self.board.get_space(row, col); //Grab a copy of the character on that space
		
		//if the space is a miss
		if space == ' ' {
		
			//set that space on the board to the miss symbol '~'
			self.board.write_space(row, col, '~');

			return (false, false, ' '); //return no hit, no sunk, empty space ' '
		}
		else if space == '~' { //If it was already a miss
		
			return (false, false, '~'); //return no hit, no sunk, miss symbol '~'
		}
		else if space == 'X' { //If it was already a hit
		
			return (false, false, 'X'); //return no hit, no sunk, hit symbol 'X'
		}
		else { //If it is anything else, then it is a hit!

			//set that space on the board to the hit symbol 'X'
			self.board.write_space(row, col, 'X');
		
			//for each ship in the fleet, 
			for ship_no in 0..self.fleet.size() {
			
				//if the character matches the ship letter
				if self.fleet.ships[ship_no as usize].get_letter() == space {
				
					//Damage the ship
					let sunk = self.fleet.ships[ship_no as usize].damage();

					//if the damage caused the ship to sink
					if sunk {
					
						return (true, true, space); //return hit, sunk, and the letter
					}
					else {
					
						return (true, false, space); //return hit, no sunk, and the letter
					}
				}
			}

			//if this statement is reached, the character that was read is not part of the fleet
			panic!("Error with attack. Letter detected is not in the fleet");
		}
	 }

	 /**********************************************************************************************
	 * Function Name: have_user_attack
	 * 
	 * Input: &mut self, round: i32
	 * Output: bool
	 *
	 * Changes: board, fleet
	 *
	 * Description: Handles the flow of a user attacking this player. Calls the user "Admiral"
	 *              Returns true if this process ends the game by sinking the fleet
	 **********************************************************************************************/

	 fn have_user_attack(&mut self, round: usize) -> bool {
	 
		//Have the user attack
		let mut attacking = true; //Set attacking to true

		while attacking {

			println!("ROUND {}\n", round); //Print the round Number
				
			self.board.print_board(true,11,11); //Show the enemy board

			print!("{}", &(self.update_string)); //Print the update_string

			self.update_string = String::new(); //reset update string

			println!("It is your turn, Admiral\n"); //Tell the user it is their turn

			let mut row = -1; //declare row and col to hold selected coordinates
			let mut col = -1;

			let mut getting_coordinates = true; //Set getting_coordinates to true

			while getting_coordinates {
					
				//Attempt to parse a row and column
				let input = get_input_or_exit("Select a space to attack by typing in a pair of coordinates.");

				let (r,c) = find_coordinates(&input);

				row = r; //Deconstruct tuple
				col = c;

				//If row and column are Invalid
				if row == -1 {
						
					self.board.print_board(true,11,11); //Show the enemy's board

					println!("Invalid coordinates.\n"); //Tell the user they entered invalid coordinates

					getting_coordinates = true; //set getting_coordinates to true
				}
				else {
						
					//Check to see if that spot has already been attacked
							
					//If that spot is an 'X'
					if self.board.get_space(row as usize, col as usize) == 'X' {
							
						self.board.print_board(true,11,11); //Show the enemy's board

						println!("You already attacked {}{}. It was a HIT!.\n", (row as u8 + 65) as char, col+1); //Tell the user that spot was already attacked

						getting_coordinates = true; //set getting_coordinates to true
					}
					//If that spot is a '~'
					else if self.board.get_space(row as usize, col as usize) == '~' {
							
						self.board.print_board(true,11,11); //Show the enemy's board

						println!("You already attacked {}{}. It was a miss.\n", (row as u8 + 65) as char, col+1); //Tell the user that spot was already attacked

						getting_coordinates = true; //set getting_coordinates to true
					}
							
					//But if it is anything Else
					else {
							
						self.board.print_board(true, row as usize, col as usize); //Show the enemy's board with the dot where we want to attack

						//Ask if the user wants to attack that space, if so set getting_coordinates to false
						if prompt_yn(&(format!("Are you sure you want to attack {}{}? Type 'yes' or 'no'.",
												(row as u8 + 65) as char, col+1)
										)) {

							getting_coordinates = false; //We have the coordinates so we can attack
						}
						else {
								
							getting_coordinates = true; //Set getting_coordinates to true
						}
					}
				}
			}

			//Attack the space with the given coordinates, and get status of attack
			let (hit,sink,letter) = self.attack(row as usize, col as usize);

			if hit != true { //If the attack was a miss
					
				println!("ROUND {}\n", round); //Print the round Number
				
				self.board.print_board(true,11,11); //Show the enemy's board

				println!("The attack on {}{} was a miss.\n", (row as u8 + 65) as char, col+1);

				println!("It is now the CPU's turn.\n");

				
				attacking = false; //Set attacking to false to end player1's Turn
						
				pause_for_enter();
			}
			else { //if the attack was a hit
					
				//If the CPU has any ships remaining
				if self.fleet.ships_remaining() > 0 {
						
					//Continue the turn

					
					attacking = true; //set attacking to true so player can continue their turn

					//format an update string
					let s = format!("The attack on {}{} was a hit!\n\n", (row as u8 + 65) as char, col+1);
					self.update_string.push_str(&s);

					//If that hit sunk a ship
					if sink {
					
						//Add on to the update string which ship was sunk
						for ship_no in 0..self.fleet.size() { //For each ship in the fleet
						
							if self.fleet.ships[ship_no as usize].get_letter() == letter { //If the letter matches
							
								//Add on to the update string which ship was sunk
								let s = format!("Good news, Admiral! You sunk the enemy's {}!\n\n", self.fleet.ships[ship_no as usize].get_name());
								self.update_string.push_str(&s);
							}
						}
					}
				}
				else { //GAME OVER
					
					self.board.print_board(true,11,11); //Show the enemy's final board layout

					println!("All enemy ships have been destroyed.\n");

					println!("Congratulations, Admiral! You are victorious!\n");
					
					attacking = false; //set attacking to false.
				}
			}	
		}

		//Check if there are still ships in the Fleet
		if self.fleet.ships_remaining() > 0 {
			
			return false; //return false, as the game is not over
		}

		return true; //return true, the game is over
	 }

	 /**********************************************************************************************
	 * Function Name: have_cpu_attack
	 * 
	 * Input: &mut self, round: usize, difficulty: Difficulty
	 * Output: bool
	 *
	 * Changes: board, fleet
	 *
	 * Description: Handles the flow of the cpu attacking this player. Calls the user "Admiral"
	 *              Returns true if this process ends the game by sinking the fleet
	 **********************************************************************************************/

	 fn have_cpu_attack(&mut self, round: usize, difficulty: &Difficulty) -> bool {
	 
		let mut attacking = true; //set attacking to true

		while attacking {
		
			//Have the cpu select a space to attack based on game difficulty
			let (row, col) = match difficulty {
			
				Difficulty::Easy => self.cpu_easy_logic(),
				Difficulty::Normal => self.cpu_hard_logic(Difficulty::Normal),
				Difficulty::Hard => self.cpu_hard_logic(Difficulty::Hard),
			};

			//Attack the space with the given coordinates, and get status of attack
			let (hit,sink,letter) = self.attack(row as usize, col as usize);

			println!("ROUND {}\n", round); //Print the round Number
				
			self.board.print_board(false,11,11); //Show the user's board

			if hit != true { //If the attack was a miss

				println!("The CPU attacked {}{}, and it was a miss!\n", (row as u8 + 65) as char, col+1);

				println!("It is now your turn, Admiral.\n");
				
				attacking = false; //Set attacking to false to end cpu's Turn
						
				pause_for_enter();
			}
			else { //if the attack was a hit
					
				//If the user has any ships remaining
				if self.fleet.ships_remaining() > 0 {
						
					//Set AI tracking data
					
					//If we were not currently honed
					if self.honed != true {
					
						//println!("Setting CPU to honed\n");
						self.honed = true;

						//Set hit_origin_row and hit_origin_col
						self.hit_origin_row = row;
						self.hit_origin_col = col;

						//println!("Focusing attacks around {}{}\n", (self.hit_origin_row as u8 + 65) as char, self.hit_origin_col+1);
					}

					//Continue the turn
					attacking = true; //set attacking to true so cpu can continue their turn

					//format an update string
					println!("The CPU attacked {}{} and it was a hit.\n", (row as u8 + 65) as char, col+1);

					//If that hit sunk a ship
					if sink {
					
						//set AI tracking data
						self.honed = false; //set honed to false
						//println!("Setting CPU to not honed. Resuming random attacks\n");

						//update the user as to which ship was sunk
						for ship_no in 0..self.fleet.size() { //For each ship in the fleet
						
							if self.fleet.ships[ship_no as usize].get_letter() == letter { //If the letter matches
							
								//update the user as to which ship was sunk
								println!("Bad news, Admiral. The enemy sunk your {}.\n", self.fleet.ships[ship_no as usize].get_name());
							}
						}
					}
					else { //if it did not sink the ship
					
						//update the user as to which ship was damaged
						for ship_no in 0..self.fleet.size() { //For each ship in the fleet
						
							if self.fleet.ships[ship_no as usize].get_letter() == letter { //If the letter matches
							
								//update the user as to which ship was sunk
								println!("The enemy has damaged your {}.\n", self.fleet.ships[ship_no as usize].get_name());
							}
						}
					}
				}
				else { //GAME OVER

					println!("The CPU has destroyed all of your ships. You have lost.\n");

					return true;  //Return true, as the game is over
				}

				println!("It is still the CPUs turn.\n");
				pause_for_enter();
			}
		}

		return false; //Return false, the game is not over yet
	 }

	 /**********************************************************************************************
	 * Function Name: cpu_easy_logic
	 * 
	 * Input: &mut self,
	 * Output: (row: usize, col: usize)
	 *
	 * Description: selects a row and column for the cpu to attack at random
	 **********************************************************************************************/

	 fn cpu_easy_logic(&mut self) -> (usize, usize) {
	 
		let mut selecting = true; //Set selecting to true
		let mut row = 0; //Declare row and column to arbitrary value
		let mut col = 0;

		while selecting {
		
			row = rand::thread_rng().gen_range(0,10) as usize; //for row and col generate a random number 0-9
			col = rand::thread_rng().gen_range(0,10) as usize;

			match self.board.get_space(row as usize, col as usize) {
			
				'X' | '~' => selecting = true, //If the board has already been attacked here, set selecting to true
				_=> selecting = false, //Otherwise set selecting to false, we are done selecting.
			}
		}

		return (row as usize, col as usize); //return the selected row and column
	 }

	 /**********************************************************************************************
	 * Function Name: cpu_hard_logic
	 * 
	 * Input: &mut self, difficulty: Difficulty
	 * Output: (row: usize, col: usize)
	 *
	 * Description: selects a row and column for the cpu to attack at random, but if the last Attack
	 *              the cpu made was a hit, it hones in and starts scanning for the rest of the ship
	 *
	 *              It uses Player variables honed, hit_origin_row, hit_origin_col, and sunk to
	 *              make decisions about its next move
	 **********************************************************************************************/

	 fn cpu_hard_logic(&mut self, difficulty: Difficulty) -> (usize, usize) {
	 
		let mut row = 0; //Initialize row and col
		let mut col = 0;

		if self.honed { //If we are currently honed in

			//println!("CPU is honed.\n");
			
			//self.hit_origin_row and hit_origin_col are both a hit 'X'

			//Start scanning up
			row = self.hit_origin_row as i32; //put stored hit origin coordinates into row and col
			col = self.hit_origin_col as i32;
			//println!("CPU is scanning up from {}{}\n", (row as u8 +65) as char, col+1);

			//as long as row is in bounds
			while row >= 0 {
			
				//if the space is a hit, decrement row
				if self.board.get_space(row as usize, col as usize) == 'X' {
				
					//println!("{}{} was a hit, moving up one\n", (row as u8 +65) as char, col+1);
					//Decrement row
					row -= 1;
				}
				//if the space is a miss, break out of this loop
				else if self.board.get_space(row as usize, col as usize) == '~' {
				
					//println!("{}{} was a miss, halting upward scan.\n", (row as u8 +65) as char, col+1);
					break;
				}
				//if the space is anything else, attack it!
				else {
				
					//println!("Empty space detected at {}{}, attacking\n", (row as u8 +65) as char, col+1);
					return (row as usize, col as usize);
				}
			}

			//Start scanning down
			row = self.hit_origin_row as i32; //put stored hit origin coordinates into row and col
			col = self.hit_origin_col as i32;
			//println!("CPU is scanning down from {}{}\n", (row as u8 +65) as char, col+1);

			//as long as row is in bounds
			while row < self.board.get_height() as i32 {
			
				//if the space is a hit, increment row
				if self.board.get_space(row as usize, col as usize) == 'X' {
				
					//println!("{}{} was a hit, moving down one\n", (row as u8 +65) as char, col+1);
					//Decrement row
					row += 1;
				}
				//if the space is a miss, break out of this loop
				else if self.board.get_space(row as usize, col as usize) == '~' {
				
					//println!("{}{} was a miss, halting downward scan.\n", (row as u8 +65) as char, col+1);
					break;
				}
				//if the space is anything else, attack it!
				else {
				
					//println!("Empty space detected at {}{}, attacking\n", (row as u8 +65) as char, col+1);
					return (row as usize, col as usize);
				}
			}

			//Start scanning left
			row = self.hit_origin_row as i32; //put stored hit origin coordinates into row and col
			col = self.hit_origin_col as i32;
			//println!("CPU is scanning left from {}{}\n", (row as u8 +65) as char, col+1);

			//as long as col is in bounds
			while col >= 0 {
			
				//if the space is a hit, increment col
				if self.board.get_space(row as usize, col as usize) == 'X' {
				
					//println!("{}{} was a hit, moving left one\n", (row as u8 +65) as char, col+1);
					//Decrement col
					col -= 1;
				}
				//if the space is a miss, break out of this loop
				else if self.board.get_space(row as usize, col as usize) == '~' {
				
					//println!("{}{} was a miss, halting leftward scan.\n", (row as u8 +65) as char, col+1);
					break;
				}
				//if the space is anything else, attack it!
				else {
				
					//println!("Empty space detected at {}{}, attacking\n", (row as u8 +65) as char, col+1);
					return (row as usize, col as usize);
				}
			}

			//Start scanning right
			row = self.hit_origin_row as i32; //put stored hit origin coordinates into row and col
			col = self.hit_origin_col as i32;
			//println!("CPU is scanning right from {}{}\n", (row as u8 +65) as char, col+1);

			//as long as col is in bounds
			while col < self.board.get_width() as i32 {
			
				//if the space is a hit, increment col
				if self.board.get_space(row as usize, col as usize) == 'X' {
				
					//println!("{}{} was a hit, moving right one\n", (row as u8 +65) as char, col+1);
					//increment col
					col += 1;
				}
				//if the space is a miss, break out of this loop
				else if self.board.get_space(row as usize, col as usize) == '~' {
				
					//println!("{}{} was a miss, halting rightward scan.\n", (row as u8 +65) as char, col+1);
					break;
				}
				//if the space is anything else, attack it!
				else {
				
					//println!("Empty space detected at {}{}, attacking\n", (row as u8 +65) as char, col+1);
					return (row as usize, col as usize);
				}
			}

			//All scans completed
			//println!("All scanning completed. Setting back to not honed\n");
			self.honed = false;
		}

		//println!("CPU is not honed. Resuming normal attacks\n");

		//Roll a 1 in 4 chance that cpu will attack a ship
		let roll = rand::thread_rng().gen_range(1,5);

		if (roll != 1) || (difficulty == Difficulty::Normal) { //If the roll was not 1, or if difficulty is set to Normal we attack at random
		
			return self.cpu_easy_logic(); //Get random coordinates
		}
		else { //Else, we attack a ship by totally cheating

			//println!("Difficulty is Hard, and roll was {}. Intentionally attacking ship.\n", roll);

			let mut selecting = true; //Set selecting to true
			while selecting {
		
				row = rand::thread_rng().gen_range(0,10); //for row and col generate a random number 0-9
				col = rand::thread_rng().gen_range(0,10);

				match self.board.get_space(row as usize, col as usize) {
			
					'X' | '~' | ' ' => selecting = true, //If the board has already been attacked here, or is an empty space, set selecting to true
					_=> selecting = false, //Otherwise set selecting to false, we are done selecting.
				};
			}
		}

		return (row as usize, col as usize);
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

	/*fn get_health(&self) -> u32 {
	
		self.health
	}*/

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
	 * Output: bool
	 *
	 * Description: Decrements ship health by 1. If ships health is already at 0 it does nothing
	 *              Returns true if this damage caused the ship to sink, or if ship was already
	 *              sunk.
	 **********************************************************************************************/

	 fn damage(&mut self) -> bool {
	 
		if self.health > 0 { //If health is greater than 0
		
			self.health -= 1; //remove one level of health
		}

		if self.health == 0 {
		
			return true;
		}

		return false;
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
	 * Description: returns a new instance of Salvo. By default, ai_difficulty is set to Easy.
	 *              2 Players are created, User and CPU, and each is given a standard unplaced
	 *              fleet and an empty game board
	 **********************************************************************************************/

	pub fn new_game() -> Salvo {
	
		Salvo {
		
			ai_difficulty: Difficulty::Easy, //Set AI difficulty to Easy
			player1: Player::new_player(),
			player2: Player::new_player(),
		}
	}

	/**********************************************************************************************
	 * Function Name: run_game
	 * 
	 * Input: &mut self
	 * Output: None
	 *
	 * Changes: self.ai_difficulty, player1, player2
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
		loop {
	
			self.select_difficulty(); //Have the user select difficulty for new game

			//Have the user arrange their ships before the game Begins
			println!("Admiral, it is time to deploy the fleet! Arrange your ships on the board below:\n");
			
			self.player1.arrange_fleet(); //Have the user arrange their fleet manually

			self.player2.randomize_fleet(); //Have the CPU arrange fleet randomly

			println!("Beginning combat simulation.\n");

			//As long as either player has ships remaining, do the following loop
			while (self.player1.fleet.ships_remaining() > 0) && (self.player2.fleet.ships_remaining() > 0) {

				//Turn based attack flow
				for round in 1..(BOARD_WIDTH*BOARD_HEIGHT) { //rounds should not exceed board area

					//Have player1, the user, attack the CPU, player 2
					if (self.player2.have_user_attack(round)) == true { //if player 1 ends the game
					
						break; 
					} //If player1 does not end the game
					else { 
					
						if (self.player1.have_cpu_attack(round, &self.ai_difficulty)) == true { break; } //If player2 ended the game, break
					}
				}
			}

			println!("GAME OVER\n");
			pause_for_enter();

			//Reset game objects
			self.player1 = Player::new_player();
			self.player2 = Player::new_player();
		}
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
*              Returns -1 if no options are found
*              Returns -2 if multiple options are found
*
*              However, if you would like there to be more than one option that return the same result, place
*              the same unsigned integer in the corresponding locations of the third argument for those options.
*              In this case the function will return that result if two different options that return
*              the same result are entered on one line. Otherwise, have a unique unsigned integer for each option.
*
*              Behavior is undefined if the second and third arguments are not equally
*              sized arrays.
**********************************************************************************************/

pub fn find(input: &str, options: &[&str], results: &[u32]) -> i32 {

	let mut result: i32 = -1; //declare result. This will be the return value of the Function. -1 indicates invalid input, -2 indicates double entry

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
				
				result = -2; //Reset result to -2, double entry
				break; //Break out of the for loop
			}
		}

		n = n + 1; //Increment n, and repeat for loop to check next option
	}

	//If result is -1 at this point, no options were found, otherwise it will be something from options array, or -2 if more than one was found

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
* Function Name: find_coordinates
* 
* Input: &str text
* Output: (u32, u32) coordinates
*
* Description: This method interpets a line of text to see if it contains valid Salvo coordinates. It is
*              actually quite robust in detecting coordinates of any variation of the letters A-J
*              for row letters, and numbers 1-10 or one-ten for column numbers.
*
*              Column and row can be entered in any order.
*
*              It is able to detect upper or lower case row numbers.
*
*              It is able to detect column numbers whether they are numerals or words.
*
*              Not case sensitive
*
*              Is a tad bit glitchy on reading user input that contain E F G H or I, since
*              those letters specifically exist inside some number words.
*
*              Returns coordinates as zero based, or returns -1,-1 if none were found
**********************************************************************************************/

pub fn find_coordinates(input: &str) -> (i32, i32) {

	//Attempt to parse a number name. -1 is nothing, -2 means double entry and is invalid
	let number_name = find(input, &["on","two","thr","fou","fiv","six","sev","eight","nin","ten"], &[1,2,3,4,5,6,7,8,9,10]);

	if number_name == -2 { //If there was a double entry
	
		//println!("Multiple number words detected");
		return (-1, -1); //Invalid coordinates, return -1, -1
	}

	//Here number_name is either -1 for nothing found, or it is something, 1-10

	//Attempt to parse a numeral. -1 is nothing, -2 means double entry and is invalid
	let mut number = find(input, &["1","2","3","4","5","6","7","8","9"], &[1,2,3,4,5,6,7,8,9]);

	if number == -2 { //If there was a double entry
	
		//println!("Multiple number digits detected");
		return (-1, -1); //Invalid coordinates, return -1, -1
	}

	if number == 1 { //If number is 1, go back and look for a 10

		//Attempt to find a 10. -1 means no
		number = find(input, &["10"], &[10]);

		if number == -1 { //If a 10 was not found, set number back to 1
		
			number = 1;
		}
	}

	//At this point. number_name and number may have both found something, or not

	//If neither one found anything
	if (number_name == -1) && (number == -1) {
	
		//println!("No numbers detected");
		return (-1, -1); //Invalid coordinates, return -1, -1
	}

	//If both number_name and number found something
	else if (number_name != -1) && (number != -1) {
	
		//If number_name and number are not the same thing, that is not okay
		if number_name != number {
		
			//println!("A number word and a differend number digit was detected");
			return (-1, -1); //Invalid coordinates, return -1, -1
		}
	}
	//Else if number_name is -1
	else if number == -1 {

		number = number_name; //Set number to number_name, since it contains the interpreted column value
	}

	//A B C D E F G H I J
	//one two three four five six seven eight nine ten
	//one contains E
	//three contains E
	//four contains F
	//five contains E, F, and I
	//six contains I
	//seven contains E
	//Eight contains E, G, H and I
	//Nine contains E and I
	//ten contains E

	//Next, attempt to parse a row letter
	let letter; //Initialize letter to invalid input

	//If a number name was found, we need to be careful about some letters, and check that there are spaces before or after them

	match number_name {
		
		1 => letter = find(input, &["a","b","c","d","e "," e","f","g","h","i","j"], &[1,2,3,4,5,5,6,7,8,9,10]),
		3 => letter = find(input, &["a","b","c","d","e "," e","f","g","h "," h","i","j"], &[1,2,3,4,5,5,6,7,8,9,10]),
		4 => letter = find(input, &["a","b","c","d","e","f ","g","h","i","j"], &[1,2,3,4,5,6,7,8,9,10]),
		5 => letter = find(input, &["a","b","c","d","e "," e","f ","g","h","i "," i","j"], &[1,2,3,4,5,5,6,7,8,9,9,10]),
		6 => letter = find(input, &["a","b","c","d","e","f","g","h","i "," i","j"], &[1,2,3,4,5,6,7,8,9,9,10]),
		7 => letter = find(input, &["a","b","c","d","e "," e","f","g","h","i","j"], &[1,2,3,4,5,5,6,7,8,9,10]),
		8 => letter = find(input, &["a","b","c","d","e ","f","g "," g","h "," h","i "," i","j"], &[1,2,3,4,5,6,7,7,8,8,9,9,10]),
		9 => letter = find(input, &["a","b","c","d","e "," e","f","g","h","i "," i","j"], &[1,2,3,4,5,5,6,7,8,9,9,10]),
		10 => letter = find(input, &["a","b","c","d","e "," e","f","g","h","i","j"], &[1,2,3,4,5,5,6,7,8,9,10]),
		_ => letter = find(input, &["a","b","c","d","e","f","g","h","i","j"], &[1,2,3,4,5,6,7,8,9,10]),
	}

	if letter == -2 { //If a double entry occured
	
		//println!("Multiple row letters detected");
		return (-1, -1); //Invalid coordinates, return -1, -1
	}
	else if letter == -1 { //If letter didn't find anything
	
		//println!("No row letters detected");
		return (-1, -1); //Invalid coordinates, return -1, -1
	}
	
	//At this point, we have valid coordinates, return them, as zero-based indexing
	(letter - 1, number - 1)
}

/**********************************************************************************************
* Function Name: display_instructions
* 
* Input: None
* Output: None
*
* Description: Displays the instructions of Salvo
**********************************************************************************************/

pub fn display_instructions() {

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

/**********************************************************************************************
* Function Name: display_arrangement_instructions
* 
* Input: None
* Output: None
*
* Description: Displays the instructions for how to arrange ships
**********************************************************************************************/


pub fn display_arrangement_instructions() {

	//Give instructions on how to place ships

	println!("To select a ship to place or move, enter the ship's name.");
	println!("You can cancel a 'move' command at any time by typing 'cancel'.");
	println!("To remove a ship, type 'remove' or 'clear', followed by the ship's name.");
	println!("To remove all ships from the board, type 'remove all' or 'clear all'.");
	println!("For a random arrangement, type 'random'.\n");
}