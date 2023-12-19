use std::env;

struct Phil {
    number_of_threads: u32,
    // time_to_die: u32,
    // time_to_eat: u32,
    // time_to_sleep: u32,
    // number_of_times_each_philosopher_must_eat: u32,
}

fn initialize_phil(args: Vec<String>, _len: usize) -> Phil {
    let phil = Phil {
        number_of_threads: args[1].to_string().parse::<u32>().unwrap()
        // time_to_die: args[2],
        // time_to_eat: args[3],
        // time_to_sleep: args[4],
    };
    phil
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    println!("len: {}", len);
    // if len != 5 && len != 6 {
    //     println!("Invalid number of parameters");
    //     return ;
    // }
    let phil = initialize_phil(args.clone(), len);
    // println!("{}", &phil.number_of_threads);

    println!("number_of_threads: {}", &phil.number_of_threads);
    // println!("time_to_die: {}", &args[2]);
    // println!("time_to_eat: {}", &args[3]);
    // println!("time_to_sleep: {}", &args[4]);
    // if len == 6 {
    //     println!("number_of_times_each_philosopher_must_eat: {}", &args[5]);
    // }
}
