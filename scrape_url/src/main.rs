
// use std::fs;

// fn main() {

//     let url = "https://www.rust-lang.org/";
//     let output = "rust.md";

//     println!("Fetching url: {}", url);
//     let body = reqwest::blocking::get(url).unwrap().text().unwrap();

//     println!("Converting html to markdown...");
//     let md = html2md::parse_html(&body);

//     let n = fs::write(output, md.as_bytes()).unwrap();
//     println!("Converted markdown has been saved in {}. n is {:?}", output, n);
// }

// fn pi() ->f64 {
//     3.1415
// }

// fn not_pi() {
//     3.1415;
// }

// fn main() {
//     let is_pi = pi();
//     let is_unit1 = not_pi();
//     let is_unit2 = {
//         pi();
//     };

//     println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2)
// }


// #[derive(Debug)]
// enum Gender {
//   Unspecified = 0,
//   Female = 1,
//   Male = 2,
// }

// #[derive(Debug, Copy, Clone)]
// struct UserId(u64);

// #[derive(Debug, Copy, Clone)]
// struct TopicId(u64);

// #[derive(Debug)]
// struct User {
//   id: UserId,
//   name: String,
//   gender: Gender,
// }

// #[derive(Debug)]
// struct Topic {
//   id: TopicId,
//   name: String,
//   owner: UserId,
// }

// // 定义聊天室中可能发生的事件
// #[derive(Debug)]
// enum Event {
//   Join((UserId, TopicId)),
//   Leave((UserId, TopicId)),
//   Message((UserId, TopicId, String)),
// }

// fn main() {
//     let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
//     let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

//     let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
//     let event1 = Event::Join((alice.id, topic.id));
//     let event2 = Event::Join((bob.id, topic.id));
//     let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

//     println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
// }

fn main() {
    for arg in std::env::args() {
        println!("{}", arg)
    }

    println!("{:?}", std::env::args());
    println!("{:#?}", std::env::args());

    // let mut a = 1;
    // let mut b = 2;
    // a, b = b, a
}

fn fib_for(n:u8)  {
    let (mut a, mut b) = (1, 2);

    for _i in 2..n {
        let c = a+b;
        a = b;
        b = c;

        println!("fib_for next {}", c)
    }
}

fn fib_while(n:u8) {
    let (mut a, mut b) = (1, 2);
    let mut i = 2u8;
    while i < n {
        let c = a+b;
        a = b;
        b = c;

        println!("fib_for next {}", c);
        i+=1;
    }
}

fn add(a: &mut u8, b: &mut u8) ->(u8, u8, u8) {
    let c = *a+*b;
        a = b;
        b = &c;
        (a, b, c)
}