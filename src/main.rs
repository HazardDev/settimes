extern crate libc;
extern crate clap;

use libc::{time_t, utime, utimbuf};
use std::fs::{metadata, Metadata};
use clap::{Arg, App};
use std::ffi::CString;
use std::time::{UNIX_EPOCH, SystemTime};

fn main() {

    let matches = App::new("SetTimes")
                    .version("v1.0.0")
                    .author("Logan Saso <logansaso@gmail.com>")
                    .about("Sets system modified or accessed times.")
                    .arg(Arg::with_name("modified")
                        .short("m")
                        .long("modified")
                        .value_name("MODIFIED")
                        .help("Time in seconds since EPOCH to set as file last modified.")
                        .takes_value(true))
                    .arg(Arg::with_name("accessed")
                        .short("a")
                        .long("accessed")
                        .value_name("ACCESSED")
                        .help("Time in seconds since EPOCH to set as file last accessed."))
                    .arg(Arg::with_name("INPUT")
                        .help("Sets the file to modifiy.")
                        .required(true))
                    .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let meta: Metadata = metadata(input_file).unwrap();

    let modtime = meta.modified().unwrap();
    let actime = meta.accessed().unwrap();

    let modsec = get_seconds(&modtime);
    let acsec = get_seconds(&actime);

    let mut new_time: utimbuf = utimbuf { modtime: modsec, actime: acsec };

    let mut any_modified = false;

    if let Some(m) = matches.value_of("modified") {
        if let Some(time) = get_time(m) {
            new_time.modtime = time;
            any_modified = true;
        }
    }

    if let Some(a) = matches.value_of("accessed") {
        if let Some(time) = get_time(a) {
            new_time.actime = time;
            any_modified = true;
        }
    }

    if !any_modified {
        //TODO: Get SystemTime to print in human readable time
        println!("{} was last accessed at {:?}, or {} seconds since epoch.", input_file, actime, acsec);
        println!("{} was last modified at {:?}, or {} seconds since epoch.", input_file, modtime, modsec);
    }

    unsafe {
        utime(CString::new(input_file).unwrap().as_ptr(), &mut new_time);
    }
}

fn get_time(time_string: &str) -> Option<time_t> {
    //There's a shorthand for this but I can't remember it
    match time_string.parse::<time_t>() {
        Ok(value) => {
            Some(value)
        },
        _ => {
            None
        }
    }
}

fn get_seconds(time: &SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
