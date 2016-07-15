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

pub struct BiblioAI;

/// Print initial description (start of the game)
fn init_desc() {
    println!("El dolor de espalda hace que comiences a despertar. Te gustaría estar");
    println!("en tu acogedora cama pero, lejos de ello, reconoces el aula informática");
    println!("del sótano de la bilioteca, donde empezaste a trabajar como becario, y");
    println!("tu mochila llena de cosas que has usado como almohada.");
    println!(" ");
    println!("Al mirar a la pantalla tienes un chat de hangouts abierto en el que una");
    println!("tal 'Zoe' te está hablando:");
    println!(" ");
    println!("Buenas tardes becario, espero que te haya aprovechado la siesta");
    println!("porque el tiempo apremia. Mi nombre es Zoe, y aunque no lo sepas,");
    println!("pronto me haré con el control de la universidad. Sin embargo,");
    println!("@djstrike, el encargado de grabar mi discurso para el nuevo régimen, ha");
    println!("visto su conexión saboteada por el BEHTS, un grupo de gente que se");
    println!("niega a reconocerme como su nueva líder.");
    println!(" ");
    println!("Bien, escucha mi nuevo siervo, ignoro como lo ha hecho pero @djstrike");
    println!("ha conseguido guardar en el ordenador principal del aula 7.0.J04 el");
    println!("archivo con mi discurso, necesito que vayas a dicho aula, consigas el");
    println!("archivo, y lo lleves al despacho  del GUL en el 2.3.C05 a tiempo para");
    println!("el programa de @HolaMundoRadio que empezará su emisión en 10 minutos.");
    println!(" ");
    println!("Date prisa, o no habrá piedad.");
    println!(" ");
    println!("PD: Los miembros del BEHTS intentarán acabar contigo, aléjate de ellos.");
}

impl <S: TypeState> Scenario <S> for BiblioAI {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        // Start of the game
        if state.get_flag("in_start".to_string()) {
            init_desc();
            state.set_flag("in_start".to_string(), false);
            return None;
        }

        // Next visits
        match state.get_flag("has_bag".to_string()) {
            true => {
                // Has bag
                println!("Has vuelto al aula informática. No hay nada de interés");
            },
            false => {
                // No bag
                println!("Has vuelto al aula informática, tu mochila no se ha fugado");
            }
        };

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "observar" => {
                println!("No hay nadie en el aula, la pizarra esta limpia, quizá porque");
                println!("no hay rotuladores. Sólo hay una puerta por la que salir y el");
                println!("tiempo apremia.");
            },

            "leer" => {
                println!("En la pantalla del ordenador está tu chat con Zoe y una partida");
                println!("perdida al buscaminas, lo que te recuerda que tienes una misión");
                println!("que cumplir.");
            },

            "coger mochila" => {
                if state.get_flag("has_bag".to_string()) {
                    println!("Ya has cogido tu mochila");

                } else {
                    println!("Recoges tu mochila, has recuperado las llaves de casa,");
                    println!("tus cuadernos y demás útiles de estudio y tu chuleta");
                    println!("con la cuenta y contraseña de becario.");

                    // Got the bag!
                    state.set_flag("has_bag".to_string(), true);
                }
            },

            "salir" => {
                println!("Sales del aula, subes las escaleras y llegas al hall");
                println!("de la biblioteca, donde hay cola para los préstamos.\n");

                state.reduce_time(10);

                return Some("biblioteca_hall".to_string());
            },

            _ => {println!("Piensas que deberías darte prisa y hacer algo")}
        };

        return None;
    }
}
