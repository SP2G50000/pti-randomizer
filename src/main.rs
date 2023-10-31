use clap::{Parser};
use rand::{thread_rng, Rng};
use element::{AllowValue, ElementType, TestElement};

mod element;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    //Sets the amount of input elements to draw. (-1 to randomize.)
    #[arg(short, long, default_value_t = -1)]
    input_limit: i32,
    //Sets the maximum number of major elements to draw. (-1 to randomize.)
    #[arg(short, long, default_value_t = 3)]
    major_limit: i32,
    //Draw Rexaura and Portal 1 elements instead of Portal 2 elements.
    #[arg(short, long, default_value_t = false)]
    rexaura: bool,
    //Allows Conversion Gel to be drawn (Always Disabled if Rexaura Mod is enabled.)
    #[arg(short, long, default_value_t = false)]
    conversion_gel: bool,
    //Allows Auto Portals to be drawn.
    #[arg(short, long, default_value_t = false)]
    auto_portals: bool,
}

fn main() {
    let args = Args::parse();

    print_introduction(&args);

    let mut rng = thread_rng();

    //The list of elements. Each element is known at compile time so a standard array is used.
    let element_list = [
        //Major Portal 2 Elements
        TestElement::p2_major("Bounce Gel"),
        TestElement::conversion_gel(),
        TestElement::p2_major("Speed Gel"),
        TestElement::p2_major("Funnel"),
        TestElement::p2_major("Light Bridge"),
        TestElement::p2_major("Faith Plate"),
        //Laser Elements
        TestElement::laser_emitter(),
        TestElement::laser("Laser Catcher", true),
        TestElement::laser("Laser Relay", true),
        TestElement::laser("Reflection Cube", false),
        //Major Rexaura Elements
        TestElement::pellet_emitter(),
        TestElement::pellet("Pellet Catcher", true),
        TestElement::pellet("Pellet Destroyer", false),
        TestElement::pellet("Flux Field", true),
        TestElement::pellet("Cube Deflector", false),
        //Minor Elements
        TestElement::auto_portal(),
        TestElement::minor("Button", true),
        TestElement::minor("Storage Cube", false),
        TestElement::minor("Fizzler", false),
        TestElement::minor("Laser Field", false),
        TestElement::minor("Lift", false),
        TestElement::minor("Angled Panel", false),
        TestElement::minor("Flip Panel", false),
        TestElement::minor("Turret", false),
        //Portal 2 Minor Elements
        TestElement::p2_minor("Track Platform", false),
        //Rexaura Minor Elements
        TestElement::rex_minor("Unstationary Scaffold", false),
    ];

    let mut random_elements: Vec<TestElement> = Vec::new();

    let number_of_inputs: i32 = if args.input_limit < 0 {
        rng.gen_range(2..5)
    } else {
        args.input_limit
    };

    let number_of_majors: i32 = if args.major_limit < 0 {
        rng.gen_range(2..3)
    } else {
        args.major_limit
    };

    let mut current_inputs: i32 = 0;
    let mut current_majors: i32 = 0;
    let mut has_pellet_emitter: bool = false;
    let mut has_laser_emitter: bool = false;
    while current_inputs <= number_of_inputs {
        let index = rng.gen_range(0..(element_list.len() - 1));

        if let AllowValue::Rexaura = element_list[index].allowed {
            if !args.rexaura {
                continue;
            }
        } else if let AllowValue::Portal2 = element_list[index].allowed {
            if args.rexaura {
                continue;
            }
        }

        if let ElementType::Pellet = element_list[index].element_type {
            if element_list[index].is_major {
                if current_majors <= number_of_majors {
                    has_pellet_emitter = true;
                    current_majors += 1;
                } else {
                    continue;
                }
            } else if !has_pellet_emitter {
                continue;
            }
        }

        if let ElementType::Laser = element_list[index].element_type {
            if element_list[index].is_major {
                if current_majors <= number_of_majors {
                    has_laser_emitter = true;
                    current_majors += 1;
                } else {
                    continue;
                }
            } else if !has_laser_emitter {
                continue;
            }
        }

        if let ElementType::ConversionGel = element_list[index].element_type {
            if !args.conversion_gel {
                continue;
            }
        }

        if let ElementType::AutoPortal = element_list[index].element_type {
            if !args.auto_portals {
                continue;
            }
        }

        if element_list[index].is_major {
            match element_list[index].element_type {
                ElementType::Laser => {},
                ElementType::Pellet => {},
                _ => {
                    if current_majors <= number_of_majors {
                        current_majors += 1;
                    } else {
                        continue;
                    }
                }
            }
        }
        //Adds the element to the list and counts if it is an input.
        if element_list[index].is_input == true {
            current_inputs += 1;
        }
        random_elements.push(element_list[index].clone());
    }

    println!("\nYour Element List:\n");
    for i in random_elements {
        println!("{}", i.name);
    }
}



fn print_introduction(args: &Args) {
    println!("Welcome to SP2G's PTI Randomizer! Version 2.0!");
    println!("These are the current settings, run with --help for arguments.");
    println!("---------------------------------------------------------------");
    println!("Input Limit    : {}", args.input_limit);
    println!("Major Limit    : {}", args.major_limit);
    println!("Rexaura Mode   : {}", args.rexaura);
    if !args.rexaura {
        println!("Conversion Gel : {}", args.conversion_gel);
    }
    println!("Auto Portals   : {}", args.auto_portals);
    if args.rexaura {
        println!("NOTE: Rexaura mode disables Conversion Gel.");
    }
    println!("---------------------------------------------------------------");
}
