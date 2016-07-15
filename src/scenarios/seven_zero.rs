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
use texture::scenario::Scenario;
use state::TypeState;

pub struct SevenZero;

impl <S: TypeState> Scenario <S> for SevenZero {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Un chico y una chica un poco distraidos vigilan la puerta hacia el");
        println!("exterior, deben pertenecer al BEHTS, mejor no tentar a la suerte.");
        println!(" ");
        println!("Las aulas informáticas son la 4 y la 5, la 5 está cerrada.");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "salir" => {
                println!("Claro, salgamos por la puerta que viliga gente de BEHTS,");
                println!("¿qué podría salir mal?");
                println!(" ");
                println!("Te han pillado");

                return Some("game_over".to_string());
            },

            "subir" => {
                println!("Subes a la primera planta");
                state.reduce_time(10);

                return Some("seven_one".to_string());
            },

            "ir a aula 4" | "ir al aula 4" | "ir aula 4" | "aula 4" => {
                match state.get_string("deleted".to_string()).as_ref() {
                    "none" => {
                        state.reduce_time(10);
                        return Some("seven_computer".to_string());
                    },

                    _ => { println!("No tienes nada más que hacer ahí"); }
                }
            }

            _ => {
                println!("Ya estás muy cerca del aula informática, sólo tienes que conseguir");
                println!("el discurso y salir de ahí. No lo estropees");
            }
        };

        return None;
    }
}
