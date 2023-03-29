use crate::{utils::utils::append_to_file, Giocata};

pub struct GiocataComputed {
    giocata: Giocata,
    estrazione: Vec<i32>,
    indovinate: i32,
    vincita: i32,
}

impl GiocataComputed {
    pub fn new(giocata: Giocata, estrazione: Vec<i32>) -> GiocataComputed {
        let mut giocata_computed = GiocataComputed {
            giocata,
            estrazione,
            indovinate: 0,
            vincita: 0,
        };

        giocata_computed.calcola_indovinate();
        giocata_computed.calcola_vincita();

        return giocata_computed;
    }

    pub fn to_string(&self) -> String {
        let estrazioni_string = self
            .estrazione
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

        let stringa = format!(
            "{};{};{};{}",
            self.giocata.to_string(),
            estrazioni_string,
            self.indovinate,
            self.vincita
        );

        return stringa;
    }

    pub fn update_file(&self) {
        let stringa = self.to_string() + "\n";
        append_to_file("storico_concorso.txt", &stringa);
    }

    pub fn get_vincita(&self) -> i32 {
        return self.vincita;
    }

    pub fn get_puntata(&self) -> i32 {
        return self.giocata.get_puntata();
    }

    fn calcola_indovinate(&mut self) {
        let numeri_giocati = self.giocata.get_numeri_giocati();
        for numero in numeri_giocati {
            if self.estrazione.contains(&numero) {
                self.indovinate += 1;
            }
        }
    }

    fn calcola_vincita(&mut self) {
        let puntata = self.giocata.get_puntata();
        let mut vincita = 0;

        if puntata == 1 && self.indovinate == 5 {
            vincita = 5;
        } else if puntata == 2 {
            if self.indovinate == 5 {
                vincita = 10;
            } else if self.indovinate == 4 {
                vincita = 5;
            }
        };

        self.vincita = vincita;
    }
}
