use super::Indentor;

use std::cell::{RefCell, Ref};

use corlib::{impl_rfc_borrow_call, impl_rfc_borrow}; //impl_rfc_borrow_mut_call, impl_rfc_borrow_get, 

use paste::paste;

///
/// Scope based indentation for convenience.
/// 
pub struct ScopedIndentor
{

    rfc_indentor: RefCell<Indentor>

}

impl ScopedIndentor
{

    pub fn new() -> Self
    {

        Self
        {

            rfc_indentor: RefCell::new(Indentor::new())

        }

    }

    pub fn from_existing(indentor: Indentor) -> Self
    {

        Self
        {

            rfc_indentor: RefCell::new(indentor)

        }

    }

    pub fn try_indent<'a>(&'a self) -> Result<AutoOutdenter<'a>, &'static str>
    {

        self.rfc_indentor.borrow_mut().try_add_level()?;

        Ok(AutoOutdenter::new(&self.rfc_indentor))

        /*
        if self.rfc_indentor.borrow_mut().try_add_level()
        {

            Ok(AutoOutdenter::new(&self.rfc_indentor))

        }
        else {
            
            Err("Maximum indentaion level reached")

        }
        */

    }

    pub fn try_indent_push_onto<'a>(&'a self, value: &mut String) -> Result<AutoOutdenter<'a>, &'static str>
    {

        let res = self.try_indent();

        if let Ok(_res_val) = &res
        {

            self.rfc_indentor.borrow_mut().push_onto(value);

        }

        res

    }

    /*
     
     cannot return reference to temporary value
    returns a reference to data owned by the current functionrustcClick for full compiler diagnostic
    getters_setters_callers.rs(846, 13): Actual error occurred here
    getters_setters_callers.rs(846, 13): temporary value created here

     */

    //impl_rfc_borrow_call!(rc_indentor, get_indentor_string_ref, &String);

    //impl_rfc_borrow_call!(rc_indentor, get_indentor_string_str, &str);

    impl_rfc_borrow!(rfc_indentor, Indentor);

    impl_rfc_borrow_call!(rfc_indentor, clone_indentor_string, String);

    impl_rfc_borrow_call!(rfc_indentor, get_level, usize);

    //impl_rfc_borrow_call!(rc_indentor, push_onto, value: &mut String);

    pub fn push_onto(&self, value: &mut String)
    {

        self.rfc_indentor.borrow().push_onto(value);

    }
    /*
    delegate! {
        to self.indentor {

            pub fn get_indentor_string_ref(&self) -> &String;
        
            pub fn get_indentor_string_str(&self) -> &str;
  
            pub fn clone_indentor_string(&self) -> String;
        
            pub fn get_level(&self) -> usize;
 
            pub fn push_onto(&self, value: &mut String);

        }
    }
    */

}

///
/// Automatically removes an indentation level when dropped.
/// 
pub struct AutoOutdenter<'a>
{

    rfc_indentor: &'a RefCell<Indentor>

}

impl<'a> AutoOutdenter<'a>
{

    pub fn new(rfc_indentor: &'a RefCell<Indentor>) -> Self
    {

        Self
        {

            rfc_indentor

        }

    }

}

impl<'a> Drop for AutoOutdenter<'a>
{

    fn drop(&mut self)
    {

        _ = self.rfc_indentor.borrow_mut().try_remove_level();

    }

}

/* 
//use delegate::delegate;

pub struct ScopedIndentor
{

    indentor: Indentor

}

impl ScopedIndentor
{

    pub fn new() -> Self
    {

        Self
        {

            indentor: Indentor::new()

        }

    }

    pub fn from_existing(indentor: Indentor) -> Self
    {

        Self
        {

            indentor

        }

    }

    pub fn get_indentor_ref(&self) -> &Indentor
    {

        &self.indentor

    }

    /*
    pub(crate) fn get_indentor_mut(&mut self) -> &mut Indentor
    {

        &mut self.indentor

    }
    */

    pub fn try_indent<'a>(&'a mut self) -> Result<AutoOutdenter<'a>, &str> //String>
    {

        if self.indentor.try_add_level()
        {

            Ok(AutoOutdenter::new(self)) //::<'a>

        }
        else {
            
            Err("Maximum indentaion level reached")

        }

    }

    pub fn try_indent_push_onto<'a>(&'a mut self, value: &mut String) -> Result<AutoOutdenter<'a>, &str>
    {

        let res = self.try_indent();

        if let Ok(res_val) = &res
        {

            //self.indentor.push_onto(value);

            res_val.scoped_indentor.push_onto(value);
        }

        res

    }

    delegate! {
        to self.indentor {

            pub fn get_indentor_string_ref(&self) -> &String;
        
            pub fn get_indentor_string_str(&self) -> &str;
  
            pub fn clone_indentor_string(&self) -> String;
        
            pub fn get_level(&self) -> usize;
 
            pub fn push_onto(&self, value: &mut String);

        }
    }

}

pub struct AutoOutdenter<'a>
{

    scoped_indentor: &'a mut ScopedIndentor

}

impl<'a> AutoOutdenter<'a>
{

    pub fn new(scoped_indentor: &'a mut ScopedIndentor) -> Self
    {

        Self
        {

            scoped_indentor

        }

    }

    pub fn get_scoped_indentor_mut(&mut self) -> &mut ScopedIndentor //'a 
    {

        &mut self.scoped_indentor

    }

}

impl<'a> Drop for AutoOutdenter<'a>
{

    fn drop(&mut self)
    {

        self.scoped_indentor.indentor.try_remove_level();

    }

}
*/