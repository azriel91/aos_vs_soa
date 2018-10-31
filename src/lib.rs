#![feature(test)]

extern crate chrono;
extern crate rand;
extern crate test;

use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
enum Country {
    Nz,
    Oz,
}

#[derive(Debug)]
enum State {
    Hobbiton,
}

#[derive(Debug)]
struct Address {
    country: Country,
    state: Option<State>,
    city: String,
}

#[derive(Debug)]
struct PhoneNumber {
    country_code: u32,
    number: String,
}

#[derive(Debug)]
struct ContactDetails {
    address: Address,
    phone_number: PhoneNumber,
    data: Vec<u32>,
}

#[derive(Debug)]
struct Person {
    contact_details: ContactDetails,
    dob: DateTime<Utc>,
}

#[allow(dead_code)]
struct World {
    contact_details: Vec<ContactDetails>,
    dobs: Vec<DateTime<Utc>>,
}

fn random_contact_details() -> ContactDetails {
    let country = if rand::random::<u8>() < 128 {
        Country::Nz
    } else {
        Country::Oz
    };
    let state = if rand::random::<u8>() < 128 {
        None
    } else {
        Some(State::Hobbiton)
    };
    let city = "city".to_string();
    let address = Address {
        country,
        state,
        city,
    };
    let phone_number = PhoneNumber {
        country_code: 64,
        number: "123".to_string(),
    };
    ContactDetails {
        address,
        phone_number,
        data: Vec::with_capacity(150_000),
    }
}

fn random_dob() -> DateTime<Utc> {
    Utc.ymd(1900 + rand::random::<u8>() as i32 >> 1, 10, 31)
        .and_hms(12, 0, 9)
}

fn random_person() -> Person {
    let contact_details = random_contact_details();
    let dob = random_dob();
    Person {
        contact_details,
        dob,
    }
}

#[cfg(test)]
mod tests {
    const SAMPLE_SIZE: usize = 100_000;

    mod array_of_structs {
        use chrono::Utc;
        use crate::{random_person, test::Bencher};

        use super::SAMPLE_SIZE;

        #[bench]
        fn avg_age(b: &mut Bencher) {
            let people = (0..SAMPLE_SIZE)
                .map(|_| random_person())
                .collect::<Vec<_>>();

            let now = Utc::now();

            b.iter(|| {
                people
                    .iter()
                    .fold(0., |sum, p| sum + ((now - p.dob).num_weeks() as f32 / 52.))
                    / SAMPLE_SIZE as f32
            });
        }
    }

    mod struct_of_arrays {
        use chrono::Utc;

        use crate::{random_contact_details, random_dob, test::Bencher, World};

        use super::SAMPLE_SIZE;

        #[bench]
        fn avg_age(b: &mut Bencher) {
            let contact_details = (0..SAMPLE_SIZE)
                .map(|_| random_contact_details())
                .collect::<Vec<_>>();
            let dobs = (0..SAMPLE_SIZE).map(|_| random_dob()).collect::<Vec<_>>();

            let world = World {
                contact_details,
                dobs,
            };

            let now = Utc::now();
            b.iter(|| {
                world
                    .dobs
                    .iter()
                    .fold(0., |sum, dob| sum + ((now - *dob).num_weeks() as f32 / 52.))
                    / SAMPLE_SIZE as f32
            });
        }
    }
}
