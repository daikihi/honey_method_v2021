use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PrefectureId{
    pub id: i16,
}

#[derive(Serialize, Deserialize)]
pub struct PrefectureName{
    pub name : String,
}

#[derive(Serialize, Deserialize)]
pub struct PrefectureAlphabet{
    pub name_alphabet: String,
}

#[derive(Serialize, Deserialize)]
pub struct PrefectureKana{
    pub name_kana: String,
}

#[derive(Serialize, Deserialize)]
pub struct Prefecture{
    pub id: PrefectureId,
    pub name: PrefectureName,
    pub name_alphabet: PrefectureAlphabet,
    pub name_kana: PrefectureKana
}