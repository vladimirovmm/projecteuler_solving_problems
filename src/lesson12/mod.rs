///
/// Задача 12
///
/// https://projecteuler.net/problem=12
///
/// Последовательность треугольных чисел образуется путем сложения натуральных чисел.
/// К примеру, 7-ое треугольное число равно 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
/// Первые десять треугольных чисел:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Перечислим делители первых семи треугольных чисел:
///   1: 1
///   3: 1, 3
///   6: 1, 2, 3, 6
///  10: 1, 2, 5, 10
///  15: 1, 3, 5, 15
///  21: 1, 3, 7, 21
///  28: 1, 2, 4, 7, 14, 28
///
/// Как мы видим, 28 - первое треугольное число, у которого более пяти делителей.
///
/// Каково первое треугольное число, у которого более пятисот делителей?
///
pub fn run(){
    let mut treug = 0;
    for num in 1..1_000_000{
        treug +=num;

        let divider_count = (1..={ treug as f32}.sqrt() as i32)
            .filter(|x| treug %*x==0)
            .count() as i32;
        if divider_count*2-1 > 500 {
            // Для красивого вывода
            let divider:Vec<String> = (1..=treug/2)
                .filter(|x| treug %*x==0)
                .map(|x| x.to_string())
                .collect();
            println!("number: {} | step: {} | divider: {}", treug, divider.len(), divider.join(" ") );
            break;
        }
    }
}