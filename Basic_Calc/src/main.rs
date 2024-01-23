use std::io;

fn main() {
    loop{
        println!("Choose a number");
        println!("1. Sum\n 2. Difference\n 3. Product\n 4. Division\n");
        let mut entry= String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read input");
    
        let entry:u32 = match entry.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
        };
    }

    if entry= 1{
        sum;
    }
    else if entry= 2{
        difference;
    }
    else if entry=3{
        product;
    }
    else if entry=4{
        ChiefQuotient;
    }
    else if entry=5{
        Remainder;
    }
    else {
        println!("You have not entered a relevant choice");
    }
}

fn sum(w: u32,b: u32){
    println!("sum ={},w+b");
}
fn difference(w: u32,b: u32){
    println!("Difference= {},w-b");
}
fn product(w: u32,b: u32){
    println!("Product= {},w*b");
}
fn ChiefQuotient(w: u32,b: u32){
    println!("Quotient= {},w/b"); 
}
fn Remainder(w: u32,b: u32){
    println!("Remainder= {},w%b");
}