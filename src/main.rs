#[path = "module_print_result.rs"]
mod my_module;

// Option型なので戻り値は値を返さない場合がある。
// Option<i32>としているので値が入る場合にはi32型となる。
fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    // 変数ansにifの式(None or Some(x / y))を代入
    // ifは必ず式になる？
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    };
    // 変数ansを返す
    ans
}

// モジュールの定義
mod module_div_result {
    // この関数の戻り値はResult<成功した場合に返される値の型(T), エラーが発生した場合に返される値の型(E)>となっている
    pub fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
        if y == 0 {
            // Result型の場合、エラーの場合はErr()で包む
            Err("div by zero")
        } else {
            // Result型の場合、成功した場合はOK()で包む
            Ok(x / y)
        }
    }
}

// Option<T>型はTypeScriptでいうany
// <T: std::fmt::Display>　-> 「型Tはstd::fmt::Displayいうトレイトで宣言されたメソッドが実装されている型に制限する」と解釈されるもので、トレイト境界と呼ばれる
// 要はそのメソッドが実装されている型じゃないとだめよ、ということか
fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    // 引数ansはOption型。
    // if let Some(x)に変数に値が入っているかをチェックしている
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    // 変数に値が入っているかのチェックのやり方その2。
    // ansに値が入ってたら上、nullだったら下の処理を行う
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None")
    }
}

fn main() {
    func_ex_print_some(func_ex_div_some(10, 5));
    func_ex_print_some(func_ex_div_some(10, 0));
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
    my_module::func_ex_print_result(module_div_result::func_ex_div_result(10, 5));
    my_module::func_ex_print_result(module_div_result::func_ex_div_result(10, 0));
}

#[test]
fn test_1() {
    assert_eq!(func_ex_div_some(10, 5), Some(2)); // 成功
}

#[test]
fn test_2() {
    assert_eq!(func_ex_div_some(10, 5), Some(3)); // 失敗
}