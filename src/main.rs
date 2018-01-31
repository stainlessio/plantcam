#![feature(conservative_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate camera_capture;
extern crate clap;
extern crate chrono;
extern crate tokio_timer;
extern crate futures;
extern crate image;

use std::path::Path;
use std::fs::File;
use clap::{App, Arg};
use chrono::prelude::*;
use std::time;
use std::error::Error;
use tokio_timer::Timer;
use futures::{Future, future};
use futures::future::FutureResult;
use camera_capture::ImageIterator;

type Image = image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;

// struct ImageIterator {

// }

// impl ImageIterator {
//     fn next(&mut self) -> Option<Image> {
//         None
//     }
// }

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
    
    let cam_device_number = matches.value_of("device").unwrap().parse::<u32>().unwrap();
    let template = template_from_str(matches.value_of("time").unwrap()).unwrap();
    let next_run = next_run_time(&template);
    let sleep_duration = until_time(&next_run);
    println!("Sleep Duration = {:?}", sleep_duration);

    let timer = Timer::default();

    let wait_on_picture_time = timer.sleep(sleep_duration);
    
    fn camera_start(device: u32) -> impl Future<Item=ImageIterator, Error=()> {
        // future::ok(ImageIterator {})
        future::ok(camera_capture::create(device).unwrap()
            .fps(1.0).unwrap()
            .resolution(1920, 1080).unwrap()
            .start().unwrap())
    }
    fn take_picture(cam: Result<ImageIterator, ()>) -> impl Future<Item=Option<Image>, Error=()> {
        future::ok(cam.unwrap().next())
    }
    let camera_warmup = timer.sleep(time::Duration::from_secs(10)); 
    let capture_chain = wait_on_picture_time
        .then(|_| camera_start(cam_device_number))
        .then(|res| {
            let _ = camera_warmup.wait();
            res
        })
        ;

    match capture_chain.wait() {
        Err(e) => println!("Failed! {:?}", e),
        _ => println!("Chain Done")
        
    };

    // let img = cam.next().unwrap();

    // let filename = "test.png";
    // let path = Path::new(&filename);
    // let _ = &mut File::create(&path).unwrap();
    // let _ = img.save(&path).unwrap();
}
