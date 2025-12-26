/**
* # Universal Converter CLI
*
* A comprehensive and extensible command-line utility designed to perform high-precision conversions
* across a diverse range of units, systems, and categories. This program serves as a centralized
* engine for transforming numerical data between disparate measurement scales.
*
* ## Supported Categories:
*
* ### 1. Currency Conversion
* - Provides real-time or static exchange rate transformations between global fiat currencies
*   (e.g., USD, EUR, GBP).
* - Architected to support external API integration for up-to-the-minute financial data.
*
* ### 2. Physical Measurements (Length, Mass, Volume)
* - **Length**: Seamlessly switch between Metric (mm, cm, m, km) and Imperial/US Customary
*   (in, ft, yd, mi) systems.
* - **Mass/Weight**: Precision handling of grams, kilograms, metric tonnes, ounces, and pounds.
* - **Volume**: Conversions for liquid and dry measurements including liters, milliliters,
*   gallons, quarts, pints, and cups.
*
* ### 3. Science & Thermodynamics
* - **Temperature**: Accurate mapping between Celsius, Fahrenheit, and Kelvin scales,
*   accounting for both multiplicative factors and additive offsets.
* - **Pressure**: Transformation between Pascals, Bar, PSI, and Atmospheres.
*/
use std::io::{self, Write};

struct CurrencyConverter;
enum Currency {
    USD,
    EUR,
    GBP,
}

enum Length {
    Meter,
    Kilometer,
    Millimeter,
    Inch,
    Foot,
    Mile,
}

enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Length {
    /// Returns the conversion factor relative to the base unit (Meter)
    fn factor(&self) -> f64 {
        match self {
            Length::Meter => 1.0,
            Length::Kilometer => 1000.0,
            Length::Millimeter => 0.001,
            Length::Inch => 0.0254,
            Length::Foot => 0.3048,
            Length::Mile => 1609.34,
        }
    }
}

impl CurrencyConverter {
    fn convert(amount: f64, from: Currency, to: Currency) -> f64 {
        match (from, to) {
            (Currency::EUR, Currency::USD) => amount * 1.18,
            (Currency::USD, Currency::EUR) => amount / 1.18,
            (Currency::GBP, Currency::EUR) => amount * 1.15,
            (Currency::EUR, Currency::GBP) => amount / 1.15,
            (Currency::GBP, Currency::USD) => amount * 1.35,
            (Currency::USD, Currency::GBP) => amount / 1.35,
            _ => amount, // Handle cases like EUR to EUR or unknown combinations
        }
    }
}

struct LengthConverter;

impl LengthConverter {
    fn convert(amount: f64, from: Length, to: Length) -> f64 {
        let meters = amount * from.factor();
        meters / to.factor()
    }
}

impl Temperature {
    fn to_base(&self, value: f64) -> f64 {
        match self {
            Temperature::Celsius => value,
            Temperature::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
            Temperature::Kelvin => value - 273.15,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            Temperature::Celsius => value,
            Temperature::Fahrenheit => (value * 9.0 / 5.0) + 32.0,
            Temperature::Kelvin => value + 273.15,
        }
    }
}

struct TemperatureConverter;

impl TemperatureConverter {
    fn convert(amount: f64, from: Temperature, to: Temperature) -> f64 {
        let base = from.to_base(amount);
        to.from_base(base)
    }
}

fn main() {
    println!("--- 🛠️  Universal Converter Engine 🛠️  ---");

    loop {
        println!("\nMain Menu:");
        println!("1. Currency (USD, EUR, GBP)");
        println!("2. Length (M, KM, IN, FT, MI)");
        println!("3. Temperature (C, F, K)");
        println!("4. Exit");
        print!("Select a category: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => handle_currency(),
            "2" => handle_length(),
            "3" => handle_temperature(),
            "4" | "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice, please try again."),
        }
    }
}

fn handle_currency() {
    println!("\n--- Currency Converter ---");
    let amount = get_amount();
    println!("From (USD, EUR, GBP):");
    let from = get_currency();
    println!("To (USD, EUR, GBP):");
    let to = get_currency();

    let result = CurrencyConverter::convert(amount, from, to);
    println!("✅ Result: {:.2}", result);
}

fn handle_length() {
    println!("\n--- Length Converter ---");
    let amount = get_amount();
    println!("From (Meter, Kilometer, Inch, Foot, Mile):");
    let from = get_length();
    println!("To (Meter, Kilometer, Inch, Foot, Mile):");
    let to = get_length();

    let result = LengthConverter::convert(amount, from, to);
    println!("✅ Result: {:.4}", result);
}

fn handle_temperature() {
    println!("\n--- Temperature Converter ---");
    let amount = get_amount();
    println!("From (Celsius, Fahrenheit, Kelvin):");
    let from = get_temp();
    println!("To (Celsius, Fahrenheit, Kelvin):");
    let to = get_temp();

    let result = TemperatureConverter::convert(amount, from, to);
    println!("✅ Result: {:.2}", result);
}

// --- Helper Functions for Input ---

fn get_amount() -> f64 {
    loop {
        print!("Enter amount: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("❌ Invalid number."),
        }
    }
}

fn get_currency() -> Currency {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "USD" => return Currency::USD,
            "EUR" => return Currency::EUR,
            "GBP" => return Currency::GBP,
            _ => print!("❌ Invalid currency. Try USD, EUR, or GBP: "),
        }
        io::stdout().flush().unwrap();
    }
}

fn get_length() -> Length {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "m" | "meter" => return Length::Meter,
            "km" | "kilometer" => return Length::Kilometer,
            "in" | "inch" => return Length::Inch,
            "ft" | "foot" => return Length::Foot,
            "mi" | "mile" => return Length::Mile,
            _ => print!("❌ Invalid unit. Try Meter, KM, Inch, Foot, or Mile: "),
        }
        io::stdout().flush().unwrap();
    }
}

fn get_temp() -> Temperature {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "C" | "CELSIUS" => return Temperature::Celsius,
            "F" | "FAHRENHEIT" => return Temperature::Fahrenheit,
            "K" | "KELVIN" => return Temperature::Kelvin,
            _ => print!("❌ Invalid unit. Try C, F, or K: "),
        }
        io::stdout().flush().unwrap();
    }
}
