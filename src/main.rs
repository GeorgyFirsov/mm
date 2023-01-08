//! # mm
//! 
//! `mm` is a simple command line program, that stores your notes on your computer. 
//! Internally it uses `git` to track all the changes step-by-step and allow a user 
//! to get back to any state.


mod repo;
mod error;
mod data;
mod misc;
mod editor;


extern crate clap;
extern crate git2;
extern crate dirs;


fn main() {
    let r = repo::Repository::open_or_create(None)
        .unwrap();

    println!("{:?}", r.add_note("some_note", None).expect("fail").as_os_str());

    //r.commit_changes().unwrap();
}
