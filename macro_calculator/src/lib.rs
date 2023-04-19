use json::*;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64    
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let cal_value: f64 = food.calories[1].trim_end_matches("kcal").parse().unwrap();
        cals += cal_value * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    cals = (cals * 100.0).round() / 100.0;
    carbs = (carbs * 100.0).round() / 100.0;
    proteins = (proteins * 100.0).round() / 100.0;
    fats = (fats * 100.0).round() / 100.0;

    object! {
        cals: cals,
        carbs: carbs,
        proteins: proteins,
        fats: fats
    }
}