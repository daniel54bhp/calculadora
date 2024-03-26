use regex::Regex;

enum Operations {
    ADDITION,
    SUBTRACT,
    DIVIDE,
    MULTIPLY
}

fn main() {

    
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    println!("Introducir la expresión: ");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();

    user_input = math_operation(re_div, user_input, Operations::DIVIDE);
    user_input = math_operation(re_mult, user_input, Operations::MULTIPLY);
    user_input = math_operation(re_add, user_input, Operations::ADDITION);
    user_input = math_operation(re_sub, user_input, Operations::SUBTRACT);

    println!("El resultado es: {}", user_input);

}


fn math_operation ( regex_expression: Regex, mut user_input: String, operator: Operations ) -> String {

    loop {
        
        let caps = regex_expression.captures(user_input.as_str());
        if caps.is_none() { break }

        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result_math = match operator {
            Operations::MULTIPLY  => left_value * right_value,
            Operations::DIVIDE    => left_value / right_value,
            Operations::ADDITION  => left_value + right_value,
            Operations::SUBTRACT  => left_value - right_value
        };

        user_input = user_input.replace(cap_expression, &result_math.to_string());
    }

    return user_input;
}