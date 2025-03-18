fn main() {
    println!("{}", convert_temperature(10.0, 'C'));
}

fn convert_temperature(temperature: f32, unit: char) -> String {
    let celsius_to_fahrenheit = temperature * (9.0 / 5.0) + 32.0;
    let fahrenheit_to_celsius = (temperature - 32.0) * 5.0 / 9.0;

    let converted = match unit {
        'C' => celsius_to_fahrenheit,
        'F' => fahrenheit_to_celsius,
        _ => panic!("Invalid input please enter Celsius or Fahrenheit"),
    };

    format!(
        "{:.1} Degrees {}",
        converted,
        if unit == 'C' { 'F' } else { 'C' }
    )
}
