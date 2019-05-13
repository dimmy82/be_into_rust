struct Data1 {
    value: String
}

struct Data2 {
    value: String
}

fn convert_own_vec_own_element(datas: Vec<Data1>) -> Vec<Data2> {
    unimplemented!()
}

fn convert_own_vec_ref_element(datas: Vec<&Data1>) -> Vec<Data2> {
    unimplemented!()
}

fn convert_ref_vec_own_element(datas: &Vec<Data1>) -> Vec<Data2> {
    unimplemented!()
}

fn convert_ref_vec_ref_element(datas: &Vec<&Data1>) -> Vec<Data2> {
    unimplemented!()
}

