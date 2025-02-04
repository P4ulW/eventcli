use hdf5::{File, H5Type, Result};
use ndarray::s;
use std::env;

fn file_hdf_open_read(filename: &str) -> File {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error opening the file \"{filename}\", Details: {error:?}"),
    };
    return file;
}

fn info_display(filename: &String) -> () {
    let file = file_hdf_open_read(filename);
    let dataset_names = file.member_names().expect("Non-empty file");
    for dataset_name in dataset_names.iter() {
        println!("{dataset_name:?}");
    }
    return ();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //dbg!(&args);

    let instruction = &args[1];
    if instruction == "-info" {
        assert!(args.len() > 2, "missing filename argument!");
        println!("giving info");
        let filename = &args[2];
        info_display(&filename);
    }

    //let filename = "../eventlist_testing_diode_chessy_12_100_022494_1.h5";
    //let file = file_hdf_open_read(filename);
    //println!("opened file {file:?}");
    //let streamx = file.dataset("Stream_0").expect("valid dataset");
    //let streamy = file.dataset("Stream_1").expect("valid dataset");
    //let streamt = file.dataset("Stream_2").expect("valid dataset");
    //
    //let shape = streamx.shape();
    //println!("{shape:?}");
    //
    //let data = streamx.read_1d::<i32>().expect("data");
    ////let data = data.iter().map(|x| x * 2).collect::<Vec<i32>>();
    ////println!("{data:?}");
}
