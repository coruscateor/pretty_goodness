
pub mod json;

//pub use json::*;

mod errors_and_results;

pub use errors_and_results::*;

/* 
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
*/

#[cfg(test)]
mod tests {
    //use super::*;

    use crate::json::PrettyEr;

    /*
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    */

    //const AGOQL_SCHEMA_RESULT_UGLY: &str = "{\"data\":{\"__schema\":{\"types\":[{\"name\":\"BoolValue\"},{\"name\":\"Boolean\"},{\"name\":\"Char\"},{\"name\":\"CharValue\"},{\"name\":\"F32Value\"},{\"name\":\"F64Value\"},{\"name\":\"Float\"},{\"name\":\"I128Scalar\"},{\"name\":\"I128ScalarValue\"},{\"name\":\"I16Value\"},{\"name\":\"I32Value\"},{\"name\":\"I64Value\"},{\"name\":\"I8Value\"},{\"name\":\"ID\"},{\"name\":\"InputWhatever\"},{\"name\":\"Int\"},{\"name\":\"IsizeValue\"},{\"name\":\"MutationRoot\"},{\"name\":\"QueryRoot\"},{\"name\":\"String\"},{\"name\":\"StringValue\"},{\"name\":\"U128Scalar\"},{\"name\":\"U128ScalarValue\"},{\"name\":\"U16Value\"},{\"name\":\"U32Value\"},{\"name\":\"U64Value\"},{\"name\":\"U8Value\"},{\"name\":\"UsizeValue\"},{\"name\":\"Whatever\"},{\"name\":\"__Directive\"},{\"name\":\"__DirectiveLocation\"},{\"name\":\"__EnumValue\"},{\"name\":\"__Field\"},{\"name\":\"__InputValue\"},{\"name\":\"__Schema\"},{\"name\":\"__Type\"},{\"name\":\"__TypeKind\"}]}}}";

    //2 Field simple object:

    //Ugly:

    //const TWO_FIELD_OBJECT_UGLY: &str = "{\"Simple Object Field\": 1,\"String Field\":\"abc 123\"}";

    //Pretty:

    /*
    const TWO_FIELD_OBJECT_PRETTY: &str = "{
        \"Simple Object Field\": 1,
        \"String Field\": \"abc 123\"
    }";
    */

    //VS code uses four space characters is stead of the tab character to reperestnt tabs - so they have to be literally inculded.

    #[test]
    fn two_field_object()
    {

        let two_field_object_ugly = "{\"Simple Object Field\": 1,\"String Field\":\"abc 123\"}".to_string();

        let two_field_object_pretty = 
"{
\t\"Simple Object Field\": 1,
\t\"String Field\": \"abc 123\"
}".to_string();

        println!("{}\n", "two_field_object_ugly:");

        println!("{}\n", two_field_object_ugly);

        println!("{}\n", "two_field_object_pretty:");

        println!("{}\n", two_field_object_pretty);

        let mut prettyer = PrettyEr::new();

        let res = prettyer.prettyafy(&two_field_object_ugly);

