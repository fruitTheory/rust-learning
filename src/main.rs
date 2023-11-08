
// Comment
fn main(){

    // print!("Hello\n");
    // println!("Hello {}", 42);
    // println!("Hello {}", "World");
    // println!("Hello {} {}", "World", 60);
    // println!("Hello {1} {0}", "World", 60);

    println!("Hello {noun} {number}", noun = "World", number = 60);
    println!("{:b} {:X} {}", 1, 10, 10); // binary , hex, decimal

    println!("{num:0>4}", num = 3);
    println!("{num:0>fill$}", num = 3, fill = 4);

    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    let numb: u8 = 255;
    let numb: u8 = 0xFF;

    println!("{numb}");
    println!("{:X} {0}", numb);

}