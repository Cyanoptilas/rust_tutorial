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
    print!("x : {}", x);

    // ex
    {
        let x = 5;
        {
            let x = 7;
            print!("inside x : {}", x)
        }
        print!("outside x : {}", x)
    }

    {
        let spaces = "   ";
        let spaces = spaces.len();
        print!("spaces : {}", spaces)
    }
}
