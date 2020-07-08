use std::io;
use std::vec;
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

fn bin_to_base64(bin: String) -> String {
    let mut raw_base64: Vec<String> = Vec::new();
    let alpha_base64: Vec<char> = init_const();
    let mut base64: String = String::new();
    let mut split_6bit = String::new();
    let mut pad_check = 0;

    for (i, c) in bin.chars().enumerate() {
        if i % 6 == 0 && i != 0 {
            raw_base64.push(split_6bit);
            split_6bit = "".to_string();
        } 
        split_6bit += &format!("{}", c);
    }

    if split_6bit.chars().count() == 2 {
        split_6bit += &format!("0000");
        pad_check = 2;
        raw_base64.push(split_6bit);
    } else if split_6bit.chars().count() == 4 {
        split_6bit += &format!("00");
        pad_check = 1;
        raw_base64.push(split_6bit);
    }

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
    println!("{}", pad_check);
    base64
}

fn main() {
    let s1 = String::from("ab@yz");
    let s2b = str_to_bin(s1);
    
    println!("{:?}", s2b);
    println!("{:?}", bin_to_base64(s2b));
    

}

