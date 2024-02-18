# 일반적인 프로그래밍 개념

변수, 기본 타입, 함수, 주석, 그리고 제어 흐름에 대해서 배우게 됩니다.

## 변수와 가변성

변수는 기본적으로 불변 (immutable)
- 안정성과 쉬운 동시성을 활용하는 방식으로 코드를 작성할 수 있도록 하는 넛지 (nudge, 슬며시 선택을 유도하기) 중 하나

변수가 불변일 때, 어떤 이름에 한번 값이 묶이면 그 값은 바꿀 수 없습니다

```rs
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

```shell
❯ cargo check
    Checking common v0.1.0 (/Users/jyno/Desktop/Project/study-rust/03_common/common)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `common` (bin "common") due to 1 previous error
```

컴파일러는 그저 여러분의 프로그램이 아직은 원하는 대로 안전하게 동작하지 않는다고 할 뿐입니다

불변 변수 `x`에 두 번 값을 할당할 수 없다라는 내용의 에러 메시지를 받았습니다.

불변 변수 x에 두 번째 값을 할당하려고 했기 때문이죠.

코드의 한 부분이 변숫값은 변하지 않는다는 전제 하에 작동하고 코드의 다른 부분이 그 값을 바꾼다면, 앞부분의 코드는 원래 지정된 일을 못할 가능성이 생깁니다

러스트 컴파일러는 값이 바뀌지 않을 것이라고 여러분이 지정하면 실제로 그렇도록 보증합니다


하지만 가변성은 아주 유용할 수 있고, 코드 작성을 더 편하게 해 줍니다. 변수는 기본적으로 불변이더라도, 여러분이 2장에서 했던 것처럼 변수명 앞에 mut을 붙여서 가변으로 만들 수 있습니다.

mut를 추가하는 것은 또한 미래에 코드를 읽는 이들에게 코드의 다른 부분에서 이 변수의 값이 변할 것이라는 의도를 전달합니다.

mut를 사용해 x의 값을 5에서 6으로 바꿀 수 있었습니다. 궁극적으로 가변성을 사용할지 말지는 여러분의 몫이고, 특정 상황에서 가장 명확하다고 생각하는 것이 어떤 것이냐에 따라 달라집니다.

---

## 상수

상수 (constant) 는 불변 변수와 비슷한데, 어떤 이름에 묶여 있는 값이고 값을 바꾸는 것이 허용되지 않지만, 변수와는 약간 다른 점들이 있습니다.

상수는 mut와 함께 사용할 수 없습니다. 상수는 기본적으로 불변인 것이 아니고, 항상 불변입니다.

상수는 let 키워드 대신 const 키워드로 선언하며, 값의 타입은 `반드시 명시`되어야 합니다

상수는 전역 스코프를 포함한 어떤 스코프에서도 선언 가능하므로 코드의 많은 부분에서 알 필요가 있는 값에 유용합니다.

상수는 반드시 상수 표현식으로만 설정될 수 있고 런타임에서만 계산될 수 있는 결괏값으로는 안된다는 것입니다.

---

## 섀도잉

새 변수를 이전 변수명과 같은 이름으로 선언할 수 있습니다

이는 해당 변수의 이름을 사용할 때 컴파일러가 두 번째 번수를 보게 될 것이라는 의미입니다.

사실상 두 번째 변수는 첫 번째 것을 가려서, `스스로를 다시 가리거나` `스코프가 끝날 때`까지 변수명의 사용을 가져가 버립니다

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        // 그 후 중괄호를 사용하여 만들어진 안쪽 스코프 내에 있는 세 번째 let 구문 또한 x를 가리고 새로운 변수를 만드는데,
        // 이전 값에 2를 곱해 x에 할당해서 x의 최종값은 12가 됩니다
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // 이 스코프가 끝나면 안쪽의 섀도잉은 끝나서 x는 다시 6으로 돌아옵니다.
    println!("The value of x is: {x}");
}
```

섀도잉은 변수를 mut로 표시하는 것과는 다릅니다. 실수로 let 키워드 없이 변수에 값을 재할당하려고 한다면 컴파일 타임 에러가 발생하기 때문입니다. let을 사용하면, 값을 변형하면서 변형이 완료된 후에는 불변으로 유지할 수 있습니다.


