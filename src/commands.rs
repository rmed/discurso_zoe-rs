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
use std::process;

use texture::command::GameCommand;
use state::TypeState;


// Ends the game
pub struct Exit;

impl <S: TypeState> GameCommand <S> for Exit {
    fn execute(&self, _: &mut Box<S>) -> Option<String> {
        println!("¡Gracias por jugar!");
        process::exit(0);
    }
}

// Restarts the game
pub struct Restart;

impl <S: TypeState> GameCommand <S> for Restart {
    fn execute(&self, state: &mut Box<S>) -> Option<String> {
        // Clear the state and load first scenario
        state.clear();

        return Some("start".to_string());
    }
}

// Shows remaining time
pub struct Time;

impl <S: TypeState> GameCommand <S> for Time {
    fn execute(&self, state: &mut Box<S>) -> Option<String> {
        let time = state.get_time();
        let mins = time / 60;
        let secs = time % 60;

        println!("Te quedan {} minutos {} segundos", mins, secs);

        return Some("_tick".to_string());
    }
}
