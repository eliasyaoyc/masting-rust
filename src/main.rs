mod linked_list;
mod custom_iterator;

struct Character {
    name: String,
}
impl Drop for Character {
    fn drop(&mut self) {
        println!("{} went away",self.name);
    }
}

fn main() {
    let steve = Character {
        name: "Steve".to_string()
    };
    let john = Character {
        name: "john".to_string()
    };
}