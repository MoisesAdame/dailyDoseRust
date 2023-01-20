// Author: Moisés Adame-Aguilar
// Date: January 20, 2023
// Description: Introduction to Automated Tests.

#![allow(dead_code)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_loop(left: usize, right: usize) -> usize{
    let mut res: usize = left;
    for _i in 0..right{
        res += 1;
    }
    res
}

pub fn chinese_greeting(name: &str) -> String{
    format!("您好，欢迎来到，{}", name)
}

#[derive(Debug, PartialEq)]
struct Rectangle{
    length: u32,
    width: u32,
}

impl Rectangle{
    fn new(length: u32, width: u32) -> Self{
        if length == 0 || width == 0{
            panic!("Length or width cannot be 0.");
        }

        Self{
            length,
            width,
        }
    }

    fn area(&self) -> u32{
        self.length * self.width
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool{
        self.area() >= other_rectangle.area()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // #[test] Annotation that indicates that this is a test function.
    #[test]
    fn add_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_loop_test() {
        let result = add_loop(2, 2);
        assert_eq!(result, 4);
    }

    // Inputting informative error messages.
    #[test]
    fn chinese_greeting_test(){
        let result: String = chinese_greeting("莫伊");
        assert_eq!(result.contains("莫伊"), true, 
        "chinese_greeting(), doesn't include the &str name. Value: {}", result);
    }

    #[test]
    fn rectangle_new_test(){
        let result: Rectangle = Rectangle{length: 25, width: 28};
        assert_eq!(result, Rectangle::new(25, 28));
    }

    // Verifying that a panic!() occurs.
    #[test]
    #[should_panic]
    fn rectangle_new_test_panic(){
        Rectangle::new(0, 0);
    }

    #[test]
    fn rectangle_area_test(){
        let result: Rectangle = Rectangle{length: 25, width: 28};
        assert_eq!(result.area(), 700);
    }

    // Using the macro assert_ne!()
    #[test]
    fn rectangle_can_hold_test(){
        let rect1: Rectangle = Rectangle::new(25, 28);
        let rect2: Rectangle = Rectangle::new(12, 11);
        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&rect1), true);
        assert_ne!(rect2.can_hold(&rect1), true);
    }
}