mut과 섀도잉의 또 다른 차이점은 다시금 let 키워드를 사용하여 새로운 변수를 만드는 것이기 때문에 같은 변수명으로 다른 타입의 값을 저장할 수 있다는 것입니다. 예를 들어, 프로그램이 사용자에게 어떤 텍스트 사이에 몇 개의 공백을 넣고 싶은지 공백문자를 입력하도록 요청하고, 이 값을 숫자로 저장하고 싶다 칩시다:

```rs
    let spaces = "   ";
    let spaces = spaces.len();
```

```rs
    let mut spaces = "   ";
    spaces = spaces.len();

```
첫 번째 spaces는 문자열 타입이고 두 번째 spaces는 숫자 타입입니다. 따라서 섀도잉은 spaces_str과 spaces_num 같이 구분되는 변수명을 쓸 필요가 없도록 여유를 줍니다; 즉, 더 간단한 spaces라는 이름을 재사용할 수 있게 해 줍니다. 그런데 여기에서 mut을 사용하려 한다면, 보시다시피 컴파일 타임 에러가 발생합니다:


```sh
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error

```

---

# 데이터 타입

러스트의 모든 값은 특정한 타입을 가지며, 이는 러스트가 해당 데이터로 작업하는 방법을 알 수 있도록 어떤 종류의 데이터가 지정되고 있는지 알려줍니다. 여기서는 타입을 스칼라 타입과 복합 타입, 두 가지 부분 집합으로 나누어 보겠습니다.

러스트는 `정적 타입의 (statically typed)` 언어라는 점을 주지하세요. 이게 의미하는 바는 모든 변수의 `타입이 컴파일 시점`에 반드시 정해져 있어야 한다는 겁니다.

## 스칼라 타입

스칼라 (scalar) 타입은 하나의 값을 표현합니다

러스트는 정수, 부동 소수점 숫자, 부울린 (boolean), 그리고 문자. 이렇게 4개의 타입을 가지고 있습니다.

## 복합 타입


복합 타입 (compound type) 은 여러 값을 하나의 타입으로 묶을 수 있습니다.

러스트에는 튜플 (tuple) 과 배열 (array), 두 가지 기본 복합 타입이 있습니다.


튜플 타입
- 튜플은 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만드는 일반적인 방법입니다. 튜플은 고정된 길이를 갖습니다.
- 한번 선언되면 그 크기를 늘리거나 줄일 수 없습니다.

튜플은 하나의 복합 요소로 취급되므로 변수 tup은 튜플 전체가 바인딩됩니다. 튜플로부터 개별 값을 얻어오려면 아래와 같이 패턴 매칭을 하여 튜플 값을 해체하면 사용하면 됩니다:

```rs
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

마침표(.) 뒤에 접근하고자 하는 값의 인덱스를 쓰는 방식으로도 튜플 요소에 접근할 수 있습니다. 예를 들면:

파일명: src/main.rs
```rs
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```


배열 타입

여러 값의 집합체를 만드는 다른 방법으로는 배열이 있습니다.

튜플과는 달리 `배열의 모든 요소는 모두 같은 타입`이여야 합니다. 몇몇 다른 언어들과는 달리 `러스트의 배열은 고정된 길이`를 갖습니다.

대괄호 안에 쉼표로 구분한 값들을 나열해서 배열을 만들 수 있습니다:

파일명: src/main.rs
```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

힙보다는 스택에 데이터를 할당하고 싶을 때나 (힙과 스택은 4장에서 더 다루겠습니다) 항상 고정된 개수의 요소로 이루어진 경우라면 배열이 유용합니다.

하지만 배열은 벡터 타입처럼 유연하지는 않습니다.

벡터는 표준 라이브러리가 제공하는 배열과 유사한 컬렉션 타입인데 크기를 늘리거나 줄일 수 있습니다

배열을 이용할지 혹은 벡터를 이용할지 잘 모르겠다면, 아마도 벡터를 사용해야 할 겁니다

배열 요소에 접근하기
배열은 스택에 할당될 수 있는 계산 가능한 고정된 크기의 단일 메모리 뭉치입니다. 아래와 같이 인덱스를 통해 배열 요소에 접근할 수 있습니다:

