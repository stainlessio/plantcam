#![allow(dead_code)]
#![allow(unused_imports)]
extern crate camera_capture;
extern crate clap;
extern crate chrono;

use std::path::Path;
use std::fs::File;
use clap::{App, Arg};
use chrono::prelude::*;
use std::time;
use std::error::Error;


fn until_time(from_date: &DateTime<Local>) -> time::Duration {
    let now = Local::now();
    from_date.signed_duration_since(now).to_std().unwrap()
}

fn next_run_time(template: &DateTime<Local>) -> DateTime<Local> {
    let now = Local::now();
    let adjusted_time = template.with_year(now.year()).unwrap().with_month(now.month()).unwrap().with_day(now.day()).unwrap();
    if now < adjusted_time {
        adjusted_time
    } else {
        adjusted_time.checked_add_signed(chrono::Duration::days(1)).unwrap()
    }
}

fn template_from_str(input: &str) -> Result<DateTime<Local>, chrono::ParseError> {
    let mut full_str = String::with_capacity(16);
    full_str.push_str("20170202 ");
    full_str.push_str(input);
    Local.datetime_from_str(&full_str, "%Y%m%d %I:%M%P")
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("device")
            .short("d")
            .long("device")
            .value_name("DEVICE_NUM")
            .help("The webcam device to use")
            .takes_value(true)
            .multiple(false)
            .default_value("0")
            )
        .arg(Arg::with_name("time")
            .value_name("HH:MMAP")
            .help("The hour and minute to take the picture during the day")
            .required(true)
            )
        .get_matches();
    
    let template = template_from_str(matches.value_of("time").unwrap()).unwrap();
    let next_run = next_run_time(&template);
    let sleep_duration = until_time(&next_run);
    println!("{:?} => {:?} ({:?})", template, next_run, sleep_duration);
    
    // let expressive_duration = chrono::Duration::from_std(sleep_duration);
    // println!("{:?}", expressive_duration.unwrap().num_hours());
    // let cam = camera_capture::create(2).unwrap();
    // let mut cam = cam.fps(1.0).unwrap().resolution(1920, 1080).unwrap().start().unwrap();
    // let img = cam.next().unwrap();

    // let filename = "test.png";
    // let path = Path::new(&filename);
    // let _ = &mut File::create(&path).unwrap();
    // let _ = img.save(&path).unwrap();
}
