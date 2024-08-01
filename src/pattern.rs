pub fn rectangle_pattern(row: u8, col: u8) {
    for _ in 1..=row {
        for _ in 1..col {
            print!(" * ");
        }
        println!();
    }
}

pub fn hollow_rectangle(row: u8, col: u8) {
    for i in 1..=row {
        for j in 1..=col {
            if i == 1 || i == row || j == 1 || j == col {
                print!(" * ");
            } 
            // else if j == 1 || j == col {
            //     print!("*");
            // }
             else {
                print!("   ");
            }
        }
        println!();
    }
} 

pub fn invert_half_pyramid(num:u8){
    for i in (1..=num).rev() {
        for _ in  1..=i{
            print!(" * ");
        }
        println!();
    }
}

pub fn half_180deg_rot(num:u8){
    for i in 1..=num {
        for j in 1..=num {
            if j <= num-i {
                print!("   ");
            } else {
                print!(" * ");
            }
        }
        println!();
    }
}


pub fn butterfly (num:u8){
   
    for i in 1..=num {
        for _ in 1..=i {
            print!(" * ");
        }
        let space = 2*num - 2*i;
        for _j in 1..=space {
            print!("   ");
        }
        for _k in 1..=i {
            print!(" * ");
        }
        println!();

    }
    for i in (1..=num).rev() {
        for _ in 1..=i {
            print!(" * ");
        }
        let space = 2*num - 2*i;
        for _j in 1..=space{
            print!("   ");
        }
        for _k in 1..=i {
            print!(" * ");
        }
        println!();

    }

}