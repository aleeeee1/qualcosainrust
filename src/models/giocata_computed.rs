use crate::Giocata;

pub struct GiocataComputed {
    giocata: *mut Giocata,
    estrazione: Vec<i32>,
    indovinate: i32,
    vincita: i32,
}

impl GiocataComputed {
    pub fn new(giocata: &mut Giocata, estrazione: Vec<i32>) -> GiocataComputed {
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

    pub fn get_vincita(&self) -> i32 {
        return self.vincita;
    }

    pub fn get_puntata(&self) -> i32 {
        unsafe {
            return (*self.giocata).get_puntata();
        }
    }

    fn calcola_indovinate(&mut self) {
        unsafe {
            let numeri_giocati = (*self.giocata).get_numeri_giocati();
            for numero in numeri_giocati {
                if self.estrazione.contains(&numero) {
                    self.indovinate += 1;
                }
            }
        }
    }

    fn calcola_vincita(&mut self) {
        unsafe {
            let puntata = (*self.giocata).get_puntata();
            let mut vincita = 0;

            if puntata == 1 && self.indovinate == 5 {
                vincita = 5;
            } else if puntata == 2 {
                if self.indovinate == 5 {
                    vincita = 10;
                } else if self.indovinate == 4 {
                    vincita = 5;
                }
            }
            self.vincita = vincita;
        }
    }
}
