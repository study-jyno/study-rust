use std::io; // io 입출력 라이브러리를 스코프로 가져옴
use rand::Rng;
// Rng는 난수 생성기를 구현한 메서드들을 정의한 트레이트 (trait) 
// 해당 메서드들을 이용하기 위해서는 반드시 스코프 내에 있어야 합니다

use std::cmp::Ordering;

fn main() { // main 함수는 프로그램의 진입점
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 특정 난수 생성기를 제공하는 rand::thread_rng 함수를 호출
    // gen_range - use rand::Rng; 구문을 통해 스코프로 가져온 Rng 트레이트에 정의되어 있습니다.
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // loop 키워드는 무한루프를 제공합니다. 루프를 추가하여 사용자들에게 숫자를 추리할 기회를 더 주겠습니다.
    loop {
        let mut guess = String::new();
        // 러스트에서 변수는 기본적으로 불변 (immutable)
        // 변수의 값을 가변 (mutable), 즉 변경 가능하도록 하려면 변수명 앞에 mut를 추가

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");


        //게임의 동작을 더욱 다듬기 위해 사용자가 숫자가 아닌 값을 입력할 때 프로그램을 종료시키는 대신 이를 무시하여 사용자가 계속 추릿값을 입력할 수 있도록 만들어 봅시다
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("뭐하냐?");
                continue;
            },
        };
        // 러스트는 이전에 있던 guess의 값을 새로운 값으로 가리는 (shadow) 것을 허용
        // guess.trim() - 좌우 공백 제거
        // .parse() 문자열의 parse 메서드는 문자열을 다른 타입으로 바꿔줍니다.
        // let guess: u32를 사용하여 필요로 하는 정확한 숫자 타입을 러스트에 알려줄 필요가 있습니다
        // i32는 32비트 정수, u32는 32비트의 부호 없는 정수, i64는 64비트의 정수
        // parse 메서드의 호출은 에러가 발생하기 쉽습니다. 예를 들어 A👍%과 같은 문자열이 포함되어 있다면 정수로 바꿀 방법이 없습니다.
        // parse 메서드는 실패할 수도 있으므로, ‘Result 타입으로 잠재적 실패 다루기’에서 다루었던 read_line와 마찬가지로 Result 타입을 반환합니다. 

        // Ordering은 열거형이고 Less, Greater, Equal이라는 배리언트들을 가지고 있습니다.
        match guess.cmp(&secret_number) { // cmp 메서드는 두 값을 비교하며 비교 가능한 모든 것들에 대해 호출
            //  match 표현식을 이용하여 cmp가 guess와 secret_number를 비교한 결과인 Ordering의 값에 따라 무엇을 할 것인지 결정
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
                // 사용자가 정답을 맞혔을 때 게임이 종료되도록 break문을 추가합니다.
            },
                
        }
    }
}

// 카고의 또 다른 멋진 기능에는 cargo doc --open 명령어를 사용하여 의존하는 크레이트의 문서를 로컬에서 모두 빌드한 다음, 브라우저에서 열어주는 기능이 있습니다


/*
근데 컴파일 되지 않네여

$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: associated function defined here
  --> /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/cmp.rs:783:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error

에러의 핵심은 일치하지 않는 타입 (mismatched types) 이 있음을 알려주는 것입니다

러스트는 강한 정적 타입 시스템을 가지고 있습니다. 하지만 타입 추론도 수행합니다.

let guess = String::new() 를 작성한다면 러스트는 guess가 String 타입이어야 함을 추론할 수 있으므로 타입을 작성하지 않아도 됩니다

이 에러의 원인은 러스트가 문자열과 정수형을 비교할 수 없기 때문입니다.

최종적으로는 프로그램이 입력으로 읽은 String을 실제 숫자 타입으로 바꿔서 비밀번호와 숫자로 비교할 수 있도록 하고 싶습니다
*/