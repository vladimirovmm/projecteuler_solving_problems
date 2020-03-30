# Задача
https://projecteuler.net/problem=5

2520 - самое маленькое число, которое делится без остатка на все числа от 1 до 10.
Какое самое маленькое число делится нацело на все числа от 1 до 20?
 
 ### Решение
```
let predel = 20;
let mut prime:Vec<(i32, i32)>=Vec::new();

for mut n in (2..=predel){
    for p in prime.iter_mut(){
        let mut exp = 0;
        while n%p.0==0 && n>1 {
            exp+=1;
            n/=p.0;
        }
        if p.1 < exp { p.1 = exp; }
    }
    if n>1 { prime.push((n,1)); }
}
println!("result: {:?}", prime.iter().fold(1, |r, p|{
        r*p.0.pow(p.1 as u32)
    }));
```
 ​