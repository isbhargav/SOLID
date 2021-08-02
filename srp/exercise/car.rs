use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Car {
    id: i32,
    model: String,
    brand: String,
}

impl Car {
    pub fn new(id: i32, model: String, brand: String) -> Self {
        return Car { id, model, brand };
    }
}

#[derive(Debug)]
pub struct CarManager {
    cars_db: Vec<Car>,
}

impl CarManager {
    pub fn new() -> Self {
        let mut cars = Vec::new();
        cars.push(Car::new(
            1,
            "Golf III".to_string(),
            "Volkswagen".to_string(),
        ));
        cars.push(Car::new(2, "Multipla".to_string(), "Fiat".to_string()));
        cars.push(Car::new(3, "Megane".to_string(), "Renault".to_string()));
        return Self { cars_db: cars };
    }
    pub fn get_from_db(&self, car_id: i32) -> Option<&Car> {
        self.cars_db.iter().find(|car| car.id == car_id)
    }
    pub fn get_car_names(&self) -> Vec<String> {
        let names = self
            .cars_db
            .iter()
            .map(|c| [&c.brand, " ", &c.model].join(""))
            .collect::<Vec<String>>();
        return names;
    }
    pub fn get_best_car(&self) -> Option<&Car> {
        if self.cars_db.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        let rand_idx = rng.sample(Uniform::new(0, self.cars_db.len()));
        let best_car = &self.cars_db[rand_idx];
        return Some(best_car);
    }
}

