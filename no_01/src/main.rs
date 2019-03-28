#![allow(warnings)]

fn main() {
    ownership_moved();
    copy_trait();
    clone_trait();
    borrow_reference();
    slice();
    object_method();
}

fn ownership_moved() {
//    let s1: String = String::from("hello");
//    {
//        let s2: String = s1;
//    }
//    println!("{}, world!", s1);

    let data2 = Data2 { value1: "value1".to_string(), value2: "value2".to_string() };
    let data2_value1 = data2.value1;
//    println!("{:?}", data2.value1);
    println!("{:?}", data2.value2);
//    println!("{:?}", data2);

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
    println!("{}, {}", t1.0, t1.1);

    let d1: Data3 = Data3 { value1: 0, value2: 'a' };
    {
        let d2: Data3 = d1;
    }
    println!("{}, {}", d1.value1, d1.value2);
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

fn borrow_reference() {
    let s1: String = String::from("hello");
    let s: &String = &s1;
    let s: &String = &&&s1;

    let s1: String = String::from("hello");
    let length = does_not_takes_ownership(&s1);
    println!("{} : size({})", s1, length);

//        let s1: String = String::from("hello");
//        let s: &mut String = &mut s1;

    let mut s1: String = String::from("hello");
    println!("{}", s1);
    let s: &mut String = &mut s1;
    s.push_str(", mut ref");
    println!("{}", s1);

//    let mut s1: String = String::from("hello");
//    let s2: &String = &s1;
//    let s3: &mut String = &mut s1;
//    println!("{}", s2);

//    let mut s1: String = String::from("hello");
//    let s2: &mut String = &mut s1;
//    let s3: &mut String = &mut s1;
//    s2.push_str(", error occur here");
    // ↑上と同じ理由ですが、mut所有権から発生したコンパイルエラー
//    let mut s1: String = String::from("original");
//    let s2: &mut String = &mut s1;
//    s1.push_str(", pushed by owner");
//    s2.push_str(", pushed by mut ref");

//    let s;
//    {
//        let s1: String = String::from("hello");
//        s = &s1;
//    }
//    println!("{}", s);
}

fn does_not_takes_ownership(s: &String) -> usize {
    s.len()
}

fn slice() {
    let s: &str = "hello";
    let s1: String = String::from("hello");
    let ss1: &str = s1.as_str();
    println!("&str: {:?}", ss1);
    let ss1: &str = &s1[..];
    println!("&str: {:?}", ss1);
    let ss1: &str = &s1[2..4];
    println!("&str: {:?}", ss1);
    let ss1: &str = &s1[2..=4];
    println!("&str: {:?}", ss1);

    // 実行エラー
//    let s2: String = String::from("こんにちは");
//    let ss2 = &s2[2..4];
//    println!("&str: {}", ss2);

    takes_ref_str(&s1);
    takes_ref_str(s1.as_str());
    takes_ref_string(&s1);
//    takes_ref_string(s1.as_str());

    let s_i32: &[i32; 3] = &[1, 2, 3];
}

fn takes_ref_str(s: &str) {}

fn takes_ref_string(s: &String) {}

fn object_method() {
    let o1 = Object { data: "hello".to_string() };
    o1.takes_self();
//    println!("{:?}", o1);

    let o2 = Object { data: "hello".to_string() };
    o2.takes_ref_self();
    println!("{:?}", o2);

    let mut o3 = Object { data: "hello".to_string() };
    o3.takes_ref_mut_self();
    println!("{:?}", o3);
}

#[derive(Clone)]
struct Data {
    pub value: String
}

#[derive(Debug)]
struct Data2 {
    pub value1: String,
    pub value2: String,
}

#[derive(Copy, Clone)]
struct Data3 {
    pub value1: i32,
    pub value2: char,
}

#[derive(Debug)]
struct Object {
    pub data: String,
}

impl Object {
    fn takes_self(self) {}

    fn takes_ref_self(&self) {}

    fn takes_ref_mut_self(&mut self) {
        self.data.push_str(", &mut self")
    }
}