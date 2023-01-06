//! # mm
//! 
//! `mm` is a simple command line program, that stores your notes on your computer. 
//! Internally it uses `git` to track all the changes step-by-step and allow a user 
//! to get back to any state.


mod repo;
mod error;
mod data;
mod misc;

extern crate clap;
extern crate git2;
extern crate dirs;


fn main() {
    let r = repo::open_repo(None)
        .unwrap();

    println!("{}", r.workdir().unwrap().to_str().unwrap());

    for i in r.index().unwrap().iter() {
        println!("{}", String::from_utf8(i.path).unwrap());
    }
}
