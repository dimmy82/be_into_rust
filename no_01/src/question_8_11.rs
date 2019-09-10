///////////////////////////////////////////////////////////////////////
// println!文は修正せず、コンパイルエラーを直し、以下の文言を標準出力してください。
// vec0の中身は`[]`です。
// vec1の中身は`[0, 1, 2, 3]`です。

pub fn q_8_main() {
    //    let vec0 = Vec::new();
    //
    //    let mut vec1 = q8_fill_vec(vec0);
    //
    //    println!("{}の中身は`{:?}`です。", "vec0", vec0);
    //
    //    vec1.push(3);
    //
    //    println!("{}の中身は`{:?}`です。", "vec1", vec1);
}

fn q8_fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(0);
    vec.push(1);
    vec.push(2);

    vec
}

///////////////////////////////////////////////////////////////////////
// println!文は修正せず、コンパイルエラーを直し、以下の文言を標準出力してください。
// vec0の中身は`[0, 1, 2]`です。
// vec1の中身は`[0, 1, 2, 3]`です。

pub fn q_9_main() {
    //    let vec0 = Vec::new();
    //
    //    let mut vec1 = q9_fill_vec(vec0);
    //
    //    println!("{}の中身は`{:?}`です。", "vec0", vec0);
    //
    //    vec1.push(3);
    //
    //    println!("{}の中身は`{:?}`です。", "vec1", vec1);
}

fn q9_fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(0);
    vec.push(1);
    vec.push(2);

    vec
}

///////////////////////////////////////////////////////////////////////
// println!文は修正せず、コンパイルエラーを直し、以下の文言を標準出力してください。
// vec0の中身は`[]`です。
// vec1の中身は`[0, 1, 2, 3]`です。

pub fn q_10_main() {
    //    let vec0 = Vec::new();
    //
    //    let mut vec1 = q10_fill_vec(vec0);
    //
    //    println!("{}の中身は`{:?}`です。", "vec0", vec0);
    //
    //    vec1.push(3);
    //
    //    println!("{}の中身は`{:?}`です。", "vec1", vec1);
}

fn q10_fill_vec(vec: Vec<i32>) -> Vec<i32> {
    //    vec.push(0);
    //    vec.push(1);
    //    vec.push(2);

    vec
}

///////////////////////////////////////////////////////////////////////
// & (0, &mut (1, 2)) から2を取り出して、add_oneの引数として渡してください。
// add_oneの戻り値である３を標準出力してください。

pub fn q_11_main() {
    let hoge = &mut (0, &(1, 2));
}

fn q11_add_one(i: i32) -> i32 {
    i + 1
}
