// hexchk - a simple hexdump utility. reads from the specified file, or stdin if no file is specified. prints similar to hexdump -C

use std::{env, fs, io::Read};

fn hex_to_color(hex: u8, min: u8, max: u8) -> String {
    // hex will be 0x20 to 0x7e
    // generate an RGB value based on the hex value
    
    // we have 0x5e = 94 possible values
    // we want to map this to 255^3 = 16581375 possible colors
    // so we will map 94 to 16581375
    // 16581375 / 94 = 176225.266
    // so we will multiply the hex value by 176225.266 to get the RGB value

    if hex < min || hex > max {
        return "\x1b[0m".to_string();
    }

    let rgb = hex as u32 * (16581375u32 / (max-min) as u32);

    // now, we extract the red, green, and blue values
    let red = (rgb >> 16) & 0xff;
    let green = (rgb >> 8) & 0xff;
    let blue = rgb & 0xff;

    // now we return the color code
    format!("\x1b[38;2;{};{};{}m", red, green, blue)
}

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<String>>();
    let stdin = "/dev/stdin".to_string();
    let arg  = args.get(1).unwrap_or(&stdin);

    let mut file = std::io::BufReader::new(fs::File::open(arg)?);

    let mut buffer = [0; 16];
    let mut offset = 0;

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                print!("{:08x}  ", offset);
                offset += n;

                for i in 0..16 {
                    if i < n {
                        print!("{}{:02x} ", hex_to_color(buffer[i], 0x20, 0x7e), buffer[i]);
                    } else {
                        print!("   ");
                    }
                }

                print!(" \x1b[0m|");

                for i in 0..16 {
                    if i < n {
                        let c = buffer[i];
                        if c >= 0x20 && c <= 0x7e {
                            print!("{}", hex_to_color(c, 0x20, 0x7e));
                            print!("{}", c as char);
                        } else {
                            print!("\x1b[0m.");
                        }
                    } else {
                        print!(" ");
                    }
                }

                println!("{}|", "\x1b[0m");
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}