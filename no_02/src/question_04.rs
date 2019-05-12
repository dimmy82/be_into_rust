struct Data1 {
    value: String
}

struct Data2 {
    value: String
}

fn convert_own(data1: Data1) -> Data2 {
    unimplemented!()
}

fn convert_ref(data1: &Data1) -> Data2 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::question_04::{convert_own, Data1, Data2, convert_ref};

    #[test]
    fn test_convert_own() {
        assert_eq!(convert_own(Data1 { value: String::from("value") }), Data2 { value: String::from("value") })
    }

    #[test]
    fn test_convert_ref() {
        assert_eq!(convert_ref(&Data1 { value: String::from("value") }), Data2 { value: String::from("value") })
    }
}