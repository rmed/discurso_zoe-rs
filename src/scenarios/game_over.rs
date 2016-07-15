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

pub struct GameOver;

impl <S: TypeState> Scenario <S> for GameOver {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        println!(" _______  _______  _______  _______    _______           _______  _______ ");
        println!("(  ____ \\(  ___  )(       )(  ____ \\  (  ___  )|\\     /|(  ____ \\(  ____ )");
        println!("| (    \\/| (   ) || () () || (    \\/  | (   ) || )   ( || (    \\/| (    )|");
        println!("| |      | (___) || || || || (__      | |   | || |   | || (__    | (____)|");
        println!("| | ____ |  ___  || |(_)| ||  __)     | |   | |( (   ) )|  __)   |     __)");
        println!("| | \\_  )| (   ) || |   | || (        | |   | | \\ \\_/ / | (      | (\\ (   ");
        println!("| (___) || )   ( || )   ( || (____/\\  | (___) |  \\   /  | (____/\\| ) \\ \\__");
        println!("(_______)|/     \\||/     \\|(_______/  (_______)   \\_/   (_______/|/   \\__/");
        println!("                                                                          ");

        println!("\nHas perdido la partida.\n");
        println!("Puedes iniciar una nueva escribiendo 'RESTART'.");

        return None;


    }

    fn do_action(&self, _: &str, _: &mut Box<S>) -> Option<String> {
        println!("Has perdido la partida.\n");
        println!("Puedes iniciar una nueva escribiendo 'RESTART'.");

        return None;
    }
}
