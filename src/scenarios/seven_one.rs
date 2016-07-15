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

pub struct SevenOne;

fn random_quote() {
    let quotes = [
        "Yo uso Xubuntu, que es la versión ligera de Ubuntu".to_string(),
        "Vim es un editor de textos ligero".to_string(),
        "Yo uso gedit".to_string(),
        "¿Crees que puedo virtualizar cosas con un Pentium 2?".to_string(),
        "¿Es eso un virtualizador por consola?".to_string(),
        "¿A qué te refieres con 'hablar'?".to_string(),
        "¿En qué lenguaje está hecho supertuxkart?".to_string(),
    ];

    println!("En cuanto te acercas al muchacho, empieza a hablarte:");

    let mut rng = thread_rng();
    let quote = match rng.choose(&quotes) {
        Some(q) => { q.to_string() },
        None => { "Hola".to_string() }
    };

    println!("\n{}\n", quote);

    println!("Decides alejarte");
}

impl <S: TypeState> Scenario <S> for SevenOne {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Hay aulas numeradas de la 1 a la 7, un baño y una máquina de");
        println!("café en la que hay un muchacho");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "ir al baño" | "ir a baño" | "baño" => {
                println!("¡Oh, qué alivio!");
                println!(" ");
                println!("Vuelves al pasillo");

                state.reduce_time(50);
            },

            "ir a 1" | "ir al 1" | "ir 1" => {
                println!("Cruzas el pasillo y llegas al edificio 1");

                state.reduce_time(30);
                state.set_string("one_zone".to_string(), "C".to_string());

                return Some("one_one".to_string());
            },

            "hablar" => {
                state.reduce_time(15);
                random_quote();
            },

            "bajar" => {
                println!("Bajas a la planta baja.");

                state.reduce_time(10);

                return Some("seven_zero".to_string());
            },

            "ir a aula" | "ir al aula" => {
                println!("Parece que todas las aulas están cerradas o hay gente dando clase.");

                state.reduce_time(10);
            }

            _ => {
                println!("Las aulas informáticas están abajo, aunque parece que el muchacho");
                println!("de la máquina de café tiene cosas interesantes que decir");
            }
        }

        return None;
    }
}
