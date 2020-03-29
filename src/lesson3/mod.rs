///
/// https://projecteuler.net/problem=3
///
/// Простые делители числа 13195 - это 5, 7, 13 и 29.
///
/// Каков самый большой делитель числа 600851475143, являющийся простым числом?
///
/// Решето Эратосфена
/// https://temofeev.ru/info/articles/algoritmy-poiska-prostykh-chisel/
///
pub fn run_variant_1(){
    let number:i64 = 600851475143;
    let mut max_num = 0;
    let mut prime_number:Vec<i64> = vec![2,3];

    'go_to_num: for i in (3..number/2 ).step_by(2){
        for pr in prime_number.iter() {
            if i%pr==0  {  continue 'go_to_num; }
        }
        prime_number.push(i);
        if number%i==0 { max_num=i; }
    }
    println!("result: {}", max_num);
}
///
/// Быстрое решение
///
pub fn run_variant_2(){
    // Стартовое число. Его остаток и будет максимальным делителем
    let mut number:i64 = 600851475143;
    // Стартовой делитель
    let mut delitel = 2;

    loop {
        // Проверяем делится ли число
        if number%delitel==0 {
            // Получаем остаток
            number/=delitel;
            // Повторно пробуем поделить на тоже число
            continue;
        }
        delitel+=1;
        if delitel > number/2 { break; }
    }
    println!("result: {}", number);
}