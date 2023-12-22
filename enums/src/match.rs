#[derive(debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

// 未知のアメリカコインを表す列挙型
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 未知のアメリカコインを一枚とり、どの種類のコインなのか決定し、その価値をセントで返す関数
fn value_in_cents(coin: Coin) -> u32 {
    // machキーワードに続けて式を並べる
    match coin {
        // ひとつひとつのブロックをアームと呼ぶ
        // 一本のアームには２つの部品がある
        // パターンと何らかのコードでできている
        Coin::Penny => {
            // マッチのアームで複数行のコードを走らせたいときは、波かっこを使用する
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // Quarterが持つ値をstate変数に束縛できる
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


// Option を引数に取る関数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // Rustにおけるマッチは、包括的
        // すべてのあらゆる可能性を網羅し尽さなければならない
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// _ というプレースホルダー
let some_u8_value = 0u8;
match some_u8_value {

}