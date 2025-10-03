use std::{fs::File, io::{self, Write}};
use rand::Rng;

fn main() {

    let result = loop{
        let mut guess = String::new();
        println!("Podaj liczbę!");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let mut number: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("{}", error);
                break true;
            }
        };

        if number == 0 {
            break false;
        }
        
        number += rand::thread_rng().gen_range(0..=5);
        println!("Nowa wartość x: {}", number);

        let array:[u64; 10] = pow_table(number);
        println!("{:?}", array);
        let mut collatz_res_arr = [false; 10];
        for i in 0..10 {
            collatz_res_arr[i] = is_collatz(array[i], 100);
        } 
        println!("{:?}", collatz_res_arr);

        save_to_file(collatz_res_arr, "xyz.txt".to_string());


    };

    if result {
        println!("Pętla zakończona z powodu błędu.")
    }
    else {
        println!("Pętla zakończona z woli użytkownika.")
    }

}

fn pow_table<const LEN: usize>(x: u64) -> [u64; LEN] {
    let mut arr = [x; LEN];
    let mut val = x;
    for item in arr.iter_mut() {
        *item = val;
        val *= x;
    }
    arr
}

fn is_collatz(mut x: u64, limit: u32) -> bool {
    for _ in 0..=limit {
        x = collatz(x);
        if x == 1 {
            return true;
        }
    }
    false
}

fn collatz(x: u64) -> u64 {
    if x % 2 == 1 {
        return 3 * x + 1;
    }
    x/2
}

fn save_to_file(arr: [bool; 10], file_name: String) -> std::io::Result<()>{
    let mut file = File::create(file_name).expect("Nie udało się otworzyć/utworzyć pliku");
    let mut text = String::new();

    for value in arr.iter() {
        text.push_str(&value.to_string());
        text.push(',');
    }

    if text.ends_with(',') {
        text.pop();
    }

    file.write_all(text.as_bytes()).expect("Nie udało się napisać do pliku");
    Ok(())
}