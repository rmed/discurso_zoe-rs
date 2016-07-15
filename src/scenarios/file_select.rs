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

pub struct FileSel;

impl <S: TypeState> Scenario <S> for FileSel {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Examinas el pincho. Hay 3 archivos:");
        println!(" ");
        println!("- Matrix.mkv");
        println!("- liveDebian.iso");
        println!("- liveWindows7.iso");
        println!(" ");
        println!("Tienes que borrar uno a la fuerza. ¿Cuál será?");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "Matrix.mkv" => {
                println!("Decides borrar la película de Matrix. Como friky que eres, te duele");
                println!("un poco cuando pulsas enter");
                println!("\n...\n");
                println!("El archivo ha terminado de copiarse, así que decides marcharte del");
                println!("aula.");

                state.set_string("deleted".to_string(), "matrix".to_string());

                return Some("seven_zero".to_string())
            },

            "liveDebian.iso" => {
                println!("Decides borrar la distribución de Debian, porque es lo que menos");
                println!("ocupa y tampoco necesitas tanto espacio");
                println!("\n...\n");
                println!("El archivo ha terminado de copiarse, así que decides marcharte del");
                println!("aula.");

                state.set_string("deleted".to_string(), "debian".to_string());

                return Some("seven_zero".to_string())
            },

            "liveWindows7.iso" => {
                println!("Decides borrar la iso de Windows 7, a fin de cuentas...");
                println!("¿quién la iba a necesitar?");
                println!("\n...\n");
                println!("El archivo ha terminado de copiarse, así que decides marcharte del");
                println!("aula.");

                state.set_string("deleted".to_string(), "windows".to_string());

                return Some("seven_zero".to_string())
            },

            _ => {}
        };


        println!("Parece que ese nombre de archivo no existe. Quizá te hayas equivocado");
        println!("al escribirlo por la tensión.");

        return None;
    }
}
