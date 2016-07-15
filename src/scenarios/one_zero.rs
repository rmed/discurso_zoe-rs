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

pub struct OneZero;

/// Actions zone B
fn actions_b<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "ir a C" | "ir C" => {
            state.set_string("one_zone".to_string(), "C".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "ir a E" | "ir E" => {
            state.set_string("one_zone".to_string(), "E".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "ir a F" | "ir F" => {
            state.set_string("one_zone".to_string(), "F".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "subir" => {
            println!("Subes por las escaleras y tienes la mala suerte de encontrarte");
            println!("de frente con alguien del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "salir" => {
            return Some("exterior_ir".to_string());
        },

        _ => {}
    }

    println!("No es buena idea quedarse aquí");

    return None;
}

/// Actions zone C
fn actions_c<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "ir a B" | "ir B" => {
            state.set_string("one_zone".to_string(), "B".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "ir a F" | "ir F" => {
            state.set_string("one_zone".to_string(), "F".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "ir a G" | "ir G" => {
            state.set_string("one_zone".to_string(), "G".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "subir" => {
            state.reduce_time(10);
            return Some("one_one".to_string());
        },

        "salir" => {
            println!("Mira por dónde, has tentado a la suerte y has perdido. El BETHS");
            println!("te ha atrapado.");

            return Some("game_over".to_string());
        },

        _ => {}
    }

    println!("No es buena idea quedarse aquí");

    return None;
}

/// Actions zone E
fn actions_e<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "ir a B" | "ir B" => {
            state.set_string("one_zone".to_string(), "B".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "subir" => {
            println!("Subes por las escaleras y tienes la mala suerte de encontrarte");
            println!("de frente con alguien del BEHTS. Te han atrapado.");

            return Some("game_over".to_string());
        },

        "salir" => {
            return Some("exterior_ir".to_string());
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
            state.set_string("one_zone".to_string(), "B".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "ir a C" | "ir C" => {
            state.set_string("one_zone".to_string(), "C".to_string());
            state.reduce_time(10);

            return Some("one_zero".to_string());
        },

        "subir" => {
            state.reduce_time(10);
            return Some("one_one".to_string());
        },

        "salir" => {
            return Some("exterior_ir".to_string());
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

            return Some("one_zero".to_string());
        },

        "subir" => {
            state.reduce_time(10);
            return Some("one_one".to_string());
        },

        "salir" => {
            return Some("exterior_ir".to_string());
        },

        _ => {}
    }

    println!("No es buena idea quedarse aquí");

    return None;
}

impl <S: TypeState> Scenario <S> for OneZero {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        match state.get_string("one_zone".to_string()).as_ref() {
            "B" => {
                println!("Parece despejado. Puedes salir al exterior o ir a los pasillos E, F");
                println!("o C, o subir a la planta de arriba, lo cual te da mala espina.");
            },

            "C" => {
                println!("Te llama la atención que en la puerta de salida del edificio hacia");
                println!("el 7 hay un joven que te mira sospechosamente. Debe ser uno del");
                println!("BEHTS, aunque es probable que sea un novato, pues parece que no");
                println!("está seguro de si eres un alumno random o un siervo de Zoe.");
                println!("Piensas que es mejor no tentar a la suerte.");
                println!(" ");
                println!("Aparte de la puerta que vigila el joven, puedes ir a los pasillos F,");
                println!("B o G, o subir las escaleras.");
            },

            "E" => {
                println!("Estás en la entrada del edificio, todo en orden. Ves las escaleras");
                println!("para subir al primer piso, aunque se oye mucho ruido, mejor no ir");
                println!("por ahi, el pasillo que comunica con la zona B y la puerta para");
                println!("salir del edificio.");
            },

            "F" => {
                println!("Estás en la entrada del edificio, nada que te llame la atención.");
                println!("Ves las escaleras para subir al primer piso y el pasillo que");
                println!("comunica con las zonas B o C y la puerta para salir del edificio.");
            },

            "G" => {
                println!("Estás en la entrada del edificio, nada fuera de lo común. Ves las");
                println!("escaleras para subir al primer piso, el pasillo que comunica con");
                println!("la zona C y la puerta para salir del edificio.");
            },

            _ => {}

        };

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match state.get_string("one_zone".to_string()).as_ref() {
            "B" => return actions_b(command, state),
            "C" => return actions_c(command, state),
            "E" => return actions_e(command, state),
            "F" => return actions_f(command, state),
            "G" => return actions_g(command, state),
            _ => {}
        };

        println!("No es buena idea quedarse aquí.");

        return None;
    }
}
