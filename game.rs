use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{rysuj_blok, rysuj_kolo, rysuj_prostokat};
use crate::pacman::{Kierunek, Pacman};

const KOLOR_SCIANEK: Color = [0.80, 0.00, 0.00, 1.0];
const KOLOR_TLA: Color = [0.00, 0.00, 0.00, 1.0];
const KOLOR_JEDZENIA: Color = [0.0, 1.0, 0.00, 1.0];
const GAMEOVER_KOLOR: Color = [0.90, 0.00, 0.00, 0.5];

const CZAS_RUCHU: f64 = 0.5;
const CZAS_RESETU: f64 = 10.0;
const OKRES_JEDZENIA: f64 = 10.0;

pub struct Game {
    pacman: Pacman,
    scianki: Vec<(i32, i32)>,
    width: i32,
    height: i32,
    game_over: bool,
    czas_oczekiwania: f64,
    czas_scianki: f64,
    okres_scianki: f64,
    jedzenie: (i32, i32),
    istnienie_jedzenia: bool,
    czas_jedzenia: f64,
    czas_gry: f64,
    ranking: Vec<f64>,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            pacman: Pacman::new(10, 10),
            czas_oczekiwania: 0.0,
            scianki: [
                (6, 4),
                (3, 10),
                (12, 4),
                (28, 20),
                (23, 4),
                (12, 21),
                (6, 18),
            ]
            .to_vec(),
            width,
            height,
            czas_scianki: 1.0,
            okres_scianki: 1.0,
            jedzenie: (0, 0),
            istnienie_jedzenia: false,
            czas_jedzenia: 0.0,
            game_over: false,
            czas_gry: 0.0,
            ranking: [].to_vec(),
        }
    }

    pub fn wcisniety_klawisz(&mut self, key: Key) {
        if self.game_over {
            let spacja = match key {
                Key::Space => true,
                _ => false,
            };
            if spacja {
                self.restart();
            }
            return;
        }

        let dir = match key {
            Key::Up => Some(Kierunek::Gora),
            Key::Down => Some(Kierunek::Dol),
            Key::Left => Some(Kierunek::Lewo),
            Key::Right => Some(Kierunek::Prawo),
            _ => Some(self.pacman.pacman_kierunek()),
        };

        self.up_pacman(dir);
    }

    pub fn rysuj(&self, con: &Context, g: &mut G2d) {
        self.pacman.rysuj(con, g);

        for fot in &self.scianki {
            let x = fot.0;
            let y = fot.1;
            rysuj_blok(KOLOR_SCIANEK, x, y, con, g);
        }

        if self.istnienie_jedzenia {
            let x = self.jedzenie.0;
            let y = self.jedzenie.1;
            rysuj_kolo(KOLOR_JEDZENIA, x, y, con, g);
        }
        rysuj_prostokat(KOLOR_TLA, 0, 0, self.width, 1, con, g);
        rysuj_prostokat(KOLOR_TLA, 0, self.height - 1, self.width, 1, con, g);
        rysuj_prostokat(KOLOR_TLA, 0, 0, 1, self.height, con, g);
        rysuj_prostokat(KOLOR_TLA, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            rysuj_prostokat(GAMEOVER_KOLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.czas_oczekiwania += delta_time;
        self.czas_scianki += delta_time;
        self.czas_jedzenia += delta_time;
        self.czas_gry += delta_time;

        if self.game_over {
            if self.czas_oczekiwania > CZAS_RESETU {
                self.restart();
            }
            return;
        }
        if self.czas_jedzenia > OKRES_JEDZENIA {
            self.czas_jedzenia = 0.0;
            self.istnienie_jedzenia = false;
        }

        if !self.istnienie_jedzenia {
            self.dodaj_jedzenie();
        }
        if self.czas_scianki > self.okres_scianki {
            self.okres_scianki *= 0.98;
            self.dodaj_scianke();
            self.czas_scianki = 0.0;
        }
        if self.czas_oczekiwania > CZAS_RUCHU {
            self.up_pacman(None);
        }
    }

    fn czy_pacman_zyje(&self, dir: Option<Kierunek>) -> bool {
        let (next_x, next_y) = self.pacman.nastepna_pozycja(dir);

        for s in &self.scianki {
            let x = s.0;
            let y = s.1;
            if x == next_x && y == next_y {
                return false;
            }
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn czy_je(&self) -> bool {
        let (pac_x, pac_y) = self.pacman.pozycja_pacmana();
        let (jedz_x, jedz_y) = self.jedzenie;

        if pac_x == jedz_x && pac_y == jedz_y {
            return true;
        }
        return false;
    }

    fn dodaj_scianke(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);

        while self.pacman.xy_na_pacmanie(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }
        self.scianki.push((new_x, new_y));
    }

    fn usun_scianki(&mut self) {
        let ilosc_scianek = (self.scianki.len() / 4) as i32;
        for _s in 0..ilosc_scianek {
            self.scianki.pop();
        }
    }

    fn dodaj_jedzenie(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);

        while self.pacman.xy_na_pacmanie(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }
        self.jedzenie = (new_x, new_y);
        self.istnienie_jedzenia = true;
        self.czas_jedzenia = 0.0;
    }

    fn up_pacman(&mut self, dir: Option<Kierunek>) {
        if self.czy_pacman_zyje(dir) {
            self.pacman.ruch_w_przod(dir);
        } else {
            self.game_over = true;
            println!("Twoj czas gry {} sekund", self.czas_gry);
            println!("Ranking:");
            self.ranking.push(self.czas_gry);
            self.ranking.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let mut miejsce = 1;
            for t in &self.ranking {
                println!("MIEJSCE {}: {} sekund", miejsce, t);
                miejsce += 1;
            }
        }
        if self.czy_je() {
            self.istnienie_jedzenia = false;
            self.usun_scianki();
        }
        self.czas_oczekiwania = 0.0;
    }

    fn restart(&mut self) {
        self.pacman = Pacman::new(2, 2);
        self.czas_oczekiwania = 0.0;
        self.scianki = [
            (6, 4),
            (3, 10),
            (12, 4),
            (28, 20),
            (23, 4),
            (12, 21),
            (6, 18),
        ]
        .to_vec();
        self.game_over = false;
        self.czas_scianki = 1.0;
        self.okres_scianki = 1.0;
        self.jedzenie = (0, 0);
        self.istnienie_jedzenia = false;
        self.czas_jedzenia = 0.0;
        self.czas_gry = 0.0;
    }
}
