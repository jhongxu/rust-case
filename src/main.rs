mod r#struct;

fn main() {
    println!("Hello, world!");
    let name = "JiangHongxu";
    say_hello(name.to_string());

    let result = {
        let temp = 2;
        temp - 1
    };
    println!("result is {}", result);

    let sum1 = add(3, 4);
    let sum2 = add(5, 6);
    println!("sum1 is {}, sum2 is {}", sum1, sum2);

    let score = 62;
    let exam = pass_exam(score);
    println!("exam is {}", exam);

    let temperature = 20;
    let weather = if temperature < 20 { "cool" } else { "hot" };
    println!("The weather today is {}.", weather);

    fn_loop();
    fn_loop_sum();
}

fn say_hello(name: String) {
    println!("Hello {}", name)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn pass_exam(scoce: u32) -> bool {
    if scoce < 60 {
        return false;
    }
    true
}

fn fn_loop() {
    let mut counter = 1;
    loop {
        counter += 1;
        if counter < 5 {
            continue;
        }
        println!("hello world {}", counter);
        if counter >= 5 {
            break;
        }
    }
}

fn fn_loop_sum() {
    const DEFAULT_VALUE: i32 = 0;
    let target = 20;
    let mut sum = DEFAULT_VALUE;
    let mut counter = DEFAULT_VALUE;

    let result = loop {
        sum += counter;
        if sum > target {
            break counter;
        }
        counter += 1;
    };
    println!("result is {}", result);
}
