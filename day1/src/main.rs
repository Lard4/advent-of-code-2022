use std::{env, fs::File, io::{BufReader, BufRead}, path::Path, thread::current, time::Instant};


fn most_cals(reader: BufReader<File>) {
    let now = Instant::now();

    /////// begin timing 
    let mut most_cals: f32 = 0.0;
    let mut current_cals: f32 = 0.0;
    for line in reader.lines() {
        let amount = line.unwrap();
        if amount.is_empty() {
            if (current_cals) > most_cals {
                most_cals = current_cals;
            }
            current_cals = 0.0;
        } else {
            current_cals += amount.parse::<f32>().unwrap();
        }
    }
    ////// end timing
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Most cals is {}", most_cals);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = Path::new(args.get(1).expect("An input_file_path parameter"));
    let input_file: File = File::open(input_file_path).expect("A valid input file");
    let reader = BufReader::new(input_file);

    most_cals(reader);
}
