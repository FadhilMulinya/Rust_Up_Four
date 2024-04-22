fn concatenate_strings(x:&String, y:&String) -> String{
    let mut result = String::from(x);

    result.push_str(y);

    result
}

fn main(){
    let string1 = String::from("Hello");
    let string2 = String::from("World");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("The concatenated string is:{}", concatenated_string);
}