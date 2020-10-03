fn main() {
    let mut num = 5;
    let wrong_address = 0x12345usize;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = wrong_address as *const i64;

    println!("{:#?}", r1);
    println!("{:#?}", r2);
    println!("{:#?}", r3);


    unsafe {
        println!("{:#?}", *r1);
        println!("{:#?}", *r2);
        println!("{:#?}", *r3);
    }
}
