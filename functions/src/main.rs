fn main() {
    // another_functionVar(4);
    outputNameAndAge("Salman", 21);
    let mut y = 30;
    let mut x = {
            y = y + 2;
            y + 1
        };
    println!("the value of x {x}");
}

fn another_function(){
    println!("hey");
}

fn another_functionVar(x:i32){
    println!("the value is {x}");
}

fn outputNameAndAge(name:&str, x:u32){
    let name: String = name.into();
    println!("Your name is {name} and age is {x}");
}