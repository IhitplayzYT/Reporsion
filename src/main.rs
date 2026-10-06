use std::path::PathBuf;

use crate::helper::Helper::spread;

mod helper;
fn main() {
    let clargs = std::env::args().skip(1).collect::<Vec<String>>();
    let sandbox = if clargs.len() != 0{
        PathBuf::from(&clargs[0])
    }  else{
        std::env::current_dir().unwrap()
    };
    spread(std::env::current_dir().unwrap(), sandbox).unwrap();
}
