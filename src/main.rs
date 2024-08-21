// TODO explain overall project structure and imports.
mod api;
mod categories;
mod entities;

use std::io::{stdin, stdout, Write};

use anyhow::{Result, Context};
use colored::*;
use html_escape::decode_html_entities;
use rand::seq::SliceRandom;

use crate::api::get_questions;
use crate::categories::get_categories;

fn main() -> Result<()> {
    let categories = get_categories();

    println!("Choose your category, mortal:");
    for (index, (_, name)) in categories.iter().enumerate() {
        println!("{}. {}", index + 1, name);
    }

    print!("Enter the number of the category you want to play: ");
    // TODO explain why flush was used.
    stdout().flush().context("Failed to flush stdout in category prompt")?;

    let mut input = String::new();
    stdin().read_line(&mut input).context("Failed to read user category selection")?;
    let category_choice = input.trim().parse::<usize>().unwrap_or(0);

    // TODO explain this approach.
    let category_id = if category_choice > 0 && category_choice <= categories.len() {
        categories[category_choice - 1].0
    } else {
        println!("{}", "Invalid category. Using default category (Film).".yellow());
        11 // Default to Film category
    };

    // TODO explain ergonomics of adding error context here
    let response = get_questions(category_id).context("Problem fetching questions from API from inside main")?;

    let mut score = 0;
    let total_questions = response.results.len();

    for (index, question) in response.results.iter().enumerate() {
        println!("\n{}", format!("Question {} of {}:", index + 1, total_questions).blue().bold());
        // TODO explain why I used decode.
        println!("{}", decode_html_entities(&question.question).yellow().bold());

        let mut options = question.incorrect_answers.clone();
        options.push(question.correct_answer.clone());
        // TODO explain why I used shuffle.
        options.shuffle(&mut rand::thread_rng());

        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, decode_html_entities(option));
        }

        print!("Enter your answer (1-4): ");
        stdout().flush().context("Failed to flush stdout during answer prompt")?;

        let mut input = String::new();
        stdin().read_line(&mut input).context("Failed to read user answer selection")?;

        let user_answer = input.trim().parse::<usize>().unwrap_or(0);
        if user_answer > 0 && user_answer <= options.len() {
            let selected_answer = &options[user_answer - 1];
            if selected_answer == &question.correct_answer {
                println!("{}", "Correct!".green().bold());
                score += 1;
            } else {
                println!("{}", "Incorrect.".red().bold());
                println!("The correct answer was: {}", decode_html_entities(&question.correct_answer).green());
            }
        } else {
            println!("{}", "Invalid input.".red().bold());
            println!("The correct answer was: {}", decode_html_entities(&question.correct_answer).green());
        }
    }

    println!("\n{}", "Quiz completed!".blue().bold());
    println!("Your score: {} out of {}", score, total_questions);
    let percentage = (score as f32 / total_questions as f32) * 100.0;
    println!("Percentage: {:.1}%", percentage);
    match percentage as u8 {
        0..=59 => {
            let snarky_responses = [
                "Did you even try?",
                "Making random choices on every question isn't a great strategy.",
                "Maybe trivia isn't your thing. Have you considered a career in silent meditation?",
                "Well, at least you're consistent... consistently wrong.",
                "If ignorance is bliss, you must be ecstatic right now.",
            ];
            let response = snarky_responses.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", response.red().italic());
        },
        80..=100 => {
            let praise_responses = [
                "Impressive!",
                "Alright, you know what you're doing.",
                "You're a human encyclopedia!",
                "You must be everyone's favorite teammate on trivia night!",
                "Are you sure you didn't cheat?",
            ];
            let response = praise_responses.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", response.green().bold());
        },
        _ => println!("Not bad, but there's room for improvement!"),
    }

    Ok(())
}
