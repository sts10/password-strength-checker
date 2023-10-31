// I wrote this Rust a while ago!
use std::io;
use zxcvbn::zxcvbn;

fn main() {
    let pass = rpassword::prompt_password("Enter the password to check: ").unwrap();

    println!("What's your most common username?");
    let username = gets().unwrap();

    let estimate = zxcvbn(&pass, &[&username, "email", "gmail", "twitter", "facebook"]).unwrap();
    println!("\nScore: {} out of 4\n", estimate.score());

    print_guess_time(&estimate.crack_times());
    println!();
    give_feedback(estimate.feedback());
    println!();
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
            println!("Suggestions");
            for suggestion in feedback.suggestions() {
                println!("{}- {}", spacer, suggestion)
            }
        }
        None => println!("No specific suggestions."),
    }
}

fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}
