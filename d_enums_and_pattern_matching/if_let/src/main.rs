#[derive(Debug)]
enum House {
    Kitchen,
    LivingRoom,
    Bedrooms(u32),
    Bathrooms(u32),
    None,
}
#[derive(Debug)]
struct HouseS {
    a: House,
    b: House,
    c: House,
    d: House,
    e: House,
    f: House,
}

fn main() {
    let house_one = HouseS {
        a: House::Kitchen,
        b: House::LivingRoom,
        c: House::Bedrooms(4),
        d: House::Bathrooms(2),
        e: House::None,
        f: House::None,
    };

    if let House::Kitchen = house_one.a {
        println!("{:?} has a kitchen", house_one);
    } else {
        println!("is not a kitchen");
    }
}
