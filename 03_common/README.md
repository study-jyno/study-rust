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

...