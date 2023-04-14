// I wrote this Rust a while ago!
extern crate zxcvbn;

use std::env;
use std::io;
use zxcvbn::zxcvbn;

fn main() {
    let args: Vec<_> = env::args().collect();

    println!("\n");
    loop {
        let pass = if args.len() > 1 && args[1].contains('u') {
            println!("Enter the password to check (it will be displayed here): ");
            gets().unwrap()
        } else {
            rpassword::prompt_password("Enter the password to check: ").unwrap()
        };

        println!("What's your most common username?");
        let username = gets().unwrap();

        let estimate =
            zxcvbn(&pass, &[&username, "email", "gmail", "twitter", "facebook"]).unwrap();
        println!("\nScore: {} out of 4\n", estimate.score());
        // println!(
        //     "Estimated number of guesses needed to crack: {} (10 ^ {})",
        //     estimate.guesses, estimate.guesses_log10
        // );

        print_guess_time(&estimate.crack_times());
        println!();
        give_feedback(estimate.feedback());
        println!();

        if args.len() > 1 && args[1].contains('l') {
            println!("---------------------------");
            continue;
        } else {
            break;
        }
    }
}

fn print_guess_time(crack_times: &zxcvbn::time_estimates::CrackTimes) {
    println!(
        "In a throttled online attack:      {}",
        crack_times.online_throttling_100_per_hour()
    );
    println!(
        "In an unthrottled online attack:   {}",
        crack_times.online_no_throttling_10_per_second()
    );
    println!(
        "In a slowed offline attack:        {}",
        crack_times.offline_slow_hashing_1e4_per_second()
    );
    println!(
        "In a fast offline attack:          {}",
        crack_times.offline_fast_hashing_1e10_per_second()
    );
}

fn give_feedback(feedback: &Option<zxcvbn::feedback::Feedback>) {
    let spacer = "   ";
    match feedback {
        Some(feedback) => {
            if let Some(warning) = feedback.warning() {
                println!("Warning: {}\n", warning);
            }
            // match feedback.warning {
            //     Some(warning) => println!("Warning: {}", warning),
            //     None => (),
            // }
            println!("Suggestions");
            for suggestion in feedback.suggestions() {
                println!("{}- {}", spacer, suggestion)
            }
        }
        None => println!("No suggestions."),
    }
}

fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}
