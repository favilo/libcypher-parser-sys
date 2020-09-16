#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::CString, ptr::null_mut};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sanity_check() {
        unsafe {
            let stmt = "MATCH (n) RETURN n";
            let result = cypher_uparse(
                CString::new(stmt).unwrap().as_ptr(),
                stmt.len() as u64,
                null_mut(),
                null_mut(),
                CYPHER_PARSE_ONLY_STATEMENTS.into(),
            );
            assert_ne!(result, null_mut());
            assert_eq!(10, cypher_parse_result_nnodes(result));
            assert_eq!(1, cypher_parse_result_ndirectives(result));
            assert_eq!(1, cypher_parse_result_nroots(result));
            assert_eq!(0, cypher_parse_result_nerrors(result));
            cypher_parse_result_free(result);
        }
    }
}
