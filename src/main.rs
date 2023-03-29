mod models;
mod utils;

use models::{dati_concorso::DatiConcorso, giocata::Giocata, giocata_computed::GiocataComputed};
use rand::{rngs::ThreadRng, Rng};
use utils::utils::get_giocate;

use crate::utils::utils::write_file;

static mut MOST_FREQ: Vec<i32> = Vec::new();

fn generate_random_numbers(mut random_coso: ThreadRng) -> Vec<i32> {
    let mut numeri_giocati: Vec<i32> = Vec::new();

    for _ in 0..5 {
        let mut numero = random_coso.gen_range(0..10);
        while numeri_giocati.contains(&numero) {
            numero = random_coso.gen_range(0..10);
        }
        numeri_giocati.push(numero);
    }

    return numeri_giocati;
}

fn generate_extration() -> Vec<i32> {
    let mut estrazione: Vec<i32> = vec![0; 10];

    for giocata in get_giocate() {
        let cifre_giocate = giocata.get_numeri_giocati();

        for cifra in cifre_giocate {
            estrazione[cifra as usize] += 1;
        }
    }

    let mut estrazione: Vec<(i32, i32)> = estrazione
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as i32, v))
        .collect();

    estrazione.sort_by(|a, b| a.1.cmp(&b.1));

    unsafe {
        MOST_FREQ = estrazione.iter().map(|x| x.0).collect::<Vec<i32>>();
    }

    return estrazione.iter().map(|x| x.0).take(5).collect::<Vec<i32>>();
}

fn generate_random_play(dati_concorso: DatiConcorso) -> Giocata {
    let mut random_coso = rand::thread_rng();

    let puntata = random_coso.gen_range(1..3);
    let giocate = generate_random_numbers(random_coso);

    let giocata = Giocata::new(dati_concorso, puntata, giocate);

    return giocata;
}

fn main() {
    for _ in 0..1000 {
        let dati_concorso = DatiConcorso::load_from_file().unwrap();
        let mut giocata = generate_random_play(dati_concorso);
        giocata.effettua_giocata();
    }

    let giocate = get_giocate();
    let estrazione = generate_extration();

    let mut giocate_1e = 0;
    let mut giocate_2e = 0;

    let mut guadagno_1e = 0;
    let mut guadagno_2e = 0;

    let mut vittorie_1e = 0;
    let mut vittorie_2e = 0;

    for giocata in giocate {
        let giocata_computed = GiocataComputed::new(giocata, estrazione.clone());
        if giocata_computed.get_puntata() == 1 {
            giocate_1e += 1;
            vittorie_1e -= giocata_computed.get_vincita();
        } else {
            giocate_2e += 2;
            vittorie_2e -= giocata_computed.get_vincita();
        }

        giocata_computed.update_file();
    }
    guadagno_1e += giocate_1e * 1 - vittorie_1e;
    guadagno_2e += giocate_2e * 2 - vittorie_2e;

    write_file("righe_concorso.txt", "");

    let mut dati_concorso = DatiConcorso::load_from_file().unwrap();
    dati_concorso.increment_numero_concorso();
    dati_concorso.save_to_file().expect("Eee");

    unsafe { MOST_FREQ.reverse() }
    println!("Numeri più giocati: {:?}", unsafe { MOST_FREQ.clone() });

    println!(
        "Guadagno da 1€: {}\nGuadagno da 2€: {}",
        guadagno_1e, guadagno_2e
    );
    println!(
        "Vittorie da 1€: {}\nVittorie da 2€: {}",
        vittorie_1e, vittorie_2e
    );
}
