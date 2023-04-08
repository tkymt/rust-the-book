fn main() {
    // 変数
    // mutを付けると可変変数になる
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数
    // 値の型を必ず注釈しなければならない
    // 命名規則は、すべて大文字でアンダースコアで単語区切りすること
    const MAX_POINTS: u32 = 100_000;

    // シャドーイング
    let x = 5;
    let x = x + 1;
    {
        // 12が出力される
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    
    // 6が出力される
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
}
