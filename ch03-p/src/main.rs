use std::io;

fn main() {
    // take in input
    // control structure
    // convert
    println!("Would you to convert from C or F?");
    let mut tempType = String::new();
    io::stdin().read_line(&mut tempType).expect("error");
    println!("temptype is {tempType}");
    let tempType = tempType.trim();

    println!("Provide the value: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Error 2");
    let mut temp: f64 =  temp.trim().parse().expect("T"); // convert from string to float
    println!("The value is {temp}");


    if tempType == "C"{
        println!("You will convert from C to F");
        conv(tempType, temp);
    }else if tempType == "F"{
        println!("You will convert from F to C");
        conv(tempType, temp);
    }else{
        println!("Incorrect input");
    }


    println!("Generate nth fibonacci number!");
    println!("Enter your nth number NOWWW!");
    let mut fibN = String::new();
    io::stdin().read_line(&mut fibN).expect("Error with reading fibN");
    let mut fibN: u32 = fibN.trim().parse().expect("Error converting to integer");
    let fibNum: u32 = fibonacci(fibN);
    println!("Our fibonacci number is: {fibNum}");

}

fn fibonacci(fibN: u32) -> u32{
    if fibN == 1{
        return 1;
    }
    if fibN == 0{
        return 0;
    }
    return fibonacci(fibN-1) + fibonacci(fibN-2);
}

fn conv(tempType: &str, temp:f64 ){
    let tempType:String = tempType.into(); // read into string 
    if tempType == "C"{
        let fahrenheit: f64 = (temp * 9.0/5.0) + 32.0; // explicit typing
        println!("F = {fahrenheit}F");
    }else if tempType == "F"{
        let celsius: f64 = (temp - 32.0) * (5.0/9.0);
        println!("C = {celsius}C");
    }
    
}
