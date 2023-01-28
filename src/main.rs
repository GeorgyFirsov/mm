//! # mm
//! 
//! `mm` is a simple command line program, that stores your notes on your computer. 
//! Internally it uses `git` to track all the changes step-by-step and allow a user 
//! to get back to any state.


mod editor;

extern crate libmm;
extern crate clap;

use libmm::error;


fn dump_error<E: std::error::Error>(err: E) {
    eprint!("An error occurred during execution: {}", err);
}


fn run() -> error::Result<()> {
    //
    // TODO: running everything here
    //
    
    Ok(())
}


fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => dump_error(err)
    }
}
