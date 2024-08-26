use rand::Rng;
use std::any::type_name;

fn roll_die() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

fn roll_dice(num_dice: u8) -> Vec<u8> {
    let mut dice: Vec<u8> = Vec::new();
    for _ in 0..num_dice {
        dice.push(roll_die() as u8);
    }
    dice.sort_by(|a, b| b.cmp(a));
    dice
}

//only take 6's or largest
fn run_turn_strategy_a() -> u8 {
    let mut selected_dice: Vec<u8> = Vec::new();
    let mut unselected_dice_count: u8 = 6;
    'outer: while unselected_dice_count > 0 {
        let dice = roll_dice(unselected_dice_count);
        for (i, die) in dice.into_iter().enumerate() {
            if die == 6 || i == 0 {
                selected_dice.push(die);
                unselected_dice_count -= 1;
            } else {
                continue 'outer;
            }
        }
    }
    selected_dice.iter().sum::<u8>()
}

//only take 6's, 5's or largest
fn run_turn_strategy_b() -> u8 {
    let mut selected_dice: Vec<u8> = Vec::new();
    let mut unselected_dice_count: u8 = 6;
    'outer: while unselected_dice_count > 0 {
        let dice = roll_dice(unselected_dice_count);
        for (i, die) in dice.into_iter().enumerate() {
            if die == 6 || i == 0 {
                selected_dice.push(die);
                unselected_dice_count -= 1;
            } else {
                continue 'outer;
            }
        }
    }
    selected_dice.iter().sum::<u8>()
}

//only take 6's, largest or with the last two dice, take both if total is over 30
fn run_turn_strategy_c() -> u8 {
    let mut selected_dice: Vec<u8> = Vec::new();
    let mut unselected_dice_count: u8 = 6;
    'outer: while unselected_dice_count > 0 {
        let dice = roll_dice(unselected_dice_count);
        let mut dice_iterator = dice.into_iter().enumerate();
        loop {
            let (i, die) = match dice_iterator.next() {
                Some(t) => t,
                None => break,
            };
            if die == 6 || i == 0 {
                selected_dice.push(die);
                unselected_dice_count -= 1;
            } else if unselected_dice_count == 2 {
                let (_, last) = dice_iterator.next().unwrap();
                if selected_dice.iter().sum::<u8>() + die + last > 30 {
                    selected_dice.push(die);
                    selected_dice.push(last);
                    unselected_dice_count -= 2;
                    break;
                } else {
                    continue 'outer;
                }
            }
        }
    }
    selected_dice.iter().sum::<u8>()
}

fn evaluate_strategy<F>(strategy: F, number_of_runs: usize)
where
    F: Fn() -> u8,
{
    let mut negative: usize = 0;
    let mut neutral: usize = 0;
    let mut positive: usize = 0;

    for _ in 0..number_of_runs {
        let score = strategy();
        match score {
            score if score > 30 => {
                positive += 1;
            }
            30 => {
                neutral += 1;
            }
            score if score < 30 => {
                negative += 1;
            }
            _ => unreachable!(),
        }
    }
    println!("Evaluating Strategy {}", type_name::<F>());
    println!(
        "Percentage chance of staying neutral or being positive: {:.4}%",
        ((neutral + positive) as f64 / number_of_runs as f64) * 100f64
    );
    println!(
        "Percentage chance of staying neutral: {:.4}%",
        (neutral as f64 / number_of_runs as f64) * 100f64
    );
    println!(
        "Percentage chance of being positive: {:.4}%",
        (positive as f64 / number_of_runs as f64) * 100f64
    );
    println!(
        "Percentage chance of being negative: {:.4}%",
        (negative as f64 / number_of_runs as f64) * 100f64
    );
    println!("");
}

fn evaluate_strategies(number_of_runs: usize) {
    evaluate_strategy(run_turn_strategy_a, number_of_runs);
    evaluate_strategy(run_turn_strategy_b, number_of_runs);
    evaluate_strategy(run_turn_strategy_c, number_of_runs);
}

fn main() {
    evaluate_strategies(10000000);
}
