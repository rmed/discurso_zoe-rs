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

pub struct EndGame;

impl <S: TypeState> Scenario <S> for EndGame {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        println!("   _              _                         _                        ");
        println!("  /_\\   __      _(_)_ __  _ __   ___ _ __  (_)___   _   _  ___  _   _ ");
        println!(" //_\\  \\ \\ /\\ / / | '_ \\| '_ \\ / _ \\ '__| | / __| | | | |/ _ \\| | | |");
        println!("/  _  \\  \\ V  V /| | | | | | | |  __/ |    | \\__ \\ | |_| | (_) | |_| |");
        println!("\\_/ \\_/   \\_/\\_/ |_|_| |_|_| |_|\\___|_|    |_|___/  \\__, |\\___/ \\__,_|");
        println!("                                                    |___/             ");

        println!("\n¡Has completado tu misión!\n");

        match state.get_string("final".to_string()).as_ref() {
            "hacker" => {
                println!("Has llevado a cabo tu misión con éxito y con notables resultados.");
                println!("No sólo has ayudado a Zoe a dominar el campus, sino que además has");
                println!("sido fiel al software libre y has salvado la situación.");
                println!(" ");
                println!("¡El GUL está orgulloso!");
            },

            "lame" => {
                println!("Has llevado a cabo tu misión con éxito y con notables resultados.");
                println!("Has ayudado a Zoe a dominar el campus y salvado la situación con");
                println!("tu imagen de Debian. Aunque igual podrías haber destruido la imagen");
                println!("de Windows ya que estabas...");
                println!(" ");
                println!("Esa decisión pesará sobre tu cabeza por el resto de tus días.");
            },

            "noob" => {
                println!("Has llevado a cabo tu misión con éxito y has ayudado a Zoe a");
                println!("dominar el campus. Sin embargo, has obligado al GUL a usar software");
                println!("no libre.");
                println!(" ");
                println!("Esa decisión pesará sobre tu cabeza por el resto de tus días.");
            },

            _ => {}
        }

        return None;

    }

    fn do_action(&self, _: &str, _: &mut Box<S>) -> Option<String> {
        println!("Escribe 'RESTART' para reiniciar el juego");

        return None;
    }
}
