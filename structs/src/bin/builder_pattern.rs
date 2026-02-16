#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

fn main() {
    // the builder pattern
    // it is a design parrent
    // recommended to write or structure code
    // to solve specific problems
    // the code idea is simple: given a type
    // we chain methods on the type
    // in order to do something
    // the most used example is to mutate data on that type
    // the builder pattern returns the self keyword back
    // very simular to how other languages do it
    // they basically return the class / type instance back
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(3);

    println!("Stats: {computer:#?}");
}
