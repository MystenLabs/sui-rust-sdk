pub mod proto;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    enum Foo {
        A = 2,
        #[serde(skip_deserializing)]
        B = 3,
        C,
    }

    #[test]
    fn foo() {
        let f = Foo::A;

        let bytes = bcs::to_bytes(&f).unwrap();

        println!("{:?}", bytes);

        println!("{:?}", bcs::from_bytes::<Foo>(&[1]).unwrap());
    }
}
