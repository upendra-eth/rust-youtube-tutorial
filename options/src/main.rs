struct AadhaarCard {
    name: String,
    gender: Gender,
    date_of_birth: String,
    aadhaar_number: String,
    address: String,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Other,
}

impl AadhaarCard {
    fn display_details(&self) {
        println!("Name: {}", self.name);
        println!("Gender: {:?}", self.gender);
        println!("Date of Birth: {}", self.date_of_birth);
        println!("Aadhaar Number: {}", self.aadhaar_number);
        println!("Address: {}", self.address);
    }

    fn is_male(&self) -> bool {
        if self.gender == Gender::Male {
              return true
        } else {
            return false
        }
    }

    fn validate_aadhaar_number(&self) -> bool {
        // Add your validation logic here
        true
    }
}

fn main() {
    let card = AadhaarCard {
        name: "John Doe".to_string(),
        gender: Gender::Female,
        date_of_birth: "1990-01-01".to_string(),
        aadhaar_number: "1234-5678-9012".to_string(),
        address: "123, ABC Street".to_string(),
    };

    let my_gender = Gender::Male;


    // card.display_details();

    if card.is_male() {
        println!("The card belongs to a male cardholder.");
    } else {
        println!("The card does not belong to a male cardholder.");
    }

    // if card.validate_aadhaar_number() {
    //     println!("The Aadhaar number is valid.");
    // } else {
    //     println!("The Aadhaar number is invalid.");
    // }
}
