
// 取りうる値をすべて列挙できるから、列挙型と呼ぶ
enum IpAddrKind {
    V4,
    V6,
}

// enumの各列挙子に値を紐づけることができる
// それぞれ型が違ってもいい
// いかなる種類のデータでも格納できる
// 文字列、数値型、構造体、他のenumなど
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 列挙子各々が異なる型と量の値を格納する Message enum
enum Message {
    Quit,                       // 紐づけられたデータは全くなし
    Move { x: i32, y: i32 },    // 中に匿名構造体を含む
    Write(String),              // 単独の String オブジェクトを含む
    ChangeColor(i32, i32, i32), // ３つの i32 の値を含む
}

// enum にもメソッドを定義できる
impl Message {
    fn call(&self) {
        // メソッド本体はここに定義できる
    }
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // どちらの列挙子に対しても呼び出せる
    route(four);
    route(six);

    // enum に定義したメソッドを呼び出す
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> は Option:: の接頭辞なしに直接使える
    let some_number = Some(5);
    let some_string = Some("a string");
    
    // None を使うときは、コンパイラに Option<T> の型が何になるかを教えなければいけない
    let absent_number: Option<i32> = None;

}

// どんな IpAddKind を取る関数も定義できる
fn route (ip_type: IpAddrKind) { }