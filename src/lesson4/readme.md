# Задача
https://projecteuler.net/problem=4

Число-палиндром с обеих сторон (справа налево и слева направо) читается одинаково.
Самое большое число-палиндром, полученное умножением двух двузначных чисел – 9009 = 91 × 99.

Найдите самый большой палиндром, полученный умножением двух трехзначных чисел.
 
### Решение
```
let mut max = 0;
let mut num= 0;
let mut deliver = None;

for x1 in (100..=999).rev() {
    num = x1 *1000
            + x1 /100
            + (x1 /10)%10*10
            + x1 %10*100;
    deliver = ( 100..999 ).find(|x2|{
            num%x2==0 && num/x2>100 && num/x2<999
        });
    if deliver.is_some() {
        max = num;
        break;
    }
}
println!("result: {} {} {}", num, deliver.unwrap(), num/deliver.unwrap() );
```
 ​