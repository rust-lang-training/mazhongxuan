use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("随机生成一个数，并判断是不是质数");
    let a = rand::thread_rng().gen_range(0..100);
    println!("随机数是{a}");
    if a == 0 {
        println!("0不是质数")
    } else if a == 1 {
        println!("1不是质数")
    }
    let mut i = 2u64;
    while i * i <= a {
        if a % i == 0 {
            return println!("{a}不是质数");
        }
        i += 1;
    }
    println!("{a}是质数")

    // println!("随机生成10个数值进行排序");
    // let mut numbers = [0u32; 10];
    // for i in 0..10 {
    //     numbers[i] = rand::thread_rng().gen_range(1..100);
    // }
    // println!("BEFORE: {:?}", numbers);
    // let len = numbers.len();
    // for i in 0..len {
    //     for j in 0..len - i - 1 {
    //         if numbers[j]>numbers[j+1]{
    //             numbers.swap(j, j+1);
    //         }
    //     }
    // }

    // println!("AFTER: {:?}",numbers);

    /*     let mut boo = 3;
    let mut num = 6;
    let mut sum = 0;
    let mut c_sum = 0;

    'b: loop {
        boo += 1;
        'c: loop {
            if boo > 5 {
                break 'b;
            } else if num < 2 {
                break 'c;
            }
            num -= 1;
            c_sum += 1;
        }
        sum += 1;
        boo += 2;
    }
    println!("boo={boo},num={num}");
    println!("c循环{c_sum}次");
    println!("循环{sum}次"); */

    /* loop {
        //loop：循环
        println!("Guess the ngueumber!");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is:{secret_number}");

        println!("Please input your guess.!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("falied");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            //match:模式匹配
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too large"),
        }
        println!("You guessed :{guess}")
    } */
}

#[test]
fn test() {
    let a: u8 = 13;
    assert_eq!(32_u32 as u8, a);
}

/*fn c_to_f(target: f32) -> f32 {
    target * 1.8 + 32.0
}
fn f_to_c(target: f32) -> f32 {
    (target - 32.0) / 1.8
}
 fn main() {
    loop {
        println!("1:摄氏度转化湿度；2：华氏度转摄氏度；0：退出");
        println!("请输入你的选择:");
        let mut gess = String::new();
        io::stdin().read_line(&mut gess).expect("读取输入失败");
        let gess = gess.trim();
        if gess == "0" {
            break;
        }
        if gess == "1" {
            println!("请输入摄氏度:");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("读取输入失败");
            let c: f32 = c.trim().parse().expect("请输入有效的数值");
            let f = c_to_f(c);
            println!("{:.2} 摄氏度 -> {:.2} 华氏度\n", c, f);
        } else if gess == "2" {
            println!("请输入华氏度:");
            let mut f = String::new();
            io::stdin().read_line(&mut f).expect("读取输入失败");
            let f: f32 = f.trim().parse().expect("请输入有效的数值");
            let c = f_to_c(f);
            println!("{:.2} 华氏度 -> {:.2} 摄氏度\n", f, c);
        }
    }
} */
/* fn sum(arr: &[u32]) -> u32 {
    let mut a = 0;
    for item in arr {
        a += item
    }
    a
}
fn main() {
    println!("请输入五个整数，用空格分隔：");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取失败");
    let mut arr = [0u32; 5];
    let mut index = 0;
    let guess = guess.trim().split(" ");
    for n in guess {
        let nn: u32 = n.trim().parse().expect("请输入有效的整数");
        arr[index] = nn;
        index += 1;
        if index == 5 {
            break;
        }
    }
    let total = sum(&arr[..]);
    println!("sum {:?} = {} ", arr, total);
} */
