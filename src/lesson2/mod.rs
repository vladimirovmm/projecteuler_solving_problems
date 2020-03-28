
///
/// Каждый следующий элемент ряда Фибоначчи получается при сложении двух предыдущих.
/// Начиная с 1 и 2, первые 10 элементов будут:
///
/// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// Найдите сумму всех четных элементов ряда Фибоначчи,
/// которые не превышают четыре миллиона.
///
pub fn run(){
    let mut a1 = 0;
    let mut a2=1;
    let mut a;
    let mut sum = 0;
    loop {
        a = a1+a2;
        a1 = a2;
        a2 = a;
        if a%2!=0 { continue; }
        if a > 4_000_000 { break; }
        sum+=a;
    }
    println!("result: {}" , sum);
}