use std::io;
fn main() {
    let mut summ : i128 = 0;
    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line);
        if bytes_read.is_ok_and(|n| n == 0) {
            println!("{}", summ);
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
            match trimmed.parse::<i128>() {
                Ok(num) => {
                    if num > 0 {
                        summ += num;
                    } else if num == -1 {
                        println!("{}", summ);
                        break
                    } else {
                        println!("NaN");
                        break
                    }
                }
                Err(_) => {
                    println!("NaN");
                    break;
                }
            }
    }
}
