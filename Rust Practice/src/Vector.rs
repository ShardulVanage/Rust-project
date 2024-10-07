struct Test {
    score: i32, 
}

fn main() {
    let myscoore = vec![
        Test{score: 100},
        Test{score: 200},
        Test{score: 300},   
    ];
    for test in myscoore{
        println!("{}",test.score);
    }
}

//------------------------------------------------------------------------------------------------------------

