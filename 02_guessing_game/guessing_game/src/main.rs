use std::io;
// 사용자 입력을 받고 결과값을 표시하기 위해서는 io (input/output) 라이브러리를 스코프로 가져와야 합니다
// io 라이브러리는 std라고 불리는 표준 라이브러리에 있습니다.
// 러스트는 모든 프로그램의 스코프에 prelude 내의 타입들을 가져옵니다.
// 만약 여러분이 원하는 타입이 prelude에 없다면 use문을 활용하여 명시적으로 그 타입을 가져와야 합니다.

// fn 문법은 새로운 함수를 선언하며 ()는 인자가 없음을 나타내고 {는 함수 본문의 시작을 나타냅니다.
fn main() { // main 함수는 프로그램의 진입점입니다.
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}