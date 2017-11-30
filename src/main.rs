use std::io;
//git push -u origin master
struct Onus {
    name: String,
    created: String,
    visible: bool,
    svisible: bool,
    rvisible: bool,
    ivisible: bool,
    stvisible: bool,
    started: String,
    finished: String,
    span: String,
    time_worked: String,
    time_interupted: String,
    actual_time_worked: String,
    seq: Vec<Sequence>,
    rem: Vec<String>,
    int: Vec<Interupts>,
}

impl Onus {
    fn new(
        name: String,
        created: String,
        visible: bool,
        svisible: bool,
        rvisible: bool,
        ivisible: bool,
        stvisible: bool,
        started: String,
        finished: String,
        span: String,
        time_worked: String,
        time_interupted: String,
        actual_time_worked: String,
    ) -> Onus {
        Onus {
            name: name,
            created: created,
            visible: visible,
            svisible: svisible,
            rvisible: rvisible,
            ivisible: ivisible,
            stvisible: stvisible,
            started: started,
            finished: finished,
            span: span,
            time_worked: time_worked,
            time_interupted: time_interupted,
            actual_time_worked: actual_time_worked,
            seq: Vec::new(),
            rem: Vec::new(),
            int: Vec::new(),
        }
    }
}

struct Sequence {
    name: String,
    status: String,
    started: String,
    stoped: String,
    duration: String,
}

impl Sequence {
    fn new(
        name: String,
        status: String,
        started: String,
        stoped: String,
        duration: String,
    ) -> Sequence {
        Sequence {
            name: name,
            status: status,
            started: started,
            stoped: stoped,
            duration: duration,
        }
    }
}

struct Interupts {
    name: String,
    status: String,
    started: String,
    stoped: String,
    duration: String,
}

impl Interupts {
    fn new(
        name: String,
        status: String,
        started: String,
        stoped: String,
        duration: String,
    ) -> Interupts {
        Interupts {
            name: name,
            status: status,
            started: started,
            stoped: stoped,
            duration: duration,
        }
    }
}

fn main() {
    // globals
    let mut debug = false;
    let mut onera: Vec<Onus> = vec![];
    let mut msg = vec![];
    let mut ban = 54u8;
    let mut indt = 4u8;

    // start
    msg.push(String::from("start Onera"));

    // populating data structure
    onera = test_data(onera);

    msg.push(String::from("start loop"));

    // main loop
    loop {
        // TO_DO: Clear Screen or use curses

        //old_io::stdout().write_all("\x1b[2Joo\x1b[1;1H".as_bytes()).unwrap();
        //Command::new("cls").stdin(Stdio::piped()).spawn().unwrap();
        //std::process::Command::new("cls").status().expect("failed");
        //let mut options = std::run::ProcessOptions::new();
        //let process = std::run::Process::new("ls", &[your, arguments], options);
        //Command::new("cmd.exe cls").status().unwrap().success();
        //assert!(Command::new("cls").status().or_else(|_| Command::new("clear")).unwrap().success());
        //sntd::process::Command::new("cls").stdatus().unwrap().success();
        //print!("{}[2J", 27 as char);

        // header
        do_header(ban, onera.len() as u32);

        // Onera
        for (i, x) in onera.iter().enumerate() {
            if x.visible == false {
                continue;
            };

            for _n in 0..ban {
                print!("-");
            }

            print!("\n");

            println!("\n{} {}\n", { i + 1 }, x.name);

            // seq
            if onera[i].svisible {
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

                print!("\n");

                for (j, y) in onera[i].seq.iter().enumerate() {
                    for _n in 0..indt {
                        print!(" ");
                    }

                    println!("  {} {} {}", { j + 1 }, y.status, y.name);
                }
                print!("\n");
            }

            // interuptions
            if onera[i].ivisible {
                for _n in 0..indt {
                    print!(" ");
                }

                print!("Interupts ({})\n", onera[i].int.len());

                for _n in 0..indt {
                    print!(" ");
                }

                for _n in 0..ban - indt {
                    print!("-");
                }

                print!("\n");

                for (l, a) in onera[i].int.iter().enumerate() {
                    for _n in 0..indt {
                        print!(" ");
                    }

                    println!("  {} {} {}", { l + 1 }, a.status, a.name);
                }
                print!("\n");
            }

            // remarks
            if onera[i].rvisible {
                for _n in 0..indt {
                    print!(" ");
                }

                print!("Remarks\n");

                for _n in 0..indt {
                    print!(" ");
                }

                for _n in 0..ban - indt {
                    print!("-");
                }

                print!("\n");

                for (k, z) in onera[i].rem.iter().enumerate() {
                    for _n in 0..indt {
                        print!(" ");
                    }

                    println!("  {}.) {}", { k + 1 }, z);
                }

                print!("\n");
            }

            //statistics
            if onera[i].stvisible {
                do_stats(
                    indt,
                    ban,
                    onera[i].created.to_string(),
                    onera[i].started.to_string(),
                    onera[i].finished.to_string(),
                    onera[i].span.to_string(),
                    onera[i].time_worked.to_string(),
                    onera[i].time_interupted.to_string(),
                    onera[i].actual_time_worked.to_string(),
                );
            }
        }

        // do footer
        do_footer(ban, msg);

        msg = vec![];

        // get input
        let mut cmds = ask("v by your command" as &str);

        if cmds.len() == 0 {
            cmds.push(String::from("no command entered"));
        }

        msg.push(String::from("input received"));

        if debug == true {
            for x in &cmds {
                msg.push(x.to_string());
            }
        }

        // process commands
        match &cmds[0] as &str {
            "db" => debug = toggle_debug(debug), // toggle debug

            "ab" => {
                // adjust banner
                if cmds[1].parse::<u8>().unwrap() < 46 {
                    ban = 46;
                    continue;
                } else {
                    ban = cmds[1].parse::<u8>().unwrap();
                }
            }

            "ai" => indt = cmds[1].parse::<u8>().unwrap(), // adjust indent

            "si" => msg.push(format!("banner = {} indent = {}", ban, indt)), // show current banner and indent values

            "ex" => break, // exit program

            "tm" => msg.push(String::from("test..")), // test debug message

            "ha" => {
                // hide all onera
                onera[1].visible = false;
                onera[0].visible = false;
                msg.push(String::from("Hide all"));
            }

            "h1" => {
                // hide first onera
                onera[0].visible = false;
                msg.push(String::from("Hide first"));
            }

            "h2" => {
                // hide second onera
                onera[1].visible = false;
                msg.push(String::from("Hide second"));
            }

            "sa" => {
                // show all onera
                onera[1].visible = true;
                onera[0].visible = true;
                msg.push(String::from("Show all"));
            }

            "s1" => {
                // show first onera
                onera[0].visible = true;
                msg.push(String::from("Show first"));
            }

            "s2" => {
                // show second onera
                onera[1].visible = true;
                msg.push(String::from("Show second"));
            }

            "has" => {
                // hide all seq
                onera[0].svisible = false;
                onera[1].svisible = false;
                msg.push(String::from("Hide seq"));
            }

            "sas" => {
                // show all seq
                onera[0].svisible = true;
                onera[1].svisible = true;
                msg.push(String::from("Show seq"));
            }

            "har" => {
                // hide all remarks
                onera[0].rvisible = false;
                onera[1].rvisible = false;
                msg.push(String::from("Hide rem"));
            }

            "hai" => {
                // hide all interuptions
                onera[0].ivisible = false;
                onera[1].ivisible = false;
                msg.push(String::from("Hide int"));
            }

            "sar" => {
                // show all remarks
                onera[0].rvisible = true;
                onera[1].rvisible = true;
                msg.push(String::from("Show rem"));
            }

            "sai" => {
                // show all ineruptions
                onera[0].ivisible = true;
                onera[1].ivisible = true;
                msg.push(String::from("Show int "));
            }

            "hast" => {
                // hide all statistics
                onera[0].stvisible = false;
                onera[1].stvisible = false;
                msg.push(String::from("Hide stats"));
            }

            "sast" => {
                // show all statistics
                onera[0].stvisible = true;
                onera[1].stvisible = true;
                msg.push(String::from("Show stats"));
            }

            _ => msg.push(String::from("unknown command!")), //catch all
        }
    }
}

