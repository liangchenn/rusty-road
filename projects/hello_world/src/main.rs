fn greet_world() {
    let japanese_greet = "こんにちは世界";
    let taiwanese_greet = "你好世界";
    let english_greet = "hello world";
    let spanish_greet = "hola mundo";

    let greets = [japanese_greet, taiwanese_greet, english_greet, spanish_greet];
    
    // `for greet in greets` also works
    for greet in greets.iter() {
        println!("{}", greet);
    }
}

fn main() {
    greet_world();
}
