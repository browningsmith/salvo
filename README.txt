#                   Program Name: salvo
#
#                         Author: Browning Keith Smith <browningsmith@email.arizona.edu>
#                           Date: December 2, 2019
#                   Last Updated: December 9, 2019
#
#                     Assignment: Project: Learn a New (to You!) Programming Language
#                         Part 3: Custom Program, Salvo (Battleship-like game)
#                       Due Date: December 9, 2019
#                          Class: CSc 372
#                     Instructor: L. McCann
#                            TAs: Tito Ferra, Josh Xiong
#
#                   Dependencies: See salvo_Cargo.toml
#
#                    Description: This program allows the user to play a Battleship-like game against
#                                 the CPU. The game board is displayed
#                                 in the console using lines and dashes to make the classic 10x10
#                                 board. Ships are denoted by capital letters. A miss is denoted
#                                 by the '~' symbol (it looks like a wave, doesn't it!). A hit is
#                                 denoted by a 'X' symbol. The user interfaces with the game by typing
#                                 commands into the console when prompted.
#
#                          Input: None
#                         Output: None
# Rust Language Study Google Doc: https://docs.google.com/document/d/1iInK2BWCybQMu_Oqyt0Vk5t5cwM1oJUSXZOpX7Y-5Jw/edit?usp=sharing_eip&ts=5ddc4adc
#
#    Example Execution (Windows): .\salvo.exe

Hello! This is a list of all the files that are required to compile my Salvo project:

  salvo_main.rs
  salvo_lib.rs
  salvo_Cargo.toml

In order to use Rust's cargo compiler to create a project for these files, type the following command:

  cargo new salvo

It will create a salvo directory for the project, with some other files created automatically

Replace the contents of Cargo.toml with the contents of salvo_Cargo.toml
Replace the contents of src\main.rs with the contents of salvo_main.rs
Replace the contents of src\lib.rs with the contents of salvo_lib.rs

After doing that type the following command to compile and run the program, from anywhere in the salvo directory:

  cargo run
