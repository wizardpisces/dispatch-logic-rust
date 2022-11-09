pub mod portal {
    use crate::constant::DispatchLogicType;
    pub fn text(portal: &DispatchLogicType) -> DispatchLogicType{
        if *portal==DispatchLogicType::Normal {
            DispatchLogicType::Normal
        }else {
            DispatchLogicType::Seller
        }
    }
}
