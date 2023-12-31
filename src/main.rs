use std::time::{Instant};
use std::io;
use chrono::Duration;
use rand::Rng;

fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_index = i;

        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}


fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);


    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
            
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}

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
    let mut rng = rand::thread_rng();
    (0..length)
    .map(|_| rng.gen_range(-(length as i32)..=length as i32))
    .collect()
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
        Ok(1) => {
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
                    let duration = Duration::from_std(elapsed_time).unwrap();

                    let minutes = duration.num_minutes();
                    let seconds = duration.num_seconds() % 60;

                    println!("Время сортировки: {} мин {} сек", minutes, seconds);
                }
                Err(_) => {
                    println!("Ошибка ввода числа");
                    return;
                }
            }
        }
        Ok(2) => {
            println!("Введите длину вектора:");
            let mut length_input = String::new();
            io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

            let length: Result<usize, _> = length_input.trim().parse();
            match length {
                Ok(length) => {
                    let mut numbers = generate_random_vector(length);
                    print_text(&mut numbers);
                    // println!("Элементы: {:?}", &mut numbers);
                }
                Err(_) => {
                    println!("Ошибка ввода числа");
                    return;
                }
            }
        }
        Ok(3) => {
            println!("Введите длину вектора:");
            let mut length_input = String::new();
            io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

            let length: Result<usize, _> = length_input.trim().parse();
            match length {
                Ok(length) => {
                    let mut numbers = generate_random_vector(length);
                    let start_time = Instant::now();
                    quicksort(&mut numbers);
                    let elapsed_time = start_time.elapsed();
                    let duration = Duration::from_std(elapsed_time).unwrap();

                    let minutes = duration.num_minutes();
                    let seconds = duration.num_seconds() % 60;

                    println!("Время сортировки: {} мин {} сек", minutes, seconds);
                }
                Err(_) => {
                    println!("Ошибка ввода числа");
                    return;
                }
            }
        }
        Ok(4) => {
            println!("Введите длину вектора:");
            let mut length_input = String::new();
            io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

            let length: Result<usize, _> = length_input.trim().parse();
            match length {
                Ok(length) => {
                    let mut numbers = generate_random_vector(length);
                    // let mut static_num = vec![5, 3, 2, 1, 4];
                    let start_time = Instant::now();
                    insertion_sort(&mut numbers);
                    let elapsed_time = start_time.elapsed();
                    let duration = Duration::from_std(elapsed_time).unwrap();

                    let minutes = duration.num_minutes();
                    let seconds = duration.num_seconds() % 60;

                    println!("Время сортировки: {} мин {} сек", minutes, seconds);
                }
                Err(_) => {
                    println!("Ошибка ввода числа");
                    return;
                }
            }
        }
        Ok(5) => {
            println!("Введите длину вектора:");
            let mut length_input = String::new();
            io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

            let length: Result<usize, _> = length_input.trim().parse();
            match length {
                Ok(length) => {
                    let mut numbers = generate_random_vector(length);
                    //  let mut static_num = vec![5, 3, 2, 1, 4];
                    let start_time = Instant::now();
                    selection_sort(&mut numbers);
                    let elapsed_time = start_time.elapsed();
                    let duration = Duration::from_std(elapsed_time).unwrap();

                    let minutes = duration.num_minutes();
                    let seconds = duration.num_seconds() % 60;

                    println!("Время сортировки: {} мин {} сек", minutes, seconds);
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
