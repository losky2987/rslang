use std::env;

fn calc_coins(coin: u64, amount: u64) -> u64 {
    let count = amount / coin;
    println!("{:>5}ct: {}", coin, count);
    amount - count * coin
}

fn read_one_argument() -> Result<u64, ()> {
    let arg_count = env::args().len() - 1;
    if arg_count != 1 {
        eprintln!("Error: Please provide exactly one argument.");
        return Err(());
    }

    let arg = env::args().nth(1).unwrap();
    match arg.parse::<u64>() {
        Ok(value) => Ok(value),
        Err(_) => {
            eprintln!("Error: Argument must be a valid unsigned integer.");
            Err(())
        }
    }
}

fn main() {
    let ret = read_one_argument();
    if ret.is_err() {
        return;
    }
    let mut amount = ret.unwrap();
    println!("You provided the amount: {}", amount);

    let coins = [50, 20, 5, 2, 1];
    for c in coins {
        amount = calc_coins(c, amount);
    }
}
