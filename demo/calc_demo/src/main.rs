use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;


// enum
enum Gender {
    Male,
    Female,
}

// enum's method
impl Gender {
    fn from_str(input: &str) -> Option<Gender> {
        match input.to_lowercase().as_str() {
            "m" | "male" => Some(Gender::Male),
            "f" | "female" => Some(Gender::Female),
            _ => None,
        }
    }
}


// struct for result 
struct CalorieResults {
    gender: Gender,
    weight: f32,
    height: f32,
    age: f32,
    activity_level: usize,
    tdee: f32,
}

// struct's method
impl CalorieResults {
    fn new(gender: Gender, weight: f32, height: f32, age: f32, activity_level: usize) -> Self {
        let mut result = CalorieResults {
            gender,
            weight,
            height,
            age,
            activity_level,
            tdee: 0.0,
        };
        result.calculate_tdee();
        result
    }

    fn calculate_bmr(&self) -> f32 {
        match self.gender {
            Gender::Male => 88.362 + (13.397 * self.weight) + (4.799 * self.height) - (5.677 * self.age),
            Gender::Female => 447.593 + (9.247 * self.weight) + (3.098 * self.height) - (4.330 * self.age),
        }
    }
    fn calculate_tdee(&mut self) {
        let activity_multipliers = [1.2, 1.375, 1.55, 1.725, 1.9];
        self.tdee = self.calculate_bmr() * activity_multipliers[self.activity_level - 1];
    }
}


fn main() {
    loop {
        println!("Calorie Calculator");

        let gender = loop {
            let input = input("Enter your gender (m/f): ");
            match Gender::from_str(&input) {
                Some(gender) => break gender,
                None => println!("Invalid gender. Please enter 'm' for male or 'f' for female."),
            }
        };

        let weight = loop {
            let input = input("Enter your weight in kg: ");
            match input.trim().parse::<f32>() {
                Ok(num) => break num,
                Err(_) => println!("Invalid input for weight. Please enter a number."),
            }
        };

        let height = loop {
            let input = input("Enter your height in cm: ");
            match input.trim().parse::<f32>() {
                Ok(num) => break num,
                Err(_) => println!("Invalid input for height. Please enter a number."),
            }
        };

        let age = loop {
            let input = input("Enter your age in years: ");
            match input.trim().parse::<f32>() {
                Ok(num) => break num,
                Err(_) => println!("Invalid input for age. Please enter a number."),
            }
        };

        let activity_level = loop {
            let input = input("Enter your activity level (1-5): ");
            match input.trim().parse::<usize>() {
                Ok(num) if (1..=5).contains(&num) => break num,
                _ => println!("Invalid activity level. Please enter a number between 1 and 5."),
            }
        };

        let results = CalorieResults::new(gender, weight, height, age, activity_level);

        print_calorie_results_to_console(&results);

        if input("Do you want to calculate for another person? (Y/N): ").trim().to_lowercase() != "y" {
            break;
        }
    }
        println!("Thank you for using the Calorie Calculator!");
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


// function for print
fn print_calorie_results_to_console(results: &CalorieResults) {
    let border = "|--------------------------------------------------------|";
    let gender = format!("Gender: {}", match results.gender {
        Gender::Male => "Male",
        Gender::Female => "Female",
    });
    let weight = format!("Weight: {}kg", results.weight);
    let height = format!("Height: {}cm", results.height);
    let age = format!("Age: {}years", results.age);
    let activity_level = format!("Activity Level: {}", results.activity_level);
    let tdee = format!("Estimated Daily Calorie Needs: {}calories", results.tdee);

    println!("{}", border);
    println!("| Calorie Consumption Details                            |");
    println!("{}", border);
    println!("| {:<54} |", gender);
    println!("| {:<54} |", weight);
    println!("| {:<54} |", height);
    println!("| {:<54} |", age);
    println!("| {:<54} |", activity_level);
    println!("| {:<54} |", tdee);
    println!("{}", border);
}

// ???
fn save_results_to_file(results: &CalorieResults, file_path: &Path) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    writeln!(file, "Calorie Consumption Details")?;
    writeln!(file, "Gender: {}", match results.gender {
        Gender::Male => "Male",
        Gender::Female => "Female",
    })?;
    writeln!(file, "Weight: {}kg", results.weight)?;
    writeln!(file, "Height: {}cm", results.height)?;
    writeln!(file, "Age: {}years", results.age)?;
    writeln!(file, "Activity Level: {}", results.activity_level)?;
    writeln!(file, "Estimated Daily Calorie Needs: {}calories", results.tdee)?;
    Ok(())
}