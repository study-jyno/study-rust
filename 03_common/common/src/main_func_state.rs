fn main() {
    let y = {
        let x = 3;
        let z = 4;
        x  + z
        // 4를 평가하는 코드 블록
        // x + 1 줄의 마지막이 세미콜론으로 끝나지 않은 점을 주목
        // 표현식은 종결을 나타내는 세미콜론을 쓰지 않습니다


    };

    println!("The value of y is: {y}");
}