// El discurso de Zoe
//
// Copyright (C) 2016  GUL UC3M
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use rand::{thread_rng, Rng};
use texture::scenario::Scenario;
use state::TypeState;

pub struct ExtIr;

/// Time lost
fn open_bank() -> i32 {
    let mut rng = thread_rng();
    let lost: i32 = rng.gen_range(2, 5);

    println!("De camino a la entrada aparece una muchacha con una carpeta roja entre los");
    println!("brazos. 'Hola, ¿tienes un momentito?' Vengo a ofrecerte una cuenta");
    println!("open-bank...\n");

    println!("Saven los dioses el tiempo que te retuvo la muchacha. Entras en el");
    println!("edificio, miras tu teléfono, has perdido {} minutos.", lost);

    return lost * 60;
}

impl <S: TypeState> Scenario <S> for ExtIr {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Para llegar al edificio 7 puedes elegir las entradas F, G o E.");
        println!("También puedes ir hacia el edificio Sabatini o la biblioteca");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "ir a E" | "ir a entrada E" | "E" | "ir E" => {
                let lost = open_bank();

                state.reduce_time(lost + 30);
                state.set_string("one_zone".to_string(), "E".to_string());

                return Some("one_zero".to_string())
            },

            "ir a F" | "ir a entrada F" | "F" | "ir F" => {
                let lost = open_bank();

                state.reduce_time(lost + 30);
                state.set_string("one_zone".to_string(), "F".to_string());

                return Some("one_zero".to_string())
            },

            "ir a G" | "ir a entrada G" | "G" | "ir G" => {
                state.reduce_time(30);
                state.set_string("one_zone".to_string(), "G".to_string());

                return Some("one_zero".to_string())
            },

            "ir a Sabatini" | "ir al Sabatini" | "Sabatini" | "ir 2" | "ir al 2" => {
                let lost = open_bank();

                state.reduce_time(lost + 30);
                state.set_value("sabatini_floor".to_string(), 1);
                state.set_string("sabatini_corner".to_string(), "C-D".to_string());

                return Some("sabatini".to_string())
            },

            "ir a biblioteca" | "ir a la biblioteca" | "biblioteca" | "ir biblioteca" => {
                println!("Vas a la biblioteca.");

                state.reduce_time(30);

                return Some("biblioteca_hall".to_string())
            },

            _ => {println!("El tiempo es oro, ¿a dónde vas a ir?")}
        }

        return None;
    }
}
