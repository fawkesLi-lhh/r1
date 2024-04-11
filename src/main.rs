trait T {
    const TT: i32;
}
struct A {}
impl T for A {
    const TT: i32 = 1;
}
struct B {}
impl T for B {
    const TT: i32 = 2;
}
enum C {
    A(A),
    B(B),
}

fn tt(c: C) -> Box<dyn T> {
    match c {
        C::A(a) => Box::new(a),
        C::B(b) => Box::new(b),
    }
}

fn main() {
    let c = C::A(A {});
    let t = tt(c);
}
