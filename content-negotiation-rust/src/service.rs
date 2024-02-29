use crate::models::{Value, Values};

pub(crate) struct SampleService {}

impl SampleService {
    pub fn get_data() -> Values {
        Values {
            values: vec![
                Value {
                    message: String::from("Foo"),
                    is_foo: true,
                },
                Value {
                    message: String::from("Bar"),
                    is_foo: false,
                },
            ],
        }
    }

    pub fn get_single() -> Value {
        Value {
            message: String::from("Baz"),
            is_foo: true,
        }
    }
}
