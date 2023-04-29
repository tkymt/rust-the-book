use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101); // 乱数を生成する
    
    // 乱数の範囲は 開始..終了 という形をとる。
    // 下限値は含むが上限値は含まない。
    // 1から100までのかすをリクエストするには1..101と指定する必要がある
    
    loop {

        println!("Please input your guess."); // ほら、予想を入力してね

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        // read_line()は、Result型を返す。Result型は列挙型。
        // Result型の目的は、エラー処理に係る情報を符号化すること。
        // Resultの列挙子は、Ok　または　Err
        // Ok列挙子は処理が成功したことを示す。Okの中には正常に生成された値が入っている
        // Err列挙子は処理が失敗したことを意味し、Errには処理が失敗した過程や理由についての情報が含まれる

        // io::Resultのインスタンスにexpectメソッドがある。
        // io::ResultインスタンスがErrの値の場合、expectメソッドはプログラムをクラッシュさせ、引数として渡されたメッセージを表示します。

        let guess: u32 = match guess.trim().parse() { // シャドーイングを使って型を文字列から数値に変換している
            // trimメソッドは先頭と末尾の空白と\nや\r\nを削除する。
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess); // 次のように予想しました: {}

        // 一組の波括弧の{}は、プレースホルダーです。
        // {}の位置に値を表示します。

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),    // 小さすぎ！
            Ordering::Greater => println!("Too big"),   // 大きすぎ！
            Ordering::Equal => {
                println!("You win!");    // やったね！
                break;
            }
        }
        // マッチ式は複数のアームから構成される。アームにはパターンと実行する処理を書く
        // cmpメソッドは二つの値を比較する。比較できるものになら何に対しても呼び出せます。
        // cmpを読んだ結果、返されたOrderingの列挙子に基づき、動作を決定しています。
    
    }

}
