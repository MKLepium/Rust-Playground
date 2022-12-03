
use chrono::{Datelike, NaiveDate};
/*
November 2021
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21
22 23 24 25 26 27 28
29 30

December 2021
       1  2  3  4  5
 6  7  8  9 10 11 12
13 14 15 16 17 18 19
20 21 22 23 24 25 26
27 28 29 30 31

*/

fn add_day(string: String, day: &str) -> String{
    let mut  to_append = String::from(string);
    if day.len() == 1 {
        to_append.push_str(" ");
    }
    to_append.push_str(&day);
    to_append.push_str(" ");
    to_append.to_string()
}

fn how_many_days(month: String, str_year: String) -> u32{
    if month == 1.to_string() || month == 3.to_string() || month == 5.to_string() 
    || month == 7.to_string() || month == 8.to_string() || month == 10.to_string() 
    || month == 12.to_string() {
        return 31;
    } else if month == 2.to_string(){
        let year = str_year.parse::<u32>().unwrap();
        if (year%100 != 0 && year as u32 % 4 == 0) || year%400 == 0 {
            return 29;
        } else {
            return 28;
        }
    } else {
        return 30;
    }
}

fn get_first_week_day(str_month: &String, str_year: &String) -> String{
    let month = str_month.parse::<u32>().unwrap();
    let year = str_year.parse::<i32>().unwrap();
    let d = NaiveDate::from_ymd(year, month, 1);
    let weekday = d.weekday();
    weekday.to_string()
}

fn build_string(str_month: String, str_year: String) -> String{
    let mut first_day = get_first_week_day(&str_month, &str_year);
    println!("Year: {} \n, Month: {}", str_year, str_month);
    let mut string = String::from("\n");


    /*
        Mon,
        Tue,
        Wed,
        Thu,
        Fri,
        Sat,
        Sun,
    */
    if first_day == "Sun"{
        string = add_day(string, " ");
        first_day = "Sat".to_string();
    } 
    if first_day == "Sat"{
        string = add_day(string, " ");
        first_day = "Fri".to_string();
    } 
    if first_day == "Fri"{
        string = add_day(string, " ");
        first_day = "Thu".to_string();
    } 
    if first_day == "Thu"{
        string = add_day(string, " ");
        first_day = "Wed".to_string();
    } 
    if first_day == "Wed"{
        string = add_day(string, " ");
        first_day = "Tue".to_string();
    } 
    if first_day == "Tue"{
        string = add_day(string, " ");
    }
    let days_this_month = how_many_days(str_month, str_year);

    for day in 1..days_this_month +1 {
        string = add_day(string, &day.to_string());
        let to_be_checked = string.trim_start_matches("\n");
        if (to_be_checked.len() - 1)  / 3 == 6 
        || (to_be_checked.len() - 1)  / 3 == 14 
        || (to_be_checked.len() - 1)  / 3 == 21
        || (to_be_checked.len() - 1)  / 3 == 28
        || (to_be_checked.len() - 1)  / 3 == 36
        {
            string.push_str("\n");
        }
    }

    string
}


fn main(){
    let start_month = 10;
    let start_year = 2021;
    for date in     (0..24).into_iter().map(|num| {
        let mut current_month = start_month;
        let mut current_year = start_year;
        current_month += num;
        while current_month > 12 {
            current_month -= 12;
            current_year += 1;
            
        }
        let str = build_string((current_month).to_string(), (current_year).to_string());
        return str;
    })
    {
        println!("{}", date);
    }
}

