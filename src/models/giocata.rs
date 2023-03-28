use crate::utils::utils::*;
use crate::DatiConcorso;

pub struct Giocata {
    dati_concorso: *mut DatiConcorso,
    puntata: i32,
    numeri_giocati: Vec<i32>,
}

impl Giocata {
    pub fn new(
        dati_concorso: &mut DatiConcorso,
        puntata: i32,
        numeri_giocati: Vec<i32>,
    ) -> Giocata {
        let giocata = Giocata {
            dati_concorso,
            puntata: puntata,
            numeri_giocati: numeri_giocati,
        };

        return giocata;
    }

    pub fn from_line(line: String) -> Giocata {
        let mut infos = line.split(";");
        let mut dati_concorso = DatiConcorso::new(
            infos.next().unwrap().parse().unwrap(),
            infos.next().unwrap().parse().unwrap(),
        );

        let giocata = Giocata {
            dati_concorso: &mut dati_concorso,
            puntata: infos.next().unwrap().parse().unwrap(),
            numeri_giocati: infos
                .next()
                .unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect(),
        };

        return giocata;
    }

    pub fn effettua_giocata(&self) {
        let numeri_giocati_string = self
            .numeri_giocati
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

        let puntata_string = self.puntata.to_string();

        unsafe {
            let stringa = format!(
                "{};{};{};{}\n",
                (*self.dati_concorso).get_numero_concorso().to_string(),
                (*self.dati_concorso).get_numero_matrice().to_string(),
                puntata_string,
                numeri_giocati_string,
            );

            append_to_file("righe_concorso.txt", &stringa);

            (*self.dati_concorso).increment_numero_matrice();
            (*self.dati_concorso)
                .save_to_file()
                .expect("Qualcosa è andato storto");
        };
    }

    pub fn get_puntata(&self) -> i32 {
        return self.puntata;
    }

    pub fn get_numeri_giocati(&self) -> Vec<i32> {
        return self.numeri_giocati.clone();
    }
}
