// @todo Should change to lib
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Year {
    pub value : i16
}

#[derive(Serialize, Deserialize)]
pub struct Month {
    pub value : i8
}

impl Month {
    
}