#[cfg(test)]
mod tests {
    /* #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    } */
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{
            width: 8,
            heigth: 7,
        };
        let smaller = Rectangle {
            width: 5,
            heigth: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            heigth: 7,
        };
        let smaller = Rectangle {
            width:5,
            heigth:1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}
