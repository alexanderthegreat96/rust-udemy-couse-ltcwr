#![allow(dead_code)]

enum Optional {
    Some(i32),
    None,
}

impl Optional {
    fn unwrap(self) -> i32 {
        match self {
            Optional::Some(value) => value,
            Optional::None => {
                panic!("Nope, no value found")
            }
        }
    }

    fn unwrap_or(self, default_val: i32) -> i32 {
        match self {
            Optional::Some(value) => value,
            Optional::None => default_val,
        }
    }
}

fn main() {
    let unwraped_number: i32 = Optional::None.unwrap_or(100);
    println!("{}", unwraped_number);
    let opt_number: Optional = Optional::None;
    opt_number.unwrap();
}