fn toggle_debug(flag: bool) -> bool {
    if flag {
        false
    } else {
        true
    }
}

fn ask(question: &str) -> Vec<String> {
    println!("{}", question);

    let mut cmd = String::new();

    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");

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

    print!("\n");
}

fn do_section(name: String, ban: u8, indt: u8, lng: u32, onera: Vec<Onus>, i: usize) {
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

    print!("\n");

    for (j, y) in onera[i].seq.iter().enumerate() {
        for _n in 0..indt {
            print!(" ");
        }

        println!("  {} {} {}", { j + 1 }, y.status, y.name);
    }
}

fn do_stats(
    indt: u8,
    ban: u8,
    created: String,
    started: String,
    finished: String,
    span: String,
    time_worked: String,
    time_interupted: String,
    actual_time_worked: String,
) {
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

    print!("\n");

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Created           : {}", created);

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Started           : {}", started);

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Finsihed          : {}", finished);

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Span              : {}", span);

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Time worked       : {}", time_worked);

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Time interupted   : {}", time_interupted);

    for _n in 0..indt {
        print!(" ");
    }

    println!("  Actual time wored : {}\n", actual_time_worked);
}

fn do_footer(ban: u8, msg: Vec<String>) {
    // footer
    for _n in 0..ban {
        print!("-");
    }

    print!("\n");

    println!("Status:{:?}", msg);

    for _n in 0..ban {
        print!("-");
    }

    print!("\n");
}

fn test_data(mut onera: Vec<Onus>) -> Vec<Onus> {
    onera.push(Onus::new(
        "Task name".to_string(),
        "--/--/----".to_string(),
        true,
        true,
        true,
        true,
        true,
        "--/--/----".to_string(),
        "--/--/----".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
    ));

    onera[0].seq.push(Sequence::new(
        "my seq".to_string(),
        "-".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
    ));

    onera[0].rem.push("my rem".to_string());

    onera[0].int.push(Interupts::new(
        "my int".to_string(),
        "*".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
    ));

    onera.push(Onus::new(
        "Second task name".to_string(),
        "--/--/----".to_string(),
        true,
        true,
        true,
        true,
        true,
        "--/--/----".to_string(),
        "--/--/----".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
        "0 days 0 hours 0 minutes".to_string(),
    ));

    onera[1].seq.push(Sequence::new(
        "my seq second".to_string(),
        "-".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
    ));


    onera[1].rem.push("my rem second".to_string());

    onera[1].int.push(Interupts::new(
        "my int second".to_string(),
        "*".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
    ));

    onera[0].seq.push(Sequence::new(
        "anothersecond".to_string(),
        "-".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
    ));

    onera[0].rem.push("another rem".to_string());

    onera[0].int.push(Interupts::new(
        "another int".to_string(),
        "-".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
        "00:00:00".to_string(),
    ));

    onera
}
