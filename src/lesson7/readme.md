# Задача
https://projecteuler.net/problem=7

Выписав первые шесть простых чисел, получим 2, 3, 5, 7, 11 и 13. Очевидно, что 6-ое простое число - 13.

Какое число является 10001-ым простым числом?

##Решето Эратосфена
https://prog-cpp.ru/eratosfen/
 
 ### Решение
```
let mut prime_numbers:Vec<i32> = vec![2, 3];
let steps = 10001;

'go_to_num: for i in (3..i32::max_value() ).step_by(2){
    for pr in prime_numbers.iter() {
        if i%pr==0  {  continue 'go_to_num; }
    }
    prime_numbers.push(i);
    if prime_numbers.len() >= steps {
        break
    }
}
println!("result: {}", prime_numbers.last().unwrap());
```
 ​