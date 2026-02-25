#[derive(Debug)]
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    // Aufgabe b)
    Stormy(u32),
}

fn describe(weather: Weather) {
    print!("describe({:?}): ", weather);
    match weather {
        Weather::Sunny => println!("It's a sunny day!"),
        Weather::Cloudy => println!("It's cloudy today."),
        Weather::Rainy => println!("It's raining today."),
        // Aufgabe b)
        Weather::Stormy(speed) => {
            if speed > 80 {
                println!("Storm warning! Wind speed: {} km/h!", speed);
            } else {
                println!("It's windy ({} km/h).", speed);
            }
        }
    }
}

fn main() {
    let today = Weather::Sunny;
    let tomorrow = Weather::Rainy;
    let weekend = Weather::Stormy(95);

    describe(today);
    describe(tomorrow);
    describe(weekend);
}
