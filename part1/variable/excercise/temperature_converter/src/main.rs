fn main() {
    let celcius: f64 = 30.0;
    let fahrenheit: f64 = celcius * 9.0 / 5.0 + 32.0;
    let kelvin: f64 = celcius + 273.15;

    println!("{}°C = {}°F = {}K", celcius, fahrenheit, kelvin);
}
