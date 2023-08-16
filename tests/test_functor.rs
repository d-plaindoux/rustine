#[cfg(test)]
mod tests_map {
    use rustine::specs::functor::Functor;
    use rustine::standard::option::OptionK;
    use rustine::standard::result::ResultK;

    fn test_map<This: Functor>(ma: This::T<i32>) -> This::T<i32> {
        This::map(|a| a + 1, ma)
    }

    #[test]
    fn map_option_some() {
        type This = OptionK;
        assert_eq!(test_map::<This>(Some(1)), Some(2))
    }

    #[test]
    fn map_option_none() {
        type This = OptionK;
        assert_eq!(test_map::<This>(None), None)
    }

    #[test]
    fn map_result_ok() {
        type This = ResultK<&'static str>;
        assert_eq!(test_map::<This>(Ok(1)), Ok(2))
    }

    #[test]
    fn map_result_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_map::<This>(Err("Error")), Err("Error"))
    }
}