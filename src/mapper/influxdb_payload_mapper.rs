use dyn_clone::DynClone;

pub trait InfluxDbPayloadMapper<T>: DynClone + Send {
    fn item(&self, payload: T) -> String;

    fn items(&self, payloads: Vec<T>) -> String {
        let mut vec = vec![];
        for payload in payloads {
            let body = self.item(payload);
            vec.push(body);
        }
        vec.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestStruct {
        field_1: String,
    }

    #[derive(Clone)]
    struct TestStructMapper {
        mapper_field_1: String
    }

    impl InfluxDbPayloadMapper<TestStruct> for TestStructMapper {
        fn item(&self, payload: TestStruct) -> String {
            return format!("{}:{}", self.mapper_field_1, payload.field_1)
        }
    }

    #[test]
    fn maps_correctly() {
        let mapper = TestStructMapper { mapper_field_1: "mapper_field_1".to_string() };
        let result = mapper.items(vec![
            TestStruct {
                field_1: "field_1_1".to_string()
            },
            TestStruct {
                field_1: "field_1_2".to_string()
            }
        ]);
        assert_eq!("mapper_field_1:field_1_1\nmapper_field_1:field_1_2", result);
    }
}