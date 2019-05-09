#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_with_empty_arguments_is_invalid() {
        let args: Vec<String> = vec!();

        let result = Config::new(&args);

        assert!(result.is_err());
    }

    #[test]
    fn config_with_one_argument_is_invalid() {
        let args: Vec<String> = vec!("program_path.exe".to_owned(), "first_argument".to_owned());

        let result = Config::new(&args);

        assert!(result.is_err());
    }

    
}