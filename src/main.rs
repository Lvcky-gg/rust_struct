struct Car{
    mpg:i16,
    color:String,
    top_speed:i16,
}

impl Car{
    fn set_mpg(&mut self, mpg:i16){
        self.mpg = mpg;
    }
    fn set_color(&mut self, color:String){
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed:i16){
        self.top_speed = top_speed;
    }
}

fn main() {
    let mut car = Car{
        mpg: 20,
        color: "red".to_string(),
        top_speed: 200,
    };
    println!("{}", car.mpg);
    car.set_mpg(30);
    println!("{}", car.mpg);
    println!("{}", car.color);
    car.set_color( "blue".to_string());
    println!("{}", car.color);
    println!("{}", car.top_speed);
    car.set_top_speed(250);
    println!("{}", car.top_speed);
}
