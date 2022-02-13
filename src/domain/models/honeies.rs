use super::{beekeepers::Beekeeper, times::{Year, Month}, prefectures::Prefecture};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HoneyName{
    pub name: String
}

#[derive(Serialize, Deserialize)]
// @todo should be implemented
pub struct Flower{}

#[derive(Serialize, Deserialize)]
pub struct Honey{
    pub name : HoneyName,
    pub year : Year,
    pub month: Option<Month>,
    pub beekeeper: Beekeeper,
    pub prefecture: Option<Prefecture>,
    pub flowers: Vec<Flower>
}