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
use texture::util::clear_screen;
use state::TypeState;

pub struct Start;

impl <S: TypeState> Scenario <S> for Start {
    fn load(&self, _: &mut Box<S>) -> Option<String> {
        clear_screen();

        println!("-/++++++++++++++/++++++++++++++++++++++++++++++++++++++++++++/-");
        println!("/++++++++++++++++//+++++++++++++++++++++++++++++++++++++++++++++/");
        println!("++++++++++++++++++.-+++++++++++++++++++++++++++++++++++++++++++++");
        println!("+++++++++/++++++++:  ./++++++++++++++++++++++++++++++++++++++++++");
        println!("+++++++++/-+++++++-    ./++++++++++++++++++++++++++++++++++++++++");
        println!("++++++++++-`/+++++`      -+++++++++++++++++++++++++++++++++++++++");
        println!("+++++++++++ `:++/`        `/+++++++++++++++++++++++++++++++++++++");
        println!("+++++++++++`  `.:-.         -/+++++++++++++++++++++++++++++++++++");
        println!("++++++++++/      `.--.        `.--::://++++++++++++++++++++++++++");
        println!("++++++++++.          `..`              `.-:++++++++++++++++++++++");
        println!("++++++++++:             `                  `-++++++++++++++++++++");
        println!("++++++++++/:                          `..-`  -+++++++++++++++++++");
        println!("++++++++:. .:                      `.:/+++/`  .:/++++++++++++++++");
        println!("++++++:`    .:`                  `--````.://     `-:/++++++++++++");
        println!("++++:`       `.                  ``       ``        `./++++++++++");
        println!("+/-`                                                  :++++++++++");
        println!(".`                                                    /++++++++++");
        println!("                                                   `.-+++++++++++");
        println!("                                          ```.://:.++++++++++++++");
        println!("                    `                   -/++/+++++/++++++++++++++");
        println!("                    .                  /+++++++++++++++++++++++++");
        println!("                    -                  -+++++++++++++++++++++++++");
        println!("                    `.     `            `-+++++++++++++++++++++++");
        println!("                     .    `/`      .`     .:++++:++++++++++++++++");
        println!("                      -   :/`     `:        `.-`-++++++++++++++++");
        println!("                      `-  :-.     :`           `+++++++++++++++++");
        println!("                       `:`:.:    `/          `./+++++++++++++++++");
        println!("                        `/: /    ./.....--://++++++++++++++++++++");
        println!("                            --   .+++++++++++++++++++++++++++++++");
        println!("                          `.-+-   +++++++++++++++++++++++++++++++");
        println!("                        -++++++/. -++++++++++++++++++++++++++++++");
        println!("                      .+++++++++++:/++++++++++++++++++++++++++++/");
        println!("                    `:++++++++++++++++++++++++++++++++++++++++/-");

        // Title
        println!("\n\n");
        println!(" ___ _     __  _   __   ____  _ ___   __   __    __  ___   ___ __  ___");
        println!("| __| |   | _\\| |/' _/ / _/ || | _ \\/' _/ /__\\  | _\\| __| |_  /__\\| __|");
        println!("| _|| |_  | v | |`._`.| \\_| \\/ | v /`._`.| \\/ | | v | _|   / / \\/ | _|");
        println!("|___|___| |__/|_||___/ \\__/\\__/|_|_\\|___/ \\__/  |__/|___| |___\\__/|___|");

        // Authors
        println!("\nGuión: Adrián Borja");
        println!("Motor: Rafael Medina");

        println!("Escribe 'comenzar' para iniciar la partida");

        return None;
    }


    fn do_action(&self, command: &str, _: &mut Box<S>) -> Option<String> {
        match command {
            "comenzar" => { return Some("biblioteca_ai".to_string()) },
            _ => {println!("Escribe 'comenzar' para iniciar la partida")}
        };

        return None;
    }
}
