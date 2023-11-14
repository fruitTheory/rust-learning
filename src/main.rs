
// Comment
fn main(){

    println!("Hello {noun} {number}", noun = "World", number = 60);
    println!("{:b} {:X} {}", 1, 10, 10); // binary , hex, decimal

    println!("{num:0>4}", num = 3);
    println!("{num:0>fill$}", num = 3, fill = 4);

    let numb: u8 = 0xFF;
    println!("{:X} {0}", numb);

    let i = test(9);
    println!("printing i: {i}");



}

fn test(a: i32) -> i32{

    let a = a;
    return a;

}