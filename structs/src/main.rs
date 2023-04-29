fn main() {

    // 構造体のインスタンスを生成する
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // User インスタンスの email フィールドを変更する
    // インスタンス全体が可変でなければならない
    // Rustでは、一部のフィールドのみを可変にすることはできない。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // user1 の一部の値を使用しつつ、新しい User インスタンスを生成する
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // 構造体更新記法
    // .. という記法により、明示的にセットされていない残りのフィールドが、与えられたインスタンスのフィールドと同じ値になるよう指定できる
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // タプル構造体
    // 構造体名は付けるけど
    // フィールド名を付けなくていい
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // タプルと同じように各要素にアクセスできる
    let r = black.0;
    let g = black.1;
    let b = black.2;

    // タプルと同じように分配できる
    // 構造体名を変数名の前に書く必要がある
    let Point (x, y, z) = origin;

    // ユニット様構造体
    // 空のタプルをユニット型と呼ぶ
    // ユニット型のようなタプル構造体をユニット様構造体と呼ぶ
    // トレイトを実装するけど、フィールドは無いときに有効になる
    struct Unit();
    let unit = Unit();

}

// User構造体の定義
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Eメールとユーザ名を取り、Userインスタンスを返す関数
fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true
    }
}

// フィールド初期化省略法を使う
// 仮引数名と構造体のフィールド名が、一致しているときに使える
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

