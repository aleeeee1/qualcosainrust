use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::Giocata;

pub fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn write_file(filename: &str, contents: &str) {
    let mut file = File::create(filename).expect("file not found");

    file.write_all(contents.as_bytes())
        .expect("something went wrong writing the file");
}

pub fn append_to_file(filename: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .expect("file not found");

    file.write_all(contents.as_bytes())
        .expect("something went wrong writing the file");
}

pub fn get_giocate() -> Vec<Giocata> {
    let content = read_file("righe_concorso.txt");
    let linee = content.split("\n");

    let mut giocate: Vec<Giocata> = Vec::new();

    for linea in linee {
        let giocata = Giocata::from_line(linea.to_string());
        giocate.push(giocata);
    }

    return giocate;
}
