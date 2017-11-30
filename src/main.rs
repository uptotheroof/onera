extern crate chrono;

mod interrupts;
mod messages;
mod onus;
mod sequence;
mod time;

use messages::Messages;
use onus::Onus;
use std::io;

fn main() {
    // globals
    let mut debug = false;
    let mut onera = vec![];
    let mut msg = Messages::new();
    let mut ban = 54u8;
    let mut indt = 4u8;

    // start
    msg.push(String::from("start Onera"));

    // populating data structure
    test_data(&mut onera);

    msg.push(String::from("start loop"));

    // main loop
    loop {
        do_header(ban, onera.len() as u32);

        // Onera
        for (i, x) in onera.iter().enumerate() {
            if !x.visible() {
                continue;
            }

            for _n in 0..ban {
                print!("-");
            }

            println!();
            println!("\n{} {}\n", i + 1, x.name);

            // seq
            if x.sequence_visible() {
                for _n in 0..indt {
                    print!(" ");
                }

                print!("Sequences ({})\n", onera[i].seq.len());

                for _n in 0..indt {
                    print!(" ");
                }

                for _n in 0..ban - indt {
                    print!("-");
                }

                println!();

                for (j, y) in onera[i].seq.iter().enumerate() {
                    for _n in 0..indt {
                        print!(" ");
                    }

                    println!(
                        "  {} {} {}",
                        {
                            j + 1
                        },
                        y.status,
                        y.name
                    );
                }
                println!();
            }

            // interuptions
            if x.interruptions_visible() {
                for _n in 0..indt {
                    print!(" ");
                }

                print!("Interrupts ({})\n", onera[i].int.len());

                for _n in 0..indt {
                    print!(" ");
                }

                for _n in 0..ban - indt {
                    print!("-");
                }

                println!();

                for (l, a) in onera[i].int.iter().enumerate() {
                    for _n in 0..indt {
                        print!(" ");
                    }

                    println!(
                        "  {} {} {}",
                        {
                            l + 1
                        },
                        a.status,
                        a.name
                    );
                }
                println!();
            }

            // remarks
            if x.remarks_visible() {
                for _n in 0..indt {
                    print!(" ");
                }

                println!("Remarks");

                for _n in 0..indt {
                    print!(" ");
                }

                for _n in 0..ban - indt {
                    print!("-");
                }

                println!();

                for (k, z) in onera[i].rem.iter().enumerate() {
                    for _n in 0..indt {
                        print!(" ");
                    }

                    println!(
                        "  {}.) {}",
                        {
                            k + 1
                        },
                        z
                    );
                }

                println!();
            }

            //statistics
            if onera[i].stats_visible() {
                do_stats(indt, ban, &onera[i]);
            }
        }

        // do footer
        do_footer(ban, &msg);

        msg.clear();

        // get input
        let mut cmds = ask("v by your command" as &str);

        if cmds.is_empty() {
            cmds.push(String::from("no command entered"));
        }

        msg.push(String::from("input received"));

        if debug {
            for x in &cmds {
                msg.push(x.to_string());
            }
        }

        // process commands
        match &cmds[0] as &str {
            "db" => debug = toggle_debug(debug),                                 // toggle debug                                

            "ab" => {
                // adjust banner
                if cmds[1].parse::<u8>().unwrap() < 46 {
                    ban = 46;
                    continue;

                } else {
                    ban = cmds[1].parse::<u8>().unwrap();
                }
            }

            "ai" => indt = cmds[1].parse::<u8>().unwrap(),                          // adjust indent

            "si" => msg.push(format!("banner = {} indent = {}", ban, indt)), // show current banner and indent values

            "ex" => break,                                                          // exit program

            "tm" => msg.push(String::from("test..")),                         // test debug message

            "ha" => {
                onera.iter_mut().for_each(|onus| onus.set_visibility(false));
                msg.push(String::from("Hide all"));
            }

            "h1" => {
                onera.get_mut(0).map(|onus| onus.set_visibility(false));
                msg.push(String::from("Hide first"));
            }

            "h2" => {
                onera.get_mut(1).map(|onus| onus.set_visibility(false));
                msg.push(String::from("Hide second"));
            }

            "sa" => {
                onera.iter_mut().for_each(|onus| onus.set_visibility(true));
                msg.push(String::from("Show all"));
            }

            "s1" => {
                onera.get_mut(0).map(|onus| onus.set_visibility(true));
                msg.push(String::from("Show first"));
            }

            "s2" => {
                onera.get_mut(1).map(|onus| onus.set_visibility(true));
                msg.push(String::from("Show second"));
            }

            "has" => {
                onera.iter_mut().for_each(
                    |onus| onus.set_sequence_visibility(false),
                );
                msg.push(String::from("Hide seq"));
            }

            "sas" => {
                onera.iter_mut().for_each(
                    |onus| onus.set_sequence_visibility(true),
                );
                msg.push(String::from("Show seq"));
            }

            "har" => {
                onera.iter_mut().for_each(
                    |onus| onus.set_remark_visibility(false),
                );
                msg.push(String::from("Hide rem"));
            }

            "hai" => {
                onera.iter_mut().for_each(|onus| {
                    onus.set_interruption_visibility(false)
                });
                msg.push(String::from("Hide int"));
            }

            "sar" => {
                onera.iter_mut().for_each(
                    |onus| onus.set_remark_visibility(true),
                );
                msg.push(String::from("Show rem"));
            }

            "sai" => {
                onera.iter_mut().for_each(|onus| {
                    onus.set_interruption_visibility(true)
                });
                msg.push(String::from("Show int "));
            }

            "hast" => {
                onera.iter_mut().for_each(
                    |onus| onus.set_stats_visibility(false),
                );
                msg.push(String::from("Hide stats"));
            }

            "sast" => {
                onera.iter_mut().for_each(
                    |onus| onus.set_stats_visibility(true),
                );
                msg.push(String::from("Show stats"));
            }

            _ => msg.push(String::from("unknown command!")),               //catch all
        }
    }
}

