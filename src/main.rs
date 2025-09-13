use std::process;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

fn main(){
	
	print!("1 for Fahrenheit to kelvin \n2 for Kelvin to Fahrenheit \n3 for Celsius to Kelvin \n4 for Kelvin to Celsius \n5 for Fahrenheit to Celsius \n6 for Celsius to to Fahrenheit. \n\nPlease enter 1, 2,3....6 to select your choice: ");
	io::stdout().flush().unwrap();
	let mode = mode_condition();
	println!("\n ꧁you picked mode {}", mode);
	
	print!("\n꧁Enter your temperature value: ");
	io::stdout().flush().unwrap();
	
	
	 let mut user_input:String = String::new();
	 io::stdin().read_line(&mut user_input).expect("");
	 let user_input: f64 = user_input.trim().parse().expect("");
	 
	 conv_pick(mode, user_input);
	 
	 
	 
	 
	 
}

fn mode_condition() -> u8 {
	let mut x = 3;
	while x > 0{
		let mut mode:String = String::new();
		io::stdin().read_line(&mut mode).expect("");
		let mode: u8 = mode.trim().parse().expect("");
		
		if mode != 1 && mode != 2 && mode != 3 && mode != 4 && mode != 5 && mode != 6{
		x -= 1;
		if x ==0{break}
		print!("\nWrong input, your have {x} attempts left: ");
		io::stdout().flush().unwrap();
		
		}
		else{return mode};
	}
	println!("\nYOU ENTERED TOO MANY WRONG ATTEMPTS, EXITING IN 3SECS");
	sleep(Duration::from_secs(3));
	process::exit(1);
	0
}

fn conv_pick(mode:u8, user_input:f64){
	if mode == 1{fah_to_kev(user_input)}
	else if mode == 2 {kev_to_fah(user_input)}
	else if mode == 3 {cel_to_kev(user_input)}
	else if mode == 4 {kev_to_cel(user_input)}
	else if mode == 5 {fah_to_cel(user_input)}
	else if mode == 6 {fah_to_cel(user_input)}
}

fn fah_to_kev(user_input:f64){
	let result = (user_input - 32.0)*5.0/9.0+273.15;
	println!("{user_input}⁰F to kelvin is {result}⁰K");
}

fn kev_to_fah(user_input:f64){
	let result = (user_input - 273.15) * 9.0/5.0 + 32.0;
	println!("{user_input}⁰K to Fahrenheit is {result}⁰F");
}

fn cel_to_kev(user_input:f64){
	let result = user_input + 273.15;
	println!("{user_input}⁰C to Kelvin is {result}⁰K");
}

fn kev_to_cel(user_input:f64){
	let result = user_input - 273.15;
	println!("{user_input}⁰K to Celsius is {result}⁰C");
}

fn fah_to_cel(user_input:f64){
	let result = (user_input - 32.0) * 5.0/9.0;
	println!("{user_input}⁰F to Celsius is {result}⁰C");
}

fn cel_to_fah(user_input:f64){
	let result = (user_input * 9.0/5.0) + 32.0;
	println!("{user_input}⁰C to Fahrenheit is {result}⁰F");
}