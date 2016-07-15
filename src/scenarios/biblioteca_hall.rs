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

pub struct BiblioHall;

impl <S: TypeState> Scenario <S> for BiblioHall {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("En la máquina de café no hay cola, cosa bastante rara. Las únicas");
        println!("puertas útiles son las del baño y la de salida, pues consideras que en");
        println!("la planta superior no tienes nada que hacer y en las salas de estudio");
        println!("menos, aunque siempre puedes volver a bajar al aula de informática");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "ir al baño" | "ir a baño" | "baño" | "ir baño" => {
                println!("¡Oh, qué alivio!");
                println!(" ");
                println!("Vuelves al hall");

                state.reduce_time(50);
            },

            "bajar" => {
                state.reduce_time(10);
                return Some("biblioteca_ai".to_string());
            },

            "comprar café" | "tomar café" | "café" => {
                state.reduce_time(20);
                println!("No llevas dinero encima, te jodes.");
            },

            "salir" => { return Some("exterior_bo".to_string() )},

            _ => {
                println!("Si haces algo raro, los bibliotecarios te echarán la bronca");
            }
        };

        return None;
    }
}
