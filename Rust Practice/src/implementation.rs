struct Temprature{
    degree:f32,
}

impl Temprature{
fn freezing()-> Self {
    Self {degree:4.6}
}

    fn showtemp(&self)  {
        println!("current temp:{}C",self.degree);
    }
}

fn main(){
let hot = Temprature{degree:99.3};
hot.showtemp();

let cold =Temprature::freezing();
cold.showtemp();
}
//------------------------------------------------------------------------------------------------------------
//practice code for impl

enum Color{
Red,
Blue,
Green,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Red=>println!("red"),
            Color::Blue=>println!("blue"),
            Color::Green=>println!("green"),
        }

    }
}
struct Dimension {
    height:f32,
    width:f32,
    depth:f32,
}
impl Dimension{
    fn print(&self){
        println!("height: {:?}",self.height);
        println!("width: {:?}",self.width);
        println!("depth: {:?}",self.depth);

    }
}
struct Box{
    dimension:Dimension,
    wight:i32,
    color:Color,
}

impl Box {
    fn createbox(color:Color,wight:i32,dimension:Dimension)->Self{
        Self{dimension,wight,color}
    }
    fn print(&self){
        self.dimension.print();
        self.color.print();
        println!("wight is :{}",self.wight);
    }
}

fn main(){
    let dimension = Dimension{
        height:12.65,
        width:324.345,
        depth:12.34,
    };
    let shbox = Box::createbox(Color::Red,12,dimension);
    shbox.print();
}
