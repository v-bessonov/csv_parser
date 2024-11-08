use chrono::{Duration, NaiveDate, Utc};
use crate::{util, types::Visit, types::Record};

const PERIOD_OF_INTEREST : i64 = 180;
pub fn do_menu(visits : &mut Vec<Visit>) -> bool {
    let message = String::from(r"
Choose option:
1 Display all visit
2 Calculate visit days within period of interest
3 Add visit
4 Save and exit
5 Quit without saving
>>>");

    loop {
        let selection = util::prompt_for_string(&message);
        match selection.trim() {
            "1" => display_all_visits(visits),
            "2" => display_days_within_period_of_interest(visits),
            "3" => add_visit(visits),
            "4" => return true,
            "5" => return false,
            _ => println!("???")
        }
    }

}

fn display_days_within_period_of_interest(visits: &mut Vec<Visit>) {
    let message = String::from("What reference date do you want to use? [yyyy-mm-dd]:");
    let mut date = util::prompt_for_date(&message);

    let mut records : Vec<Record> = Vec::new();

    while date >= Utc::now().naive_utc().date() {
        let days = get_days_prior_to(visits, date);
        records.push(Record::new(date, days));
        date = date - Duration::days(1);
    }

    println!("Reverse chronological order:");
    for record in records.iter() {
        println!("{}", record);
    }

    println!("\nReverse number of days order:");
    records.sort();
    for record in records.iter() {
        println!("{}", record);
    }

}

fn get_days_prior_to(visits: &mut Vec<Visit>, window_end_date: NaiveDate) -> i64 {
    let window_start_date = window_end_date - Duration::days(PERIOD_OF_INTEREST);

    let total_days = visits.iter()
        .map(|v| {
            v.get_days_in_period(window_start_date, window_end_date)
        })
        .sum();
    total_days
}

fn add_visit(visits: &mut Vec<Visit>) {
    let message = String::from("Enter start-date,end-date,description [yyyy-mm-dd] format:");
    let visit = util::prompt_for_string(&message).trim().to_string();
    let visit = Visit::from_string(visit);

    visits.push(visit);
}

fn display_all_visits(visits: &mut Vec<Visit>) {
    println!("All visits:");
    for visit in visits {
        println!("{}", visit);
    }
}