#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

/*

    Created by: Arnab Chanda
    date: 21.05.20202

    A simple implementation of hangman in rust

*/


use std::io::{stdin,stdout,Write};
use std::char;
use std::collections::HashSet;
use std::cmp;
use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


//file reader
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

//user input to take single characters
fn get_user_input(s:&mut String) -> &mut String {
    let _=stdout().flush();
    stdin().read_line(s).expect("Did not enter a correct string");

    s.pop();
    return s;
}


// A word struct ro represent a word
struct Word{
    word: String,
    desc: String,
}

impl Word{
    //constructor
    pub fn new(word: String, desc: String) -> Self {
        Self{
            word,
            desc
        }
    }
}


//partial eq class for word
impl PartialEq for Word {
    fn eq(&self, other: &Word) -> bool {
        self.word == other.word
    }
}

// struct to draw words from
struct WordPool{
    words: Vec<Word>,
    index: usize,
    indexes: Vec<usize>,
}

impl WordPool{
    pub fn new() -> Self{
        Self{
            words   : vec![],
            index   : 0,
            indexes : vec![],
        }
    }

    //add new word in pool
    pub fn add_word( &mut self, word: String, desc: String) {
        self.words.push(Word::new(word, desc));
        self.indexes.push(self.index);
        self.index+=1;
    }

    //get new word from pool
    pub fn get_random_word(&mut self) -> (&Word,bool) {
        let default: usize = 999999999;
        let mut word_index = 0 as usize;
        let option = {
            let op = self.indexes.choose(&mut rand::thread_rng());
            word_index = match op{
                Some(number) => *number,
                None => default,
            };
        };        


        let mut end = false;

        if word_index != default {
            self.indexes.retain(|x| *x != word_index);
        }else if word_index == default {
            end = true;
        }

        if self.indexes.len() == 0 {
            end = true;
        }

        let word = &self.words[word_index];
        return (word,end);
    }
}

// fuction to put spaces
fn spacer() {
    println!();
    println!();
}

//funtion that prints wholesome patterns
fn print_graphics(s: String){
    let patt1 = ".........................................................";
    spacer();
    for i in 0..5{
        if i!=2 {
            println!("{}",patt1);
        }
        else {
            let len1 = patt1.len() as i32;
            let len2 = s.len() as i32;
            let count:i32 = (len1 - len2) /2;
            let mut c:i32 = 0;
            while c<count {
                print!("{}",'.');
                c+=1;
            }
            print!("{}",s);
            while c>0 {
                print!("{}",'.');
                c-=1;
            }
            print!("{}","\n");
        }
    }
    spacer();
}

//halp function for user
fn help() {
    print_graphics(String::from("R U L E S"));
    println!("you will be given the length of a word and a short descri");
    println!("-ption about what it is. Now you have to choose alphabets");
    println!("from a pool of letters which you think make up the word. ");
    println!("Your score will be the number of guesses that you will hav");
    println!("-e left after you are done guessing the word. If you fail ");
    println!("to guess the word before running out of guesses, you loose");
    print_graphics(String::from("E N J O Y !"));
    spacer();
}

//function that populates word pool from file
fn create_word_pool(wordPool: &mut WordPool) {
    let word = lines_from_file("words.in").expect("Could not load lines");
    let desc = lines_from_file("desc.in").expect("Could not load lines");

    for i in 0..word.len(){
        wordPool.add_word(word[i].to_string(), desc[i].to_string());
    }
}

fn validate_input(s: & String) -> bool {

    let x = &*s; //de referencing

    if x.len() == 0 {
        return false;
    }

    if x.len() > 1 {
        return false;
    }
    if !x.chars().next().unwrap().is_alphabetic() {
        return false;
    }

    return true;
}

fn new_game(wordPool: &mut WordPool, high_score: &mut i32){

    println!("..............................................High score : {}", high_score);
    println!();
    println!("Ready to begin? haha you have no choice anyway!");

    
    let mut game_over = false;
    let mut score = 0;

    while game_over == false {
        let (word_obj,end) = wordPool.get_random_word();

        let curr_word = &word_obj.word;
        let curr_desc = &word_obj.desc;

        let mut chars_selected:HashSet<char> = HashSet::new();
        let mut chars_inWord:HashSet<char> = HashSet::new();

        let mut won = false;

        let mut lives_left = 8;
        let mut hangman_str = String::from("");
        let mut char_pool = String::from("");
        
        let char_vec:Vec<char> = curr_word.chars().collect();

        for k in 0..char_vec.len() {
            chars_inWord.insert(char_vec[k]);
        }

        while lives_left > 0 && won == false {

            let mut cnt = 0;

            for i in 0..char_vec.len() {

                let ch = char_vec[i];
                if chars_selected.contains(&ch) {
                    hangman_str.push(ch);
                    hangman_str.push(' ');
                }
                else{
                    hangman_str.push('_');
                    hangman_str.push(' ');
                    cnt+=1;
                }
            } 

            if cnt == 0{
                won = true;
                break;
            }

            for i in 97u8..123u8 {
                let ch = i as char;
                if !chars_selected.contains(&ch) {
                    char_pool.push(ch);
                    char_pool.push(' ');
                }
            }

            println!("Word: {} ", hangman_str);
            println!("Description: {} ", curr_desc);

            println!("");
            println!("Choose a letter from : {}", char_pool);
            println!("Guesses left : {}",lives_left);
            spacer();

            let mut s = String::new();
               
            s = get_user_input(&mut s).to_string();

            if !validate_input(&s) {
                println!("Please enter a single alhabet which you think can make up the word!");
                println!();
                hangman_str = "".to_string();
                char_pool = "".to_string();
                continue;
            }

            let inp_ch = s.chars().next().unwrap();
            if chars_selected.contains(&inp_ch) {
                println!("You had aldready selected that character! Try Again!");
            }
            else if !chars_inWord.contains(&inp_ch) {
                println!("Sorry thats not in the word! You lost a life!");
                lives_left -= 1;
            }
            

            chars_selected.insert(inp_ch);
            hangman_str = "".to_string();
            char_pool = "".to_string();
        }

        if won == true {
            score+=lives_left;
            print_graphics(String::from("W E L L   D O N E"));
            println!("You guessed the word!! You got +{} Your current score is : {}",lives_left, score);
        }
        else if lives_left == 0 {
            game_over = true;
            print_graphics(String::from("G A M E   O V E R"));
        }

        if end == true {
            print_graphics(String::from("Y O U  B E A T  T H E  G A M E"));
            break;
        }
    }

    
    *high_score = cmp::max(*high_score, score);
}


fn main() {
    
    print_graphics(String::from("H A N G M A N"));
    let mut high_score = 0;
    
    loop {        
        println!("Press 1 to start a new game and 2 to exit and 3 for the rules!");
        let mut s = String::new();
        s = get_user_input(&mut s).to_string();

        match s {
            _ if s == "1" => {
                let mut wordPool = WordPool::new();
                create_word_pool(&mut wordPool);
                new_game(&mut wordPool, &mut high_score);
            }
            _ if s == "2" => break,
            _ if s == "3" => help(),
            _ => println!("Enter proper values please! you had entered:{} Either press 1 or press 2", s)
        }
        
    }
}
