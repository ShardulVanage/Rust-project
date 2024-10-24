


struct Pilot;
impl CheckIn for Pilot{
    fn check_in(&self){
        println!("Checkin in as pilot");
    }
    fn procces(&self){
        println!("pilot enter the cockpit");
    }
}
struct Passenger; 
impl CheckIn for Passenger{
    fn check_in(&self){
        println!("Checkin in as Passenger");
    }
    fn procces(&self){
        println!("Passenger take a seat");
    }
}
struct Cargo;
impl CheckIn for Cargo{
    fn check_in(&self){
        println!("Checkin in as Cargo");
    }
    fn procces(&self){
        println!("Cargo move to storage");
    }
}

trait CheckIn {
    fn check_in(&self);
    fn procces(&self);
}

fn procces_item<T:CheckIn>(item:T){
    item.check_in();
    item.procces();
}
fn main(){
  let sv = Passenger;
  let john = Pilot;
  let cargo1 =Cargo;
  let cargo2= Cargo;

  procces_item(sv);
  procces_item(john);
  procces_item(cargo1);
}
