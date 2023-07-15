use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_card() {
        let valid_credit_card_number = String::from("79927398713");
        assert!(validate_card(valid_credit_card_number));
    }
}

fn main() {
    let mut credit_card_number = String::new();

    println!("Enter the credit card number: ");
    io::stdin()
        .read_line(&mut credit_card_number)
        .expect("Error");

    let valid = validate_card(credit_card_number);

    if valid {
        println!("This credit card is valid!");
    } else {
        println!("This credit card isn't valid!");
    }
}

fn validate_card(credit_card_number: String) -> bool {
    let mut second = false;
    let mut sum = 0;

    let chars = credit_card_number.trim().chars();

    for character in chars {
        let mut temp = character.to_string().parse::<u32>().unwrap();
        if second {
            temp *= 2;
        }

        sum += temp / 10;
        sum += temp % 10;

        second = !second
    }

    sum % 10 == 0
}
