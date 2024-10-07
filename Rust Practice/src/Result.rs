
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn getchoice(input:&str)->Result<MenuChoice,String> {
    match input {
        "mainmenu"=>Ok(MenuChoice::MainMenu),
        "start"=>Ok(MenuChoice::Start),
        "quit"=>Ok(MenuChoice::Quit),
        _=>Err("menu choice not found".to_owned()),
    }
}

fn main() {
let choice =getchoice("mainmenu");
println!("{:?}",choice);

}
//  {}
// []
// _
//""
//-
