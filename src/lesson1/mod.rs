///
/// https://projecteuler.net/problem=1
///
/// Если выписать все натуральные числа меньше 10, кратные 3 или 5, то получим 3, 5, 6 и 9. Сумма этих чисел равна 23.
/// Найдите сумму всех чисел меньше 1000, кратных 3 или 5.
///
/// ! Понятие натуральные числа возникло естественным образом при счете. Перечисляя или исчисляя человек еще в древности дал определение натуральных чисел.
///


pub fn run_variant_1(){
    let max:i32 = 1000;
    let result = (1..max).filter(|x| x%3==0 || x%5==0 )
                        .fold(0,|sum, x|sum+x);
    println!("result: {}", result );
}
///
/// Сумма первых n членов арифметической прогрессии
///
/// amount = (a1+aN)/2*n
/// n - количество шагов прогрессии
/// a1 - первый элемент прогрессии
/// aN - последний элемент прогрессии
///
/// https://youclever.org/book/arifmeticheskaya-progressiya-1
///
pub fn run_variant_2(){
    let max = 1000-1;
    let n_3:f32 = {max/3} as f32;
    let n_5:f32 = {max/5} as f32;
    let n_15:f32 = {max/15} as f32;
    let amount_3:f32 = ((3.0+3.0*n_3)/2.0)*n_3;
    let amount_5:f32 = ((5.0+5.0*n_5)/2.0)*n_5;
    let amount_15:f32 = ((15.0+15.0*n_15)/2.0)*n_15;
    println!("result: {}", amount_3 + amount_5 - amount_15);
}
