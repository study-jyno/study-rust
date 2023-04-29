# 추리 게임

이번 장은 실제 프로젝트에서 몇몇 일반적인 Rust 개념이 어떻게 활용되는지를 소개하려 합니다.

이 과정에서 let, match, 메소드, 연관함수(associated functions), 외부 크레이트(external crates) 등의 활용 방법을 배울 수 있습니다.

우리는 고전적인 입문자용 프로그래밍 문제인 추리 게임을 구현해 보려 합니다. 이 프로그램은 1~100 사이의 임의의 정수를 생성합니다. 다음으로 플레이어가 프로그램에 추리한 정수를 입력합니다. 프로그램은 입력받은 추리값이 정답보다 높거나 낮은지를 알려줍니다. 추리값이 정답이라면 축하 메세지를 보여주고 종료됩니다.

---

## 새로운 프로젝트를 준비하기

새로운 프로젝트를 준비하기 위해 1장에서 생성했던 디렉토리인 projects 로 이동하고 아래 예제처럼 Cargo를 이용하여 새로운 프로젝트를 생성합니다.

```shell
$ cargo new guessing_game --bin
$ cd guessing_game
```

첫 명령문인 cargo new는 프로젝트의 이름 (guessing_game)을 첫번째 인자로 받습니다. --bin 플래그는 Cargo가 1장과 비슷하게 바이너리용 프로젝트를 생성하도록 합니다. 두번째 명령문은 작업 디렉토리를 새로운 프로젝트의 디렉토리로 변경합니다.

생성된 Cargo.toml 파일을 살펴봅시다.

```toml
Filename: Cargo.toml

[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

```shell
$ cargo run
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
Running `target/debug/guessing_game`
Hello, world!
```

run 명령어는 이번 실습 프로젝트처럼 빠르게 반복(iteration)을 하고 싶을 때 유용합니다. 우리는 다음 iteration으로 넘어가기 전 빠르게 각 iteration을 테스트하고 싶습니다.

src/main.rs 를 다시 열어 두세요. 이 파일에 모든 코드를 작성할 것입니다.

---

## 추리값을 처리하기

프로그램의 첫 부분은 사용자 입력 요청, 입력값의 처리 후 입력값이 기대하던 형식인지 검증합니다. 첫 시작으로 플레이어가 추리한 값을 입력받을 수 있게 할 것입니다. Listing 2-1의 코드를 src/main.rs 에 작성하세요.

```rs
// Filename: src/main.rs

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

// Listing 2-1: 사용자가 추리한 값을 입력 받아 그대로 출력하는 코드
```

이 코드에 담긴 다양한 정보를 하나씩 살펴 보겠습니다. 사용자 입력을 받고 결과값을 표시하기 위해서는 io (input/output) 라이브러리를 스코프로 가져와야 합니다. io 라이브러리는 std라고 불리는 표준 라이브러리에 있습니다.

---

## 값을 변수에 저장하기

다음으로 우리는 다음 코드처럼 사용자의 입력값을 저장할 공간을 생성할 수 있습니다.

```rs
let mut guess = String::new();
```

이제 프로그램이 점점 흥미로워지고 있습니다! 이 짧은 라인에서 여러 일들이 벌어집니다. 이 라인이 변수 를 생성하는 let문임을 주목하세요. 다음 코드도 변수를 선언하는 예시입니다.

```rs
let foo = bar;
```

이 라인은 foo라는 변수를 선언하고 bar라는 값과 묶습니다. 러스트에서 변수는 기본적으로 불변입니다. 다음 예시는 변수 앞에 mut을 이용하여 가변변수를 만드는 법을 보여줍니다.

```rs
let foo = 5; // immutable
let mut bar = 5; // mutable
```

`::new`에 있는 `::`는 `new`가 String 타입의 연관함수 임을 나타냅니다.
연관함수는 하나의 타입을 위한 함수이며, 이 경우에는 하나의 `String` 인스턴스가 아니라 `String` 타입을 위한 함수입니다.

몇몇 언어에서는 이것을 `정적 메소드` 라고 부릅니다.

요약하자면 `let mut guess = String::new();` 라인은 새로운 빈 String 인스턴스와 연결된 가변변수를 생성합니다.
