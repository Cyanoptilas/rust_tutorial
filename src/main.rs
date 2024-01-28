use std::io;

fn main() {
    println!("Hello, world!");

    ////// 3.1 //////

    // エラーになる
    // let x = 5;
    // x = 6;

    // シャドーイング
    // エラーにならない
    let x = 1;
    let x = 2;
    println!("x : {}", x);

    // ex
    {
        let x = 5;
        {
            let x = 7;
            println!("inside x : {}", x)
        }
        println!("outside x : {}", x)
    }

    {
        let spaces = "   ";
        let spaces = spaces.len();
        println!("spaces : {}", spaces)
    }

    ////// 3.2 //////
    println!("////// 3.2 //////");
    // Taple
    let taple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = taple;
    println!("x:{} / y:{} / z:{}", x, y, taple.2);

    // 配列
    let arr: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
    println!("arr : {}", arr[1]);

    // コンソールでユーザー入力
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 入力値確認
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element: i32 = arr[index];
    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index,
        element
    );
}
