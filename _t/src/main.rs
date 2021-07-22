fn main() {
   fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
       let mut larger = list[0];
       for &item in list.iter() {
           if item > larger {
               larger = item
           }
       }
       larger
   }
   let number_list = vec![1, 2, 3, 4, 44];
   let char_list = vec!['a', 'b', 'y'];
   println!("number_list is {}",largest(&number_list));
   println!("char_list is {}",largest(&char_list));

   #[derive(Debug)]
   struct Point<T> {
       x: T,
       y: T,
   }

   struct Point2<T, U> {
       x: T,
       y: U
   }

   let integer = Point{x: 1, y:2};
   let float = Point{x:2.1, y:3.2};
   let a = Point2{x:2, y: "ss"};
   println!("int x is {}, y is {}, struct is {:#?}", integer.x, integer.y, integer);
   println!("float x is {}, y is {}, struct is {:#?}", float.x, float.y, float);
   println!("point2 x is {}, y is {}", a.x, a.y);



    impl<T> Point<T>{
        fn get_x(&self) -> &T {
            &self.x
        }
        fn get_y(&self) -> &T {
            &self.y
        }
    }

    impl<T, U> Point2<T, U> {
        fn create_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W>{
            Point2 {
                x: self.x,
                y: other.y
            }
        }
    }
    let p1 = Point2{x: 5, y: 1.1};

    let p2 = Point2{x: "hello world", y: "vidian"};
    let p3 = p1.create_point(p2);
    println!("get x is {}, y is {}", integer.get_x(), integer.get_y());
    println!("p3 get x is {}, y is {}", p3.x, p3.y);

}
 