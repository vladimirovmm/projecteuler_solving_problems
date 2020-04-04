extern crate chrono;
use chrono::prelude::*;

///
/// Задача 19
/// https://projecteuler.net/problem=19
///
/// Дана следующая информация (однако, вы можете проверить ее самостоятельно):
//
// 1 января 1900 года - понедельник.
// В апреле, июне, сентябре и ноябре 30 дней.
// В феврале 28 дней, в високосный год - 29.
// В остальных месяцах по 31 дню.
// Високосный год - любой год, делящийся нацело на 4,
// однако последний год века (ХХ00) является високосным в том и только том случае,
// если делится на 400.
// Сколько воскресений выпадает на первое число месяца в двадцатом веке (с 1 января 1901 года
// до 31 декабря 2000 года)?
///
pub fn run(){
    let month_days = vec![
        31, 28, 31, 30,
        31, 30, 31, 31,
        30, 31, 30, 31
    ];

    let date_start = NaiveDate::from_ymd(1901, 1, 1)
        .and_hms_milli(0, 0, 0, 0).timestamp();
    let date_end = NaiveDate::from_ymd(2000, 12, 31)
        .and_hms_milli(0, 0, 0, 0).timestamp();

    let mut time = 0;
    let time_end = date_end-date_start;
    let mut year = 1;
    let one_day = 60*60*24;
    let mut result = 0;

    'main_loop: loop {
        for m in 0..12{
            time+=month_days[m]* one_day;
            if m==3 && year%4==0{ time+=one_day; }
            if time%{ one_day *7 }==one_day*5{
                if time>=time_end { break 'main_loop; }
                result+=1;
            }
        }
        year+=1;
    }
    println!("result: {}",  result );
}