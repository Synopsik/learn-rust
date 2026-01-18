

pub fn chapter_work() {
    println!("Hello World!");

    let built_vec = build_vector();

    for i in built_vec {
        println!("{}", i)
    }

    test_assertions();
    let b = pointer_allocation();
    println!("{}", b.0);
    println!("{}", b.1);
}

fn test_assertions() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original module 2^N,
    // where N is the width of the destination in bits. This
    // is sometimes called "truncation."
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
}

fn pointer_allocation() -> Box<(i32, &'static str)> {
    let t = (12, "eggs");
    let b = Box::new(t);
    b
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}