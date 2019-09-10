#[derive(Debug)]
struct Object1 {
    pub a: String,
    pub b: String,
}

#[derive(Debug)]
struct Object2 {
    a: String,
}

impl Object2 {
    fn new(a: String) -> Self {
        Object2 { a }
    }
}

pub fn q_1_7_main() {
    q1()
}

///////////////////////////////////////////////////////////////////////

fn q1() {
    let s1 = String::from("q1s1");
    //    let s2 = s1; // この行を修正してコンパイルできるようにする
    //    println!("{}", s1);
    //    println!("{}", s2);
}

///////////////////////////////////////////////////////////////////////

fn q2() {
    let s1 = String::from("q1s1");
    //  s1 を受け取る q1_func1 関数を作ってここで呼び出す
    println!("{}", s1);
}

///////////////////////////////////////////////////////////////////////

fn q3() {
    let o = Object1 {
        a: String::from("aaa"),
        b: String::from("bbb"),
    };
    // q3_func1, q3_func2 関数を使って o.a, o.b を出力する
}

fn q3_func1(o: Object1) {
    println!("{}", o.a);
}

fn q3_func2(o: Object1) {
    println!("{}", o.b);
}

///////////////////////////////////////////////////////////////////////

fn q4() {
    let o = Object1 {
        a: String::from("aaa"),
        b: String::from("bbb"),
    };
    // q4_func1 関数を使って o.a, o.b を出力する
}

fn q4_func1(s: String) {
    println!("{}", s);
}

///////////////////////////////////////////////////////////////////////

fn q5() {
    let o = Object2::new(String::from("aaa"));
    //  Object2 に q5_func1 というメソッドを作って Object2.a を返すようにして、ここで println をする
    println!("{:?}", o)
}

impl Object2 {
    // q5_func1 のメソッドはここに実装する
}

///////////////////////////////////////////////////////////////////////

fn q6() {
    // Object2::new でインスタンスを作成する
    // Object2 に q6_func1 というメソッドを作って Object2.a を変更する
    // （&str の引数を受け取って、 Object.a の後ろにくっ付ける）
    // Object2.q5_func1 を呼び出して、ここで println をする
}

impl Object2 {
    // q6_func1 のメソッドはここに実装する
}

///////////////////////////////////////////////////////////////////////

fn q7() {
    // Object2::new でインスタンスを作成する
    // Object2 に q7_func1 というメソッドを作って、新しい Object2 インスタンスを返すし、Object2.a の値を移譲する（clone()は使わない）
}

impl Object2 {
    // q7_func1 のメソッドはここに実装する
}
