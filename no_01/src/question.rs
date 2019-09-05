fn q1() {
    let s1 = String::from("q1s1");
//    let s2 = s1; // この行を修正してコンパイルできるようにする
//    println!("{}", s1);
//    println!("{}", s2);
}

fn q2() {
    let s1 = String::from("q1s1");
//  s1 を受け取る q1_func1 関数を作ってここで呼び出す
    println!("{}", s1);
}

fn q3() {
    let o = Q3Object {
        a: String::from("aaa"),
        b: String::from("bbb"),
    };
    // q3_func1 関数を使って o.a, o.b を出力する
}

fn q3_func1(s: String) {
    println!("{}", s);
}

struct Q3Object {
    pub a: String,
    pub b: String,
}

