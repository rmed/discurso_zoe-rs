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
use std::collections::HashMap;

pub trait TypeState {
    fn new() -> Self;

    /// Reset internal values
    fn clear(&mut self);

    /// Get the value of a flag
    fn get_flag(&self, name: String) -> bool;

    /// Set the value of a flag
    fn set_flag(&mut self, name: String, value: bool);

    /// Get a string value
    fn get_string(&self, name: String) -> String;

    /// Set a string value
    fn set_string(&mut self, name: String, value: String);

    /// Get an integer value
    fn get_value(&self, name: String) -> i32;

    /// Set an integer value
    fn set_value(&mut self, name: String, value: i32);

    /// Get the remaining time
    fn get_time(&self) -> i32;

    /// Reduce the remaining time
    fn reduce_time(&mut self, seconds: i32);
}

pub struct State {
    // Boolean flags
    flags: HashMap<String, bool>,
    // String values
    strings: HashMap<String, String>,
    // Integer values
    values: HashMap<String, i32>,
    // Remaining time
    time: i32,
}


impl TypeState for State {
    fn new() -> State {
        State {
            flags: HashMap::new(),
            strings: HashMap::new(),
            values: HashMap::new(),
            time: 850
        }
    }

    fn clear(&mut self) {
        // Reset flags
        self.flags.insert("in_start".to_string(), true);
        self.flags.insert("has_bag".to_string(), false);
        self.flags.insert("can_search".to_string(), true);
        self.flags.insert("sabatini_flee".to_string(), false);

        // Reset values
        self.strings.insert("deleted".to_string(), "none".to_string());

        // Reset time
        self.time = 850;
    }

    fn get_flag(&self, name: String) -> bool {
        let val = match self.flags.get(&name) {
            Some(s) => { s.clone() },
            None => { false }
        };

        return val;
    }

    fn set_flag(&mut self, name: String, value: bool) {
        self.flags.insert(name, value);
    }

    fn get_string(&self, name: String) -> String {
        let val = match self.strings.get(&name) {
            Some(s) => { s.clone() },
            None => { "".to_string() }
        };

        return val;
    }

    fn set_string(&mut self, name: String, value: String) {
        self.strings.insert(name, value);
    }

    fn get_value(&self, name: String) -> i32 {
        let val = match self.values.get(&name) {
            Some(s) => { s.clone() },
            None => { 0 }
        };

        return val;
    }

    fn set_value(&mut self, name: String, value: i32) {
        self.values.insert(name, value);
    }

    fn get_time(&self) -> i32 {
        return self.time.clone();
    }

    fn reduce_time(&mut self, seconds: i32) {
        self.time = self.time - seconds;
    }
}