파일명: src/main.rs
```rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```


유효하지 않은 배열 요소에 대한 접근

만약 배열의 끝을 넘어선 요소에 접근하려고 하면 어떤 일이 벌어지는지 알아봅시다.

```rs
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

이 코드는 성공적으로 컴파일됩니다.

cargo run으로 코드를 실행한 뒤 0, 1, 2, 3, 혹은 4를 입력한다면 프로그램은 그 인덱스에 해당하는 배열 값을 출력할 것입니다.

그 대신에 이 배열의 끝을 넘어서는 10 같은 숫자를 입력하면, 아래와 같은 출력을 보게 될 것입니다:

```sh

thread 'main' panicked at src/main.rs:44:19:
index out of bounds: the len is 5 but the index is 10
```

프로그램은 인덱스 연산에서 잘못된 값을 사용한 시점에서 런타임 에러를 발생시켰습니다.

이 프로그램은 에러 메시지와 함께 종료되고 마지막 println! 구문을 실행하지 못했습니다.

인덱스를 이용하여 요소에 접근을 시도하는 경우, 러스트는 여러분이 명시한 인덱스가 배열 길이보다 작은지 검사할 것입니다.

인덱스가 배열 길이보다 크거나 같을 경우 러스트는 패닉 (panic) 을 일으킵니다.

특히 위의 경우 이러한 검사는 런타임에서 일어나야 하는데, 이는 사용자가 코드를 실행한 뒤에 어떤 값을 입력할지 컴파일러로서는 알 수 없기 때문입니다.

이 예제는 러스트의 안전성 원칙이 동작하는 하나의 예입니다.

많은 저수준 언어들에서는 이러한 검사가 이루어지지 않고, 여러분이 잘못된 인덱스를 제공하면 유효하지 않은 메모리에 접근이 가능합니다.

러스트는 이런 메모리 접근을 허용하고 계속 실행하는 대신 `즉시 실행을 종료`함으로써 이런 종류의 에러로부터 여러분을 보호합니다.

---

# 함수

여러분은 이미 이 언어에서 가장 중요한 함수 중 하나를 보셨습니다: 바로 많은 프로그램의 시작점인 main 함수를 말이죠. 또한 새로운 함수를 선언하도록 해주는 fn 키워드도 보셨습니다.


러스트 코드는 함수나 변수 이름을 위한 관례로 스네이크 케이스 (snake case) 방식을 이용

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

소스 코드 내에서 another_function이 main 함수 이후에 정의되어 있다는 점을 주목하세요.

이 함수를 main 함수 앞에서 정의할 수도 있습니다.

러스트는 여러분의 함수 위치를 고려하지 않으며, 호출하는 쪽에서 볼 수 있는 스코프 어딘가에 정의만 되어있으면 됩니다.

## 매개변수

함수는 매개변수 (parameter) 를 갖도록 정의될 수 있으며, 이는 함수 시그니처 (function signiture) 의 일부인 특별한 변수입니다


**함수 시그니처에서는 각 매개변수의 타입을 반드시 선언해야 합니다.**

**이는 러스트를 설계하면서 신중하게 내린 결정 사항입니다:**

1. 함수의 정의에 타입 명시를 강제하면 이 함수를 다른 코드에서 사용할 때 여러분이 의도한 타입을 컴파일러가 추측하지 않아도 되게 됩니다.
2. 컴파일러는 또한 함수가 기대한 타입이 무엇인지 알고 있으면 더욱 유용한 에러 메시지를 제공할 수 있습니다.

## 구문과 표현식

함수 본문은 필요에 따라 `표현식 (expression)` 으로 끝나는 `구문 (statement)` 의 나열로 구성됩니다

러스트는 표현식 기반의 언어이므로, 구문과 표현식의 구분은 러스트를 이해하는데 중요합니다

다른 언어들은 이런 구분이 없으므로, 구문과 표현식이 무엇이며 둘 간의 차이가 함수의 본문에 어떤 영향을 주는지 살펴보겠습니다.

- 구문은 어떤 동작을 수행하고 값을 반환하지 않는 명령입니다.
- 표현식은 결괏값을 평가합니다. 몇 가지 예제를 살펴봅시다.


우리는 실제로는 이미 구문과 표현식을 사용해 봤습니다.

let 키워드로 변수를 만들고 값을 할당하는 것은 구문입니다. 예제 3-1의 let y = 6;은 구문입니다:
```rs
fn main() {
    let y = 6;
}
```

또한 함수 정의도 구문입니다; 위 예제는 그 자체로 구문에 해당됩니다.


구문은 값을 반환하지 않습니다. 따라서 아래와 같이 let 구문을 다른 변수에 할당하려고 하면 에러가 납니다:
```rs
fn main() {
    let x = (let y = 6);
}
```

```shell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 3 previous errors; 1 warning emitted