        match res
        {

            Ok(val) => {

                println!("{}\n\n", "val:");

                println!("{}\n", val);
        
                assert_eq!(two_field_object_pretty, val);

            }
            Err(err) =>
            {

                println!("{}\n", "err:");

                println!("{}\n\n", err.to_string());

                panic!();

            }

        }

    }

    //2 element simple array:

    //Uglu:

    //const TWO_FIELD_ARRAY_UGLY: &str = "[1,\"abc 123\"]";

    //Pretty:

    /*
    const TWO_FIELD_ARRAY_PRETTY: &str = "[
        1,
        \"abc 123\"
    ]";
    */

    #[test]
    fn two_element_array()
    {

        let two_element_array_ugly = "[1,\"abc 123\"]".to_string();

        let two_element_array_pretty = 
"[
\t1,
\t\"abc 123\"
]".to_string();

        println!("{}\n\n", "two_element_array_ugly:");

        println!("{}\n", two_element_array_ugly);

        println!("{}\n\n", "two_element_array_pretty:");

        println!("{}\n", two_element_array_pretty);

        let mut prettyer = PrettyEr::new();

        let res = prettyer.prettyafy(&two_element_array_ugly); //.unwrap();

        //println!("{}\n\n", "res:");

        //println!("{}\n", res);

        //assert_eq!(two_element_array_pretty, res);

        match res
        {

            Ok(val) => {

                println!("{}\n", "val:");

                println!("{}\n", val);
        
                assert_eq!(two_element_array_pretty, val);

            }
            Err(err) =>
            {

                println!("{}\n", "err:");

                println!("{}\n", err.to_string());

                panic!();

            }

        }

    }

    #[test]
    fn agql_schema_query_result()
    {

        let agql_schema_query_result_ugly = "{\"data\":{\"__schema\":{\"types\":[{\"name\":\"BoolValue\"},{\"name\":\"Boolean\"},{\"name\":\"Char\"},{\"name\":\"CharValue\"},{\"name\":\"F32Value\"},{\"name\":\"F64Value\"},{\"name\":\"Float\"},{\"name\":\"I128Scalar\"},{\"name\":\"I128ScalarValue\"},{\"name\":\"I16Value\"},{\"name\":\"I32Value\"},{\"name\":\"I64Value\"},{\"name\":\"I8Value\"},{\"name\":\"ID\"},{\"name\":\"InputWhatever\"},{\"name\":\"Int\"},{\"name\":\"IsizeValue\"},{\"name\":\"MutationRoot\"},{\"name\":\"QueryRoot\"},{\"name\":\"String\"},{\"name\":\"StringValue\"},{\"name\":\"U128Scalar\"},{\"name\":\"U128ScalarValue\"},{\"name\":\"U16Value\"},{\"name\":\"U32Value\"},{\"name\":\"U64Value\"},{\"name\":\"U8Value\"},{\"name\":\"UsizeValue\"},{\"name\":\"Whatever\"},{\"name\":\"__Directive\"},{\"name\":\"__DirectiveLocation\"},{\"name\":\"__EnumValue\"},{\"name\":\"__Field\"},{\"name\":\"__InputValue\"},{\"name\":\"__Schema\"},{\"name\":\"__Type\"},{\"name\":\"__TypeKind\"}]}}}".to_string();

        let agql_schema_query_result_pretty = //FIX
"{
\t\"data\":
\t\t{
\t\t\t\"__schema\":
\t\t\t\t{
\t\t\t\t\t\"types\":
\t\t\t\t\t[
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"BoolValue\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"Boolean\"
\t\t\t\t\t\r},
\t\t\t\t\t\t\t{
\t\t\t\t\t\t\t\t{
\t\t\t\t\t\t\t\t\"name\":\"Char\"
\t\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"CharValue\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"F32Value\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"F64Value\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"Float\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"I128Scalar\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"I128ScalarValue\"},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"I16Value\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"I32Value\"},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"I64Value\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"I8Value\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"ID\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"InputWhatever\"
\t\t\t\t\t\t},
\t\t\t\t\t\t{
\t\t\t\t\t\t\t\"name\":\"Int\"
\t\t\t\t\t\t},
\t\t\t\t\t}
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"IsizeValue\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"MutationRoot\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"QueryRoot\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"String\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"StringValue\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"U128Scalar\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"U128ScalarValue\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"U16Value\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"U32Value\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"U64Value\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"U8Value\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"UsizeValue\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"Whatever\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__Directive\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__DirectiveLocation\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__EnumValue\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__Field\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__InputValue\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__Schema\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__Type\"
\t\t\t\t\t},
\t\t\t\t\t{
\t\t\t\t\t\t\"name\":\"__TypeKind\"
\t\t\t\t\t}
\t\t\t\t]
\t\t\t}
\t\t}
}".to_string();

        println!("{}\n\n", "agql_schema_query_result_ugly:");

        println!("{}\n", agql_schema_query_result_ugly);

        println!("{}\n\n", "agql_schema_query_result_pretty:");

        println!("{}\n", agql_schema_query_result_pretty);

        let mut prettyer = PrettyEr::new();

        let res = prettyer.prettyafy(&agql_schema_query_result_ugly);

        match res
        {

            Ok(val) => {

                println!("{}\n", "val:");

                println!("{}\n", val);

                assert_eq!(agql_schema_query_result_pretty, val);
                
            }
            Err(err) =>
            {

                println!("{}\n", "err:");

                println!("{}\n", err.to_string());

                panic!();

            }

        }


    }

    #[test]
    fn agql_schema_input_object_type_query_result()
    {
   
       //let agql_schema_query_ugly = "{"data":{"__type":{"description":null,"inputFields":[{"name":"bool","description":null,"type":{"kind":"SCALAR","name":"Boolean"},"defaultValue":null},{"name":"char","description":null,"type":{"kind":"SCALAR","name":"Char"},"defaultValue":null},{"name":"f32","description":null,"type":{"kind":"SCALAR","name":"Float"},"defaultValue":null},{"name":"f64","description":null,"type":{"kind":"SCALAR","name":"Float"},"defaultValue":null},{"name":"i8","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i16","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i32","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i64","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i128","description":null,"type":{"kind":"SCALAR","name":"I128Scalar"},"defaultValue":null},{"name":"isize","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u8","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u16","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u32","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u64","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u128","description":null,"type":{"kind":"SCALAR","name":"U128Scalar"},"defaultValue":null},{"name":"usize","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"string","description":null,"type":{"kind":"SCALAR","name":"String"},"defaultValue":null}]}}}";
   
       let agql_query_result_ugly = "{\"data\":{\"__type\":{\"description\":null,\"inputFields\":[{\"name\":\"bool\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Boolean\"},\"defaultValue\":null},{\"name\":\"char\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Char\"},\"defaultValue\":null},{\"name\":\"f32\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Float\"},\"defaultValue\":null},{\"name\":\"f64\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Float\"},\"defaultValue\":null},{\"name\":\"i8\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"i16\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"i32\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"i64\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"i128\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"I128Scalar\"},\"defaultValue\":null},{\"name\":\"isize\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"u8\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"u16\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"u32\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"u64\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"u128\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"U128Scalar\"},\"defaultValue\":null},{\"name\":\"usize\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"Int\"},\"defaultValue\":null},{\"name\":\"string\",\"description\":null,\"type\":{\"kind\":\"SCALAR\",\"name\":\"String\"},\"defaultValue\":null}]}}}".to_string();
       
       let agql_query_result_pretty = "".to_string();
   
       println!("{}\n\n", "agql_query_result_ugly:");
   
       println!("{}\n", agql_query_result_ugly);
   
       println!("{}\n\n", "agql_query_result_pretty:");
   
       println!("{}\n", agql_query_result_pretty);

       let mut prettyer = PrettyEr::new();
   
       let res = prettyer.prettyafy(&agql_query_result_ugly);
   
       match res
       {
   
           Ok(val) => {
   
               println!("{}\n", "val:");
   
               println!("{}\n", val);
   
               assert_eq!(agql_query_result_ugly, val);
   
           }
           Err(err) =>
           {
   
               println!("{}\n", "err:");
   
               println!("{}\n", err.to_string());
   
               panic!();
   
           }
   
       }
   
    }

}

