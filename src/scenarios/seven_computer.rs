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

pub struct SevenComp;

/// Use the pc
fn use_pc<S: TypeState>(state: &mut Box<S>) {
    if state.get_flag("has_bag".to_string()) {
        // Has the bag!
        println!("Enciendes el ordenador, usas tu chuleta para iniciar sesión");
        println!("y tras buscar un poco encuentras el archivo 'DiscursoZoe.mp3',");
        println!("sin embargo te das cuenta de que no tienes un USB para");
        println!("guardarlo. Reza porque alguien se haya dejado el pincho.");

        state.set_flag("can_search".to_string(), true);
        state.reduce_time(5);

        return;
    }

    // No bag lol
    println!("Enciendes el ordenador principal en el que supuestamente debes");
    println!("recoger el discurso de Zoe. En la pantalla aparece un cuadro");
    println!("de inicio de sesión.");
    println!(" ");
    println!("Como no te sabes tu contraseña la tienes apuntada en un papel");
    println!("que está en tu mochila... recuerdas que te has dejado la");
    println!("mochila en el aula informática de la bilioteca.");
    println!(" ");
    println!("'¡Mierda!'");

    state.reduce_time(20);
}

impl <S: TypeState> Scenario <S> for SevenComp {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Entras en el aula. No hay nada del otro mundo, sólo están los");
        println!("ordenadores de los alumnos y el del profesor. Parece que alguien ha");
        println!("estado aquí hace poco.");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "salir" => {
                println!("Vuelves al pasillo.");

                state.reduce_time(10);

                return Some("seven_zero".to_string())
            },

            "usar ordenador" | "usar pc" => {
                use_pc(state);
            },

            "buscar" | "buscar pincho" | "buscar usb" => {
                if !state.get_flag("can_search".to_string()) {
                    // Already has USB
                    return None;
                }

                println!("Buscas en el aula y... ¡Oh! ¡Alguien se ha dejado un pincho de");
                println!("Peppa pig! Lo coges.");

                state.set_flag("has_usb".to_string(), true);
                state.set_flag("can_search".to_string(), false);
                state.reduce_time(5);
            },

            "usar pincho" => {
                if !state.get_flag("has_usb".to_string()) {
                    // No USB
                    return None;
                }

                println!("Enchufas el pincho de Peppa pig en el ordenador.");
                println!(" ");
                println!("Parece que funciona, sin embargo cuando intentas copiar el archivo");
                println!("'DiscursoZoe.mp3' aparece en mensaje de error en el que dice que");
                println!("la memoria extraible 'PeppaPincho' esta llena.");

                return Some("file_select".to_string());
            },

            _ => { println!("El tiempo apremia."); }
        }

        return None;
    }
}
