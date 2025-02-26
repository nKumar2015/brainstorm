mod common;

#[cfg(test)]
mod tests {
    use crate::common;

    #[test]
    fn test_assignment() {
        let (_log, _errors) 
            = common::get_program_output("tests/test_sources/test_assignment.txt");
        
        let expected_output 
            = common::read_file("tests/test_output/test_assignment.output");
        
        assert_eq!(expected_output, _log);
    }

    #[test]
    fn test_operators_int(){
        let (_log, _errors) 
            = common::get_program_output("tests/test_sources/test_operators_int.txt");
    
        let expected_output 
            = common::read_file("tests/test_output/test_operators_int.output");

        assert_eq!(expected_output, _log);
    }

    #[test]
    fn test_operators_float(){
        let (_log, _errors) 
            = common::get_program_output("tests/test_sources/test_operators_float.txt");

        let expected_output 
            = common::read_file("tests/test_output/test_operators_float.output");

        assert_eq!(expected_output, _log);

    }

    #[test]
    fn test_operators_float_int(){
        let (_log, _errors) 
            = common::get_program_output("tests/test_sources/test_operators_float_int.txt");

        let expected_output 
            = common::read_file("tests/test_output/test_operators_float_int.output");

        assert_eq!(expected_output, _log);

        let (_log, _errors) 
            = common::get_program_output("tests/test_sources/test_operators_float_int_2.txt");
        
        let expected_output 
            = common::read_file("tests/test_output/test_operators_float_int_2.output");

        assert_eq!(expected_output, _log);
    }




}