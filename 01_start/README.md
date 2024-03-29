## 1.2 Hello World


### 러스트 프로그램 작성하고 실행하기

파일명: main.rs
```rs
fn main() {
  println!("Hello, world!");
}
```

컴파일 후 실행
```shell
$ rustc main.rs
$ ./main
Hello, world!
```

main 함수는 특별한 함수로, 러스트 실행 프로그램에서 항상 가장 먼저 실행되는 함수입니다.

여기서는 매개변수를 받지 않고 아무것도 반환하지 않는 main이라는 함수를 선언합니다.

함수에 매개변수가 있을 때는 () 안쪽에 이를 작성해야 합니다.

### `println!` 에서 `!` 는 뭔가요?
! 가 붙으면 매크로 호출코드이고, 매크로는 함수와 항상 같은 규칙을 따르지 않는다는 것만 알아두시면 됩니다. - 19장에서 자세히

### 컴파일과 실행은 별개의 과정입니다

Ruby, Python, JavaScript 등 명령어 한 줄로 프로그램을 컴파일하고 실행할 수 있는 동적 프로그래밍 언어에 익숙한 분들은 컴파일과 실행이 별개의 과정으로 진행되는 게 낯설 겁니다.

하지만 이 언어들은 .rb, .py, .js 파일을 다른 곳에서 실행하려면 해당 언어의 구현체를 설치해야만 합니다.

반면 러스트는 AOT(ahead-of-time-compiled) 언어로, `컴파일`과 `실행`이 별개인 대신 여러분의 프로그래밍을 컴파일하여 만든 실행 파일을 `러스트가 설치되지 않은 곳에서도 실행`할 수 있습니다.

간단한 프로그램에는 rustc를 사용하는 것도 좋습니다.

다만 프로젝트가 커질수록 관리할 옵션이 많아지고, 코드 배포도 점점 번거로워지겠죠.

다음 내용에서 소개할 카고 (Cargo) 가 바로 이러한 문제를 해결하는, 여러분이 앞으로 rustc 대신 사용할 도구입니다.


## 1.3 Cargo

### 카고를 사용해봅시다
카고 (Cargo) 는 러스타시안이라면 대부분 사용하는 러스트 빌드 시스템 및 패키지 매니저입니다.

여태까지 카고에 대해 배운 내용을 복습해 봅시다:

- `cargo new` 로 새 프로젝트를 생성할 수 있습니다. 
- `cargo build` 명령으로 프로젝트를 빌드할 수 있습니다.
- `cargo run` 명령어는 한 번에 프로젝트를 빌드하고 실행할 수 있습니다.
- `cargo check` 명령으로 바이너리를 생성하지 않고 프로젝트의 에러를 체크할 수 있습니다.
  - `cargo check`는 실행 파일을 생성하는 단계를 건너뛰기 때문에 cargo build보다 훨씬 빠릅니다.

빌드로 만들어진 파일은 작성한 소스 코드와 뒤섞이지 않도록 target/debug 디렉터리에 저장됩니다.


### 릴리즈 빌드 생성하기

프로젝트를 완성해서 배포(릴리즈)할 준비가 끝났다면, `cargo build --release` 명령어를 사용해 릴리즈 빌드를 생성할 수 있습니다.

- 일반 빌드와 차이점
  - target/debug 가 아닌 target/release 에 실행 파일이 생성됨
  - 컴파일 시 최적화를 진행하여 컴파일이 오래 걸리는 대신 러스트 코드가 더 빠르게 작동함

작성한 코드 작동 속도를 벤치마킹할 시에는 릴리즈 빌드를 기준으로 해야 한다는 것도 알아두시기 바랍니다.