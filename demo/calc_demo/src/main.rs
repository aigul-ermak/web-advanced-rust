use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;


// TODO #1: Create an enumeration structure for Male and Female
enum Gender {
   
   // TODO #1
}

// TODO #2: Implement a method for an enumeration that matches a string input to a specific gender
impl Gender {
    fn from_str(input: &str) -> Option<Gender> {
        match input.to_lowercase().as_str() {

            // TODO #2
          
        }
    }
}


// TODO #3: Define the CalorieResults struct
struct CalorieResults {
   
}

// TODO #4: Implementation of methods forCalorieResults struct
impl CalorieResults {
    // TODO #5 Create new instanse of the CalorieResults struct
    fn new(gender: Gender, weight: f32, height: f32, age: f32, activity_level: usize) -> Self {
        let mut result = CalorieResults {
           
        };
        result.calculate_tdee();
        result
    }

    // TODO #5: Create method for calculate bmr
    fn calculate_bmr(&self) -> f32 {
        match self.gender {
            
        }
    }

    // TODO #6: Create method for calculate tdee 
    fn calculate_tdee(&mut self) {
      
    }
}

fn get_input(prompt: &str) -> f32 {
    loop {
        // Display the prompt to the user
        println!("{}", prompt); 
        let mut input = String::new();
        // Read user input
        std::io::stdin().read_line(&mut input).expect("Failed to read line"); 
        match input.trim().parse::<f32>() {

            // TODO #7: Create match arms for Ok(num) to capture valid numerical input 
            // and Err(_) with a message "Invalid input for weight. Please enter a valid number."         
        }
    }
}


fn main() {
    loop {
        println!("Calorie Calculator");

        let gender = loop {
            let input = input("Enter your gender (m/f): ");
            match Gender::from_str(&input) {

            // TODO #8: Implement match arms to handle gender input.
            // Use `Some(Gender::Male)` for 'm' or `Some(Gender::Female)` for 'f' to capture valid input.
            // Use `None` to print an error message: "Invalid gender. Please enter 'm' for male or 'f' for female."

            }
        };

        let weight = get_input("Enter your weight in kg: ");
        let height = get_input("Enter your height in cm: ");
        let age = get_input("Enter your age in years: ");        

        let activity_level = loop {
            let input = input("Enter your activity level (1-5): ");
            match input.trim().parse::<usize>() {

            // TODO #9: Implement match arms for parsing the activity level input.
            // Ensure the parsed number (num) is within the 1-5 range. If so, break the loop and return the num.
            // Otherwise, use the Err(_) arm or a condition on Ok(num) to print an error message: 
            // "Invalid activity level. Please enter a number between 1 and 5."
               
            }
        };


       // TODO #10: Instantiate a new object of CalorieResults with the collected data.    

        print_calorie_results_to_console(&results);

        if input("Do you want to calculate for another person? (Y/N): ").trim().to_lowercase() != "y" {
            break;
        }
    }
        println!("Thank you for using the Calorie Calculator!");
}


// Function to accept and return user input as a string.
// It displays a prompt to the user, reads a line from standard input, trims it, and returns the result.
fn input(prompt: &str) -> String {    
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


// Function to print the calorie results to the console.
// It formats and displays the details of the CalorieResults struct, including gender, weight, height, age, activity level, and estimated daily calorie needs.
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

