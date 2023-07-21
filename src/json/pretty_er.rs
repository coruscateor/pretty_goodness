//use std::collections::HashMap;

use std::{vec::Vec, default, str::Chars};

use super::Indentor; //{ScopedIndentor, AutoOutdenter}; //{Indentor}; //, ParsingScope}; //, WhereInStructure};

use crate::{errors_and_results::{ParsingError, ParsingResult, ParsingInProgressResult}};

//const INDENTATION_CHAR: char = '\t';

const ERROR_UNEXPECTED_EOS: &str = "Error: Unexpected end of String encountered";

const ERROR_UNEXPECTED_CHAR: &str = "Error: Unexpected char encountered";

/*
enum LastChar
{

    Comma,
    ClosingSquareBracket,
    ClossingCurlyBracket,
    Quotes

}
*/

//type JsonResult<T> = Result<T, &'static str>;

///
/// The last parsing action.
/// 
enum LastAction
{

    FoundComma,
    FoundEndOfObject,
    FoundEndOfArray,
    FoundEndOfString,
    FoundEndOfValue,
    FoundKeyColon,
    CompletedObject,
    CompletedArray,
    CompletedString,
    CompletedValue

}

///
/// A JSON stack-oriented prettyer, just call prettyafy.
/// 
pub struct PrettyEr
{

    //indentation_levels: HashMap<u32, String>
    indentior: Indentor,
    //parsing_scope_stack: Vec<ParsingScope>
    //scoped_indentior: ScopedIndentor

}

impl PrettyEr
{

    pub fn new() -> Self
    {

        //let mut indentation_levels = HashMap::with_capacity(21);

        //The first level is an empty String

        //indentation_levels.insert(0, "".to_string());

        Self
        {

           // indentation_levels

           indentior: Indentor::new(),
           //parsing_scope_stack: Vec::with_capacity(10) //Micro location info (which scope am I im?)
           //scoped_indentior: ScopedIndentor::new()

        }

    }

