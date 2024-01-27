fn main(){
    let rectangle1=Rectangle{
        length: 20,
        width: 12,
    };
    let rectangle2= Rectangle{
        length: 16,
        width: 4,
    };
    let rectangle3= Rectangle{
        length: 18,
        width: 14,
    };
    println!("The rectangle has the measurements: {:#?}",rectangle1);
    println!("The area is {}", rectangle1.area_using_struct());
    println!("Can Rectangle 1 hold rectangle 2? {}", rectangle1.can_hold(&rectangle2));
    println!("Can Rectangle 1 hold rectangle 3? {}", rectangle1.can_hold(&rectangle3));
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
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.length>other.length && self.width>other.width
    }
}