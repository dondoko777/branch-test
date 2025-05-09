// Person 構造体
#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}
// Person 構造体を借用(borrow)する
fn print_a( a: &Person ) {
    println!("print_a: a is {:?}", a ) ;
}
// Person 構造体をmoveする
fn move_a( a: Person ) {
    println!("move_a: a is {:?}", a ) ;
}
// Person 構造体の内容を変更する
fn add_age( a: &mut Person ) {
    a.age += 1 ;
}

fn main1() {
    let a = Person { name: "masuda", age: 50 };
    // 借用させる
    print_a( &a );
    // 変数aは main に残ったまま
    println!("main: a is {:?}", a );
}

fn main2() {
    let a = Person { name: "masuda", age: 50 };
    // 所有権を move する
    move_a( a );
    // 所有権は print_x に移るので、ここでは使えない
    // println!("main: a is {:?}", a );
}

fn main3() {
    let a = Person { name: "masuda", age: 50 };
    // 変数xに参照（借用）させる
    let x = &a ;
    // 変数a,xの両方が使える
    println!("a is {:?}", a );
    println!("x is {:?}", x );
}

fn main4() {
    let a = Person { name: "masuda", age: 50 };
    // 変数xに所有権をmoveする
    let x = a ;
    // 変数aは使えない
    // println!("a is {:?}", a );
    println!("x is {:?}", x );
}

fn main44() {
    let a = Person { name: "masuda", age: 50 };
    // 変数xに所有権をmoveする
    let x = a ;
    // let y = a ;
    // println!("a is {:?}", a );
    println!("x is {:?}", x );
    // println!("y is {:?}", y );
}


fn main5() {
    let mut a = Person { name: "masuda", age: 50 };
    println!("a is {:?}", a );
    add_age( &mut a );
    println!("a is {:?}", a );
}


fn main6() {
    let a = Person { name: "masuda", age: 50 };
    let mut x = a ;
    println!("x is {:?}", x );
    add_age( &mut x );
    println!("x is {:?}", x );
    // add_age( &mut a );
    // println!("a is {:?}", a );
}


fn main7() {
    let a = Person { name: "masuda", age: 50 };
    let mut x = &a ;
    println!("x is {:?}", x );
    // x.age += 1 ;
    // println!("x is {:?}", x );
}

fn main() {
    // 可変の変数aを作る
    let mut a = Person { name: "masuda", age: 50 };
    // 可変の変数xで参照する
    let mut x = &mut a ;
    println!("x is {:?}", x );
    x.age += 1 ;
    println!("x is {:?}", x );
    add_age(&mut x);
    println!("x is {:?}", x );
}

fn main9() {
    // 可変の変数aを作る
    let a = Person { name: "masuda", age: 50 };
    // 可変の変数xに所有権を移す
    let mut x = a ;
    println!("x is {:?}", x );
    x.age += 1 ;
    println!("x is {:?}", x );
    add_age(&mut x);
    println!("x is {:?}", x );
    // 変数aは所有権がないので使えない
    // a.age += 1 ;
    // println!("a is {:?}", a );
}

fn main10() {
    // 数値の場合
    let a = 100 ;
    println!("a is {}", a );
    // 数値の場合は x に値が copy されている
    let x = a ;
    println!("x is {}", x );
    // 所有権は a のままなので、copy できる
    let y = a ;
    println!("y is {}", y );
}

fn main11() {
    // タプルの場合
    let a = (100,"masuda") ;
    println!("a is {:?}", a );
    // タプルの場合は x に値が copy されている
    let x = a ;
    println!("x is {:?}", x );
    // 所有権は a のままなので、さらに copy できる
    let y = a ;
    println!("y is {:?}", y );
}

fn main12() {
    // ベクタの場合
    let a = vec!["one","two","three"];
    println!("a[] is {:?}", a );
    // ベクタの場合は x に所有権が移動（move）される
    let x = a ;
    println!("x[] is {:?}", x );
    // 所有権がないので、変数aを使えない
    // println!("a[] is {:?}", a )
}

fn main13() {
    // ベクタの場合
    let a = vec!["one","two","three"];
    println!("a[] is {:?}", a );
    // ベクタの場合は参照ならば大丈夫
    let x = &a ;
    println!("x[] is {:?}", x ) ;
    // もう一度、変数a が使える
    println!("a[] is {:?}", a )
}

fn main14() {
    // 文字列の場合
    let a = String::from("masuda");
    println!("a is {}", a );
    // 文字列の場合は x に所有権が移動（move）される
    let x = a ;
    println!("x is {}", x );
    // 所有権がないので、変数aは使えない
    // println!("a is {}", a );
}

fn main15() {
    // 文字列の場合
    let a = String::from("masuda");
    println!("a is {}", a );
    // 文字列の場合も参照ならば大丈夫
    let x = &a ;
    println!("x is {}", x );
    // もう一度、変数aが使える
    println!("a is {}", a );
}


// 変数aをxとyで所有権を争う例
fn main16() {
    // 変数aを作る
    let a = Person { name: "masuda", age: 50 };
    println!("a is {:?}", a );
    // 変数xが参照する
    let x = &a;
    println!("変数xが借用する");
    println!("x is {:?}", x );
    println!("a is {:?}", a );
    // 変数yに所有権を移す
    let y = a;
    println!("変数yに所有権を移す");
    println!("y is {:?}", y );
    // 変数aはすでに使えない
    // println!("a is {:?}", a );
    // 実は、変数aを借用している変数xも使えない
    // println!("x is {:?}", x );
}

