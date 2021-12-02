use std::io::BufRead;

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some(part @ ("1" | "2")) => {
            let lines = std::io::stdin()
                .lock()
                .lines()
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            let input = lines.iter()
                .map(|line| line.split_once(" ").unwrap())
                .map(|(direction, amount)| (direction, amount.parse::<i32>().unwrap()))
                .collect::<Vec<_>>();

            let (position, depth) = match part {
                "1" => {
                    let position = input.iter()
                        .filter_map(|&(direction, amount)| if direction == "forward" { Some(amount) } else { None })
                        .sum();

                    let depth = input.iter()
                        .filter_map(|&(direction, amount)| match direction {
                            "up"   => Some(-amount),
                            "down" => Some(amount),
                            _      => None,
                        })
                        .sum();

                   (position, depth) 
                }
                "2" => {
                    let (position, depth, _) = input.iter()
                        .fold((0, 0, 0), |(position, depth, aim), &(direction, amount)| {
                            match direction {
                                "forward" => {
                                    (position + amount, depth + aim * amount, aim)
                                }
                                "up" => {
                                    (position, depth, aim - amount)
                                }
                                "down" => {
                                    (position, depth, aim + amount)
                                }
                                _ => unreachable!(),
                            }
                        });

                    (position, depth)
                }
                _ => unreachable!(),
            };
            
            let product = position * depth;
            println!("{}", product);
        }
        _ => {
            let program = std::env::args().next().unwrap_or("<program>".to_string());

            eprintln!("Usage: {} <1|2>", program);
            return;
        }
    }
}
