use std::io;

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

    // 値の型を変えつつ名前を再利用できる
    // 1つ目のspaces変数は文字列型であり、2つ目のspacese変数は数値型
    let spaces = "   ";
    let spaces = spaces.len();

    //数値演算
    // 足し算
    let sum = 5 + 10;
    // 引き算
    let difference = 95.5 - 4.3;
    // 掛け算
    let product = 4 * 30;
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 結果は0

    // 論理値型
    let t = true;
    let f: bool = false; // 明示的型注釈付きで書くとこうなる

    // 文字型
    // char型はシングルクオーテーションで指定する
    // Rustのchar型は、ユニコードのスカラー値を表す
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; // ハート目の猫

    // 複合型
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1); // タプルを生成し、変数 tup に束縛している
    let (x, y, z) = tup; // tup 変数の中身を３つの個別の変数に変換する。この過程を分配と呼ぶ。
    println!("The value of y is: {}", y);

    // アクセスしたい値の番号をピリオド（.）に続けて書くことで、タプルの要素に直接アクセスすることができる。
    // タプルの最初の添え字は0
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 配列型
    let a = [1, 2, 3, 4, 5];
    // 1年の月を扱うときは配列を使う
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "Noveber", "December"];
    // 配列の型を指定するときは、角括弧の中に、要素の型とセミコロン、配列の要素数を与える
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 角括弧のなかに、初期値とセミコロン、配列の要素数を与えると、各要素に同じ値を持つように初期化できる
    let a = [3; 5];
    // 添え字によって、配列の要素にアクセスできる
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // 配列の終端を超えて要素にアクセスしようとすると、コンパイルは通るが、実行するとエラーで終了する。
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a); // {:?}は配列を出力するためのフォーマット指定子
    println!("Please enter an array index."); // 配列の何番目の要素にアクセスするか指定してください
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); // 値の読み込みに失敗しました
    let index: usize = index
        .trim()
        .parse()
        .expect("Index enterd was not a number"); // 入力された値は数字ではありません
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}" // {}番目の要素の値は{}です
        , index, element
    );

    // 関数
    // fnキーワードによって新しい関数を宣言することができる
    // 関数と変数の命名規則は、スネークケースを使うのが慣例（some_variableのような命名規則）
    // スネークケースとは、全文字を小文字に資、単語区切りにアンダースコアを使うこと
    /*
    another_function();
    */
    // 関数の引数
    // 引数を定義できる
    another_function(5);
    // 複数の引数を渡す
    print_laveled_measurement(5, 'h');

    // 関数本体は、文と式を含む
    // 文とは、なんらかの動作をして値を返さない命令です。
    // 式は、評価された値を返します。
    let y = 6; // letは文です。6は6に評価される式です。
    // 関数定義も文になります
    // let x = (let y = 6); 括弧の中のletは文で値を返さないからエラーになる

    let y = { // ブロック（{}）も式
        let x = 3;
        x + 1 // セミコロンを付けてはいけない。付けると文になってしまう。
    };
    println!("The value of y is: {}", y);

    // 戻り値でxを初期化する
    let x = five();
    println!("The value of x is: {}", x);

    // 1を足した数でxを初期化する
    let x = plus_one(5);
    println!("The velue of x is: {}", x);

    // コメント
    // スラッシュを２つ続けて書くとコメントになる
    // コメントは、コードが書かれた行の末尾にも配置することができる
    let lucky_number = 7; // I'm feeling lucky today（今日はラッキーな気がするよ）
    // 注釈しようとしているコードの１行上に書く形式のほうが見かける機会は多い
    // I'm feeling lucky today
    // 今日はラッキーな気がするよ
    let lucky_number = 7;

    // if式
    let number = 7;
    // キーワードの if から始めて、条件式を続けて書く
    // 条件が真のときに、実行したい一連のコードを、条件式の直後に波括弧で包んで配置する
    // number が 5 未満になっているかどうかを判定する
    if number < 5 { 
        println!("condition was true"); // 条件は真でした
    } else {
        println!("condition was false"); // 条件は偽でした
    }
    // 条件式はbool型でなければならない
    let number = 3;
    if number != 0 {
        println!("number was something other than zero"); // 数値は0以外の何かです
    }
    // else if
    // if と elseを組み合わせて else if 式にすることができる
    let number = 6;
    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 4");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }

    // let文の中でif式を使う
    // if は式なので、let 文の右辺に持ってくることができます
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // numberの値は、{}です
    println!("The value of number is: {}", number);

    // loopでコードを繰り返す
    // loopキーワードを使用すると、同じコードを明示的にやめさせるまで繰り返し実行します
    /*
    loop {
        println!("again!");
    }
    */

    // ループラベルを使用することで、breakやcontinueが適用されるループを指定することができる
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 外側のループを終了する。内側のループも同時に終了する
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}", count);

    // while で条件付きループ
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -=1;
    }
    // 発射！
    println!("LIFTOFF!!!");

    // whileで配列の要素を見る
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // forループでより効率的に配列の要素を見る
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Range型を使ったforループ
    // Range型とは、片方の数字から始まって、もう片方の数字未満の数値を順番に生成する型です
    // rev関数で逆順にできる
    for number in (1..4).rev() {
        println!("{}!", number);
        println!("LIFTOFF!!!");
    }

    // 温度を華氏と摂氏で変換する
    // 摂氏から華氏
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("値の読み込みに失敗しました");
    let celsius: f64 = celsius.trim().parse().expect("数値ではありません");
    let fahrenheit = (celsius * 1.8) + 32.0;
    println!("{}°F", fahrenheit);

    // 華氏から摂氏
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("値の読み込みに失敗しました");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("数値ではありません");
    let celsius = (fahrenheit - 32.0) / 1.8;
    println!("{}°C", celsius);

    // フィボナッチ数列
    let mut a = 0;
    let mut b = 1;
    for _ in (0..10) {
        println!("{}", a);
        let c = a + b;
        a = b;
        b = c;
    }
}
/*
fn another_function() {
    println!("Another function."); // 別の関数
}
*/

// xという名前の仮引数があり、xの型は、i32と指定されています。
//　関数シグニチャにおいて、各仮引数の型を宣言しなければならない。
fn another_function(x: i32) {
    println!("The value of x is: {}", x); // xの値は{}です
}

// 複数の仮引数を持たせたいときはカンマで区切る
fn print_laveled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// 5を返す関数
// 戻り値は矢印 -> の後に型を書いて宣言する
// 多くの関数は最後の式を暗黙的に返す。
fn five() -> i32 {
    5 // セミコロンは付けない
}

fn plus_one(x: i32) -> i32 {
    x + 1 // セミコロンを付けてしまうと文になり値に評価されなくなるため、空のタプルになってしまう。
    // そのため関数定義と矛盾するので、結果として「型があわない」というエラーになってしまう
}