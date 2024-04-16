pub mod st;

fn fa(asdasd: i32, b: B) -> i32 {
    let one = st::sta::A::aa();
    let two = b.bb();
    st::sta::cc();
    dd();
    asdasd + one + two
}

struct B {}
impl B {
    fn bb(&self) -> i32 {
        let r = || {
            let a = 1;
            a
        };
        r()
    }
}
pub struct A {}
impl A {
    pub fn aa() -> i32 {
        1
    }
}

fn aa() -> i32 {
    1
}

fn dd() {}

fn main() {
    let c = fa(2, B {});
    println!("{}", c);
    let d = aa();
    println!("{}", d);
}
