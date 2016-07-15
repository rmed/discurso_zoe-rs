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
mod commands;
mod scenarios;
mod state;

extern crate rand;
extern crate texture;

use texture::master::GameMaster;

use state::TypeState;

fn main() {
    // Create state
    let mut state = state::State::new();
    state.clear();

    // Create game master
    let mut game_master = GameMaster::new(Box::new(state));


    // Create commands
    // define_commands(&mut game_master);
    let cmd_exit = commands::Exit;
    let cmd_restart = commands::Restart;
    let cmd_time = commands::Time;

    game_master.add_command("EXIT".to_string(), Box::new(cmd_exit));
    game_master.add_command("RESTART".to_string(), Box::new(cmd_restart));
    game_master.add_command("tiempo".to_string(), Box::new(cmd_time));


    // Create scenarios
    // define_scenarios(&mut game_master);
    let start = scenarios::start::Start;
    let game_over = scenarios::game_over::GameOver;
    let end_game = scenarios::end_game::EndGame;
    let biblio_ai = scenarios::biblioteca_ai::BiblioAI;
    let biblio_hall = scenarios::biblioteca_hall::BiblioHall;
    let ext_bo = scenarios::exterior_bo::ExtBo;
    let ext_ir = scenarios::exterior_ir::ExtIr;
    let one_zero = scenarios::one_zero::OneZero;
    let one_one = scenarios::one_one::OneOne;
    let seven_one = scenarios::seven_one::SevenOne;
    let seven_zero = scenarios::seven_zero::SevenZero;
    let seven_comp = scenarios::seven_computer::SevenComp;
    let file_sel = scenarios::file_select::FileSel;
    let sabatini = scenarios::sabatini::Sabatini;
    let gul = scenarios::gul::Gul;

    game_master.add_scenario("start".to_string(), Box::new(start));
    game_master.add_scenario("game_over".to_string(), Box::new(game_over));
    game_master.add_scenario("end_game".to_string(), Box::new(end_game));
    game_master.add_scenario("biblioteca_ai".to_string(), Box::new(biblio_ai));
    game_master.add_scenario("biblioteca_hall".to_string(), Box::new(biblio_hall));
    game_master.add_scenario("exterior_bo".to_string(), Box::new(ext_bo));
    game_master.add_scenario("exterior_ir".to_string(), Box::new(ext_ir));
    game_master.add_scenario("one_zero".to_string(), Box::new(one_zero));
    game_master.add_scenario("one_one".to_string(), Box::new(one_one));
    game_master.add_scenario("seven_one".to_string(), Box::new(seven_one));
    game_master.add_scenario("seven_zero".to_string(), Box::new(seven_zero));
    game_master.add_scenario("seven_computer".to_string(), Box::new(seven_comp));
    game_master.add_scenario("file_select".to_string(), Box::new(file_sel));
    game_master.add_scenario("sabatini".to_string(), Box::new(sabatini));
    game_master.add_scenario("gul".to_string(), Box::new(gul));

    // Start game
    game_master.start_game();
}
