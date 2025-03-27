#[derive(Debug)]
enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

#[derive(Debug)]
struct Vehicle {
    brand: String,
    model: String,
    fuel_type: FuelType,
}

// Function to filter only electric vehicles and display their details
fn filter_electric_vehicles(vehicles: &Vec<Vehicle>) {
    for vehicle in vehicles {
        if let FuelType::Electric = vehicle.fuel_type {
            println!("Brand: {}, Model: {}, Fuel Type: {:?}", vehicle.brand, vehicle.model, vehicle.fuel_type);
        }
    }
}

fn main() {
    let vehicles = vec![
        Vehicle { brand: "Tesla".to_string(), model: "Model S".to_string(), fuel_type: FuelType::Electric },
        Vehicle { brand: "Toyota".to_string(), model: "Corolla".to_string(), fuel_type: FuelType::Petrol },
        Vehicle { brand: "Ford".to_string(), model: "F-150".to_string(), fuel_type: FuelType::Diesel },
    ];

    println!("Electric Vehicles:");
    filter_electric_vehicles(&vehicles);
}
