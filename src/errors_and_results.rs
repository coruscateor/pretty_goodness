
use corlib::{impl_get, impl_get_ref};

use paste::paste;

pub type ParsingInProgressResult<T> = Result<T, &'static str>;

//pub type ParsingResult<T> = Result<T, ParsingError>;

pub type ParsingResult = Result<String, ParsingError>;

#[derive(Debug)]
pub struct ParsingError
{

    result: String,
    message: &'static str

}

impl ParsingError
{

    pub fn new(result: String, message: &'static str) -> Self
    {

        Self
        {

            result,
            message

        }

    }

    impl_get_ref!(result, String);

    impl_get!(message, &'static str);

    //Get lines and columns

}

impl ToString for ParsingError
{

    fn to_string(&self) -> String {
       
       let mut as_string = String::with_capacity(self.result.len() + self.message.len() + 2);

       as_string.push_str(self.result.as_str());

       as_string.push_str("\n\n");

       as_string.push_str(self.message);

       as_string

    }

}


