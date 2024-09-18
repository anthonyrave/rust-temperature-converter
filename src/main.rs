use std::io;

enum MenuOption {
    ToFarenheit,
    ToCelsius,
}

fn main() {
    display_title();

    ask_option();

    let conversion_option = match get_menu_choice() {
        Some(opt) => opt,
        None => {
            println!("Invalid Option.");
            return;
        }
    };

    ask_value();

    let value_to_convert = match get_value_to_convert() {
        Some(value) => value,
        None => {
            println!("Invalid temperature");
            return;
        },
    };

    let converted_value = exec_conversion(&conversion_option, value_to_convert);

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

fn get_menu_choice() -> Option<MenuOption> {
    let mut menu_choice = String::new();

    io::stdin()
        .read_line(&mut menu_choice)
        .expect("Failed to read your choice.");

    match menu_choice.trim().parse::<u8>() {
        Ok(num) => match num {
            1 => Some(MenuOption::ToFarenheit),
            2 => Some(MenuOption::ToCelsius),
            _ => None,
        }
        Err(_) => None,
    }
}

fn get_value_to_convert() -> Option<f64> {
    let mut value_to_convert = String::new();

    io::stdin()
        .read_line(&mut value_to_convert)
        .expect("Failed to read the value to convert.");

    match value_to_convert.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn exec_conversion(conversion_option: &MenuOption, t: f64) -> f64 {
    match conversion_option {
        MenuOption::ToFarenheit => convert_to_farenheit(t),
        MenuOption::ToCelsius => convert_to_celsius(t),
    }
}

fn convert_to_farenheit(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn display_output(conversion_option: MenuOption, value_to_convert: f64, converted_value: f64) {
    let old_unit = match conversion_option {
        MenuOption::ToFarenheit => "°C",
        MenuOption::ToCelsius => "°F",
    };

    let new_unit = match conversion_option {
        MenuOption::ToFarenheit => "°F",
        MenuOption::ToCelsius => "°C",
    };

    println!("{value_to_convert}{old_unit} -> {converted_value}{new_unit}");
}
