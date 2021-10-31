#[derive(Debug)]
struct Number {
    sign: bool,
    integer: String,
    fractional: String,
}

impl Number {
    fn new(mut number: f64) -> Self {
        let sign = number.is_sign_negative();
        number = number.abs();

        let number_stringify = number.to_string();
        let mut number_parts = number_stringify.split(".").collect::<Vec<&str>>();
        number_parts.push("0");

        Self {
            sign,
            integer: number_parts[0].to_string(),
            fractional: number_parts[1].to_string(),
        }
    }

    fn multiply_by_10(&self) -> Self {
        let mut integer_part = self.integer.chars().collect::<Vec<char>>();
        let mut fractional_part = self.fractional.chars().collect::<Vec<char>>();

        if fractional_part[0] != '0' {
            integer_part.push(fractional_part[0]);

            if integer_part[0] == '0' {
                integer_part.remove(0);
            }
        }
        fractional_part.remove(0);

        if fractional_part.len() == 0 {
            fractional_part.push('0');
        }

        Self {
            sign: self.sign,
            integer: integer_part
                .iter()
                .map(|chr| chr.to_string())
                .collect::<Vec<String>>()
                .join(""),
            fractional: fractional_part
                .iter()
                .map(|chr| chr.to_string())
                .collect::<Vec<String>>()
                .join(""),
        }
    }

    fn devide_by_10(&self) -> Self {
        let mut integer_part = self.integer.chars().collect::<Vec<char>>();
        let mut fractional_part = self.fractional.chars().collect::<Vec<char>>();

        if integer_part[integer_part.len() - 1] != '0' {
            fractional_part.insert(0, integer_part[integer_part.len() - 1]);
            if fractional_part[fractional_part.len() - 1] == '0' {
                fractional_part.pop();
            }
        }

        integer_part.pop();

        Self {
            sign: self.sign,
            integer: integer_part
                .iter()
                .map(|chr| chr.to_string())
                .collect::<Vec<String>>()
                .join(""),
            fractional: fractional_part
                .iter()
                .map(|chr| chr.to_string())
                .collect::<Vec<String>>()
                .join(""),
        }
    }

    fn display(&self, power: i64) -> String {
        match self.sign {
            false => match self.fractional == String::from("0") {
                false => format!("{}.{}e{}", self.integer, self.fractional, power),
                true => format!("{}e{}", self.integer, power),
            },
            true => match self.fractional == String::from("0") {
                false => format!("{}.{}e{}", self.integer, self.fractional, power),
                true => format!("{}e{}", self.integer, power),
            },
        }
    }
}

fn number_to_mantissa(number: f64) -> String {
    let mut power = 0_i64;
    let mut splited_number = Number::new(number);
    println!("{:#?}", splited_number);

    if splited_number.integer == String::from("0") {
        while splited_number.integer == String::from("0") {
            power -= 1;

            splited_number = Number::multiply_by_10(&splited_number);
        }
    } else {
        while splited_number.integer.len() != 1 {
            power += 1;

            splited_number = Number::devide_by_10(&splited_number);
            println!("{:#?}", splited_number);
        }
    }

    Number::display(&splited_number, power)
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number = line.trim().parse::<f64>().unwrap();

    println!("{}", number_to_mantissa(number));
}
