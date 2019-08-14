#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_better() -> Result<(), String> {
        /* running 2 tests
           test tests::it_works_better ... ok
           test tests::it_works ... ok

           test result: ok. 2 passed; 0 failed; 
           0 ignored; 0 measured; 0 filtered out*/
        if 2 + 2 == 4 {
            Ok(()) 
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }
}
