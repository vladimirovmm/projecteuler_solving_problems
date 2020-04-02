///
/// Задача 14
///
/// https://projecteuler.net/problem=14
///
/// Следующая повторяющаяся последовательность определена для множества натуральных чисел:
///     n → n/2 (n - четное)
///     n → 3n + 1 (n - нечетное)
///
/// Используя описанное выше правило и начиная с 13, сгенерируется следующая последовательность:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// Получившаяся последовательность (начиная с 13 и заканчивая 1) содержит 10 элементов.
/// Хотя это до сих пор и не доказано (проблема Коллатца (Collatz)),
/// предполагается, что все сгенерированные таким образом последовательности оканчиваются на 1.
///
/// Какой начальный элемент меньше миллиона генерирует самую длинную последовательность?
///
/// Примечание: Следующие за первым элементы последовательности могут быть больше миллиона.
///
pub fn run(){
    let mut max= (0,0);
    // Исключаем чётные
    (1..1_000_000).step_by(2).rev().all(|n|{
        let mut n = n;
        let mut step_list:Vec<usize>=vec![n];
        loop {
            n = if n%2==0 { n/2 }else{ n*3+1 };
            step_list.push(n);
            if n <= 1 { break; }
        }
        // Если больше сохраняем
        if step_list.len() > max.1 {
            max = ( *step_list.first().unwrap(), step_list.len() );
        }
        true
    });
    println!("result: {}", max.0);
}