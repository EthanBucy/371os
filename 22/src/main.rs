use malloc::*;

fn main() {
    let p0 = malloc(16).unwrap();
    let p1 = malloc(32).unwrap();
    let x = 0x44332211;
    let y = 0x12345678;
    setter(x, p0);
    setter(y, p1);
    let z: i32 = getter(p0);
    let w: i32 = getter(p1);
    assert!(x == z);
    assert!(y == w);
    println!("A+");

    assert!(malloc(2048).is_none());
    println!("A++");

    malloc(32).unwrap();
    assert!(malloc(64).is_none());
    println!("A+++");
}
