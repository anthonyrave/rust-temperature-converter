use std::io;

fn main() {
    display_title();

    ask_option();

    let conversion_option = match get_menu_choice() {
        Ok(choice) => choice,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    ask_value();

    let value_to_convert = match get_value_to_convert() {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let converted_value = match exec_conversion(conversion_option, value_to_convert) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    display_output(
        conversion_option,
        value_to_convert,
        converted_value
    );
}

fn display_title() {
    println!("Temperature converter!");
}

fn ask_option() {
    println!("To what unit would you like to convert?");
    println!("  1. To Farenheit (F)");
    println!("  2. To Celsius (C)");
    println!("[1, 2]");
}

fn ask_value() {
    println!("What's the current Temperature?");
}

fn get_menu_choice() -> Result<usize, &'static str> {
    let mut menu_choice = String::new();

    io::stdin()
        .read_line(&mut menu_choice)
        .expect("Failed to read your choice.");

    match menu_choice.trim().parse::<usize>() {
        Ok(num) => match num {
            1 | 2 => Ok(num),
            _ => Err("You must type a number between 1 and 2."),
        },
        Err(_) => Err("Menu choice must be a number."),
    }
}

fn get_value_to_convert() -> Result<f64, &'static str> {
    let mut value_to_convert = String::new();

    io::stdin()
        .read_line(&mut value_to_convert)
        .expect("Failed to read the value to convert.");

    match value_to_convert.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err("You must type a float value."),
    }
}

fn exec_conversion(conversion_option: usize, t: f64) -> Result<f64, &'static str> {
    match conversion_option {
        1 => Ok(convert_to_farenheit(t)),
        2 => Ok(convert_to_celsius(t)),
        _ => Err("Invalid conversion option."),
    }
}

fn convert_to_farenheit(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn display_output(conversion_option: usize, value_to_convert: f64, converted_value: f64) {
    let old_unit = if conversion_option == 1 { "°C" } else { "°F" };
    let new_unit = if conversion_option == 1 { "°F" } else { "°C" };

    println!("{value_to_convert}{old_unit} -> {converted_value}{new_unit}");
}