fn toggle_debug(flag: bool) -> bool {
    !flag
}

fn ask(question: &str) -> Vec<String> {
    println!("{}", question);

    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd).expect(
        "Failed to read line",
    );

    cmd.split_whitespace().map(|s| s.to_owned()).collect()
}

fn do_header(ban: u8, lng: u32) {
    print!("\nOnera ({})", lng);

    for _n in 0..ban - 38 {
        print!(" ");
    }

    println!("|programmed by Larry McClenon");

    for _n in 0..ban {
        print!("-");
    }

    println!();
}

fn do_section(name: &str, ban: u8, indt: u8, lng: u32, onera: &[Onus], i: usize) {
    // section
    for _n in 0..indt {
        print!(" ");
    }

    print!("{} ({})\n", name, lng);

    for _n in 0..indt {
        print!(" ");
    }

    for _n in 0..ban - indt {
        print!("-");
    }

    println!();

    for (j, y) in onera[i].seq.iter().enumerate() {
        for _n in 0..indt {
            print!(" ");
        }

        println!(
            "  {} {} {}",
            {
                j + 1
            },
            y.status,
            y.name
        );
    }
}

fn do_stats(indt: u8, ban: u8, onus: &Onus) {
    for _n in 0..indt {
        print!(" ");
    }

    println!("Statistics");

    for _n in 0..indt {
        print!(" ");
    }

    for _n in 0..ban - indt {
        print!("-");
    }

    println!();

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Created           : {}", onus.created);

    for _n in 0..indt {
        print!(" ");
    }

    if let Some(started) = onus.started {
        println!("  Started           : {}", started);
    }

    for _n in 0..indt {
        print!(" ");
    }

    if let Some(finished) = onus.finished {
        println!("  Finished          : {}", finished);
    }

    for _n in 0..indt {
        print!(" ");
    }

    if let Some(span) = onus.span {
        println!("  Span              : {}", span);
    }
}

fn do_footer(ban: u8, messages: &Messages) {
    // footer
    for _n in 0..ban {
        print!("-");
    }

    println!();

    println!("Status:{:?}", messages.get());

    for _n in 0..ban {
        print!("-");
    }

    println!();
}

fn test_data(onera: &mut Vec<Onus>) {
    use interrupts::Interrupts;
    use onus::OnusBuilder;
    use sequence::Sequence;

    let mut task = OnusBuilder::new("Task name").build();

    task.seq.push(Sequence::new("my seq", "-"));
    task.rem.push("my rem".to_string());
    task.int.push(Interrupts::new("my int", "*"));

    onera.push(task);

    let mut task = OnusBuilder::new("Second task name").build();

    task.seq.push(Sequence::new("my seq second", "-"));
    task.rem.push("my rem second".to_string());
    task.int.push(Interrupts::new("my int second", "*"));

    onera.push(task);
}
