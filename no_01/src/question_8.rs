#![allow(warnings)]

///////////////////////////////////////////////////////////////////////
// println!文は修正せず、コンパイルエラーを直し、以下の文言を標準出力してください。
// vec0の中身は`[]`です。
// vec1の中身は`[0, 1, 2, 3]`です。

pub fn main() {
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
