///
/// https://projecteuler.net/problem=10
///
/// Сумма простых чисел меньше 10 равна 2 + 3 + 5 + 7 = 17.
///
/// Найдите сумму всех простых чисел меньше двух миллионов.
///
/// Решето Эратосфена
/// https://prog-cpp.ru/eratosfen/
///
/// Не очень быстрый метод для данной задачи возможно стоит попробовать
/// Решето Аткина
/// https://ru.wikipedia.org/wiki/Решето_Аткина
///
pub fn run(){
    let mut prime_numbers:Vec<usize> = vec![2, 3];
    let max_num:usize = 2_000_000;

    'go_to_num: for i in (3..max_num ).step_by(2){
        for pr in prime_numbers.iter() {
            if i%pr==0  {  continue 'go_to_num; }
        }
        prime_numbers.push(i);
    }
    println!("result: {}", prime_numbers.iter().sum::<usize>());
}