#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
// TO DO: Replace the "mileage" field from the previous exercise with an "age" field
// TO DO" The "age" field should hold tuple value of two fields: String, u32
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (String, u32),
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the age ("New" or "Used") and miles
// Return a tuple with the arrow `->` syntax
fn car_quality(miles: u32) -> (String, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    // TO DO: Correct the quality declaration so we can change the values later
    let mut quality: (String, u32) = ("New".to_string(), miles);

    // Use a conditional expression to check the miles
    // If the car has accumulated miles, then the car is used
    if miles > 0 {
        // TO DO: Add the code to set the quality value for a used car
        quality.0 = String::from("Used");
    }

    // TO DO: Return the completed tuple
    return quality;
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has closed roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // TO DO: Replace the "mileage" field from the previous exercise with an "age" field
    // TO DO" The "age" field calls the "car_quality" function with the "miles" input argument
    let car = Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    };

    // Return new instance of "Car" struct
    return car;
}


#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }


fn main() {
    use std::collections::HashMap;
    let mut orders: HashMap<String, u32> = HashMap::new();
    let (mut new_cars, mut used_cars) = (1, 1);
    let (mut manual, mut auto) = (1, 1);

    // Create car color array
    // TO DO: Set the values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver
    let colors = ["Blue", "Green", "Red", "Silver"];
    // Initialize variables
    let (mut index, mut order) = (1, 1);

    // Declare the car type and initial values
    // TO DO: Create "car" as a "Car" struct
    // TO DO: Create "engine" as a "Transmission" enum
    // TO DO: Initialize "roof" to the value when the car has a hard top
    let mut car: Car;
    let mut miles = 1000; // Start used cars with 1,000 miles
    let mut roof: bool = true;  // convertible = false | hard top = true
    let mut engine: Transmission;

    while order <= 11 {
        // Set car transmission type
        if order % 3 == 0 {
            engine = Transmission::Automatic;
            orders.insert("Automatic".to_string(), auto);
            auto += 1;
            roof = !roof;
        } else if order % 2 == 0 {
            engine = Transmission::SemiAuto;
        } else {
            engine = Transmission::Manual;
            orders.insert("Manual".to_string(), manual);
            manual += 1;
        }


        // Order the cars, New are even numbers, Used are odd numbers
        // TO DO: Fix indexing into `colors` array, vary color for the orders
        if index % 2 != 0 {
            car = car_factory(String::from(colors[index - 1]), engine, roof, miles);
            orders.insert(String::from("Used"), used_cars);
            used_cars = used_cars + 1;
        } else {
            car = car_factory(String::from(colors[index - 1]), engine, roof, 0);
            orders.insert(String::from("New"), new_cars);
            new_cars = new_cars + 1;
        }

        // Display car order details
        if car.roof && car.age.0 == "Used" {
            println!("{}: {}, {:?}, Closed roof, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1);
        } else if car.age.0 == "New" && car.roof {
            println!("{}: {}, {:?}, Closed roof, {}", order, car.age.0, car.motor, car.color);
        } else if !car.roof && car.age.0 == "Used" {
            println!("{}: {}, {:?}, Convertible, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1);
        } else {
            println!("{}: {}, {:?}, Convertible, {}", order, car.age.0, car.motor, car.color);
        }
        // Change values for next loop
        // TO DO: Increment "order" by 1, and "miles" by 1,000
        order += 1;
        miles += 1000;

        // Adjust the index for the car details
        // Order 11 cars, use index range of 0 -- 4, then repeat from 0
        if index < 4 {
            index = index + 1;
        } else {
            index = 1;
        }
    }
    println!("\nCar orders: {:?}", orders);
}

