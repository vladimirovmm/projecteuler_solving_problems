# Задача 15
https://projecteuler.net/problem=15

Начиная в левом верхнем углу сетки 2×2 и имея возможность двигаться только вниз или вправо,
существует ровно 6 маршрутов до правого нижнего угла сетки.

**Сколько существует таких маршрутов в сетке 20×20?**

### Решение через Центральный биномиальный коэффициент
Формула:  
**(2n)!/(n!)^2**

### Решение
```
///
/// Т.к. числа большие необходимо произвести сокращение до перемножения
///
/// Формула:
/// (2n)!/(n!)^2
///
pub fn bin_kof(size:usize)->usize{
    let mut a = (2..=size*2).collect::<Vec<usize>>();
    let mut b = (2..=size).collect::<Vec<usize>>();
    b.extend(2..=size );

    for bnum in b.iter_mut().rev() {
        if let Some(index) = a.iter().position(|x|*x%*bnum==0) {
            a[index]/=*bnum;
            *bnum=1;
        }
    }
    a.iter().product::<usize>() / b.iter().product::<usize>()
}
let size:usize = 20;
println!("result: {}", bin_kof(size) );
```
 ​