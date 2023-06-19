fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = gen_largest(&number_list);
    println!("The largest number is: {result}");
    
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = gen_largest(&number_list);
    println!("The largest number is: {result}");    
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
