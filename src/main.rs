#![allow(dead_code)]

/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при это цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/
fn main() {
    digit_product(123454321);
}

fn digit_product(number: u32) -> u8 {
    let mut result_number: u32 = number;
    while digits_in_a_number(result_number) > 1 {
        result_number = parsing_product(result_number);
    }
    result_number as u8
}

// Функция удаляет из вектора значения элементов, равных 0 и перемножает все элементы в векторе
fn parsing_product(_n: u32) -> u32 {
    let mut parsed_number: Vec<u32> = split_number(_n);
    parsed_number.retain(|&i| i != 0);
    let iter = parsed_number.iter();
    let mut result: u32 = parsed_number[0];
    for val in iter {
        result *= val;
    }
    result as u32
}

// Функция заполняет вектор цифрами числа
fn split_number(_n: u32) -> Vec<u32> {
    let mut splitted_number: Vec<u32> = Vec::new();
    let mut number = _n;
    let count = digits_in_a_number(number);
    for _ in 0..count {
        splitted_number.push(&number % 10);
        number /= 10;
    }
    splitted_number
}

// Функция возвращает количество цифр в числе
fn digits_in_a_number(number: u32) -> u32 {
    let mut count: u32 = 0;
    let mut counted_number: u32 = number;
    while counted_number > 0 {
        count += 1;
        counted_number /= 10;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
    }
}
