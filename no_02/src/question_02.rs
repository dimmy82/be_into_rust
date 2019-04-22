//struct Data {
//    pub value: String
//}
//
//impl Data {
//    fn append_to_value(new_value: &str) {
//        self.value.push_str(new_value)
//    }
//}
//
//fn question_02(data: Data) {
//    data.append_to_value(", hello")
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_question_02() {
//        let data = Data { value: String::from("rust") };
//        question_02(data);
//        assert_eq!(data.value, "rust, hello");
//        assert_eq!(data, Data { value: String::from("rust, hello") })
//    }
//}