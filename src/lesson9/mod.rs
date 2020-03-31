///
/// Задача 9
/// https://projecteuler.net/problem=9
///
/// Тройка Пифагора - три натуральных числа a < b < c, для которых выполняется равенство
///
/// a^2 + b^2 = c^2
///
/// Например, 3^2 + 4^2 = 9 + 16 = 25 = 52.
///
/// Существует только одна тройка Пифагора, для которой a + b + c = 1000.
///
/// Найдите произведение abc.
///
pub fn run(){
    let sum = 1000;
    'first_for: for a in 1..sum as i32 {
        for b in a+1..sum-a as i32{
            let c = sum-a-b;
            if a.pow(2)+b.pow(2)==c.pow(2){
                println!("resutl: {}", a*b*c );
                break 'first_for;
            }w
        }
    }
}