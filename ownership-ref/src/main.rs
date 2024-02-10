fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{}, {}", r1, r2);

    /*

    이러한 제한이 있는 이점은 Rust가 컴파일 시 데이터 레이스를 방지할 수 있다는 것입니다.
    데이터 레이스는 레이스 조건과 유사하며 다음 세 가지 동작이 발생할 때 발생합니다.

    1. 두 개 이상의 포인터가 동시에 동일한 데이터에 액세스합니다.
    2. 포인터 중 적어도 하나가 데이터에 쓰기 위해 사용되고 있습니다.
    3. 데이터에 대한 액세스를 동기화하는 데 사용되는 메커니즘이 없습니다.

    */

    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}