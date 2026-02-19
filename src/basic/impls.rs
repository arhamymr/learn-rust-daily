
#[allow(unused)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name)
    }

    fn show(&self) {
        println!("Show data, name {}, age {}", self.name, self.age);
    }

    fn birthday(&mut self) -> u8 {
        self.age += 1;
        self.age
    }
}

#[allow(unused)]
pub fn try_simple_impl() {
    let mut user = User {
        name: String::from("Arham"),
        age:20,
    };

    user.greet();
    user.show();
    user.birthday();

    // after birtday
    user.show();
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_user_impl() {
        let mut user = User {
            name: String::from("Arham"),
            age: 30,
        };

        assert_eq!(user.birthday(), 31);
    }
}
