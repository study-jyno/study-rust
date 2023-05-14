// use std::io;
// // 사용자 입력을 받고 결과값을 표시하기 위해서는 io (input/output) 라이브러리를 스코프로 가져와야 합니다
// // io 라이브러리는 std라고 불리는 표준 라이브러리에 있습니다.
// // 러스트는 모든 프로그램의 스코프에 prelude 내의 타입들을 가져옵니다.
// // 만약 여러분이 원하는 타입이 prelude에 없다면 use문을 활용하여 명시적으로 그 타입을 가져와야 합니다.

// // fn 문법은 새로운 함수를 선언하며 ()는 인자가 없음을 나타내고 {는 함수 본문의 시작을 나타냅니다.
// fn main() { // main 함수는 프로그램의 진입점입니다.
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess);
// }
extern crate rand; // 우리는 extern crate rand;을 추가하여 러스트에게 우리가 외부에 의존하는 크레이트가 있음을 알립니다
// 이 라인은 use rand으로도 표기할 수 있으며 이제 우리는 rand::를 앞에 붙여 rand내의 모든 것을 호출할 수 있습니다.

use std::io;
use std::cmp::Ordering;
use rand::Rng; // use rand::Rng를 추가합니다. Rng는 정수 생성기가 구현한 메소드들을 정의한 trait이며 해당 메소드들을 이용하기 위해서는 반드시 스코프 내에 있어야 합니다.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 사용자가 숫자가 아닌 값을 입력했을 때 프로그램이 종료되는 동작을 더 다듬어 숫자가 아닌 입력은 무시하여 사용자가 계속 입력할 수 있도록 해 봅시다.
        // guess가 String에서 u32로 변환되는 라인을 수정하면 됩니다.
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // cmp 메소드는 두 값을 비교하며 비교 가능한 모든 것들에 대해 호출할 수 있습니다.
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}