```

let y = 6 구문은 값을 반환하지 않으므로 x에 바인딩시킬 것이 없습니다.

이것이 C나 Ruby 같은 다른 언어와의 차이점인데, 이 언어들은 할당문이 할당된 값을 반환하죠.

이런 언어들에서는 x = y = 6라고 작성하여 x와 y에 모두 6을 대입할 수 있지만, 러스트에서는 그렇지 않습니다.

여러분이 작성하는 러스트 코드의 대부분은 표현식이며, 이는 어떤 값을 평가합니다.

5 + 6과 같은 간단한 수학 연산을 살펴봅시다.

이 수식은 11이라는 값을 평가하는 표현식입니다. 표현식은 구문의 일부일 수 있습니다:

예제 3-1의 let y = 6;이라는 구문에서 6은 6이라는 값을 평가하는 표현식입니다.

함수를 호출하는 것도, 매크로를 호출하는 것도 표현식입니다. 아래 예제처럼 중괄호로 만들어진 새로운 스코프 블록도 표현식입니다:

```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```


같은 경우에는 4를 평가하는 코드 블록입니다.

이 값이 let 구문의 일부로서 y에 바인딩됩니다.

여러분이 지금까지 봐온 것과 다르게 x + 1 줄의 마지막이 세미콜론으로 끝나지 않은 점을 주목하세요.

표현식은 종결을 나타내는 세미콜론을 쓰지 않습니다.

만약 표현식 끝에 세미콜론을 추가하면, 표현식은 구문으로 변경되고 값을 반환하지 않게 됩니다.

이 점을 상기하면서 이후부터 함수의 반환 값과 표현식을 살펴보길 바랍니다.

## 매개변수

함수는 호출한 코드에 값을 반환할 수 있습니다.

반환되는 값을 명명해야 할 필요는 없지만, 그 값의 타입은 화살표 (->) 뒤에 선언되어야 합니다. 러스트에서 함수의 반환 값은 함수 본문의 마지막 표현식의 값과 동일합니다

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```


```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

이 코드를 실행하면 The value of x is: 6이 출력됩니다. 만일 x + 1 끝에 세미콜론이 추가되어 표현식이 구문으로 변경되면 에러가 발생합니다:

```rs
// This code does not compile!
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

이 코드를 컴파일하면 다음과 같은 에러가 나타납니다:

```shell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

주 에러 메시지 mismatched types는 이 코드의 핵심 문제를 보여줍니다.

plus_one 함수의 정의에는 i32 값을 반환한다고 되어 있지만, 구문은 값을 평가하지 않기에 `()로 표현되는 유닛` 타입으로 표현됩니다.

따라서 아무것도 반환되지 않아 함수가 정의된 내용과 상충하게 되어 에러가 발생됩니다.

위의 출력 결과에서 러스트는 이 문제를 바로잡는데 도움이 될 수 있는 메시지를 제공합니다: 바로 세미콜론을 제거하면 에러가 수정될 것이란 제안이지요.


# 제어 흐름문

러스트 코드의 실행 흐름을 제어하도록 해주는 가장 일반적인 재료는 `if` 표현식과 `반복문`입니다.

## if 표현식

```rs
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```


또한 이 코드의 조건식이 반드시 bool 이어야 한다는 점을 주목할 가치가 있습니다. 조건식이 bool이 아니면 에러가 발생합니다. 예를 들자면 아래 코드를 실행해보세요:

```rs
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

이 경우에는 if 조건식의 결과가 3이고, 러스트는 에러를 발생시킵니다.
```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```


if문에는 항상 명시적으로 부울린 타입의 조건식을 제공해야 합니다.


