use std::io::BufRead;

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some(part @ ("1" | "2")) => {
            let lines = std::io::stdin()
                .lock()
                .lines()
                .map(Result::unwrap)
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            match part {
                "1" => {
                    let count = lines.windows(2)
                        .filter(|window| match window {
                            &[a, b] => a < b,
                            _       => unreachable!(),
                        })
                        .count();

                    println!("{}", count);
                }
                "2" => {
                    let count = lines.windows(4)
                        .filter(|window| match window {
                            &[a, b, c, d] => (a + b + c) < (b + c + d),
                            _       => unreachable!(),
                        })
                        .count();

                    println!("{}", count); 
                }
                _ => unreachable!(),
            }
                
        }
        _ => {
            let program = std::env::args()
                .next()
                .unwrap_or("<program>".to_string());

            eprintln!("Usage: {} <1|2>", program);
            return;
        }
    }

    
}
