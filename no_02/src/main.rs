#![allow(warnings)]

mod question_01;
mod question_02;
mod question_03;

fn main() {
    let s1: String = String::from("長〜〜〜いやつ");
    let s2: String = String::from("短いやつ");

    println!("{}", longer(s1.as_str(), s2.as_str()));
//    println!("{}", longer2(s1, s2));

    let data = create_data(s1.as_str(), s2.as_str());
    println!("{:?}", data);
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn always_left<'a>(left: &'a str, right: &str) -> &'a str {
    left
}

//fn longer2(s1: String, s2: String) -> &str {
//    if s1.len() > s2.len() {
//        s1.as_str()
//    } else {
//        s2.as_str()
//    }
//}

fn create_data<'a, 'b>(s1: &'a str, s2: &'b str) -> Data<'a, 'b> {
    Data { value1: s1, value2: s2 }
}

#[derive(Debug)]
struct Data<'a, 'b> {
    value1: &'a str,
    value2: &'b str,
}

impl<'a, 'b> Data<'a, 'b> {
    fn xxx(&self, s3: &str) -> &str {
        self.value1
    }
}