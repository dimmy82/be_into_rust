#![allow(warnings)]

fn main() {
    let gender = Gender::Male;
    match gender {
        Gender::Male => println!("a man"),
        Gender::Female => println!("a woman"),
    }
    match gender {
        Gender::Male => println!("a man"),
        _ => println!("not a man")
    }

    let ip = IpAddr::V4(192, 168, 0, 1);
    match ip {
        IpAddr::V4(i1, i2, i3, i4) => println!("v4: {}.{}.{}.{}", i1, i2, i3, i4),
        IpAddr::V6(addr) => println!("v6: {}", addr)
    }

    match generate() {
        Some(string) => println!("{}", string),
        None => println!("nothing")
    }

    let str1 = generate();
    let str2 = generate();

    match (&str1, &str2) {
        (Some(s1), Some(s2)) => println!("both str1({}), str2({}) are exist", s1, s2),
        (Some(s1), None) => println!("only str1({}) is exist", s1),
        (None, Some(s2)) => println!("only str2({}) is exist", s2),
        (None, None) => println!("both str1, str2 are nothing"),
    }

    match (&str1, &str2) {
        (Some(s1), Some(s2)) => println!("both str1({}), str2({}) are exist", s1, s2),
        (None, None) => println!("both str1, str2 are nothing"),
        _ => println!("what ever, I don't care")
    }

    let result = if generate().is_some() { "has data" } else { "has no data" };

    if let Some(string) = generate() {
        println!("has data: {}", string);
    } else {
        println!("has no data");
    }

//    panic!("oh my god");

    println!("this will never be printed");

    match parse("123") {
        Ok(int) => println!("i32 parse result: {}", int),
        Err(ErrorWrapper::ParseI32Failed(error)) => println!("{}", error)
    }

    match parse_and_double("123") {
        Ok(int) => println!("i32 parse and doubled result: {}", int),
        Err(ErrorWrapper::ParseI32Failed(error)) => println!("{}", error)
    }

    let a = A { a: "aaa", b: 0 };
    match a {
        A { a: "aaa", b: 0 } => println!("{}", ""),
        A { a: _, b: 0 } => println!("{}", ""),
        A { a: aa, b: _ } => println!("{}", aa),
    }

    IpAddr::V4(0, 0, 0, 0).a();
}

fn generate() -> Option<String> {
    Option::Some("test".into())
////    Option::None
//    unimplemented!()
}

fn parse(string: &str) -> std::result::Result<i32, ErrorWrapper> {
    match string.parse::<i32>() {
        Ok(int) => Ok(int),
        Err(error) => Err(ErrorWrapper::ParseI32Failed(
            format!("i32 ({}) parse error: {}", string, error)))
    }
}

type Res<T> = std::result::Result<T, ErrorWrapper>;

fn parse_and_double(string: &str) -> Res<i32> {
    let int = parse(string)?;
    Ok(int * 2)
}

fn a() {}

enum Gender {
    Male,
    Female,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn a(&self) {
        match self {
            IpAddr::V4(i1, i2, i3, i4) => {}
            IpAddr::V6(string) => {}
        }
    }
}

enum ErrorWrapper {
    ParseI32Failed(String)
}

impl ErrorWrapper {
    fn a(&self) {}
}

struct A<'a> {
    a: &'a str,
    b: i32,
}