// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }


// 매개변수
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// 매개변수
fn main() {
    print_labeled_measurement(5, 'h');
}

// rust 에서는 함수 매개변수에 타입이 강제 됩니다
// 강제 명시가 의도된 언어이기 때문에 무조건 지켜주셔야 합니다.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

