use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     // match guards are useful for when you want to match on an error but also want to check the value inside the error
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!(
    //             "There was a problem opening the file: {:?}",
    //             error
    //         )
    //     },
    // };

    // let f = File::open("hello.txt").unwrap();
    // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // Failed to open hello.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


// ?는 Result를 반환하는 함수에서만 사용될 수 있습니다

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // 만일 값이 Err라면, 우리가 return 키워드를 사용하여 에러 값을 호출하는 코드에게 전파하는 것과 같이 전체 함수로부터 Err 내의 값이 반환될 것입니다.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}