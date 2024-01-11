use std::time::{Instant};
use std::io;
use chrono::Duration;
use rand::Rng;

fn sort<T: Ord, F>(sort_function: F, length: usize) 
    where F: FnOnce(&mut [i32])
{
    let mut numbers = generate_random_vector(length);

    let start_time = Instant::now();
    sort_function(&mut numbers[..]); 
    let elapsed_time = start_time.elapsed();

    let duration = Duration::from_std(elapsed_time).unwrap();
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds() % 60;

    println!("Время сортировки: {} мин {} сек", minutes, seconds);
}
fn get_vector_length() -> usize {
    let mut length_input = String::new();
    io::stdin().read_line(&mut length_input).expect("Ошибка ввода");

    length_input.trim().parse().unwrap_or_else(|_| {
        println!("Ошибка ввода числа");
        std::process::exit(1);
    })
}

fn get_user_input() -> Result<u32, std::num::ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    input.trim().parse()
}

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

fn print_text<T: Ord + std::fmt::Debug>(arr: &[T]) {
    let n = arr.len();
    println!("Количество элементов: {}", n);

    print!("Элементы массива: ");
    let elements_str = arr.iter().map(|el| format!("{:?}", el)).collect::<Vec<_>>().join("; ");
    println!("[{}]", elements_str);
    println!("\n");
}

fn main() {
    loop {
        println!("Выберите тип сортировки (1-5):\n
        1. Выбором сортировка\n
        2. Быстрая сортировка\n
        3. Вставками сортировка\n
        4. Пузырьквая сортировка\n
        
        5. Сгенерировать и вывести элементы массива");
        let choice: Result<u32, _> = get_user_input();

        match choice {
            Ok(1) => {
                println!("Введите длину вектора:");
                let length = get_vector_length();
                sort::<i32, _>(selection_sort, length);
            }
            Ok(2) => {
                println!("Введите длину вектора:");
                let length = get_vector_length();
                sort::<i32, _>(quicksort, length);
            }
            Ok(3) => {
                println!("Введите длину вектора:");
                let length = get_vector_length();
                sort::<i32, _>(insertion_sort, length);
            }
            Ok(4) => {
                println!("Введите длину вектора:");
                let length = get_vector_length();
                sort::<i32, _>(bubble_sort, length);
            }
            Ok(5) => {
                println!("Введите длину вектора:");
                let length = get_vector_length();
                let mut numbers = generate_random_vector(length);
                print_text(&mut numbers);
            }
            _ => {
                println!("Некорректный выбор, выберите число от 1 до 5");
                return;
            }
        }
    }
}
