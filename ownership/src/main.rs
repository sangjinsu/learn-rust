fn main() {
    /*
    소유권은 러스트의 가장 독특한 특징이며 나머지 언어에 깊은 영향을 미칩니다.
    Rust는 쓰레기 수집기 없이도 메모리 안전을 보장할 수 있으므로 소유권이 어떻게 작동하는지 이해하는 것이 중요합니다.
    이 장에서는 소유권뿐만 아니라 차입, 슬라이스, Rust가 메모리에 데이터를 배치하는 방법 등 몇 가지 관련 기능에 대해 설명합니다.

    프로그래밍 언어에서 스택과 힙은 메모리를 다르게 구성한 두 가지 영역이다.
    스택은 값들을 순서대로 저장하고 제거하는 구조로, 크기가 고정되어 있으며 빠른 액세스가 가능하다.
    힙은 불규칙하고 동적으로 데이터를 저장하는 구조로, 메모리 할당과 해제에 시간이 더 소요되며 포인터를 통해 데이터에 접근한다.

    시스템 프로그래밍 언어인 Rust에서는 값이 스택에 있는지 힙에 있는지가 언어의 동작 방식과 결정에 영향을 미친다.
    이와 관련하여 Rust의 소유권 시스템은 힙 데이터를 관리하고 중복을 최소화하며 메모리 누수를 방지하는데 도움이 된다.
    소유권의 핵심 목적은 힙 데이터를 효과적으로 관리하는 것이며,
    이를 통해 프로그래머가 스택과 힙에 대해 자세하게 걱정하지 않아도 되게끔 도와준다.

    소유권 규칙
    먼저 소유권 규칙부터 살펴보겠습니다. 예시를 통해 작업할 때는 이러한 규칙을 염두에 두십시오.

    1. 러스트의 각 값에는 소유자가 있습니다.
    2. 한 번에 한 명의 소유자만 있을 수 있습니다.
    3. 소유자가 범위를 벗어나면 가치가 떨어집니다.
    */

    // let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // 메모리 및 할당
    /*
    String type, 변경 가능하고 확장 가능한 텍스트를 지원하려면 컴파일 시 알 수 없는 힙의 메모리를 할당하여 콘텐츠를 보관해야 합니다.
    다음을 의미합니다.

    1. 런타임에 메모리 할당자에게 메모리를 요청해야 합니다.
    2. 작업이 끝나면 이 메모리를 할당자에게 반환할 수 있는 방법이 필요합니다. String.
    */

    let s1 = String::from("hello");
    // Rust는 첫 번째 변수도 무효화하기 때문에 얕은 사본이라고 하는 것이 아니라 이동이라고 합니다
    // let s2 = s1;

    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // Stack-Only Data: Copy

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Ownership and Functions
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...

    //println!("{}", s); // s is no longer valid here
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    println!("{}", x); // x is still valid here
    // but i32 is Copy, so it's okay to still
    // use x afterward


    // Return Values and Scope
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    // println!("{}", s2)
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}