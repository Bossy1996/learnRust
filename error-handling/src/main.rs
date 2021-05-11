/* fn main() {
    //panic!("crash and burn");
    
    let _v = vec![1, 2, 3];

    // Recorable errors with Result
    use std::fs::File;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Matching on different errors

    use std::io::ErrorKind;

    let f = File::open("hello.txt"); // tries to open the file

    let f = match f {
        Ok(file) => file, // Opening the file succeded
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") { // it creates the file
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
use std::io::ErrorKind;
use std::fs::File;

fn other_way() {
    let f = File::open("hello.text").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error);
            })
        }else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn shortcuts_for_panic_on_error() {
    let f = File::open("hello.text").unwrap();
    let file = File::open("hello.text").expect("Failed to open the file");
}

fn propagating_error() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
    
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file2() -> Result<String, io::Error> {
        use std::fs;
        use std::io;
        /* let mut f = File::open("hello.txt");
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s) */
        fs::read_to_string("hello.txt");
}
 */
use std::fs::File;
use std::error::Error;
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

   Ok(())
}

fn panic_or_not_panic() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
