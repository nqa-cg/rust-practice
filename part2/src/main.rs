struct City {
    description: String,
    residents: u64,
    // 👉 TODO add a field here for is_coastal: bool
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        // panic!(
        //     "👉 TODO return a `City` described as a *non-coastal* city of approximately {} residents"
        // );
        City {
            description: format!(
                "a *non-coastal* city of approximately {} residents",
                residents
            ),
            residents,
            is_coastal,
        }
    }
}

fn main() {
    // let rustville: City = panic!("👉 TODO call new_city here, with whatever arguments you like!");
    let rustville: City = new_city(1234, true);

    // println!("This city can be described as: 👉 TODO print rustville's `description` here.");
    println!("rustville's {} here.", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
