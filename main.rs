#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
#![feature(proc_macro_hygiene, decl_macro)]
#[warn(unused_parens)]
#[macro_use]
extern crate rocket;
use rocket::http::RawStr;
use rocket::request::{Form, FormDataError, FormError};
use rocket::response::NamedFile;
mod Bengoli;
mod English;
mod Gujarati;
mod Hindi;
mod Marathi;
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap

/*fn main() {
   // println!("............Currency_To_Text.............");
    //println!("Enter 1 for English conversion\nEnter 2 for Marathi conversion\nEnter 3 for Hindi conversion\nEnter 4 for Gujarati conversion\nEnter 5 for Bengoli conversion  ");
    //let mut loopval=String::new();
    //io::stdin().read_line(&mut loopval).expect(("Fail to read line"));
    //let loopvalcopy:i32=loopval.trim().parse().ok().expect("Program can only process numbers");

    let mut Choice = String::new();
    io::stdin()
        .read_line(&mut Choice)
        .expect("Fail to read Line");
    println!("Enter your Currency:");
    let choiceagain: i32 = Choice
        .trim()
        .parse()
        .ok()
        .expect("program can only process numbers");
    let mut currency = String::new();
    let mut Complete_string = " ".to_string();
    io::stdin()
        .read_line(&mut currency)
        .expect("Fail to read Line");
    let float_currency: f64 = currency
        .trim()
        .parse()
        .ok()
        .expect("program can only process numbers");
    let int_currency = float_currency as u32;
    let _diff: f64 = float_currency - f64::from(int_currency);
    let _fn: f64 = _diff * f64::from(100);
    //finding float value
    let _int_fn = _fn.round(); //round function
    let _intval = _int_fn as u32;
    if _intval != 0 {
        //decimal Number addition to the string
        let mut _decimalNo_pow = u32::pow(10, 2);
        //str2 = [str2,_decimalNo.to_string(),"/".to_string(), _decimalNo_pow.to_string()].join(" ");
        Complete_string = [
            _intval.to_string(),
            "/".to_string(),
            _decimalNo_pow.to_string(),
        ]
        .join(" "); //Decimal number printing
    }
    //println!("{}",Complete_string);
match choiceagain {
        1 => println!("{}", English::EnglishWords(int_currency, &Complete_string)),
        2 => println!("{}", Marathi::MarathiWords(int_currency, &Complete_string)),
        3 => println!("{}", Hindi::HindiWords(int_currency, &Complete_string)),
        4 => println!("{}",Gujarati::GujaratiWords(int_currency, &Complete_string)),
        5 => println!("{}", Bengoli::BengoliWords(int_currency, &Complete_string)),
        _ => println!("Wrong choice:"),
    }
}*/

#[derive(Debug, FromFormValue)]
enum FormOption {
    English,
    Marathi,
    Hindi,
    Gujarati,
    Bengoli,
}

#[derive(Debug, FromForm)]

