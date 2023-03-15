pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn trim(dec: f32, len: i32) -> f32 {
    let string_dec = dec.to_string();
    let mut counter = -1;
    let mut past_decimal = false;
    let char_array = &string_dec.chars();
    let mut new_string = String::new();
    for c in char_array.clone().into_iter() {
        if past_decimal == false {
            if c == '.' {
                past_decimal = true;
            }
        }
        else if past_decimal {
            counter += 1;
        }
        if counter < len {
            new_string.push(c);
        }
    }
    return new_string.parse::<f32>().unwrap();
}

pub fn get_float_precision(num: f32) -> i32 {
    let string_dec = num.to_string();
    let mut past_decimal = false;
    let char_array = string_dec.chars();
    let mut counter = 0;
    for c in char_array.clone().into_iter() {
        if past_decimal == false {
            if c == '.' {
                past_decimal = true;
            }
        } else {
            counter += 1;
        }
    }
    return counter;
}

pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().replace("\n", "");
    return input;
}