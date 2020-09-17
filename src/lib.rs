#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::{CStr, CString},
        ptr::null_mut,
    };

    #[test]
    fn sanity_check() {
        unsafe {
            assert_eq!(
                CString::new("0.6.0").unwrap().as_c_str(),
                CStr::from_ptr(libcypher_parser_version())
            );
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

            let root = cypher_parse_result_get_root(result, 0);
            assert_eq!(1, cypher_astnode_nchildren(root));

            let tipe = cypher_astnode_type(root);
            assert_eq!(tipe, CYPHER_AST_STATEMENT);

            let type_string = CStr::from_ptr(cypher_astnode_typestr(tipe));
            assert_eq!(CString::new("statement").unwrap().as_c_str(), type_string,);

            cypher_parse_result_free(result);
        }
    }
}
