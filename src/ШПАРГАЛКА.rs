//enum Person {
//Adult,
//    Underage,
//    Rakamafo
//}
fn main() {
    let person = Person::Rakamafo;
    let rinat = Person::Underage;
    match person {
        Person::Adult => println!("Мега харош"),
        Person::Underage => println!("Не харош"),
        _ => {
            println!("Введите ваш возраст");
            return crate::main()
        },
    }
}
Вектора
fn maidsdn() {
    let mut list:Vec<i8> = vec![1,2,3]; //Vec::new пустой вектор
    println!("{:?}", list);
    list.push(9); // добавление числа к вектору
    list.remove(0); // удаление
    println!("{:?}", list);
    println!("{}", &list[1]);// выводим какой то елемент вектора
    println!("{:?}", &list[0..3]); //выводим 1 до 5 елемента
    println!("{:?}", list.get(1)) // еще один способ вывода None Some существуют
    //hashmap
    /*let mut userdenis = HashMap::new();
    userdenis.insert("Denis".to_string(), 6);
    userdenis.insert("Vasya".to_string(), 2);
    userdenis.insert("Rinat".to_string(), 7);
     */
    /* let s = String::from("Hello World I Am Gay And Lesbien or Gay".to_lowercase());
 let mut number = HashMap::new();
     for i in s.split_whitespace() {
         let count = number.entry(i).or_insert(0);
         *count += 1;
     }
 println!("{:?}", number);
     */
    //panic!();
    /*let f = File::open("lol.c");
    let mut path = "lol.html";
    let f = match f {
        Ok(file) => file,
        //Err(e) => panic!("Error {:?}", e)
            Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
Ok(file) => file,
                Err(e) => panic!("error {:?}",e)
            },

        },
    };
     */
}
/*use std::io::stdin;
use std::fs::File;
use std::fs::rename;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::ptr::read;

// read file
// write file
fn main() {
    ///File::create("text.txt").expect("Error");
    //File::open("text.txt").expect("error");
    let textfile = "ThisIsText.txt";
    //let mut text = File::open(textfile).expect("Error with opening a file!");
    //let mut text_data = String::new();
    //text.read_to_string(&mut text_data).expect("error reading");
    //println!("{}", text_data)
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(textfile)
        .expect("вфыв");
    let mut text_data = String::new();
    f.read_to_string(&mut text_data).expect("error");
    println!("file data \n{}", text_data);
    println!("enter something");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("error data");

    f.write_all(input.as_bytes()).expect("error");
    std::fs::rename("ThisIsText.txt", "text.txt").expect("error");
    std::fs::remove_file("text.txt").expect("error")
}
 */
/*use std::fmt::Display;

fn main() {
    let string = String::from("hello world!");
    let num = 777;
    print_value(&string)




}
/*fn print_string(str: String) {
    println!("{}",str);
}
fn print_num(num: i32) {
    println!("{}", num);
}
 */
fn print_value<T: Display>(value: T) {
    println!("{}", value);
}

 */



/*fn main() {
    let nums = [1,2,3,4,5,6,1,2,6,3,5,2,];
    let chars = ['a','a','b','b','c'];
    println!("{:?}", duplicate_number(&nums))
}
fn duplicate_number(list : &[i32]) -> Vec<i32> {
    let mut duplicates:Vec<i32> = Vec::new();
    for i in 0..(list.len()) {
        for j in (i + 1)..(list.len()) {
            if list[i] == list[j] {
                if !duplicates.contains(&list[i]) {
                    duplicates.push(list[i]);
                }
            }
        }
    }
    duplicates
}
 */
/*struct Data<T, K> {
    d1: T,
    d2: K
}

impl<T,K> Data<T,K> {
    fn get_data_1(&self) -> &T {
        &self.d1
    }
    fn get_data_2(&self) -> &K {
        &self.d2
    }
}

fn main() {
    let mut data = Data{
        d1: 324.323,
        d2: "Hello"
    };
    println!("Data 1: {} \nData: {}", data.get_data_1(), data.get_data_2());

}
 */
/*enum Data<T> {
    Something(T),
    None
}

fn main() {
//let lol = Data::Something(323);

}
 */

/*fn main() {
    //let mut list:Vec<i32> = vec![1,2,3,4,6,7,8,3];
    let mut list:Vec<i32> = vec![1,2,3,4,5,6,10];
    let mut list1:Vec<i32> = vec![1,2,3,4,5,6,10];
    //list.push(6);
    //list.push(100);
    println!("{:?}", list.capacity()); // list.capacity высшое число в векторе
    println!("{:?}",list);
    //list.pop();
    //list.truncate(4);
    list.append(&mut list1); // перемещение элементов из вектора в другой вектор
    println!("{:?}",list);
    println!("{:?}", list.capacity());


    /*if list.is_empty() {
        println!("empty")
    }
    else {
        println!("{:?}", list);
    }
     */

    /*match list.first(){
        Some(el) => { println!("{el}")},
        None => println!("vector is empty")
    }
     */
    //let index = 3;
    //match list.get(index) {
    //    Some(el) => println!("{}",el),
    //    None => println!("{}",index)
    //}

    //list.push(6);
    //list.insert(0,6);
    //println!("{}", &list[3])
}
 */
//fn is_even(a: i32) -> bool {
//    a % 2 == 0
//}



fn main() {
    let mut is_even = |a| a % 2 == 1;
    println!("{}", &is_even(5));

}