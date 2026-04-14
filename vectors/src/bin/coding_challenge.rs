/*
Let's model a file system on a computer.

Define a File struct with a `name` field set to a
String. Derive a Debug implementation.

Define a Folder struct with a `name` field set to
a String and a `contents` field set to a vector of
File structs. Derive a Debug implementation.

On the Folder struct...

Define a `new` constructor function that accepts a
`name` String. The method should create and return
a new Folder with that name. For the `contents` field,
provide a hardcoded empty vector.

Define a `create_file` method that accepts a `name`
String. The method should create a new File with that
name and add it to the end of the `contents` vector.

Define a `delete_file` method that accepts an `index`
parameter of type `usize`. The method should remove the
File at the specified index position from the `contents`
vector. It should also return the File.

Define a `get_file` method that accepts an `index`
parameter of type `usize`. The method should return
an Option containing a reference to the File at
that index position.

In the `main` function, use the `new` function to
create a Folder instance with a `name` of your choosing.

Call the `create_file` method two times. Print out
the Folder in Debug format.

Delete one of the two files using the `delete_file`
method. Print out the Folder in Debug format.

Call the `get_file` method. Use a match statement
to react to both Option variants. For the Some variant,
print out the File in Debug format. For the None variant,
print out the text "There was no file".
*/

#![allow(unused)]
#[derive(Debug)]
struct File {
    name: String
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>
}

impl Folder {
    fn new(name: String) -> Self {
        Folder { name: name, contents: Vec::<File>::new() }
    }

    fn create_file(&mut self, file_name: String) {
        self.contents.push(File{name: file_name});
    }

    fn delete_file(&mut self, idx: usize) -> Option<File> {
        let exists = self.get_file(idx);
        match exists {
            Some(_) => {
                Some(self.contents.remove(idx))
            },
            None => {
                None
            }
        }
    }
    
    fn get_file(&self, idx: usize) -> Option<&File> {
        self.contents.get(idx) 
    } 

    fn print_dir_contents(&self) {
        if self.contents.is_empty() {
            return;
        }

        println!("----------------");
        
        for f in &self.contents {
            println!("{:?}", f.name);
        }

        println!("----------------");
        println!("Total Files: {}", self.contents.len());
    }


}

fn main() {
    let mut folder: Folder = Folder::new(String::from("My Folder"));
    folder.create_file(String::from("Games"));
    folder.create_file(String::from("Documents"));
    folder.create_file(String::from("Music"));
    folder.create_file(String::from("Videos"));
    folder.print_dir_contents();
    folder.delete_file(1);
    folder.print_dir_contents();

    let some_file: Option<&File> = folder.get_file(1);
    match some_file {
        Some(file) => {
            println!("The file: {}", file.name);
        }, 
        None => {
            println!("No such file found");
        }
    }

    let not_found_file: Option<&File> = folder.get_file(10);
    match not_found_file {
        Some(file) => {
            println!("The file: {}", file.name);
        }, 
        None => {
            println!("No such file found");
        }
    }

}
