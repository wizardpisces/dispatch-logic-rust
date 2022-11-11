use std::str::FromStr;

#[repr(u8)]
#[derive(Debug,PartialEq)]
pub enum DispatchLogicType {
    Normal,
    Seller
}

// #[derive(Debug)]
// pub struct CustomError{
//     kind: ErrorType
// }

// #[derive(Debug)]
// enum ErrorType{
//     InvalidDispatchLogicType
// }
impl FromStr for DispatchLogicType {

    type Err = String;

    fn from_str(input: &str) -> Result<DispatchLogicType, Self::Err> {
        match input {
            "Normal"  => Ok(DispatchLogicType::Normal),
            "Seller"  => Ok(DispatchLogicType::Seller),
            // _      => Err(CustomError{kind:ErrorType::InvalidDispatchLogicType}),
            _      => Err(String::from("unknown dispatch logic type")),
        }
    }
}