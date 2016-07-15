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

pub struct ExtBo;

impl <S: TypeState> Scenario <S> for ExtBo {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!("Sales al exterior, todo parece normal salvo por un grupo de");
        println!("universitarios bebiendo sangría en el césped. Te invitan a unirte");
        println!("a ellos.");

        return None;
    }

    fn do_action(&self, command: &str, _: &mut Box<S>) -> Option<String> {
        match command {
            "aceptar" | "acepto" => {
                println!("Te unes a ellos, te montas tal juerga que se te hace tarde y");
                println!("fracasas en tu misión");

                return Some("game_over".to_string())
            },

            "rechazar" | "rechazo" => {
                println!("Te llaman soso, pero tienes que continuar con tu misión");
                return Some("exterior_ir".to_string())
            },

            _ => {
                println!("No es momento de tonterías, ¿aceptas o rechazas su invitación?");
            }
        }

        return None;
    }
}
