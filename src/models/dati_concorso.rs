#![allow(dead_code)]

use crate::utils::utils::*;

pub struct DatiConcorso {
    numero_concorso: i32,
    numero_matrice: i32,
}

impl DatiConcorso {
    pub fn load_from_file() -> Result<DatiConcorso, String> {
        let contents = read_file("dati_concorso.txt");
        let mut splitted = contents.split(";");

        Ok(DatiConcorso {
            numero_concorso: splitted.next().unwrap().parse::<i32>().unwrap(),
            numero_matrice: splitted.next().unwrap().parse::<i32>().unwrap(),
        })
    }

    pub fn new(numero_concorso: i32, numero_matrice: i32) -> DatiConcorso {
        DatiConcorso {
            numero_concorso,
            numero_matrice,
        }
    }

    pub fn save_to_file(&self) -> Result<(), String> {
        let contents = format!("{};{}", self.numero_concorso, self.numero_matrice);
        write_file("dati_concorso.txt", &contents);
        Ok(())
    }

    pub fn increment_numero_concorso(&mut self) {
        self.numero_concorso += 1;
    }

    pub fn increment_numero_matrice(&mut self) {
        self.numero_matrice += 1;
    }

    pub fn get_numero_concorso(&self) -> i32 {
        self.numero_concorso
    }

    pub fn get_numero_matrice(&self) -> i32 {
        self.numero_matrice
    }

    pub fn set_numero_concorso(&mut self, numero_concorso: i32) {
        self.numero_concorso = numero_concorso;
    }

    pub fn set_numero_matrice(&mut self, numero_matrice: i32) {
        self.numero_matrice = numero_matrice;
    }
}
