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

fn print_text<T: Ord>(arr: &mut[T]){
    let n = arr.len();
    println!("Количество элеметов: {}", n);

}

fn main() {
    let mut numbers = vec![5, 3, 2, 1, 4];

    println!("Исходный массив: {:?}\n", numbers);
    println!("1 или 2:\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let choice: Result<u32, _> = input.trim().parse();
    match choice {
        Ok(choice) => {
            match choice {
                1 => {
                    let start_time = Instant::now();
                    bubble_sort(&mut numbers);
                    let elapsed_time = start_time.elapsed();
                    println!("Отсортированный массив: {:?}", numbers);
                    println!("Время сортировки: {} ms", elapsed_time.as_millis());
                }
                2 => print_text(&mut numbers),
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
