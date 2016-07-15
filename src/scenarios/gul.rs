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

pub struct Gul;

/// Contents of the drive
fn drive_contents<S: TypeState>(state: &mut Box<S>) {
    match state.get_string("deleted".to_string()).as_ref() {
        "windows" => {
            println!("Te mirán asombrados mientras les entregas el pincho de");
            println!("Peppa pig que encontraste en el aula informática, argumentas");
            println!("que el pincho no es tuyo, que te lo encontraste. Parece que no");
            println!("te creen pero hoy no te van a juzgar.");
            println!(" ");
            println!("Preparan un ordenador y se disponen a intalar el SO desde tu");
            println!("PeppaPincho. En poco tiempo consiguen iniciar en Debian,");
            println!("se preparan para el programa de radio y cuando al ir a por el");
            println!("'DiscursoZoe.mp3' ven que en el pincho tienes la pélicula");
            println!("de 'Matrix' en HD, te miran orgullosos y te felicitan por tu");
            println!("trabajo. Prefieres no recordar a nadie que el pincho no");
            println!("es tuyo.");
            println!(" ");
            println!("El programa de Radio se emite exitosamente y te hacen una");
            println!("mención para agradecer tus esfuerzos e invitarte a participar");
            println!("en futuras actividades del GUL.");

            state.set_string("final".to_string(), "hacker".to_string());
        },

        "matrix" => {
            println!("Te mirán asombrados mientras les entregas el pincho de");
            println!("Peppa pig que encontraste en el aula informática, argumentas");
            println!("que el pincho no es tuyo, que te lo encontraste. Parece que no");
            println!("te creen pero hoy no te van a juzgar.");
            println!(" ");
            println!("Preparan un ordenador y se disponen a intalar el SO desde");
            println!("tu PeppaPincho. En poco tiempo consiguen iniciar en Debian,");
            println!("se preparan para el programa de radio y cuando al ir a por el");
            println!("'DiscursoZoe.mp3' ven que en el pincho tienes una iso de");
            println!("Windows7, te miran con desaprobación. Les recuerdas que el");
            println!("pincho no es tuyo, pero siguen sin creerte.");
            println!(" ");
            println!("El programa de Radio se emite exitosamente y te agradecen que");
            println!("les trajeras el archivo.");

            state.set_string("final".to_string(), "lame".to_string());
        },

        "debian" => {
            println!("Te mirán asombrados mientras les entregas el pincho de");
            println!("Peppa pig que encontraste en el aula informática, argumentas");
            println!("que el pincho no es tuyo, que te lo encontraste. Parece que no");
            println!("te creen pero hoy no te van a juzgar.");
            println!(" ");
            println!("Preparan un ordenador y se disponen a intalar el SO desde");
            println!("tu PeppaPincho. Cuando ven el instalador de Windows se llevan");
            println!("las manos a la cabeza y preguntan a Zoe qué han hecho para tener");
            println!("tan mala suerte en este gran día.");
            println!(" ");
            println!("Al menos el programa de Radio se emite exitosamente. La chica");
            println!("que presidía la mesa tiene el detalle de agradecerte que hayas");
            println!("traído el discurso de Zoe y algo para arrancar el ordenador.");
            println!("También te pide que disculpes al resto de guleros, explicandote");
            println!("que aunque están agradecidos por tus esfuerzos, ahora mismo");
            println!("están demasiado furiosos con el BEHTS, que no sólo ha intentado");
            println!("sabotear el plan por todos los medios posibles, sino que además,");
            println!("junto con el ordenador y los discos, se han llevado el peluche");
            println!("de Tux más bonito que había en el despacho.");

            state.set_string("final".to_string(), "noob".to_string());
        },

        _ => {}
    };
}

impl <S: TypeState> Scenario <S> for Gul {
    fn load(&self, state: &mut Box<S>) -> Option<String> {
        if state.get_time() <= 0 {
            // You are late
            println!("No has llegado a tiempo para el programa de radio...");

            return Some("game_over".to_string());
        }

        println!("Abres la puerta un poco preocupado por la hora y te encuentras a 5");
        println!("personas. Dos de ellas, un chico y una chica, parecen buscar algo");
        println!("en los armarios y en cajas de cartón. Un segundo muchacho está");
        println!("terminando de colocar los cables de los micrófonos a una mesa de");
        println!("mezclas. Mientras, un tercer hombre sentado entre dos mesas y la pared,");
        println!("en una silla que ni siquiera va a poder separar de la mesa para poder");
        println!("salir, intenta relajar el tenso hambiente cantando y tocando con un");
        println!("ukelele 'Over the Rainbow'. Por último, una chica presidiendo la mesa,");
        println!("con unos cuantos folios de lo que parece el guión del programa, se gira");
        println!("hacia a ti y te pregunta:");

        println!("- ¿Eres el siervo que Zoe dijo que enviaría para traernos su discurso?");

        println!("Asientes sin mucha confianza. Ella mira la hora y suspira.");

        println!("- Has hecho un buen trabajo llegando a tiempo, sin embargo creo que no");
        println!("va a servir de nada. El BEHTS se ha llevado nuestro ordenador.");

        println!("- Estamos intentando usar uno viejo de los que hay por aqui -añade el");
        println!("que estaba configurando la mesa de mezclas-, pero no tienen el sistema");
        println!("operativo instalado.");

        println!("-¡Si es que ya os vale! -comenta la chica que buscaba en los armarios-,");
        println!("tenemos el despacho lleno de cosas que deberiamos tirar u organizar desde");
        println!("hace meses, pero los discos de instalación están todos en una única caja");
        println!("con un rótulo 'CDs de instalación'.");

        println!("- Pues porque no estabas aquí hace años -objeta el de las cajas-, dos");
        println!("viajes en coche al punto limpio que hicimos.");

        println!("- Oye novato -se suma dirigiendose a tí el quinto gulero que había");
        println!("terminado su interpretación-, ¿No tendrás por casualidad un CD o");
        println!("un pincho con un sistema operativo?");

        return None;
    }

    fn do_action(&self, command: &str, state: &mut Box<S>) -> Option<String> {
        match command {
            "no" => {
                println!("'Me lo imaginaba. ¡Muy mal! ¡Siempre hay que ir con un CD de");
                println!("instalación en la mochila! A partir de ahora el que no lleve un cd");
                println!("de GNU/Linux encima, paga la cena'");
                println!(" ");
                println!("No se puede realizar el programa y el mensaje de Zoe no llega al");
                println!("resto del mundo.");

                return Some("game_over".to_string());
            },

            "sí" => { drive_contents(state); return Some("end_game".to_string()) }

            _ => {}
        }

        println!("¿Hola? ¿Tienes un sistema operativo, sí o no?");

        return None;
    }
}
