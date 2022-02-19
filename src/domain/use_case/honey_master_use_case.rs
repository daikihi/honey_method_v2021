use crate::domain::models::honeies::Honey;

pub async fn get_all_honeies() -> Vec<Honey>{
    // @todo should call to HMS API or DB repository
    get_dummy_honeies()
}


fn get_dummy_honeies() -> Vec<Honey> {
    vec![
        Honey{
            name: crate::domain::models::honeies::HoneyName { name: "里山のはちみつ".to_string() },
            year: crate::domain::models::times::Year { value: 2020 },
            month: Some(crate::domain::models::times::Month{ value: 7}),
            beekeeper: crate::domain::models::beekeepers::Beekeeper{},   
            prefecture: None,
            flowers: vec![]
        },        
        Honey{
            name: crate::domain::models::honeies::HoneyName { name: "桜の蜂蜜".to_string() },
            year: crate::domain::models::times::Year { value: 2020 },
            month: Some(crate::domain::models::times::Month{ value: 4}),
            beekeeper: crate::domain::models::beekeepers::Beekeeper{},   
            prefecture: None,
            flowers: vec![]
        }

    ]
}