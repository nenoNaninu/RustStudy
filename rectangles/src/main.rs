fn main() {
    let rect1 = RectAngle{
        width:40,
        height:50
    };

    let rect2 = RectAngle{
        width:10,
        height:40
    };

    let rect3 = RectAngle{
        width:60,
        height:45
    };
    
    let sq = RectAngle::square(3);


    println!("the area of the rectangles is {:#?} square pixels",rect1.area());

    println!("can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}",rect1.can_hold(&rect3));

    println!("{:#?}",sq);
}

#[derive(Debug)]
struct RectAngle{
    width:u32,
    height:u32,
}

impl RectAngle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other:&RectAngle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32) -> RectAngle{
        RectAngle{width:size,height:size}
    }
}

