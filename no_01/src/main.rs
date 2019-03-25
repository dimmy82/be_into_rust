fn main() {
    ownership_moved();
    copy_trait();
    clone_trait();
}

fn ownership_moved() {
//    let s1: String = String::from("hello");
//    {
//        let s2: String = s1;
//    }
//    println!("{}, world!", s1);

//    let s1: String = String::from("hello");
//    takes_ownership(s1);
//    println!("{}, world!", s1);

    let s1: String = String::from("Oh my god ...");
    let (s1, length) = takes_ownership_and_give_back(s1);
    println!("{} : size({})", s1, length);
}

fn takes_ownership(s: String) {}

fn takes_ownership_and_give_back(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn copy_trait() {
    let i1: i32 = 100;
    {
        let i2: i32 = i1;
    }
    println!("number: {}", i1);

    let t1: (i32, bool) = (1, true);
    {
        let t2: (i32, bool) = t1;
    }
    println!("{}, {}", t1.0, t1.1)
}

fn clone_trait() {
    let s1: String = String::from("hello");
    {
        let s2: String = s1.clone();
    }
    println!("{}, world!", s1);

    let d1: Data = Data { value: String::from("hello, data") };
    {
        let d2: Data = d1.clone();
    }
    println!("{}", d1.value);
}

#[derive(Clone)]
struct Data {
    pub value: String
}