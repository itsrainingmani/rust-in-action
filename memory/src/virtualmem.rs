use std::mem::drop;

pub fn main() {
    // let a: i32 = 40;
    // let b: Box<i32> = Box::new(60);
    // println!("a: {}, b: {}, a + b = {}", a, *b, a + *b);

    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a); // deletes objects before their scope ends

    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
}
