mod common;

#[cfg(test)]
mod tests {
    use crate::common;
    use std::env;
    
    #[test]
    fn test_assignment() {
        let (log, errors) 
            = common::get_program_output("tests/test_sources/test_assignment.txt");
        
        let expected_output 
            = common::read_file("tests/test_output/test_assignment.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_operators_int(){
        let (log, errors) 
            = common::get_program_output("tests/test_sources/test_operators_int.txt");
    
        let expected_output 
            = common::read_file("tests/test_output/test_operators_int.output");

        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_operators_float(){
        let (log, errors) 
            = common::get_program_output("tests/test_sources/test_operators_float.txt");

        let expected_output 
            = common::read_file("tests/test_output/test_operators_float.output");

        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);

    }

    #[test]
    fn test_operators_float_int(){
        let (log, errors) 
            = common::get_program_output("tests/test_sources/test_operators_float_int.txt");

        let expected_output 
            = common::read_file("tests/test_output/test_operators_float_int.output");

        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);

        let (log, errors) 
            = common::get_program_output("tests/test_sources/test_operators_float_int_2.txt");
        
        let expected_output 
            = common::read_file("tests/test_output/test_operators_float_int_2.output");

        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_if_else(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_if_else.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_if_else.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_for(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_for.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_for.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_while(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_while.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_while.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_pack_spread(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_pack_spread.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_pack_spread.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }
    #[test]
    fn test_user_function(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_user_function.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_user_function.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_import(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_import.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_import.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_import_env(){
        env::set_var("RUSTL_LIB", "/home/nakul/projects/rustlang/tests/common");

        let(log, errors)
            = common::get_program_output("tests/test_sources/test_import_env.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_import_env.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_import_absolute(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_import_absolute.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_import_absolute.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_list_indexing(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_list_indexing.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_list_indexing.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

    #[test]
    fn test_string_indexing(){
        let(log, errors)
            = common::get_program_output("tests/test_sources/test_string_indexing.txt");
        
        let expected_output
            = common::read_file("tests/test_output/test_string_indexing.output");
        
        assert_eq!(expected_output, log, "\nErrors:\n{}\n", errors);
    }

}