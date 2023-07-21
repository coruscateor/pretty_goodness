//use std::collections::HashMap;

const INDENTATION_CHAR: char = '\t';

pub struct Indentor
{

    //indentation_levels: HashMap<u32, String>
    indentor_string: String
}

///
/// Adds and removes indentation levels (one or more non-newline whitespace characters) to a provided String.
/// 
impl Indentor
{

    pub fn new() -> Self
    {

        //let mut indentation_levels = HashMap::with_capacity(21);

        //The first level is an empty String

        //indentation_levels.insert(0, "".to_string());

        Self
        {

           // indentation_levels

           indentor_string: String::with_capacity(21)

        }

    }


    pub fn get_indentor_string_ref(&self) -> &String
    {

        &self.indentor_string

    }

    pub fn get_indentor_string_str(&self) -> &str
    {

        self.indentor_string.as_str()

    }

    pub fn clone_indentor_string(&self) -> String
    {

        self.indentor_string.clone()

    }

    pub fn get_level(&self) -> usize
    {

        self.indentor_string.len()

    }

    pub fn push_onto(&self, value: &mut String)
    {

        value.push_str(self.get_indentor_string_str());

    }

    pub fn try_add_level(&mut self) -> Result<(), &'static str> // -> bool
    {

        if self.indentor_string.len() < usize::MAX
        {

            self.indentor_string.push(INDENTATION_CHAR);

            return Ok(());

        }

       Err("Maximum indentaion level reached")

        /*
        if self.indentor_string.len() < usize::MAX
        {

            self.indentor_string.push(INDENTATION_CHAR);

            return true;

        }

        false
        */

    }

    pub fn try_remove_level(&mut self) -> Result<(), &'static str> //-> bool
    {

        //self.indentor_string.pop().is_some()

        if self.indentor_string.len() > 0
        {

            self.indentor_string.pop();

            return Ok(());

        }

        Err("Minimum indentaion level reached")

    }

    pub fn reset(&mut self)
    {

        self.indentor_string.clear()

    }

    pub fn reset_get_len(&mut self) -> usize
    {

        let len = self.indentor_string.len();

        self.reset();

        len

    }

}

