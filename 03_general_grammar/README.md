# 보편적인 프로그래밍 개념

이번 챕터에서는 모든 프로그래밍 언어가 대부분 가진 개념이 Rust에서는 어떻게 다루어지는지 알아보고자 합니다.

많은 프로그래밍 언어가 보편적인 핵심요소를 갖습니다. 보편적인 프로그래밍 개념을 Rust의 문법을 설명하는 과정에서 토의하고자 합니다.

특히 변수, 기본 타입들, 함수, 주석, 그리고 제어문에 대해서 배울 수 있을 것입니다.

---

## 변수와 가변성

**기본 변수는 불변성** - 안전성과 손쉬운 동시성이라는 장점을 취할 수 있도록 코드를 작성하게끔 강제하는 요소 중 하나

하지만 여전히 당신은 가변 변수를 사용하고 싶을테죠. - Rust에서 불변성을 애호해주길 권장하는지 알아보면 그런 생각을 포기할 수 있을지도 모르겠습니다.

변수가 불변성인 경우, 일단 값이 이름에 bound되면 해당 값을 변경할 수 없습니다

```shell
cargo new --bin variables
```

Filename: src/main.rs

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

```shell
➜  variables git:(main) ✗ cargo run
   Compiling variables v0.1.0 (/Users/jyno/Desktop/Project/study-rust/03_general_grammar/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

에러가 나타내는 것은 불변성 변수에 재할당이고, 원인은 우리가 불변성 변수 x에 두 번째로 값을 할당했기 때문입니다.

불변성으로 선언한 것의 값을 변경하고자 하는 시도를 하면 컴파일 타임의 에러를 얻게 되고 이로 인해 버그가 발생할 수 있기 때문에 중요합니다

만약 우리 코드의 일부는 **값이 변경되지 않는다는 것을 가정**하는데 **다른 코드는 이와 다르게 값을 변경**한다면, 전자에 해당하는 코드는 **우리가 의도한 대로 수행되지 않을 수 있습니다**.

하지만 가변성은 매우 유용하게 사용될 수 있습니다

변수는 기본적으로 불변성이지만 우리는 변수명의 접두어로 mut을 추가하는 것을 통해 가변성 변수를 선언할 수 있습니다

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

```shell
➜  variables git:(main) ✗ cargo run
   Compiling variables v0.1.0 (/Users/jyno/Desktop/Project/study-rust/03_general_grammar/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

mut를 사용하여, x에 bind된 값을 5에서 6으로 변경할 수 있습니다. 불변성 변수만을 사용하는 것보다 가변성 변수를 사용하여 보다 쉽게 구현할 수 있을 경우 가변성 변수를 만들어 사용할 수도 있습니다.

그렇기에 약간의 성능 하락을 통해 가독성을 확보할 수 있다면 더 가치있는 선택입니다.

---

## 변수와 상수 간의 차이점들

변수의 값을 변경할 수 없다? 이건 상수랑 동일한 문법이 아닌가?

불변성 변수와 마찬가지로 상수 또한 이름으로 bound된 후에는 값의 변경이 허용되지 않지만, 상수와 변수는 조금 다릅니다.

첫 째로, 상수에 대해서는 mut을 사용하는 것이 허용되지 않습니다: 상수는 기본 설정이 불변성인 것이 아니고 **불변성 그 자체**입니다.

우리가 상수를 사용하고자 하면 let키워드 대신 const키워드를 사용해야 하고, 값의 유형을 선언해야 합니다

상수는 전체 영역을 포함하여 어떤 영역에서도 선언될 수 있습니다. 이는 코드의 많은 부분에서 사용될 필요가 있는 값을 다루는데 유용합니다.

상수는 오직 상수 표현식만 설정될 수 있지, 함수 호출의 결과값이나 그 외에 실행 시간에 결정되는 값이 설정될 수는 없다는 점입니다.

상수는 **자신이 선언되어 있는 영역 내에서 프로그램이 실행되는 시간 동안 항상 유효하**기에, 당신의 어플리케이션 **도메인 전체에 걸쳐 프로그램의 다양한 곳에서 사용되는 값을 상수로 하면 유용**합니다. 사용자가 한 게임에서 획득할 수 있는 최대 포인트, 빛의 속도 같은 값 등등...

당신의 프로그램 전체에 걸쳐 하드코드 해야 하는 값을 이름지어 상수로 사용하면 향후 코드를 유지보수 하게 될 사람에게 그 의미를 전달할 수 있으므로 유용합니다. 또한 향후 해당 값을 변경해야 하는 경우에 상수로 선언된 값 한 곳만 변경하면 되므로 도움이 될 겁니다.

---

## Shadowing

앞서 우리가 2장에서 추측 게임 예제를 통해 봤듯이, 이전에 선언한 변수와 같은 이름의 새 변수를 선언할 수 있고, 새 변수는 이전 변수를 shadows하게 됩니다.

Rustaceans들은 이를 첫 변수가 두 번째에 의해 shadowed 됐다고 표현하게 됩니다. 해당 변수명은 두 번째 변수의 값을 갖게 된다는 뜻이죠.

```rust
fn main() {
    let x = 5; // 처음 x에 값 5를 bind

    let x = x + 1; // 이후 반복된 let x = 구문으로 x를 shadow 하고 원본 값에 1을 더해서 x의 값은 6

    let x = x * 2; // 세 번째 let 문으로 또 x를 shadow하고, 이전 값에 2를 곱하여 x의 최종값은 12

    println!("The value of x is: {}", x);
    // 우리가 몇 번 값을 변경할 수는 있지만 그 이후에 변수는 불변성을 갖게 됩니다.
}
```

또 다른 mut과 shadowing의 차이는 let키워드를 다시 사용하여 효과적으로 새 변수를 선언하고, 값의 유형을 변경할 수 있으면서도 동일 이름을 사용할 수 있다는 점 입니다.

예를 들어, 공백 문자들을 입력받아 얼마나 많은 공백 문자가 있는지 보여주고자 할 때, 실제로는 저장하고자 하는 것은 공백의 개수일테죠.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

Shadowing은 space_str이나 space_num 과 같이 대체된 이름을 사용하는 대신 간단히 spaces 이름을 사용할 수 있게 해줍니다. 그러나 우리가 mut을 사용하려고 했다면

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

우리는 다음처럼 변수의 유형을 변경할 수 없다는 컴파일-시의 에러를 얻게 될 겁니다:

```shell
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

변수가 어떻게 동작하는지 탐구했으니, 더 많은 데이터 유형을 살펴보도록 합시다.

## 스칼라 타입들

스칼라는 하나의 값으로 표현되는 타입입니다. Rust는 정수형, 부동소수점 숫자, boolean, 그리고 문자, 네 가지 스칼라 타입을 보유하고 있습니다.

## 정수형

정수형은 소수점이 없는 숫자 입니다.

## 부동 소수점 타입

Rust에는 소수점을 갖는 숫자인 부동소수점 숫자를 위한 두 가지 기본 타입도 있습니다. Rust의 부동소수점 타입은 f32와 f64로, 예상하신 대로 각기 32bit와 64bit의 크기를 갖습니다.

기본 타입은 f64인데, 그 이유는 최신의 CPU 상에서는 f64가 f32와 대략 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문

## Boolean 타입

대부분의 다른 언어들처럼, boolean 타입은 Rust에서 둘 중 하나의 값만 가질 수 있습니다: true와 false. boolean 타입은 러스트에서 bool로 명시됩니다.

```rust
// Filename: src/main.rs

fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## 복합 타입들

복합 타입들은 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있습니다. Rust는 두 개의 기본 타입들을 갖고 있습니다: 튜플과 배열.

## 값들을 집합시켜서 튜플화하기.

튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법입니다.

우리는 괄호 안에 콤마로 구분되는 값들의 목록을 작성하여 튜플을 만듭니다. 튜플에 포함되는 각 값의 타입이 동일할 필요없이 서로 달라도 됩니다. 다음의 예제에 우리는 선택 사항인 타입 명시를 추가했습니다.

```rust
// Filename: src/main.rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

패턴 매칭을 통한 구조해체에 추가로, 우리는 마침표(.) 뒤에 우리가 접근하길 원하는 값의 색인을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있습니다. 예제를 봅시다:

```rust
// Filename: src/main.rs

fn main() {
let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

}
```

위의 프로그램은 튜플 x를 만들고, 이의 각 요소들을 그들의 색인을 통해 접근하여 새 변수를 만듭니다. 대부분의 언어가 그렇듯이, 튜플의 첫 번째 색인은 0 입니다.

## 배열

여러 값들의 집합체를 만드는 다른 방법은 배열입니다. 튜플과는 다르게, 배열의 모든 요소는 모두 같은 타입이여야 합니다.

Rust에서는 배열은 고정된 길이를 갖는다는 점입니다: 한번 선언되면, 이들은 크기는 커지거나 작아지지 않습니다.

Rust에서는 대괄호 안에 값들을 콤마로 구분하여 나열해서 배열을 만듭니다:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

### 배열 요소에 접근하기

배열은 stack에 단일 메모리 뭉치로 할당됩니다. 우리는 색인을 통해 배열의 요소에 접근할 수 있습니다.

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

}
```

### 유효하지 않은 배열 요소에 대한 접근

만약 우리가 배열의 끝을 넘어선 요소에 접근하려고 하면 어떻게 될까요? 예제를 다음처럼 변경해봤습니다.

```rs
fn main() {
let a = [1, 2, 3, 4, 5];
let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);

}
```

```shell
$ cargo run
Compiling arrays v0.1.0 (file:///projects/arrays)
Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
10', src/main.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

컴파일 시에는 아무런 에러도 발생시키지 않습니다만, 프로그램의 결과는 실행 중에 에러가 발생했고 성공적으로 종료되지 못했다고 나옵니다.

색인을 사용하여 요소에 접근하려고 하면 Rust는 지정한 색인이 배열 길이보다 작은지 확인합니다.

색인이 길이보다 길면 Rust는 프로그램이 오류와 함께 종료 될 때 Rust가 사용하는 용어인 *패닉(panic)*합니다.

이것은 Rust의 안전 원칙이 동작하는 첫 번째 예입니다.

많은 저수준 언어에서 이러한 타입의 검사는 수행되지 않으며 잘못된 색인을 제공하면 유효하지 않은 메모리에 액세스 할 수 있습니다.

Rust는 메모리 접근을 허용하고 계속 진행하는 대신 즉시 종료하여 이러한 종류의 오류로부터 사용자를 보호합니다. 9 장에서는 Rust의 오류 처리에 대해 자세히 설명합니다.

---

함수 동작 원리 해야함
https://rinthel.github.io/rust-lang-book-ko/ch03-03-how-functions-work.html