## else if로 여러 조건식 다루기

if와 else 사이에 else if를 조합하면 여러 조건식을 사용할 수 있습니다. 예를 들면:

```rs
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
이 프로그램을 실행하면 각각의 if 표현식을 순차적으로 검사하고 조건이 참일 때의 첫 번째 본문을 실행합니다.

6이 2로 나누어 떨어지지만 number is divisible by 2나 number is not divisible by 4, 3, or 2와 같은 else 블록의 텍스트가 출력되지 않는다는 점을 주목하세요.

이는 러스트가 처음으로 true인 조건의 본문을 실행하고나면 나머지는 검사도 하지 않기 때문입니다.

## let 구문에서 if 사용하기

if는 표현식이기 때문에 예제 3-2처럼 변수에 결과를 할당하기 위하여 let 구문의 우변에 사용할 수 있습니다.

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

코드 블록은 블록 안의 마지막 표현식을 계산하고, 숫자는 그 자체로 표현식임을 기억하세요.

위의 경우 전체 if 표현식의 값은 실행되는 코드 블록에 따라 결정됩니다. 그렇기에 if 표현식의 각 갈래의 결괏값은 같은 타입이어야 합니다.
```rs
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```
이 코드를 컴파일하려고 하면 에러가 발생합니다. if와 else 갈래 값의 타입이 호환되지 않고, 러스트는 프로그램의 어느 지점에 문제가 있는지 정확히 알려줍니다:
```shell
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

if 블록은 정숫값을 계산하는 표현식이고 else 블록은 문자열로 평가되는 표현식입니다.

이런 형태의 코드가 동작하지 않는 이유는 변수가 가질 수 있는 타입이 오직 하나이기 때문입니다.

러스트에서는 number의 타입이 런타임에 정의되도록 할 수 없습니다;

컴파일러가 어떤 변수에 대해 여러 타입에 대한 가정값을 추적해야 한다면 컴파일러는 더 복잡해지고 보장할 수 있는 것들이 줄어들 것입니다.

## 반복문을 이용한 반복

코드 블록을 한 번 이상 수행하는 일은 자주 쓰입니다.

반복 작업을 위해서, 러스트는 몇 가지 반복문 (loop) 을 제공하는데 이는 루프 본문의 시작부터 끝까지 수행한 뒤 다시 처음부터 수행합니다.

반복문을 실험해보기 위해 loops라는 이름의 새 프로젝트를 만듭시다.

러스트에는 loop, while, 그리고 for라는 세 종류의 반복문이 있습니다. 하나씩 써봅시다.

### loop 로 코드 반복하기

```rs
fn main() {
    loop {
        println!("again!");
    }
}
```


이 프로그램을 실행시키면, 우리가 프로그램을 강제로 정지시키기 전까지 again!이 계속 반복적으로 출력되는 것을 보게 됩니다.


다행히 러스트에서는 코드를 사용하여 루프에서 벗어나는 방법도 제공합니다.

루프 안에 `break` 키워드를 집어넣으면 루프를 멈춰야 하는 시점을 프로그램에게 알려줄 수 있습니다. 

추리 게임에서는 `continue`도 사용했었는데, 이는 프로그램에게 이번 회차에서 루프에 남은 코드를 넘겨버리고 다음 회차로 넘어가라고 알려줍니다.


### 반복문에서 값 반환하기

loop의 용례 중 하나는 어떤 스레드가 실행 완료되었는지 검사하는 등 실패할지도 모르는 연산을 재시도할 때입니다. 

여기서 해당 연산의 결과를 이후의 코드에 전달하고 싶을 수도 있습니다.

이를 위해서는 루프 정지를 위해 사용한 break 표현식 뒤에 반환하고자 하는 값을 넣으면 됩니다;

해당 값은 아래와 같이 반복문 밖으로 반환되여 사용 가능하게 됩니다:

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### 루프 라벨로 여러 반복문 사이에 모호함 없애기

만일 루프 안에 루프가 있다면, break와 continue는 해당 지점의 바로 바깥쪽 루프에 적용됩니다.

루프에 `루프 라벨 (loop label)` 을 추가적으로 명시하면 break나 continue와 함께 이 키워드들이 바로 바깥쪽 루프 대신 라벨이 적힌 특정한 루프에 적용되도록 할 수 있습니다.

