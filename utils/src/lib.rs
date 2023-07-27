mod error;

use crate::error::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref EMPLOYEE_BIRTH: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("Louis", "0727"),
            ("Cindy", "0727"),
            ("Error1", "0717"),
            ("Error2", "0101"),
        ])
    };
    static ref EMPLOYEE_ADDTRESS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("Louis", "711"),
            ("Cindy", "722"),
            ("Error1", "211"),
            ("Error2", "985"),
        ])
    };
    static ref EMPLOYEE_EMAIL: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("Louis", "996@qq.com"),
            ("Cindy", "007@email.com"),
            ("Error1", "888@error.com"),
            ("Error2", "999@error.com"),
        ])
    };
}

#[derive(Debug)]
pub struct Stuff {
    name: String,
    address: Option<String>,
    email: Option<String>,
}

impl Stuff {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            address: None,
            email: None,
        }
    }
}

pub fn inform_stuff(stuff: &Stuff, content: &str) -> Result<()> {
    let content = content.replace("{name}", &stuff.name);
    println!(
        "Informed {}<{}>: {content}",
        stuff.name,
        stuff.email.as_ref().map_or("unknown", |email| email)
    );
    Ok(())
}

pub fn inform_sender(stuff_sender: &Stuff, stuff_recv: &Stuff) -> Result<()> {
    println!(
        "Told {} to send present to {} located {}",
        stuff_sender.name,
        stuff_recv.name,
        stuff_recv.address.as_ref().map_or("unknown", |addr| addr)
    );
    Ok(())
}

/// if today is January 1st, return 0101
pub fn query_date() -> Result<String> {
    use chrono::prelude::*;

    let dt = Local::now();
    Ok(format!("{:02}{:02}", dt.month(), dt.day()))
}

pub fn query_birth_stuff(birth: &str) -> Result<Vec<Stuff>> {
    Ok(EMPLOYEE_BIRTH
        .iter()
        .filter_map(|(&name, &date)| {
            if date == birth {
                Some(Stuff::new(name))
            } else {
                None
            }
        })
        .collect::<Vec<Stuff>>())
}

pub fn query_address_and_email(stuff: &mut Vec<Stuff>) -> Result<()> {
    stuff.iter_mut().for_each(|one| {
        let name = one.name.clone();
        one.email = EMPLOYEE_EMAIL
            .get(name.as_str())
            .and_then(|&email| Some(email.to_string()));
        one.address = EMPLOYEE_ADDTRESS
            .get(name.as_str())
            .and_then(|&address| Some(address.to_string()));
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inform_stuff_test() {
        let one = Stuff::new("Louis");
        assert!(inform_stuff(&one, "Happy new year, {name}!").is_ok());
    }

    #[test]
    fn inform_sender_test() {
        let one = Stuff::new("Louis");
        let sender = Stuff::new("Sender");
        assert!(inform_sender(&sender, &one).is_ok());
    }

    #[test]
    fn query_date_test() {
        assert!(query_date().is_ok());
    }

    #[test]
    fn query_birth_test() {
        assert!(query_birth_stuff(query_date().unwrap().as_ref()).is_ok())
    }

    #[test]
    fn query_address_and_email_test() {
        let mut stuff = query_birth_stuff(query_date().unwrap().as_ref()).unwrap();
        assert!(query_address_and_email(&mut stuff).is_ok());
    }

    #[test]
    fn workflow() {
        let date = query_date().unwrap();
        let mut stuff = query_birth_stuff(&date).unwrap();
        query_address_and_email(&mut stuff).unwrap();
        let sender = Stuff::new("Sender");
        for one in stuff.iter() {
            assert!(inform_stuff(one, "Happy birthday, {name}").is_ok());
            assert!(inform_sender(&sender, one).is_ok());
        }
    }
}
