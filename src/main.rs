//Tempreture Conversion Program

//PROGRAM START

//Libraries Used
use std::io;

fn main() {
    
//Asks user what temprature unit they are using
    println!("Please tell me what unit of temprature measurement you are using");

//calls a mutable variable for user answer
    let mut unit = String::new();

//reads the user's answer and writes it to the variable "unit"
    io::stdin()
        .read_line(&mut unit)
        .expect("Could not read line");

//DEBUG:
//    println!("{}", unit);

//Trims the user input of any whitespace or new line character
    let unit = unit.trim();

//checks wether user input is one of the units of measurment and calls the
//appropriate function
    if unit == "celsius" {

//DEBUG:
//        println!("if function returned celsius");

        celsius_to_fahrenheit();
    }
    else if unit == "fahrenheit" {

//DEBUG:
//        println!("if function returned fahrenheit");

        fahrenheit_to_celsius();
    }
    else {
        println!("not a unit of measurment")
    }

}

fn celsius_to_fahrenheit() {

//DEBUG:
//    println!("Function called: celsius_to_fahrenheit");
    
//temp in celsius
    let mut temp = String::new();

    println!("Please input the temprature");

//takes user input    
    io::stdin()
        .read_line(&mut temp)
        .expect("Could not read line");

//takes input user gave, tims it of white space, and parses it.
    let temp: f64 = temp.trim().parse().expect("couldnt trim and parse");

//Conversion steps
    let x: f64 = (temp*1.8)+32.0;

    println!("{} degrees celsius in fahrenheit is {} degrees", temp, x);

}

fn fahrenheit_to_celsius() {

//DEBUG:
    println!("Function called: fahrenheit_to_celsius");

//Calls the temp variable
    let mut temp = String::new();

//Asks user the temprature
    println!("Please input the temprature");

//Grabs user input and does the trimming and parsing
    io::stdin()
        .read_line(&mut temp)
        .expect("Couldnt read line");
    let temp: f64 = temp.trim().parse().expect("couldnt trim and parse");

//does aformentioned conversion steps
    let x: f64 = ((temp - 32.0) * 5.0) / 9.0;

    println!("{} degrees fahrenheit is {} degress celsius", temp, x);

}
