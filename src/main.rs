extern crate libc;
extern crate clap;
extern crate chrono;

use std::fs::{metadata, Metadata};
use std::ffi::CString;
use std::time::{UNIX_EPOCH, SystemTime};

use clap::{Arg, App};
use libc::{time_t, utime, utimbuf};
use chrono::prelude::{DateTime, Utc, TimeZone};

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

    let input_file: &str = matches.value_of("INPUT").unwrap();
    let meta: Metadata   = metadata(input_file).unwrap();

    let modtime = meta.modified().unwrap();
    let actime  = meta.accessed().unwrap();

    let modsec = get_seconds(&modtime);
    let acsec  = get_seconds(&actime);

    let modread = system_time_to_date_time(modtime);
    let acread  = system_time_to_date_time(actime);

    let mut new_time: utimbuf = utimbuf { modtime: modsec, actime: acsec };

    let mut any_modified = false;

    if let Some(m) = matches.value_of("modified") {
        if let Ok(time) = m.parse::<time_t>() {
            new_time.modtime = time;
            any_modified = true;
        }
    }

    if let Some(a) = matches.value_of("accessed") {
        if let Ok(time) = a.parse::<time_t>() {
            new_time.actime = time;
            any_modified = true;
        }
    }

    if !any_modified {
        println!("'{0}' was last accessed at {1}, or {2} seconds since epoch.", input_file, acread.to_rfc2822(), acsec);
        println!("'{0}' was last modified at {1}, or {2} seconds since epoch.", input_file, modread.to_rfc2822(), modsec);
    }

    unsafe {
        utime(CString::new(input_file).unwrap().as_ptr(), &mut new_time);
    }
}

fn get_seconds(time: &SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Utc.timestamp(sec, nsec)
}
