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

pub struct OneOne;

/// Actions zone C
fn actions_c<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "ir a B" | "ir B" => {
            println!("Das la vuelta a la esquina y tienes la mala suerte de encontrarte");
            println!("de frente con alguien del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "ir a F" | "ir F" => {
            state.set_string("one_zone".to_string(), "F".to_string());
            state.reduce_time(10);

            return Some("one_one".to_string());
        },

        "ir a G" | "ir G" => {
            state.set_string("one_zone".to_string(), "G".to_string());
            state.reduce_time(10);

            return Some("one_one".to_string());
        },

        "ir a 7" | "ir al 7" | "ir 7" => {
            println!("Cruzas el pasillo y llegas al edificio 7.");

            state.reduce_time(30);

            return Some("seven_one".to_string());
        }

        "subir" => {
            println!("Por alguna razón desconocida sientes la imperiosa necesidad de");
            println!("subir a la segunda planta. Es una lástima que esta planta esté");
            println!("llena de miembros del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "bajar" => {
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        _ => {}
    }

    println!("No es buena idea quedarse aquí");

    return None;
}

/// Actions zone F
fn actions_f<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "ir a B" | "ir B" => {
            println!("Das la vuelta a la esquina y tienes la mala suerte de encontrarte");
            println!("de frente con alguien del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "ir a C" | "ir C" => {
            state.set_string("one_zone".to_string(), "C".to_string());
            state.reduce_time(10);

            return Some("one_one".to_string());
        },

        "subir" => {
            println!("Por alguna razón desconocida sientes la imperiosa necesidad de");
            println!("subir a la segunda planta. Es una lástima que esta planta esté");
            println!("llena de miembros del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "bajar" => {
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        _ => {}
    }

    println!("No es buena idea quedarse aquí");

    return None;
}

/// Actions zone G
fn actions_g<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "ir a C" | "ir C" => {
            state.set_string("one_zone".to_string(), "C".to_string());
            state.reduce_time(10);

            return Some("one_one".to_string());
        },

        "subir" => {
            println!("Por alguna razón desconocida sientes la imperiosa necesidad de");
            println!("subir a la segunda planta. Es una lástima que esta planta esté");
            println!("llena de miembros del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "bajar" => {
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        _ => {}
    }

    println!("No es buena idea quedarse aquí");

    return None;
}

impl <S: TypeState> Scenario <S> for OneOne {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        match state.get_string("one_zone".to_string()).as_ref() {
            "C" => {
                println!("No parece que haya nadie en los pasillos F y G. Puedes ver que la");
                println!("puerta hacia el edificio 7, también está despejada. En el pasillo B");
                println!("hay un cartel que dice '¡Sexo Gratis!'.");
            },

            "F" => {
                println!("No hay nadie en este pasillo, aunque 2 chicas del BEHTS hablan en");
                println!("la escalera hacia el segundo piso sobre una fiesta en Getafe después");
                println!("de capturar al siervo de Zoe. Por suerte no han reparado en tu belleza");
                println!("interior como estudiante de ingeniería.");
                println!(" ");
                println!("Puedes ir a los pasillos B o C o bajar a la planta baja.");
            },

            "G" => {
                println!("Reconoces a alguien sospechoso sentado en el último escalón del");
                println!("tramo que sigue hacia la segunda planta mirando su móvil.");
                println!("Parece que no te ha visto, pero crees que puede ser un");
                println!("miembro del BEHTS.");
                println!(" ");
                println!("Puedes ir al pasillo C o bajar. Mejor no subir.");
            },

            _ => {}
        }

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match state.get_string("one_zone".to_string()).as_ref() {
            "C" => return actions_c(command, state),
            "F" => return actions_f(command, state),
            "G" => return actions_g(command, state),
            _ => {}
        };

        println!("No es buena idea quedarse aquí.");

        return None;
    }
}
