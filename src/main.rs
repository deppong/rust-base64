use std::io;
use std::env;
use std::convert::TryInto;

fn init_const() -> Vec<char> {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect::<Vec<char>>()
}

fn str_to_bin(s: String) -> String {
    let mut bin = "".to_string();

    for character in s.into_bytes() {
        bin += &format!("0{:b}", character);
    }

    bin
}

fn bin_to_dec(bin: String) -> String {
    let mut counter = 0;

    for (i, c) in bin.chars().rev().enumerate() {
        let index: u32 = 2;
        counter += c.to_digit(10).unwrap() * index.pow(i.try_into().unwrap());
    }
    let dec = format!("{}", counter);
    dec
}

fn dec_to_bin(mut dec: String) -> String {
    let mut bin = String::new();
    let mut bits = String::new();
    let mut dec: i32 = dec.parse::<i32>().unwrap();

    while dec > 0 {
        if dec % 2 == 0 {
            bits.push_str("0");
        } else {
            bits.push_str("1");
        }
        dec /= 2;
    }
    bits.chars().rev().collect::<String>()
}

fn bin_to_str(bin: String) -> String {
    let mut s = String::new(); 

   s 
}

fn bin_to_base64(bin: String) -> String {
    let mut raw_base64: Vec<String> = Vec::new();
    let alpha_base64: Vec<char> = init_const();
    let mut base64: String = String::new();
    let mut split_6bit = String::new();
    let mut pad_check = 0;

    /*
       Enumerate over the binary version of the string and every 6th bit push it to the 
       Vec<String>, and only push it when it's the sixth one, and reset the placeholder while
       otherwise just add the bit to the placeholder.
     */
    for (i, c) in bin.chars().enumerate() {
        if i % 6 == 0 && i != 0 {
            raw_base64.push(split_6bit);
            split_6bit = "".to_string();
        } 
        split_6bit += &format!("{}", c);
    }

    /*
       If the last placeholder for 6 bits was either 2 bits or 4 bits long (which are the only
       possible options, due to you having 3 8-bit strings being split into 24 bits, into 4
       6-bit strings) and add the padding, while set the pad check to remember to add the pad
       character "=" to the end of the base64 string.
    */
    if split_6bit.chars().count() == 2 {
        split_6bit += &format!("0000");
        pad_check = 2;
        raw_base64.push(split_6bit);
    } else if split_6bit.chars().count() == 4 {
        split_6bit += &format!("00");
        pad_check = 1;
        raw_base64.push(split_6bit);
    }

    /*
       Take the 6bit strings and convert each one from binary to decimal, and add it to the 
       final string using the decimal value as the index to the table of each base64 alphabet
       value as a string, and if the pad check was 1 or 2 add the appropriate amount of padding
    */
    for i in raw_base64 {
        let index = bin_to_dec(i);
        let index: usize = index.parse::<usize>().unwrap();
        base64 += &format!("{}", alpha_base64[index].to_string());
    }
    if pad_check == 1 {
        base64 += &format!("=");
    } else if pad_check == 2 {
        base64 += &format!("==");
    }
    base64
}

fn base64_to_bin(base64: String) -> String {
    let mut bin: String = String::new();
    let alpha_base64: Vec<char> = init_const();
    let mut dec: Vec<String> = Vec::new();

    /*
        Convert the base64 to dec by finding the character in the base64 table and adding the 
        index.
    */ 
    for c in base64.chars() {
        for (i, b64) in alpha_base64.iter().enumerate() {
            if c == *b64 {
                dec.push(i.to_string());
            }
        }
    }

    /*
       Convert from decimal to binary in 6 bit sequences.
    */
    for c in dec {
        bin += &format!("{}", dec_to_bin(c));
    }
    bin
}

fn encode(s: String) -> String {
    bin_to_base64(str_to_bin(s))
}

fn decode(s: String) -> String {
    base64_to_bin(bin_to_str(s))
}

fn main() {
    let arg_len: Vec<String> = env::args().collect();
    let arg_len = arg_len.len();
    let option = env::args().nth(1).expect("No option given, (-e to encode string, -d to decode)");
    let mut text = String::new();

    /*
        if the option is to encode skip the 1st arg, and -e and as long as it's not the last arg
        add the arg to a string with a space, and if it's the last arg then add it to the string
        with no space, then print the text encoded.
    */
    if option == "-e" {
        for (i, arg) in env::args().skip(1).enumerate() {
            if arg != "-e" && i != arg_len  {
               if arg_len > 3 {
                   text += &format!("{} ", arg);
               } else {
                text += &format!("{}", arg);
                }

            }
        }
        println!("{}", encode(text));
        
    /*
       If the option is to decode you only need to accept one arg, so skip the 1st arg and -d
       and just decode that arg and print it. Decoding does not require spaces, which is why
       the formatting doesn't require to append all args to a string to process with the spaces.
    */
    } else if option == "-d" {
        for arg in env::args().skip(1) {
            if arg != "-d" {
                println!("{}", base64_to_bin(arg));
            }
        }
    }
    

}


