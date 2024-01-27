fn main(){
    let rectangle1=Rectangle{
        length: 20,
        width: 12,
    };
    println!("The rectangle has the measurements: {:#?}",rectangle1);
    println!("The area is {}", area_using_struct(&rectangle1));
}

// This was the innitial area function before introducing a struct
//fn area(length:u32, width: u32) -> u32 {    
//    length*width
//}

#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32,
}
impl Rectangle{
    fn area_using_struct(&self) -> u32 {
        self.length*self.width
    }
}