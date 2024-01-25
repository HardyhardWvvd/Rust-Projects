fn main() {
    let mut j=0;
    let mut k=1;
    let mut x;
    let count: [i32;9]=[1,2,3,4,5,6,7,8,9];
   
    for _i in count{
        x=j+k;
        j=k;
        k=x;
        println!("{}", j);
    }
}