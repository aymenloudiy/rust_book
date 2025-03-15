unsafe extern "C" {
    fn abs(input: i32) -> i32;
}
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
unsafe trait _Foo {
    // methods go here
}

unsafe impl _Foo for i32 {
    // method implementations go here
}
fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe { println!("Absolute value of -3 according to X : {}", abs(-3)) }
    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    println!("name is: {HELLO_WORLD}");
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