struct FormInput {
    //float_currency: f64,
    Currency: f64, // #[form(field = "type")]
    //radio: FormOption,
    //password: &'r RawStr,
    //#[form(field = "textarea")]
    //text_area: String,
    Language: FormOption,
}
#[warn(bindings_with_variant_name)]
#[post("/", data = "<Amount>", rank = 2)]
fn Amount(Amount: Form<FormInput>) -> String {
    let mut Complete_string = String::new();
    let int_currency = Amount.Currency as u32;
    let _diff: f64 = Amount.Currency - f64::from(int_currency);
    let _fn: f64 = _diff * f64::from(100);
    //finding float value
    let _int_fn = _fn.round(); //round function
    let _intval = _int_fn as u32;
    if _intval != 0 {
        //decimal Number addition to the string
        let mut _decimalNo_pow = u32::pow(10, 2);
        //str2 = [str2,_decimalNo.to_string(),"/".to_string(), _decimalNo_pow.to_string()].join(" ");
        Complete_string = [
            _intval.to_string(),
            "/".to_string(),
            _decimalNo_pow.to_string(),
        ]
        .join(""); //Decimal number printing
    }
    /* match sink {
        Ok(form) => format!("{:?}", &*form),
        Err(FormDataError::Io(_)) => format!("Form input was invalid UTF-8."),
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            format!("Invalid form input: {}", f)
        }
    }*/
    match Amount.Language {
        FormOption::English => format!("{}", English::EnglishWords(int_currency, Complete_string)),
        FormOption::Marathi => format!("{}", Marathi::MarathiWords(int_currency, Complete_string)),
        FormOption::Hindi => format!("{}", Hindi::HindiWords(int_currency, Complete_string)),
        FormOption::Gujarati => {
            format!("{}", Gujarati::GujaratiWords(int_currency, Complete_string))
        }
        FormOption::Bengoli => format!("{}", Bengoli::BengoliWords(int_currency, Complete_string)),
    }
    //format!("{}", English::EnglishWords(int_currency, &Complete_string))
}
#[get("/", rank = 1)]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, Amount])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(English::EnglishWords(1, " ".to_string()), "  One  ")
    }
    #[test]
    fn test2() {
        assert_eq!(
            English::EnglishWords(12, "23 / 100".to_string()),
            "    Twelve 23 / 100"
        )
    }
    #[test]
    fn test3() {
        assert_eq!(
            English::EnglishWords(152, "88 / 100".to_string()),
            "  One Hundred Fifty Two 88 / 100"
        )
    }
    #[test]
    fn test4() {
        assert_eq!(
            English::EnglishWords(1000, "23 / 100".to_string()),
            "  One Thousand 23 / 100"
        )
    }
    #[test]
    fn test5() {
        assert_eq!(
            English::EnglishWords(1234587, "16 / 100".to_string()),
            "    Twelve Lakh Thirty Four Thousand Five Hundred Eighty Seven 16 / 100"
        )
    }
    #[test]
    fn test6() {
        assert_eq!(
            English::EnglishWords(100000002, "29 / 100".to_string()),
            "    Ten Crore Two 29 / 100"
        )
    }
    #[test]
    fn test7() {
        assert_eq!(Marathi::MarathiWords(1, " ".to_string()), "  एक   ")
    }
    #[test]
    fn test8() {
        assert_eq!(
            Marathi::MarathiWords(12, "23 / 100".to_string()),
            "  बारा  23 / 100"
        )
    }
    #[test]
    fn test9() {
        assert_eq!(
            Marathi::MarathiWords(152, "88 / 100".to_string()),
            "  एक  शे बावन्न  88 / 100"
        )
    }
    #[test]
    fn test10() {
        assert_eq!(
            Marathi::MarathiWords(1000, "23 / 100".to_string()),
            "  एक  हजार 23 / 100"
        )
    }
    #[test]
    fn test11() {
        assert_eq!(
            Marathi::MarathiWords(1234587, "16 / 100".to_string()),
            "  बारा  लाख चौतीस  हजार पाच  शे सत्त्याऐंशी  16 / 100"
        )
    }
    #[test]
    fn test12() {
        assert_eq!(
            Marathi::MarathiWords(100000002, "29 / 100".to_string()),
            "  दहा  करोड दोन  29 / 100"
        )
    }

    //hindi testing
    #[test]
    fn test13() {
        assert_eq!(Hindi::HindiWords(1, " ".to_string()), "  एक   ")
    }
    #[test]
    fn test14() {
        assert_eq!(
            Hindi::HindiWords(12, "23 / 100".to_string()),
            "  बारह  23 / 100"
        )
    }
    #[test]
    fn test15() {
        assert_eq!(
            Hindi::HindiWords(152, "88 / 100".to_string()),
            "  एक  सौ बावन  88 / 100"
        )
    }
    #[test]
    fn test16() {
        assert_eq!(
            Hindi::HindiWords(1000, "23 / 100".to_string()),
            "  एक  हज़ार 23 / 100"
        )
    }
    #[test]
    fn test17() {
        assert_eq!(
            Hindi::HindiWords(1234587, "16 / 100".to_string()),
            "  बारह  लाख चौंतीस  हज़ार पांच  सौ सतासी  16 / 100"
        )
    }
    #[test]
    fn test18() {
        assert_eq!(
            Hindi::HindiWords(100000002, "29 / 100".to_string()),
            "  दस  करोड़ दो  29 / 100"
        )
    }

    //Gujarati testing
    #[test]
    fn test19() {
        assert_eq!(Gujarati::GujaratiWords(1, " ".to_string()), "  એક   ")
    }
    #[test]
    fn test20() {
        assert_eq!(
            Gujarati::GujaratiWords(12, "23 / 100".to_string()),
            "  બાર  23 / 100"
        )
    }
    #[test]
    fn test21() {
        assert_eq!(
            Gujarati::GujaratiWords(152, "88 / 100".to_string()),
            "  એક  સો બાવન  88 / 100"
        )
    }
    #[test]
    fn test22() {
        assert_eq!(
            Gujarati::GujaratiWords(1000, "23 / 100".to_string()),
            "  એક  હજાર 23 / 100"
        )
    }
    #[test]
    fn test23() {
        assert_eq!(
            Gujarati::GujaratiWords(1234587, "16 / 100".to_string()),
            "  બાર  લાખ ચોત્રીસ  હજાર પાંચ  સો સિત્યાસી  16 / 100"
        )
    }
    #[test]
    fn test24() {
        assert_eq!(
            Gujarati::GujaratiWords(100000002, "29 / 100".to_string()),
            "  દસ  કરોડ઼ બે   29 / 100"
        )
    }

    //Bengolitesting
    #[test]
    fn test25() {
        assert_eq!(Bengoli::BengoliWords(1, " ".to_string()), "  এক   ")
    }
    #[test]
    fn test26() {
        assert_eq!(
            Bengoli::BengoliWords(12, "23 / 100".to_string()),
            "  বারো  23 / 100"
        )
    }
    #[test]
    fn test27() {
        assert_eq!(
            Bengoli::BengoliWords(152, "88 / 100".to_string()),
            "  এক  শো বাহান্নো  88 / 100"
        )
    }
    #[test]
    fn test28() {
        assert_eq!(
            Bengoli::BengoliWords(1000, "23 / 100".to_string()),
            "  এক  হাজার  23 / 100"
        )
    }
    #[test]
    fn test29() {
        assert_eq!(
            Bengoli::BengoliWords(1234587, "16 / 100".to_string()),
            "  বারো  লাখ চৌত্রিশ  হাজার  পাঁচ  শো সাতাশি  16 / 100"
        )
    }
    #[test]
    fn test30() {
        assert_eq!(
            Bengoli::BengoliWords(100000002, "29 / 100".to_string()),
            "  দশ  কোটি  দুই  29 / 100"
        )
    }
}
