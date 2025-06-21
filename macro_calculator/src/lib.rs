use json::JsonValue;

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

fn cal_cals(cal2 : String) -> f64 {
    let s2 =  String::new()+&cal2[..cal2.len()-4];
    let n2 = s2.parse::<f64>().unwrap();
    n2
}


pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.;
    let mut carbs = 0.;
    let mut proteins = 0.;
    let mut fats = 0.;
    for v in foods {
        let (_,c2) = v.calories.clone();
        cals += ((cal_cals(c2)*v.nbr_of_portions*100.0).round())/100.0;
        carbs += ((v.carbs*v.nbr_of_portions*100.0).round())/100.0;
        proteins += ((v.proteins*v.nbr_of_portions*100.0).round())/100.0;
        fats += ((v.fats*v.nbr_of_portions*100.0).round())/100.0;
    }

    json::object! {
        "cals" => format!("{:.2}",cals).parse::<f64>().unwrap(),
        "carbs" => carbs,
        "proteins" => proteins,
        "fats" => fats,
    }
}