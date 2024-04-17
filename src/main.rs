use std::{borrow::Borrow, io::Read};

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
            let one = st::sta::A::aa();
            a + one
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
    let sled = sled::open("ph").unwrap();
    let vv = sled.scan_prefix("fn");
    for v in vv {
        let (k,v) = v.unwrap();
        let key = String::from_utf8(k.to_vec()).unwrap();
        let value = String::from_utf8(v.to_vec()).unwrap();
        println!("key: {},\n value: {}\n\n", key, value);
    }
}
