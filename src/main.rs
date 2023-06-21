fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = gen_largest(&number_list);
    println!("The largest number is: {result}");
    
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = gen_largest(&number_list);
    println!("The largest number is: {result}");    

    let art: NewsArticle = NewsArticle{
        headline: String::from("Submarine Missing: Pressure to Rescue Mounting"),
        location: String::from("Titanic"),
        author: String::from("Adam Roesner"),
        content: String::from(". . ."),
    };


    summarize(&art);

    let tweet: Tweet = Tweet {
        username: String::from("Adam"),
        content: String::from("Check out my article about that one submarine lol"),
        reply: true,
        retweet: true
    };

    summarize(&tweet);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for num in list {
        if num > &largest {
            largest = *num;
        };
    };

    largest
}

fn gen_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for el in list {
        if el > &largest {
            largest = el;
        }
    }

    largest
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})",self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",self.username, self.content)
    }
}

fn summarize<T: Summary>(thing: &T) {
    println!("{}",thing.summarize());
}