/*

Also:


{"data":{"__type":{"description":null,"inputFields":[{"name":"bool","description":null,"type":{"kind":"SCALAR","name":"Boolean"},"defaultValue":null},{"name":"char","description":null,"type":{"kind":"SCALAR","name":"Char"},"defaultValue":null},{"name":"f32","description":null,"type":{"kind":"SCALAR","name":"Float"},"defaultValue":null},{"name":"f64","description":null,"type":{"kind":"SCALAR","name":"Float"},"defaultValue":null},{"name":"i8","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i16","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i32","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i64","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"i128","description":null,"type":{"kind":"SCALAR","name":"I128Scalar"},"defaultValue":null},{"name":"isize","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u8","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u16","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u32","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u64","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"u128","description":null,"type":{"kind":"SCALAR","name":"U128Scalar"},"defaultValue":null},{"name":"usize","description":null,"type":{"kind":"SCALAR","name":"Int"},"defaultValue":null},{"name":"string","description":null,"type":{"kind":"SCALAR","name":"String"},"defaultValue":null}]}}}


From:

query GetInputObjectType($name: String)
{
	__type(name: $name)
	{
		description
		inputFields
		{
			name
  			description
  			type
  			{
				kind
				name
			}
  			defaultValue
		}
	}	
}

//

{
	"name": "InputWhatever"
}

 */
