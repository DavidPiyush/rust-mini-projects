use std::io;

fn main() {
    loop {
        println!("\n📐 Unit Converter Menu");
        println!("1. Convert Length");
        println!("2. Convert Temperature");
        println!("3. Convert Weight");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => convert_length(),
            "2" => convert_temp(),
            "3" => convert_weight(),
            "4" => {
                println!("👋 Exiting program.");
                break;
            }
            _ => println!("❌ Invalid choice. Please choose a valid option (1–4)."),
        }
    }
}

fn convert_length() {
    let mut value = String::new();
    println!("Enter the value:");
    io::stdin().read_line(&mut value).unwrap();

    let value: f64 = match value.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("⚠️ Invalid number.");
            return;
        }
    };

    println!("Enter your Source unit (m/km/mi):");
    let mut source_unit = String::new();
    io::stdin().read_line(&mut source_unit).unwrap();
    let source_unit = source_unit.trim().to_lowercase();

    println!("Enter your Target unit (m/km/mi):");
    let mut target_unit = String::new();
    io::stdin().read_line(&mut target_unit).unwrap();
    let target_unit = target_unit.trim().to_lowercase();

    if source_unit == target_unit {
        println!("✅ Result: {} {}", value, target_unit);
        return;
    }

    let result = match (source_unit.as_str(), target_unit.as_str()) {
        ("m", "km") => value / 1000.0,
        ("km", "m") => value * 1000.0,
        ("m", "mi") => value / 1609.34,
        ("mi", "m") => value * 1609.34,
        ("km", "mi") => value / 1.60934,
        ("mi", "km") => value * 1.60934,
        _ => {
            println!("❌ Unsupported unit conversion.");
            return;
        }
    };

    println!("✅ Result: {:.4} {}", result, target_unit);
}

fn convert_temp() {
    let mut value = String::new();
    println!("Enter the temperature value:");
    io::stdin().read_line(&mut value).unwrap();

    let value: f64 = match value.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("⚠️ Invalid number.");
            return;
        }
    };

    println!("Enter your Source unit (C/F/K):");
    let mut source_unit = String::new();
    io::stdin().read_line(&mut source_unit).unwrap();
    let source_unit = source_unit.trim().to_uppercase();

    println!("Enter your Target unit (C/F/K):");
    let mut target_unit = String::new();
    io::stdin().read_line(&mut target_unit).unwrap();
    let target_unit = target_unit.trim().to_uppercase();

    if source_unit == target_unit {
        println!("✅ Result: {:.2} {}", value, target_unit);
        return;
    }

    let result = match (source_unit.as_str(), target_unit.as_str()) {
        ("C", "F") => value * 9.0 / 5.0 + 32.0,
        ("F", "C") => (value - 32.0) * 5.0 / 9.0,
        ("C", "K") => value + 273.15,
        ("K", "C") => value - 273.15,
        ("F", "K") => (value - 32.0) * 5.0 / 9.0 + 273.15,
        ("K", "F") => (value - 273.15) * 9.0 / 5.0 + 32.0,
        _ => {
            println!("❌ Unsupported temperature conversion.");
            return;
        }
    };

    println!("✅ Result: {:.2} {}", result, target_unit);
}

fn convert_weight() {
    let mut value = String::new();
    println!("Enter the weight value:");
    io::stdin().read_line(&mut value).unwrap();

    let value: f64 = match value.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("⚠️ Invalid weight input.");
            return;
        }
    };

    println!("Enter source unit (g/kg/lb):");
    let mut source_unit = String::new();
    io::stdin().read_line(&mut source_unit).unwrap();
    let source_unit = source_unit.trim().to_lowercase();

    println!("Enter target unit (g/kg/lb):");
    let mut target_unit = String::new();
    io::stdin().read_line(&mut target_unit).unwrap();
    let target_unit = target_unit.trim().to_lowercase();

    if source_unit == target_unit {
        println!("✅ Result: {:.2} {}", value, target_unit);
        return;
    }

    let grams = match source_unit.as_str() {
        "g" => value,
        "kg" => value * 1000.0,
        "lb" => value * 453.592,
        _ => {
            println!("❌ Unsupported source unit.");
            return;
        }
    };

    let result = match target_unit.as_str() {
        "g" => grams,
        "kg" => grams / 1000.0,
        "lb" => grams / 453.592,
        _ => {
            println!("❌ Unsupported target unit.");
            return;
        }
    };

    println!("✅ Result: {:.2} {}", result, target_unit);
}
