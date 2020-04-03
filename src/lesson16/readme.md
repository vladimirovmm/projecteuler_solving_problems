# Задача 16
https://projecteuler.net/problem=16

2^15 = 32768, сумма цифр этого числа равна 3 + 2 + 7 + 6 + 8 = 26.
Какова сумма цифр числа 2^1000?

Подстчёт степени через вектор

### Решение через вектор
```
let num = 2;
let pow = 1000;
let mut result:Vec<u8> = vec![num];

for ex in (1..pow){
    let mut ost = 0;
    for a in result.iter_mut(){
        let r = *a*num+ost;
        ost = r/10;
        *a = r%10;
    }
    if ost>0 {
        result.push(ost);
    }
}
result.reverse();
let result = result.iter().map(|x|*x as u32).sum::<u32>();
println!("result: {}", result );
```  

​### Решение через BigUint

```
    use num_bigint::{BigUint,ToBigUint};

    let num:BigUint = 2.to_biguint().unwrap();
    let pow = 1000;
    let mut result = (1..pow)
        .fold( num.clone(), | p, _ | p*num.clone() )
        .to_string()
        .chars().map(|x|x.to_digit(10).unwrap()).sum::<u32>();
    println!("result: {}", result);
```
   ​