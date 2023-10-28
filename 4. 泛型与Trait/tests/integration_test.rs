//use course04;

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
       if item > largest {
        largest = item;
       }
    }
    largest
} 

fn largest_char(list: Vec<char>) -> char {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


#[test]
fn test0() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(char_list);
    println!("The largest char is {}", result);
}


fn largest<T: std::cmp::PartialOrd + Copy> (list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[test]
fn test1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(char_list);
    println!("The largest char is {}", result);
}


fn largest_1<T: std::cmp::PartialOrd> (list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    &largest
}

#[test]
fn test2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_1(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_1(&char_list);
    println!("The largest char is {}", result);
}

#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T
}

#[test]
fn test3() {
    let integer = Point {x:5, y:10};
    let float = Point {x:1.0, y:4.0};

    println!("integer => {:?}", integer);
    println!("float => {:?}", float);
}

#[derive(Debug)]
struct Point1<T, U> {
    x:T,
    y:U,
}
#[test]
fn test4() {
    let a = Point1 {x:5, y:1.0};
    let b = Point1 {x:1.0, y: 'A'};

    println!("a => {:?}", a);
    println!("b => {:?}", b);
}

#[derive(Debug)]
struct Point3<T> {
    x:T,
    y:T
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[test]
fn test5() {
    let p = Point3 {x:10, y:15};
    println!("p.x() => {}", p.x());

    let p = Point3 {x:10.3, y:15.1};
    println!("{}", p.distance_from_origin());
}