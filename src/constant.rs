use std::str::FromStr;
// use std::error::Error;
// use std::fmt;
// impl Error for DispatchLogicType {}

#[repr(u8)]
#[derive(Debug,PartialEq)]
pub enum DispatchLogicType {
    Normal,
    Seller
}

impl FromStr for DispatchLogicType {

    type Err = ();

    fn from_str(input: &str) -> Result<DispatchLogicType, Self::Err> {
        match input {
            "Normal"  => Ok(DispatchLogicType::Normal),
            "Seller"  => Ok(DispatchLogicType::Seller),
            _      => Err(()),
        }
    }
}