extern crate rpassword;
extern crate zxcvbn;

use std::io;
use std::str::FromStr;
use zxcvbn::zxcvbn;

fn main() {
    let pass = rpassword::read_password_from_tty(Some("Enter the password to check: ")).unwrap();
    println!("What's your most common username?");
    let username = gets().unwrap();

    let estimate = zxcvbn(&pass, &[&username, "news", "CNN", "twitter", "facebook"]).unwrap();
    println!("Score: {} out of 4", estimate.score);
    println!("");
    // println!(
    //     "Estimated number of guesses needed to crack: {} (10 ^ {})",
    //     estimate.guesses, estimate.guesses_log10
    // );

    print_guess_time(estimate.crack_times_display);
    println!("");
    give_feedback(estimate.feedback);
    println!("");
}

fn print_guess_time(crack_times: zxcvbn::time_estimates::CrackTimesDisplay) {
    println!(
        "In an (unthrottled) online attack: {}",
        crack_times.online_no_throttling_10_per_second
    );
    println!(
        "In a throttled online attack:      {}",
        crack_times.online_throttling_100_per_hour
    );
    println!(
        "In a slowed offline attack:        {}",
        crack_times.offline_slow_hashing_1e4_per_second
    );
    println!(
        "In a fast offline attack:          {}",
        crack_times.offline_slow_hashing_1e4_per_second
    );
}

fn give_feedback(feedback: Option<zxcvbn::feedback::Feedback>) {
    let spacer = "     ";
    match feedback {
        Some(feedback) => {
            match feedback.warning {
                Some(warning) => println!("Warning: {}", warning),
                None => (),
            }
            println!("Suggestions");
            for suggestion in feedback.suggestions {
                println!("{}- {}", spacer, suggestion)
            }
        }
        None => println!("No suggestions; your password is fine."),
    }
}

fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches("\n").to_string()),
        Err(error) => Err(error),
    }
}
