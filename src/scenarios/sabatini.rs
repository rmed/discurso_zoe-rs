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

pub struct Sabatini;

/// Descend
fn go_down<S: TypeState>(state: &mut Box<S>) -> Option<String> {
    let current_floor = state.get_value("sabatini_floor".to_string());

    if current_floor == 1 {
        // Out of the building
        println!("Sales del edificion.");

        return Some("exterior_ir".to_string());
    }

    // Descend
    let new_floor = current_floor - 1;
    state.set_value("sabatini_floor".to_string(), new_floor);

    println!("Bajas una planta.");
    println!(" ");
    println!("Estás en la planta {}.", new_floor);

    state.reduce_time(10);

    return None;
}

/// Ascend
fn go_up<S: TypeState>(state: &mut Box<S>) {
    let current_floor = state.get_value("sabatini_floor".to_string());

    if current_floor == 3 {
        // Cannot go up
        println!("No puedes subir más");

        return;
    }

    // Ascend
    let new_floor = current_floor + 1;
    state.set_value("sabatini_floor".to_string(), new_floor);

    println!("Subes una planta.");
    println!(" ");
    println!("Estás en la planta {}.", new_floor);

    if new_floor == 3 {
        println!("\nEsta es la planta donde están los despachos de las asociaciones,");
        println!("puedes intentar ir al despacho.")
    }

    state.reduce_time(10);
}

/// Must flee!
fn must_flee<S: TypeState>(command: &str, state: &mut Box<S>) -> Option<String> {
    match command {
        "huir" => {},
        _ => {
            println!("Debiste huir cuando tuviste la ocasión. Te han pillado.");

            return Some("game_over".to_string())
        }
    };

    flee(state);
    state.reduce_time(30);

    let time = state.get_time();
    let mins = time / 60;
    let secs = time % 60;

    println!("Has conseguido despistar al BEHTS.");
    println!(" ");
    println!("Estás en la esquina {} y te quedan {} minutos {} segundos.",
             state.get_string("sabatini_corner".to_string()),
             mins,
             secs);

    state.set_flag("sabatini_flee".to_string(), false);

    return None;
}

/// Random direction
fn go_somewhere<S: TypeState>(direction: &str, state: &mut Box<S>) {
    let corners =  ["A-B", "B-C", "C-D", "D-A"];

    let mut current_index = match state.get_string("sabatini_corner".to_string()).as_ref() {
        "A-B" => { 0 }
        "B-C" => { 1 }
        "C-D" => { 2 }
        _ => { 3 }
    };

    match direction {
        "l" => {
            if current_index == 0 {
                // Underflow
                current_index = 3;

            } else {
                current_index = current_index - 1;
            }
        },
        _ => { current_index = (current_index + 1) % 4 },
    };

    let new_corner = corners[current_index];

    println!("Avanzas por el pasillo rezando por que te lleve al lugar que deseas,");
    println!("quizá tu memoria o el historial de la consola te esté jugando una mala");
    println!("pasada, pero para bien o para mal llegas a la esquina {}.", new_corner);

    state.reduce_time(10);
    state.set_string("sabatini_corner".to_string(), new_corner.to_string());
}

/// Flee all the way
fn flee<S: TypeState>(state: &mut Box<S>) {
    let mut rng = thread_rng();

    let corners = match state.get_string("sabatini_corner".to_string()).as_ref() {
        "A-B" => { vec!["B-C", "C-D", "D-A"] },
        "B-C" => { vec!["A-B", "C-D", "D-A"] },
        "C-D" => { vec!["A-B", "B-C", "D-A"] },
        _ => { vec!["A-B", "B-C", "C-D"] }
    };

    let chosen = match rng.choose(&corners) {
        Some(c) => { c.to_string() },
        None => { "A-B".to_string() }
    };

    state.set_string("sabatini_corner".to_string(), chosen);
}

/// Random encounters
fn random_encounter<S: TypeState>(state: &mut Box<S>) {
    let mut rng = thread_rng();
    let perc: i32 = rng.gen_range(0, 100);

    if (perc >= 0 && perc < 40) || (perc >= 50 && perc < 90) {
        // BEHTS
        state.set_flag("sabatini_flee".to_string(), true);

        println!("\nAl avanzar te topas con un miembro del BEHTS, te ha visto y va a ir");
        println!("a por ti. ¡Deber huir!");

        return;
    }

    // No BEHTS
    state.set_flag("sabatini_flee".to_string(), false);
}


impl <S: TypeState> Scenario <S> for Sabatini {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Acabas de entrar en el edifio 2, el Sabatini. Tienes que");
        println!("encontrar el despacho del GUL y llevar el discurso de Zoe para el");
        println!("programa de radio en directo a tiempo.");
        println!(" ");
        println!("Un escalofrio recorre tu espalda, sabes que da igual el tiempo que");
        println!("lleves en esta universidad, siempre que busques un aula en este");
        println!("edificio te vas a perder.");
        println!(" ");
        println!("Respiras hondo y te preparas para encontrar el despacho, subes las");
        println!("esclaras, pues fijate que oportuno, ningún ascensor funciona.");
        println!(" ");
        println!("Te encuentras en la esquina C-D de la planta 1. Puedes ir al pasillo");
        println!("de la derecha, al de la izquierda, subir o bajar las escaleras.");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        if state.get_flag("sabatini_flee".to_string()) {
            // Must flee
            return must_flee(command, state);
        }

        match command {
            "ir izquierda" | "ir a la izquierda" | "izquierda" => {
                go_somewhere("l", state);
            },

            "ir derecha" | "ir a la derecha" | "derecha" => {
                go_somewhere("l", state);
            },

            "subir" => {
                go_up(state);

                return None;
            },

            "bajar" => {
                return go_down(state);
            },

            "despacho" | "ir al despacho" | "ir despacho" | "ir a despacho" => {
                if state.get_value("sabatini_floor".to_string()) != 3 {
                    println!("Aquí no hay ningún despacho");

                    return None;
                }

                if state.get_string("sabatini_corner".to_string()) != "B-C" {
                    // Not correct
                    println!("Ninguno de los 2 despachos que hay en esta esquina es el del");
                    println!("GUL, además según te ven empiezan a pegar voces, se ve que los");
                    println!("del BEHTS les han prometido una bolsa de chuches si les");
                    println!("avisaban al verte. Deberias huir.");

                    state.reduce_time(10);
                    state.set_flag("sabatini_flee".to_string(), true);
                }

                if state.get_string("deleted".to_string()) == "none" {
                    // Without speech
                    println!("Has llegado al despacho del GUL, pero no traes lo que se te");
                    println!("ha pedido. Al acercarte a la puerta puedes oír a gente");
                    println!("hablando muy rápido. Parecen estar nerviosos.");
                    println!(" ");
                    println!("Decides que no puedes entrar así como así sin el discurso de");
                    println!("Zoe y te alejas.");

                    return None;
                }

                println!("¡Has llegado al despacho del GUL!");

                return Some("gul".to_string());

            },

            _ => {
                println!("Solo un poco más...");

                return None;
            }
        };

        random_encounter(state);

        return None;
    }
}
