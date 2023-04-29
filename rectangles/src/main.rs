// derive（デリーブ）注釈でトレイトを使えるようにする
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド記法
impl Rectangle {
    // 最初の引数は必ず self になる
    // 所有権を奪う必要はないので、参照を使っている
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // ２つの引数をとるメソッド
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数
    // selfを引数に取らない場合は関連関数と呼ぶ
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // デバッグ用整形機で出力する
    // {:?} または {:#?} を使う
    println!("rect1 は {:?} です", rect1);
    println!(
        "長方形の面積は、{}平方ピクセルです", rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 に rect2 は、はまり込む？ {}", rect1.can_hold(&rect2));
    println!("rect1 に rect3 は、はまり込む？ {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(50);
    println!("{:?}", square1);

}
