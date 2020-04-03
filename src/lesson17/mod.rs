use std::collections::HashMap;

///
/// Задача 17
/// https://projecteuler.net/problem=17
///
/// Если записать числа от 1 до 5 английскими словами (one, two, three, four, five), то используется всего 3 + 3 + 5 + 4 + 4 = 19 букв.
/// Сколько букв понадобится для записи всех чисел от 1 до 1000 (one thousand) включительно?
///
/// Примечание: Не считайте пробелы и дефисы. Например,
/// число 342 (three hundred and forty-two) состоит из 23 букв,
/// число 115 (one hundred and fifteen) - из 20 букв.
/// Использование "and" при записи чисел соответствует правилам британского английского.
///
pub fn run(){
    let list:HashMap<i32, &str> = [
        (1,"one"), (2,"two"), (3,"three"), (4,"four"), (5,"five"), (6,"six"), (7,"seven"), (8,"eight"), (9,"nine"),
        (11,"eleven"), (12,"twelve"), (13,"thirteen"), (14,"fourteen"), (15,"fifteen"), (16,"sixteen"), (17,"seventeen"), (18,"eighteen"), (19,"nineteen"),
        (10,"ten"), (20,"twenty"), (30,"thirty"), (40,"forty"), (50,"fifty"), (60,"sixty"), (70,"seventy"), (80,"eighty"), (90,"ninety"),
        (100, "hundred"),
        (1000, "thousand"),
    ].iter().cloned().collect();

    let to_string = |num:i32|->Vec<String>{
        let mut ans = vec![];
        if num >= 1000 {
            ans.push(list[&1].to_string());
            ans.push(list[&1000].to_string());
        }
        let th = num%1000/100;
        let te = num%100;

        if th > 0 {
            ans.push(list[&th].to_string());
            ans.push(list[&100].to_string());
        }
        if te > 0 {
            if th > 0 { ans.push("and".to_string()); }
            if te < 20 { ans.push(list[&te].to_string()); }
            else{
                ans.push(list[&{te/10*10}].to_string());
                let n = te%10;
                if n>0{ ans.push(list[&n].to_string()); }
            }
        }

        ans
    };
    println!("result: {:?}", (1..=1000).map(|num|{ to_string(num).join("").len() }).sum::<usize>() );
}