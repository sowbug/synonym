use synonym::Synonym;
use serde::{Serialize, Deserialize};

macro_rules! check {
    ($t:ty, $v:expr, $json:expr) => {
        {
            #[derive(Synonym)]
            struct Foo($t);

            let foo = Foo($v);
            let json = serde_json::to_string(&foo).unwrap();
            let deserialized = serde_json::from_str(&json).unwrap();

            assert_eq!(foo, deserialized);
            assert_eq!(json, $json);
        }
    }
}


fn main() {
    check!(u8, 1u8, "1");
    check!(u16, 2u16, "2");
    check!(u32, 3u32, "3");
    check!(u64, 4u64, "4");
    check!(u128, 5u128, "5");
    check!(usize, 6usize, "6");
    check!(usize, 7usize, "7");
    check!(String, "Foo".to_string(), r#""Foo""#);
    check!(char, 'X', r#""X""#);

    #[derive(Synonym)]
    struct Baz(u32);

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Foo {
        bar: Baz
    }

    let foo = Foo { bar: Baz(42) };
    let json = serde_json::to_string(&foo).unwrap();
    let deserialized = serde_json::from_str(&json).unwrap();

    assert_eq!(foo, deserialized);
    assert_eq!(json, r#"{"bar":42}"#);
}
