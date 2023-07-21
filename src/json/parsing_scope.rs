
//Is the pasrser possibly in between a bracket pair or quotes?

//Could include comments (if JSON had them)

/*
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParsingScope
{

    curly_brackets,
    square_brackets,
    quotes //,
    //post_scope //To help with keeping commas on the same line as the closing scope charater

}
*/

//Where in the provided String are you? (generally)

/*
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum WhereInStructure
{

    #[default]
    NotInObject,
    Object,
    FieldName,
    FieldValue

}
*/

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParsingScope
{

    Object, //curly_brackets,
    ObjectFieldKey,
    ObjectFieldValue,
    Array, //square_brackets,
    ArrayValue,
    String //, //quotes //,

}

