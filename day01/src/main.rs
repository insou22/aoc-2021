use std::io::BufRead;

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some(part @ ("1" | "2")) => {
            let input = std::io::stdin()
                .lock()
                .lines()
                .map(Result::unwrap)
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let count = match part {
                "1" => input
                    .windows(2)
                    .filter(|window| match window {
                        &[a, b] => a < b,
                        _ => unreachable!(),
                    })
                    .count(),
                "2" => input
                    .windows(4)
                    .filter(|window| match window {
                        &[a, b, c, d] => (a + b + c) < (b + c + d),
                        _ => unreachable!(),
                    })
                    .count(),
                _ => unreachable!(),
            };

            println!("{}", count);
        }
        _ => {
            let program = std::env::args().next().unwrap_or("<program>".to_string());

            eprintln!("Usage: {} <1|2>", program);
            return;
        }
    }
}