    ///
    /// Convert you ugly JSON into pretty JSON.
    /// 
    pub fn prettyafy(&mut self, ugly_json: &String) -> ParsingResult //<String> //mut 
    {

        //Reset the indentor incase the previous prettyafaction attempt ended in an error

        self.indentior.reset();

        let mut pretty_json = String::with_capacity(ugly_json.len() * 2);

        let mut ugly_chars_iter = ugly_json.chars();

        let mut has_previous_object = false;

        //Find top-level values

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                //let mut has_found_value = false;

                /*
                if has_previous_object
                {

                    pretty_json.push_str("\n\n");

                }
                */

                match ch
                {

                    '"' =>
                    {

                        self.check_has_previous_object(has_previous_object, &mut pretty_json);

                        //Parse and append String

                        //Add opening quote character

                        //pretty_json.push('\"');

                        if let Err(err) = self.parse_string(&mut ugly_chars_iter, &mut pretty_json)
                        {

                            return ParsingResult::Err(ParsingError::new(pretty_json, err));

                        }

                        //find comma

                        //self.parse_comma(&mut ugly_chars_iter); //, &mut pretty_json);

                        has_previous_object = true;

                        break;

                    }
                    '[' =>
                    {

                        self.check_has_previous_object(has_previous_object, &mut pretty_json);

                        //Parse and append array

                        if let Err(err) = self.parse_array(&mut ugly_chars_iter, &mut pretty_json)
                        {

                            return ParsingResult::Err(ParsingError::new(pretty_json, err));

                        }

                        //find comma

                        //self.parse_comma(&mut ugly_chars_iter); //, &mut pretty_json);

                        has_previous_object = true;

                        break;

                    }
                    '{' =>
                    {

                        self.check_has_previous_object(has_previous_object, &mut pretty_json);

                        //Parse and append object

                        //pretty_json.push('\n');

                        if let Err(err) = self.parse_object(&mut ugly_chars_iter, &mut pretty_json)
                        {

                            return ParsingResult::Err(ParsingError::new(pretty_json, err));

                        }

                        //find comma

                        //self.parse_comma(&mut ugly_chars_iter); //, &mut pretty_json);

                        has_previous_object = true;

                        break;

                    }
                    ',' =>
                    {

                        //Leave adding the comma to object kvps and arrays 

                        //Error?

                        //break;

                        //continue;

                        //append erronous camma

                        pretty_json.push(ch);

                        return ParsingResult::Err(ParsingError::new(pretty_json, "Unxepected comma found"));

                    }
                    _ => 
                    {

                        //Append non-String and find comma

                        //skip whitespace
                        
                        if ch.is_whitespace()
                        {

                            continue;

                        }

                        //previous top level object...

                        self.check_has_previous_object(has_previous_object, &mut pretty_json);

                        //push found character

                        pretty_json.push(ch);

                        if let Err(err) = self.parse_non_string_array_or_object(&mut ugly_chars_iter, &mut pretty_json)
                        {

                            return ParsingResult::Err(ParsingError::new(pretty_json, err));

                        }

                        has_previous_object = true;

                        break;

                        //return true;

                    }

                }

            }
            else
            {

                //Error
                
                break;

            }

        }

        ParsingResult::Ok(pretty_json)

        //ch in ugly_chars_iter

        /*
        loop
        {

            if has_previous_object
            {

                pretty_json.push_str("\n\n\n");

            }

            //Attempt to parse a value if the iterator runs out of values it returns false

            if !self.parse_value(&mut ugly_chars_iter, &mut pretty_json)
            {

                break;

            }

            has_previous_object = true;

        }

        pretty_json
        */

        /*
        loop
        {

            //Parse Objects - Outer-most loop

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == '{'
                {

                    if has_previous_object
                    {
    
                        pretty_json.push_str("\n\n\n");
    
                    }

                    //Self::

                    //self.parse_object(&mut ugly_chars_iter, &mut pretty_json);

                    self.parse_value(&mut ugly_chars_iter, &mut pretty_json);

                    has_previous_object = true;

                }

            }
            else
            {

                //Done parsing
                
                return pretty_json;

                //break;

            }

        }
        */

        /*
        let mut previous_char;

        if let Some(val) = ugly_chars_iter.next()
        {

            previous_char = val;

        }
        else
        {

            //Error

            return "".to_string();

        }
        */

        //"".to_string()

    }

    fn check_has_previous_object(&self, has_previous_object: bool, pretty_json: &mut String)
    {

        if has_previous_object
        {

            pretty_json.push_str("\n\n"); //\n

        }

    }

    fn parse_object(&mut self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String) -> ParsingInProgressResult<LastAction> //ParsingInProgressResult<char> //<()> //+ Options? //mut 
    {

        //Indentation for opening brace

        self.indentior.push_onto(pretty_json);

        //Opening brace

        pretty_json.push_str("{\n"); //\n\n

        //Next indentation level

        //self.indentior.try_add_level();

        {

            self.indentior.try_add_level()?;

            //let mut _ind_res = self.scoped_indentior.try_indent();

            //Check for error and return it if found


            //pretty_json.push_str(self.indentior.get_indentor_string_str());

            //self.indentior.push_onto(pretty_json);

            /*
            if let Ok(res_val) = &mut ind_res
            {
    
                //self.indentor.push_onto(value);
    
                res_val.get_scoped_indentor_mut().push_onto(pretty_json);

            }
            */

            //self.scoped_indentior.push_onto(pretty_json);

            let mut has_previous_kvp = false;

            loop
            {

                let la_res;

                if let Some(ch) = ugly_chars_iter.next()
                {

                    if ch == '}'
                    {

                        //End of Object

                        //drop(_ind_res);

                        //self.scoped_indentior.push_onto(pretty_json);

                        /*
                        if has_previous_kvp
                        {

                            pretty_json.push('\n');

                        }

                        self.indentior.try_remove_level()?;

                        self.indentior.push_onto(pretty_json);

                        pretty_json.push('}');

                        //return ParsingInProgressResult::Ok(ch);

                        return ParsingInProgressResult::Ok(LastAction::CompletedObject);
                        */

                        //break;

                        la_res = LastAction::FoundEndOfObject;

                    }
                    else if ch == '"'
                    {

                        //Start of String - Field Key

                        //let last_char;

                        if has_previous_kvp
                        {

                            //Start new string on new line after a comma has been added

                            pretty_json.push_str(",\n"); //\"

                            //self.scoped_indentior.push_onto(pretty_json);

                            self.indentior.push_onto(pretty_json);

                            //pretty_json.push(',');

                            //last_char = self.parse_key_value_pair(ugly_chars_iter, pretty_json)?;

                            la_res = self.parse_key_value_pair(ugly_chars_iter, pretty_json)?;

                        }
                        else
                        {

                            //self.scoped_indentior.push_onto(pretty_json);

                            self.indentior.push_onto(pretty_json);

                            //pretty_json.push('\"');

                            //last_char = self.parse_key_value_pair(ugly_chars_iter, pretty_json)?;

                            la_res = self.parse_key_value_pair(ugly_chars_iter, pretty_json)?;

                            has_previous_kvp = true;

                        }

                        /*
                        if last_char == '}'
                        {

                            //Current KVP is "previous_kvp"

                            pretty_json.push('\n');

                            self.indentior.try_remove_level()?;

                            self.indentior.push_onto(pretty_json);

                            pretty_json.push('}');

                            //return ParsingInProgressResult::Ok(ch);

                            return ParsingInProgressResult::Ok(LastAction::CompletedObject);

                        }
                        */

                        //char is ',' continue...

                    }
                    else if ch.is_whitespace()
                    {

                        continue;

                    }
                    else if ch == ','
                    {

                        //ignore commas

                        continue;
                        
                    }
                    else
                    {

                        pretty_json.push(ch);

                        return ParsingInProgressResult::Err(ERROR_UNEXPECTED_CHAR);
                        
                    }

                    match la_res
                    {

                        LastAction::FoundComma =>
                        {

                            continue;

                        },
                        LastAction::FoundEndOfObject =>
                        {

                            if has_previous_kvp
                            {

                                pretty_json.push('\n');

                            }

                            self.indentior.try_remove_level()?;

                            self.indentior.push_onto(pretty_json);

                            pretty_json.push('}');

                            return ParsingInProgressResult::Ok(LastAction::CompletedObject);

                        },
                        LastAction::CompletedArray | LastAction::CompletedString | LastAction::CompletedValue | LastAction::CompletedObject =>
                        {

                            //if LastAction::CompletedObject, this refers to a contained object
                            
                            continue;

                        }
                        _ =>
                        {

                            return ParsingInProgressResult::Err("Error: Unexpoected Last Action")

                        }

                    }

                    ////continue if whitespace...

                    /* 
                    else if ch.is_whitespace()
                    {

                        continue;

                    }
                    */

                    /*
                    else if ch == ','
                    {

                        //let the previous condition do the work of figuring out whether or not a comma needs to be added

                        continue;

                    }
                    */

                    /*
                    else if ch == '['
                    {

                        //Start of Array

                        

                    }
                    */


                }
                else
                {

                    //Error
                    
                    break;

                    //return ParsingInProgressResult::Err(UNEXPECTED_EOS_ERROR_MESSAGE);

                }

                //self.indentior.try_remove_level();

            }

            self.indentior.try_remove_level()?;

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

        //ParsingInProgressResult::Ok(())

        //pretty_json.push('}');

        /*
        pretty_json.push_str("\n"); //\n

        //Indent before finihing the object
    
        self.scoped_indentior.push_onto(pretty_json);

        pretty_json.push_str("}\n");
        */

    }

    /*
    fn parse_inner_object(&self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String)
    {

        //Add a newline

        pretty_json.push('\n');

        //Parse the object

        self.parse_object(ugly_chars_iter, pretty_json);

    }
    */

    fn parse_key_value_pair(&mut self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String) -> ParsingInProgressResult<LastAction> //ParsingInProgressResult<char> //object_ //mut 
    {

        //Parse String - Key

        self.parse_key(ugly_chars_iter, pretty_json)?;

        //Parse value

        self.parse_value(ugly_chars_iter, pretty_json, true)

        //has found end of block or comma?

        /*
        let last_char = self.parse_value(ugly_chars_iter, pretty_json, true)?;

        match last_char
        {

            //',' | '}' | '\"'  => return Ok(last_char), //| ':'
            /* 
            ',' | '}' | '\"' | ']' => return Ok(last_char),
            _ => {

                if last_char.is_whitespace()
                {

                    self.parse_comma_or_closing_char(ugly_chars_iter)

                }
                else
                {

                    pretty_json.push(last_char);

                    Err(ERROR_UNEXPECTED_CHAR) 

                }

             }
             */

        }
        */

    }

    fn parse_string(&self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String) -> ParsingInProgressResult<LastAction> //<char> //<()> //mut 
    {

        let mut previous_char = '"';

        //Lext indentation level

        //self.indentior.try_add_level();

        //pretty_json.push_str(self.indentior.get_indentor_string_str());

        //--the qunte character is added in the calling function-- <- Incorrect

        pretty_json.push('\"');

        loop
        {

            //Parse the String

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == '"'
                {

                    //End of String? - Escaped?

                    if previous_char == '\\'
                    {

                        //Quote character has been escaped

                        previous_char = ch;

                        pretty_json.push(ch);

                        continue;

                    }

                    //End of String
                    
                    pretty_json.push(ch);

                    //break;

                    //return ParsingInProgressResult::Ok(());

                    //return ParsingInProgressResult::Ok(ch);

                    return ParsingInProgressResult::Ok(LastAction::CompletedString);

                }
                else
                {

                    //Add the current character to the end of the string

                    previous_char = ch;

                    pretty_json.push(ch);
    
                }

            }
            else
            {

                //Error - abrupt end
                
                break;

                //return ParsingInProgressResult::Err("Unexpected end of String encountered");

            }

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

    }

    fn parse_key(&self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String) -> ParsingInProgressResult<LastAction> //<()> //mut 
    {

        self.parse_string(ugly_chars_iter, pretty_json)?;

        //Find the colon

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == ':'
                {

                    //pretty_json.push_str(": ");

                    pretty_json.push(':');

                    return ParsingInProgressResult::Ok(LastAction::FoundKeyColon); //LasAction::CompletedString); //(());

                    //break;

                }

            }
            else
            {

                //return ParsingInProgressResult::Ok("");

                //Error
                
                break;

            }

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

    }

    fn parse_value(&mut self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String, prepend_with_space_if_single_line: bool) -> ParsingInProgressResult<LastAction> //<char> //LastChar> //-> bool //mut 
    {

        //Find the value and add it to pretty_json

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                //let mut has_found_value = false;

                match ch
                {

                    '"' =>
                    {

                        if prepend_with_space_if_single_line
                        {

                            pretty_json.push(' ');

                        }

                        //Parse and append String

                        //if !has_found_value
                        //{

                        return self.parse_string(ugly_chars_iter, pretty_json);

                        //find comma

                        //self.parse_comma_or_closing_char(ugly_chars_iter); //, pretty_json);

                        //break;

                        //return true;

                        /*

                            has_found_value = true;

                        }
                        else
                        {

                            //Error

                        }
                        */

                    }
                    '[' =>
                    {

                        //Parse and append array

                        //if !has_found_value
                        //{

                        pretty_json.push('\n');

                        return self.parse_array(ugly_chars_iter, pretty_json);

                        //break;

                        //find comma

                        //what if curly bracket is found?

                        //return self.parse_comma_or_closing_char(ugly_chars_iter);  //self.parse_comma_or_closing_curly_bracket(ugly_chars_iter); //, pretty_json);

                        //break;

                        //return true;

                        /*
                            has_found_value = true;

                        }
                        else
                        {
                            
                            //Error

                        }
                        */

                    }
                    '{' =>
                    {

                        //Parse and append object

                        //if !has_found_value
                        //{

                        pretty_json.push('\n');

                        //self.indentior.push_onto(pretty_json);

                        //let _ind_res = self.scoped_indentior.try_indent();

                        return self.parse_object(ugly_chars_iter, pretty_json); //inner_

                        //find comma

                        //what if curly bracket is found?

                        //self.parse_comma_or_closing_square_bracket(ugly_chars_iter); //, pretty_json);

                        //break;

                        //return true;

                        /*
                            has_found_value = true;

                        }
                        else
                        {
                            
                            //Error

                        }
                        */

                    }
                    ',' =>
                    {

                        //Leave adding the comma to the next kvp 

                        //Error

                        //break;

                        //return true;

                        //report that a comma has been found - end of value, end of key value pair

                        //return ParsingInProgressResult::Ok(ch);

                        return ParsingInProgressResult::Ok(LastAction::FoundComma);

                        //pretty_json.push(ch);

                        //return ParsingInProgressResult::Err("Error Unxexpected comma found");

                    }
                    /*
                    '}' =>
                    {

                        //Close object block


                    }
                    ']' =>
                    {

                        //Close array block

                        //Error

                    }
                    */
                    _ => 
                    {

                        //Append non-String and find comma

                        //skip whitespace
                        
                        if ch.is_whitespace()
                        {

                            continue;

                        }

                        if prepend_with_space_if_single_line
                        {

                            pretty_json.push(' ');

                        }

                        /*
                        if !has_found_value
                        {

                            //parse and append number



                            has_found_value = true;

                        }
                        */

                        //push found character

                        pretty_json.push(ch);

                        //what if curly bracket is found?

                        return self.parse_non_string_array_or_object(ugly_chars_iter, pretty_json);

                        //end of block?

                        //break;

                        //return true;

                    }

                }

                /*
                if ch == ','
                {

                    //Leave added the comma to the next kvp 

                    break;

                }
                */

            }
            else
            {

                //Error
                
                break;

                //return false;

            }

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

    }

    fn parse_non_string_array_or_object(&self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String) -> ParsingInProgressResult<LastAction> //<char> //err
    {

        //let mut has_found_comma = false;

        //first char has been added

        //let mut has_found_value = false;

        //Loop until 

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if !ch.is_whitespace()
                {

                    /*
                    if ch == ','
                    {

                        //End of non_string_array

                        //When a character is found push a comma and a newline <- NO - Incorrect

                        //pretty_json.push_str(",\n");

                        has_found_comma = true;

                        break;

                        //End parsing this value

                    }
                    else if ch == '}' || ch ==']'
                    {

                        return;

                    }
                    */

                    match ch
                    {

                        ',' =>
                        {

                            return ParsingInProgressResult::Ok(LastAction::FoundComma);

                        }
                        '}' =>
                        {

                            return ParsingInProgressResult::Ok(LastAction::FoundEndOfObject);

                        }
                        ']' =>
                        {

                            return ParsingInProgressResult::Ok(LastAction::FoundEndOfArray);

                        }
                        _ =>
                        {

                            //append the value

                            pretty_json.push(ch);

                        }

                    }

                    /*
                    if ch == ',' || ch == '}' || ch ==']'
                    {

                        return ParsingInProgressResult::Ok(ch);

                    }
                    else //if ch != 
                    {

                        pretty_json.push(ch);

                        //parse until white space ot comma

                        //self.parse_non_string_array_or_object_value(ugly_chars_iter, pretty_json);

                        //break;

                        //pretty_json.push(ch);

                        //try get next character

                    }
                    */

                }
                else
                {
                    
                    return ParsingInProgressResult::Err("Unexpected Whitespece encuntered");

                }

                /* 
                else
                {

                    self.parse_comma(ugly_chars_iter); //, pretty_json);

                    break;
                    
                }
                */

            }
            else
            {

                //Remember to break out of the loop...
                
                break;

            }

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

        /*
        if !has_found_comma
        {

            self.parse_comma_or_closing_char(ugly_chars_iter);

        }
        */

    }

    /*
    fn parse_non_string_array_or_object_value(&self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String)
    {

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                //Push character if it is not whitespace or a comma

                if !ch.is_whitespace()
                {

                    if ch == ','
                    {

                        break;

                    }

                    pretty_json.push(ch);

                }
                else
                {
                    
                    break;

                }

            }

        }

    }
    */

    /*
    fn parse_comma(&self, ugly_chars_iter: &mut Chars<'_>) //, pretty_json: &mut String) //err
    {

        //skip all whitespace values until you find the comma, in which case you return (leave add the comma to the pretty_json until later if this is nessary)

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == ','
                {

                    //When a character is found push a comma and a newline  -- actually no

                    //pretty_json.push_str(",\n");

                    return;

                }
                else if !ch.is_whitespace()
                {

                    //err - expected comma or whitespace

                    break;

                }

            }
            else {

                //err - expected comma

                break;

            }

        }

    }
    */

     /*
    fn parse_comma_or_closing_curly_bracket(&self, ugly_chars_iter: &mut Chars<'_>)
    {

        //skip all whitespace values until you find a comma or curly bracket, in which case you return (leave add the comma to the pretty_json until later if this is nessary)

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == ',' || ch == '}'
                {

                    return;

                }
                else if !ch.is_whitespace()
                {

                    //err - expected comma or whitespace

                    break;

                }
                else
                {
                    
                    //Error

                    break;

                }

            }
            else {

                //err - expected comma

                break;

            }

        }

    }

    fn parse_comma_or_closing_square_bracket(&self, ugly_chars_iter: &mut Chars<'_>)
    {

        //skip all whitespace values until you find a comma or closing square in which case you return (leave add the comma to the pretty_json until later if this is nessary)

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == ','|| ch == ']'
                {

                    return;

                }
                else if !ch.is_whitespace()
                {

                    //err - expected comma or whitespace

                    break;

                }
                else
                {
                    
                    //Error

                    break;

                }

            }
            else
            {

                //err - expected comma

                break;

            }

        }

    }
    */

    /*
    fn parse_comma_or_closing_char(&self, ugly_chars_iter: &mut Chars<'_>) -> ParsingInProgressResult<char> 
    {

        //skip all whitespace values until you find a comma or closing square or curly bracket, in which case you return (leave add the comma to the pretty_json until later if this is nessary)

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == ',' || ch == '}' || ch == ']'
                {

                    //When a character is found push a comma and a newline  -- actually no

                    //pretty_json.push_str(",\n");

                    //return;

                    return ParsingInProgressResult::Ok(ch);

                }
                else if ch.is_whitespace()
                {

                    //Skip whitespace

                    continue;

                }
                /*
                else if !ch.is_whitespace()
                {

                    //err - expected comma or whitespace

                    //break;

                    return ParsingInProgressResult::Err("Expected comma or closing curly or sqare bracket")

                }
                */
                else
                {
                    
                    //Error

                    //break;

                    return ParsingInProgressResult::Err("Error: Expected comma or closing curly or sqare bracket")

                }

            }
            else
            {

                //err - expected comma

                break;

            }

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

    }
    */

    fn parse_comma_or_end_of_array(&self, ugly_chars_iter: &mut Chars<'_>) -> ParsingInProgressResult<LastAction> 
    {

        //skip all whitespace values until you find a comma or a sqyare bracket, In which case you return (leave adding the comma to the pretty_json until later if this is found to be nessary)

        loop
        {

            if let Some(ch) = ugly_chars_iter.next()
            {

                if ch == ','
                {

                    return ParsingInProgressResult::Ok(LastAction::FoundComma);

                }
                else if ch == ']'
                {

                    return ParsingInProgressResult::Ok(LastAction::FoundEndOfArray);

                }
                else if ch.is_whitespace()
                {

                    //Skip whitespace

                    continue;

                }
                else
                {
                    
                    //Error

                    return ParsingInProgressResult::Err("Error: Comma not found")

                }

            }
            else
            {

                //err - expected comma

                break;

            }

        }

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

    }

    //An '[' has already been detected in the ugly_chars_iter

    fn parse_array(&mut self, ugly_chars_iter: &mut Chars<'_>, pretty_json: &mut String) -> ParsingInProgressResult<LastAction> //<char> //<()> //mut 
    {

        //pretty_json.push_str("\n"); //(" \n[");

        //Indentation for array opening bracket

        self.indentior.push_onto(pretty_json);

        //self.scoped_indentior.push_onto(pretty_json);

        pretty_json.push_str("[\n");

        //values

        //self.indentior.try_add_level();

        {

            //let _ind_res = self.scoped_indentior.try_indent();

            self.indentior.try_add_level()?;

            //Check for error

            //self.indentior.push_onto(pretty_json);

            //self.scoped_indentior.push_onto(pretty_json);

            let mut has_previous_element = false;

            loop
            {

                let la_res;

                if let Some(ch) = ugly_chars_iter.next()
                {

                    match ch
                    {

                        '"' =>
                        {

                            if has_previous_element
                            {

                                pretty_json.push_str(",\n");

                                //self.indentior.push_onto(pretty_json);

                            }

                            //self.indentior.try_add_level()?;

                            self.indentior.push_onto(pretty_json);

                            //Parse and append String - parses until '"' is found

                            self.parse_string(ugly_chars_iter, pretty_json)?;

                            //self.indentior.try_remove_level()?;

                            has_previous_element = true;

                            la_res = self.parse_comma_or_end_of_array(ugly_chars_iter)?;

                            /*
                            let last_char = self.parse_comma_or_closing_char(ugly_chars_iter)?;

                            match last_char
                            {

                                ',' => continue,
                                ']' => {

                                    if has_previous_element
                                    {
        
                                        pretty_json.push('\n');
        
                                    }
        
                                    self.indentior.try_remove_level()?;
        
                                    self.indentior.push_onto(pretty_json);
        
                                    pretty_json.push(']');

                                    return ParsingInProgressResult::Ok(LastAction::CompletedArray); //Ok(']');

                                },
                                _ => {

                                    pretty_json.push(last_char);

                                    return ParsingInProgressResult::Err(ERROR_UNEXPECTED_CHAR);

                                }

                            }
                            */

                        }
                        /*'[' =>
                        {

                            if has_previous_element
                            {

                                pretty_json.push_str(",\n");

                            }

                            //Sub array

                            //Parse and append array - parses until ']' is found

                            //pretty_json.push('\n');

                            self.parse_array(ugly_chars_iter, pretty_json)?;

                            has_previous_element = true;

                            let last_char = self.parse_comma_or_closing_char(ugly_chars_iter)?;

                            match last_char
                            {

                                ',' => continue,
                                ']' => {

                                    if has_previous_element
                                    {
        
                                        pretty_json.push('\n');
        
                                    }
        
                                    self.indentior.try_remove_level()?;
        
                                    self.indentior.push_onto(pretty_json);
        
                                    pretty_json.push(']');

                                    return ParsingInProgressResult::Ok(']');

                                },
                                _ => {

                                    pretty_json.push(last_char);

                                    return ParsingInProgressResult::Err(ERROR_UNEXPECTED_CHAR);

                                }

                            }

                        }*/
                        '{' =>
                        {

                            if has_previous_element
                            {

                                pretty_json.push_str(",\n");
                                
                            }

                            //Parse and append object - parses until '}' is found

                            //pretty_json.push('\n');

                            self.parse_object(ugly_chars_iter, pretty_json)?;

                            has_previous_element = true;

                            la_res = self.parse_comma_or_end_of_array(ugly_chars_iter)?;

                            /*
                            let last_char = self.parse_comma_or_closing_char(ugly_chars_iter)?;

                            match last_char
                            {

                                ',' => continue,
                                '}' => continue, //break,
                                ']' => {

                                    if has_previous_element
                                    {
        
                                        pretty_json.push('\n');
        
                                    }
        
                                    self.indentior.try_remove_level()?;
        
                                    self.indentior.push_onto(pretty_json);
        
                                    pretty_json.push(']');

                                    return ParsingInProgressResult::Ok(']');

                                },
                                _ => {

                                    pretty_json.push(last_char);

                                    return ParsingInProgressResult::Err(ERROR_UNEXPECTED_CHAR);

                                }

                            }
                            */

                        }
                        ']' =>
                        {

                            la_res = LastAction::FoundEndOfArray;

                            /* 
                            pretty_json.push('\n');

                            self.indentior.try_remove_level()?;

                            self.indentior.push_onto(pretty_json);

                            //pretty_json.push_str("]\n");

                            pretty_json.push(']');

                            return ParsingInProgressResult::Ok(LastAction::CompletedArray); //(']');
                            */


                        }
                        ',' =>
                        {

                            //return ParsingInProgressResult::Ok(ch);

                            continue;

                            //Error

                            //pretty_json.push(ch);

                            //return ParsingInProgressResult::Err("Error: Unexpected comma encountered");

                        }
                        _ => 
                        {

                            //Append non-String and find the comma or the end of the array - number, true, false and null

                            //skip whitespace
                            
                            if ch.is_whitespace()
                            {

                                continue;

                            }

                            if has_previous_element
                            {

                                pretty_json.push_str(",\n");
                                
                            }

                            self.indentior.push_onto(pretty_json);

                            //Append first char of value

                            pretty_json.push(ch);

                            has_previous_element = true;

                            la_res = self.parse_non_string_array_or_object(ugly_chars_iter, pretty_json)?;

                            /*
                            let last_char = self.parse_non_string_array_or_object(ugly_chars_iter, pretty_json)?;

                            match last_char
                            {

                                ',' => continue,
                                ']' => {

                                    if has_previous_element
                                    {
        
                                        pretty_json.push('\n');
        
                                    }
        
                                    self.indentior.try_remove_level()?;
        
                                    self.indentior.push_onto(pretty_json);
        
                                    pretty_json.push(']');

                                    return ParsingInProgressResult::Ok(']');

                                },
                                _ => {

                                    if !last_char.is_whitespace()
                                    {

                                        pretty_json.push(last_char);

                                        return ParsingInProgressResult::Err(ERROR_UNEXPECTED_CHAR);                                           

                                    }

                                }

                            }

                            let last_char = self.parse_comma_or_closing_char(ugly_chars_iter)?;

                            match last_char
                            {

                                ',' => continue,
                                //']' => break,
                                ']' => {

                                    if has_previous_element
                                    {
        
                                        pretty_json.push('\n');
        
                                    }
        
                                    self.indentior.try_remove_level()?;
        
                                    self.indentior.push_onto(pretty_json);
        
                                    pretty_json.push(']');

                                    return ParsingInProgressResult::Ok(']');

                                },
                                _ => {

                                    pretty_json.push(last_char);

                                    return ParsingInProgressResult::Err(ERROR_UNEXPECTED_CHAR);

                                }

                            }
                            */

                        }

                    }

                    match la_res
                    {

                        LastAction::FoundComma =>
                        {

                            continue;

                        },
                        LastAction::FoundEndOfArray =>
                        {

                            pretty_json.push('\n');

                            self.indentior.try_remove_level()?;

                            self.indentior.push_onto(pretty_json);

                            pretty_json.push(']');

                            return ParsingInProgressResult::Ok(LastAction::CompletedArray);

                        },
                        _ =>
                        {

                            return ParsingInProgressResult::Err("Error: Unexpoected Last Action")

                        }

                    }

                    /*
                    if has_previous_element
                    {

                        pretty_json.push_str(",\n");

                    }
                    else
                    {
                        
                        has_previous_element = true;

                    }
                    */

                    /*
                    if ch == ']'
                    {

                        //End of array

                        //drop(_ind_res);

                        //self.scoped_indentior.push_onto(pretty_json);

                        //pretty_json.push_str("]\n");

                        self.indentior.try_remove_level()?;

                        self.indentior.push_onto(pretty_json);

                        pretty_json.push(']');

                        //break;

                        return ParsingInProgressResult::Ok(ch); //(());

                    }
                    else if ch.is_whitespace()
                    {

                        continue;
                        
                    }

                    //missing first ch?

                    //self.parse_value(ugly_chars_iter, pretty_json);

                    //self.parse_array_value(ugly_chars_iter, pretty_json);

                    //indent

                    self.scoped_indentior.push_onto(pretty_json);

                    //add the current char

                    pretty_json.push(ch);

                    let res_ch = self.parse_value(ugly_chars_iter, pretty_json)?;

                    if res_ch == ']'
                    {

                        drop(_ind_res);

                        self.scoped_indentior.push_onto(pretty_json);

                        pretty_json.push(']');

                        return ParsingInProgressResult::Ok(ch); //(());

                    }
                    */

                    //pretty_json.push('\n');

                }
                else
                {

                    //break out..

                    break;
                    
                    //return ParsingInProgressResult::Err(UNEXPECTED_EOS_ERROR_MESSAGE);

                }
                
            }

            //self.indentior.try_remove_level()?;

        }

        //End of Array

        ParsingInProgressResult::Err(ERROR_UNEXPECTED_EOS)

        //pretty_json.push_str("\n]\n");

        //self.indentior.try_remove_level();

    }

}
    

    /*
    pub fn prettyafy(&mut self, ugly_json: &String) -> String
    {

        /*
        if ugly_string.len() < 1
        {

            return "".to_string();

        }
        */

        let mut previous_char; //: char; //: Option<char> = None; //: char;

        let mut ugly_chars = ugly_json.chars();

        //The pretty string could end up being twice as large and the ugly string or more

        let mut pretty_json = String::with_capacity(ugly_json.len() * 2);

        //let mut where_am_i = WhereInStructure::default(); //Macro location info (broardly where am I in this document?)

        //Get first char, check it and set first previous_char

        if let Some(val) = ugly_chars.next()
        {

            //What kind of char is it?

            if val == '{'
            {

                self.parsing_scope_stack.push(ParsingScope::curly_brackets);

                where_am_i = WhereInStructure::Object;

                //pretty_json.push(val);

                pretty_json.push_str("{\n");

                if !self.indentior.try_add_level()
                {

                    //Error - Attempt to exceeed maximum indenetaion prevented.

                }


            }
            else if !val.is_whitespace()
            {

                //Error

            }
            /*
            else
            {
                
                //Error

            }
            */

            previous_char = val;

        }
        else
        {

            //Error

            return "".to_string();

        }

        //comma, then newline

        for ch in ugly_chars //ugly_string.chars()
        {
            
            //parsing scope

            let last_opt = self.parsing_scope_stack.last().cloned(); //peek

            //let has_closed_parsing_scope;

            //Are we in a parsing scope if so do we exit it?

            if let Some(val) = last_opt
            {

                match val
                {

                    //ParsingScope::curly_brackets => 
                    ParsingScope::Object =>
                    {

                        if ch == '}'
                        {

                            self.parsing_scope_stack.pop();

                            //where_am_i = WhereInStructure::default();

                            //has_closed_parsing_scope = true;

                            self.indentior.try_remove_level();

                            pretty_json.push('\n');

                            pretty_json.push_str(self.indentior.get_indentor_string_str());

                            //pretty_json.push(ch);

                            pretty_json.push_str("}\n\n");

                            previous_char = ch;

                            continue;

                        }

                    },
                    //ParsingScope::square_brackets =>
                    ParsingScope::Array =>
                    {

                        if ch == ']'
                        {

                            self.parsing_scope_stack.pop();

                            //has_closed_parsing_scope = true;

                            self.indentior.try_remove_level();

                            pretty_json.push('\n');

                            pretty_json.push_str(self.indentior.get_indentor_string_str());

                            pretty_json.push_str("]\n\n");

                            previous_char = ch;

                            continue;

                        } else if ch == ','
                        {

                            //finish line

                            pretty_json.push_str(",\n");

                            //next line

                            pretty_json.push_str(self.indentior.get_indentor_string_str());
                            
                        }

                    },
                    //ParsingScope::quotes =>
                    ParsingScope::String =>
                    {

                        if ch == '"'
                        {

                            if previous_char != '\\'
                            {

                                self.parsing_scope_stack.pop();

                                //has_closed_parsing_scope = true;

                            }

                            pretty_json.push(ch);

                            previous_char = ch;

                            continue;

                        }
                        else
                        {

                            pretty_json.push(ch);

                            previous_char = ch;
                            
                            continue;
                            
                        }

                    } //,
                    /* 
                    _ =>
                    {

                        //has_closed_parsing_scope = false;


                    }
                    */

                    /*,
                    ParsingScope::post_scope =>
                    {


                    }
                    */

                }

                /*
                if has_closed_parsing_scope
                {

                    if c

                }
                */

                //Skip if whitespace

                if ch.is_whitespace()
                {

                    previous_char = ch;

                    continue;

                }

                /*

                //Where are we in the document?

                match where_am_i
                {
                    WhereInStructure::NotInObject => {

                        if ch == '{'
                        {

                            where_am_i = WhereInStructure::Object;

                            previous_char = ch;

                            continue;

                        }

                    },
                    WhereInStructure::Object => {



                    },
                    WhereInStructure::FieldName => {



                    },
                    WhereInStructure::FieldValue => todo!(),
                }

                */
                

                previous_char = ch;

            }

        }

        "".to_string()

    }
    */

    /*
    pub fn prettyafy_eat(&mut self, ugly_string: String) -> String
    {

        self.prettyafy(&ugly_string)

    }
    */

//}

