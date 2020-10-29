use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::rysuj_kolo;

const PACMAN_KOLOR: Color = [1.00, 1.00, 0.00, 0.90];
#[derive(Copy, Clone, PartialEq)]
pub enum Kierunek {
    Gora,
    Dol,
    Lewo,
    Prawo,
}

#[derive(Debug, Clone)]

struct Blok {
    x: i32,
    y: i32,
}

pub struct Pacman {
    kierunek: Kierunek,
    x: i32,
    y: i32,
}

impl Pacman {
    pub fn new(x: i32, y: i32) -> Pacman {
        Pacman {
            kierunek: Kierunek::Prawo,
            x,
            y,
        }
    }

    pub fn rysuj(&self, con: &Context, g: &mut G2d) {
        rysuj_kolo(PACMAN_KOLOR, self.x, self.y, con, g);
    }

    pub fn ruch_w_przod(&mut self, dir: Option<Kierunek>) {
        match dir {
            Some(d) => self.kierunek = d,
            None => (),
        }

        let old_x: i32 = self.x;
        let old_y: i32 = self.y;

        let (new_x, new_y) = match self.kierunek {
            Kierunek::Gora => (old_x, old_y - 1),
            Kierunek::Dol => (old_x, old_y + 1),
            Kierunek::Lewo => (old_x - 1, old_y),
            Kierunek::Prawo => (old_x + 1, old_y),
        };
        self.x = new_x;
        self.y = new_y;
    }

    pub fn pacman_kierunek(&self) -> Kierunek {
        self.kierunek
    }

    pub fn pozycja_pacmana(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn nastepna_pozycja(&self, dir: Option<Kierunek>) -> (i32, i32) {
        let mut kierunek_ruchu = self.kierunek;
        match dir {
            Some(d) => kierunek_ruchu = d,
            None => {}
        }

        match kierunek_ruchu {
            Kierunek::Gora => (self.x, self.y - 1),
            Kierunek::Dol => (self.x, self.y + 1),
            Kierunek::Lewo => (self.x - 1, self.y),
            Kierunek::Prawo => (self.x + 1, self.y),
        }
    }

    pub fn xy_na_pacmanie(&self, x: i32, y: i32) -> bool {
        if x == self.x && y == self.y {
            return true;
        }
        return false;
    }
}
