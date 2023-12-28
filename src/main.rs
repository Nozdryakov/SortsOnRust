use std::time::{Instant};
use std::io;

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn generate_random_vector(length: usize) -> Vec<i32> {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(-(length as i32)..=length as i32)).collect()
}

fn print_text<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    println!("Количество элементов: {}", n);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let choice: Result<u32, _> = input.trim().parse();
    match choice {
        Ok(choice) => {
            match choice {
                1 => {
                    println!("Введите длину вектора:");
                    let mut length_input = String::new();
                    io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

                    let length: Result<usize, _> = length_input.trim().parse();
                    match length {
                        Ok(length) => {
                            let mut numbers = generate_random_vector(length);
                            let start_time = Instant::now();
                            // println!("Элементы: {:?}", numbers);
                            bubble_sort(&mut numbers);
                            let elapsed_time = start_time.elapsed();
                            // println!("Отсортированный массив: {:?}", numbers);
                            
                            // Выводим время сортировки в минутах
                            let time_in_minutes = elapsed_time.as_millis() as f64 / 60000.0;
                            println!("Время сортировки: {} минут", time_in_minutes);
                        }
                        Err(_) => {
                            println!("Ошибка ввода числа");
                            return;
                        }
                    }
                }
                2 => {
                    println!("Введите длину вектора:");
                    let mut length_input = String::new();
                    io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

                    let length: Result<usize, _> = length_input.trim().parse();
                    match length {
                        Ok(length) => {
                            let mut numbers = generate_random_vector(length);
                            print_text(&mut numbers);
                            println!("Элементы: {:?}", &mut numbers);
                        }
                        Err(_) => {
                            println!("Ошибка ввода числа");
                            return;
                        }
                    }
                }
                _ => {
                    println!("Некорректный выбор, выберите 1 или 2");
                    return;
                }
            }
        }
        Err(_) => {
            println!("Ошибка ввода числа");
            return;
        }
    }
}
