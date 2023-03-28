mod models;
mod utils;

use models::{dati_concorso::DatiConcorso, giocata::Giocata, giocata_computed::GiocataComputed};
use rand::{rngs::ThreadRng, Rng};
use utils::utils::get_giocate;

static mut GIOCATE_1E: i32 = 0;
static mut GIOCATE_2E: i32 = 0;
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

        if giocata.get_puntata() == 1 {
            unsafe {
                GIOCATE_1E += 1;
            }
        } else {
            unsafe {
                GIOCATE_2E += 1;
            }
        }

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

fn generate_random_play(mut dati_concorso: DatiConcorso) -> Giocata {
    let mut random_coso = rand::thread_rng();

    let puntata = random_coso.gen_range(1..3);
    let giocate = generate_random_numbers(random_coso);

    let giocata = Giocata::new(&mut dati_concorso, puntata, giocate);

    return giocata;
}

fn main() {
    let dati_concorso = DatiConcorso::load_from_file();

    let giocata = generate_random_play(dati_concorso.unwrap());
    giocata.effettua_giocata();

    let giocate = get_giocate();
    let estrazione = generate_extration();

    let mut guadagno_1e = unsafe { GIOCATE_1E };
    let mut guadagno_2e = unsafe { GIOCATE_2E * 2 };

    for mut giocata in giocate {
        let giocata_computed = GiocataComputed::new(&mut giocata, estrazione.clone());
        if giocata_computed.get_puntata() == 1 {
            guadagno_1e -= giocata_computed.get_vincita();
        } else {
            guadagno_2e -= giocata_computed.get_vincita();
        }
    }

    unsafe { MOST_FREQ.reverse() }
    println!("Numeri più giocati: {:?}", unsafe { MOST_FREQ.clone() });
    println!("guadagno 1E: {}", guadagno_1e);
    println!("guadagno 2E: {}", guadagno_2e);
}
