extern crate clap;
extern crate chrono;

use clap::{App};
use indicatif::{ProgressBar};
use chrono::{Local, Date, NaiveDate, Datelike};

fn main() {
    App::new("Year Progress")
        .version("0.0.1")
        .author("Travis S. <t@travisshears.com>")
        .about("CLI tool to print out progress bar of the year to date")
        .get_matches();

    // caculate date delta since start of the year
    let now_local: Date<Local> = Local::today();
    let now_utc =  now_local.naive_utc();
    let current_year: i32  = now_utc.year();
    let start_of_year = NaiveDate::from_ymd(current_year, 1, 1);
    let delta_days = NaiveDate::signed_duration_since(now_utc, start_of_year).num_days();
    let delta_days_i64 = delta_days as u64;

    // print progress bar
    let bar = ProgressBar::new(365);
    bar.inc(delta_days_i64);
    bar.finish_at_current_pos()
}

