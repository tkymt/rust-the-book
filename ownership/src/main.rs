fn main() {
    // 所有権について
    // 変数のスコープ
    {
                            // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";    // sは、ここから有効になる
        // sで作業する
        println!("{}", s);
    }                       // このスコープは終わり。もうsは有効ではない

    // String型
    // ヒープにメモリを確保する
    let s = String::from("hello");
    // 可変にすることで文字列の長さを変えられる
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world`と出力する

    // メモリと確保
    // メモリは、実行時にOSに要求される。
    // String型を使用し終わったら、OSにこのメモリを変換する方法が必要である
    // スコープを抜けるときに、Rustは、自動的に drop という特別な関数を呼び出してメモリをOSに返還する
    {
        let s = String::from("hello"); // sはここから有効になる（メモリをOSに要求する）
        // sで作業する
    }                                   // このスコープはここでおしまい。sはもう有効ではない（メモリをOSに返還する）
    
    // ムーブ
    // 値のコピーが発生するコード
    // スタックに積まれる
    let x = 5; // 値 5 を x に束縛する
    let y = x; // x の値をコピーして y に束縛する

    // ポインタをコピーするコード
    // ポインタはスタックに積まれて、文字列はヒープに保持される
    // 二重解放を防ぐために s1 は無効になる
    let s1 = String::from("hello");
    let s2 = s1; // s1は無効になる。ムーブと呼ばれる（shallow copy）

    // クローン
    // 本当にString型のヒープデータのdeep copyが必要ならclone と呼ばれるメソッドを使うことができる
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // copy トレイトと呼ばれる特別な注釈に適合していれば、代入後も古い変数が使用可能になる
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 所有権と関数
    {
        let s = String::from("hello"); // s がスコープに入る
        takes_ownership(s);            // s の値が関数にムーブされ
                                        // ここではもう有効ではない
        let x = 5;  // x がスコープに入る
        makes_copy(x);  // x も関数にムーブされるが、
                        // i32はcopyなので、この後に x を使っても大丈夫
    }   // ここで x がスコープを抜け、s もスコープを抜ける。ただし、s の値はムーブされているので、何も特別なことは起きない

    // 戻り値とスコープ
    // 関数で値を返すことでも、所有権は移動します
    {
        let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1にムーブする
        let s2 = String::from("hello");     // s2がスコープに入る
        let s3 = takes_and_gives_back(s2);  // s2はtake_and_gives_backにムーブされ、戻り値もムーブされる
    }   // ここで、s3はスコープを抜けてドロップされる。s2はスコープを抜けるが、ムーブされているので何も起きない。
        // s1もスコープを抜けてドロップされる。

    // 関数に渡した変数を使いたい
    // タプルで複数の値を返す    
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_tuple(s1);
        // '{}'の長さは、{}です
        println!("The length of '{}' is {}.", s2, len);
    }

    // 参照と借用
    {
        let s1 = String::from("hello");
        let len = calculate_length_reference(&s1); // 関数の引数に参照を取ることを借用と呼ぶ
        // '{}'の長さは、{}です
        println!("The length of '{}' is {}.", s1, len);
    }
    // 可変な参照
    {
        let mut s = String::from("hello");
        change(&mut s);
    }
    // 可変な参照の制約
    // 特定のスコープの中では特定のデータに対して一つしか可変な参照を持てない
    // 以下のコードはコンパイルエラーになります
    /*
    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        let r2 = &mut s;
        println!("{}, {}", r1, r2);
    }
    */
    //スコープが違うなら複数の参照を作ることができる
    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;
    }
    // 不変な参照をしているときは、可変な参照はできない
    // 以下のコードはエラーになります
    /*
    {
        let s = String::from("hello");
        let r1 = &s; // 問題なし
        let r2 = &s; // 不変な参照は同じスコープで複数つくることができるから問題なし
        let r3 = &mut s; // すでに不変で借用されているので、可変で借用できません
    }
    */

    // 宙に浮いた参照
    // Rustではコンパイラがダングリング参照を作らせないよう保証してくれる
    // 以下はエラーになる
    /*
        let reference_to_nothing = dangle();
        
        fn dangle() -> &String {
            let s = String::from("hello");

            &s // String s の参照を返す
        } // ここで、s はスコープ抜けてドロップされる。そのメモリは消される。危険である
    */

    // 解決策は、String を直接返すこと
    // 以下はコンパイルできる
    {
        let s = no_dangle();

        fn no_dangle() -> String {
            let s = String::from("hello");
            s // sは呼び出し元にムーブされる
        }
    }

    // スライス型
    {
        let s = String::from("hello world");
        
        // stringの一部への参照を取ることができる
        // [starting_index..ending_index]と指定する
        // starging_indexはスライスの最初の位置。ending_endexはスライスの終端位置よりも、１大きい値。
        // なぜ終端位置よりも１大きい値かというと、スライスデータ構造は、開始地点とスライスの長さを保持しており、スライスの長さは ending_index から starting_index を引いたものだから
        let hello = &s[0..5];
        let world = &s[6..11]; // s の添え字6のバイトへのポインタと5という長さを持つスライスになる

        let s = String::from("hello");
        // 以下は等価 0から始めるとき、値を省略できる
        let slice = &s[0..2];
        let slice = &s[..2];

        // これらは等価　最後のバイトをスライスが含むなら、末尾の値を省略できる
        let s = string::from("hello");
        let len = s.len();

        let slice = &s[3..len];
        let slice = &s[3..];

        // さらに、両方の値を省略すると、文字列全体のスライスを得られます。故に、これらは等価
        let s = String::from("hello");
        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];

        // 文字列の一部をスライスで返す関数
        fn first_word(s: &string) -> &str {
            let bytes = s.as_bytes();

            for (i, item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        // 文字列リテラルはスライスである
        // s の型は &str です
        let s = "Hello, world!";

        // 他のスライス
        // i32 の配列でも有効
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
    }


}

fn takes_ownership(some_string: String) { // some_string がスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾していたメモリが解放される

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここで some_integer がスコープを抜ける。何も特別なことはない

fn gives_ownership() -> String { // gives_ownershipは、戻り値を呼び出し元の関数にムーブする
    let some_string = String::from("hello"); // some_stringがスコープに入る
    some_string // ムーブされる
}

// take_and_gives_backは、Stringをひとつ受け取り、返す
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る
    a_string // a_stringが返され、呼び出し元の関数にムーブされる
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len(); // lenメソッドは、Stringの長さを返す
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になる。けれど、参照しているだけで所有権を持っているわけではないので、何も起こらない

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