루프 라벨은 반드시 `작은 따옴표`로 시작해야 합니다. 아래에 루프가 두 개 중첩된 예제가 있습니다

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

바깥쪽 루프는 'counting_up 이라는 라벨이 붙어있고, 0에서부터 2까지 카운트합니다.

라벨이 없는 안쪽 루프는 10에서 9까지 거꾸로 카운트합니다.

라벨이 명시되지 않은 첫 번째 break는 안쪽 루프만 벗어납니다.

`break 'counting_up;` 구문은 바깥쪽 루프를 탈출할 것입니다. 이 코드는 다음을 출력합니다:

```shell
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

### while을 이용한 조건 반복문

조건문이 true인 동안에는 계속 반복하는 형태죠.

조건문이 true가 아니게 될 때 프로그램은 break를 호출하여 반복을 종료합니다.

이러한 반복문 형태는 loop, if, else와 break의 조합으로 구현할 수 있습니다;

여러분이 원하신다면 그렇게 시도해볼 수 있습니다.

하지만 이러한 패턴은 매우 흔하기 때문에 러스트에서는 while 반복문이라 일컫는 구조가 내장되어 있습니다.

```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```


### for를 이용한 컬렉션에 대한 반복문

while를 사용하여 배열과 같은 컬렉션의 각 요소에 대한 반복문을 작성할 수 있습니다.

한 가지 예로 예제 3-4의 반복문은 a라는 배열의 각 요소를 출력합니다.

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

하지만 이런 접근 방식은 에러가 발생하기 쉽습니다;

즉 인덱스의 길이가 부정확하면 패닉을 발생시키는 프로그램이 될 수 있습니다.

예를 들어, a 배열이 네 개의 요소를 갖도록 정의 부분을 변경했는데 while index < 4의 조건문을 고치는걸 잊어버린다면 코드는 패닉을 일으킬 것입니다.

또한 컴파일러가 루프의 매 반복 회차마다 인덱스가 범위 안에 있는지에 대한 조건문 검사를 수행하는 코드를 붙이기 때문에 느려집니다.

좀 더 간편한 대안으로 for 반복문을 사용하여 컬렉션의 각 아이템에 대하여 임의의 코드를 수행시킬 수 있습니다. for 반복문은 예제 3-5의 코드처럼 생겼습니다.

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```


이 코드를 실행하면 예제 3-4의 결과와 동일한 결과를 보게 됩니다.

그보다 더 중요한 것은 이렇게 함으로써 코드의 안전성이 강화되고 배열의 끝을 넘어서거나 끝까지 가지 못해서 몇 개의 아이템을 놓쳐서 발생할 수도 있는 버그의 가능성을 제거했다는 것입니다.


for 루프를 사용하면 여러분이 배열 내 값의 개수를 변경시키더라도 수정해야 할 다른 코드를 기억해둘 필요가 없어질 겁니다.



이러한 안전성과 간편성 덕분에 for 반복문은 러스트에서 가장 흔하게 사용되는 반복문 구성요소가 되었습니다.

심지어 while 반복문을 사용했던 예제 3-3의 카운트다운 예제처럼 어떤 코드를 몇 번 정도 반복하고 싶은 경우라도, 대부분의 러스타시안들은 for 반복문을 이용할 겁니다.

표준 라이브러리가 제공하는 Range 타입을 이용하면 특정 횟수만큼의 반복문을 구현할 수 있는데, 

Range는 어떤 숫자에서 시작하여 다른 숫자 종료 전까지의 모든 숫자를 차례로 생성해줍니다.

```rs
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

정리
해냈군요! 정말 긴 장이었습니다: 여러분은 변수, 스칼라 타입 및 복합 타입, 함수, 주석, if 표현식, 그리고 루프에 대해 배웠습니다! 이 장에서 다룬 개념들을 연습하고 싶다면 아래 프로그램 만들기에 도전해보세요:

화씨 온도와 섭씨 온도 간 변환하기
n번째 피보나치 수 생성하기
크리스마시 캐롤 ‘The Twelve Days of Christmas’ 노래의 반복성을 활용하여 가사 출력해보기
