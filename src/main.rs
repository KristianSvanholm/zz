use chrono::{Duration, Timelike};

fn main() {
   
    let cycle_count = 6;
    let now = chrono::offset::Local::now();
    let nod_time = Duration::minutes(15);
    let cycle_time = Duration::minutes(90);

    for i in 1..cycle_count+1 {
        let cycles = cycle_time.checked_mul(i).expect("ooh");
        let itt = now + nod_time + cycles;
        println!("{:0>2}:{:0>2}:{:0>2}", itt.hour(), itt.minute(), itt.hour());
    }
}
