
use std::io;


fn input() -> String {
    let mut text: String = String::new();

    io::stdin()
        .read_line(&mut text)
        .unwrap();

    text.trim().to_string()
}

fn parser(x:&str) -> Vec<&str>{
    let it: Vec<&str> =x.split_whitespace().collect();
    it

      
}

fn command_detechtor(x:&Vec<&str>)-> Result<i8, String>{
    // let command = x[0];
    let result:i8;
    if &x[0] == &"add" {
        result =1;
        Ok(result)
    }
    else if &x[0] == &"remove" || &x[0] == &"rm" {
        result = 2;
       Ok(result)
    }else if &x[0] == &"show" {
        result = 3;
        Ok(result)
        
    }
    else {
            Err("unknown command".to_string())
        }
}
fn remove(what_to_remove:&String,the_vector_to_remove_from_the_save_var :&mut Vec<String>)-> String {
    let length_of_vec = the_vector_to_remove_from_the_save_var.len();
    
    for i in 0..length_of_vec{
        if &the_vector_to_remove_from_the_save_var[i] == what_to_remove {
            the_vector_to_remove_from_the_save_var.remove(i);
            return "Done!".to_string();
            
        }
    }
    dbg!(the_vector_to_remove_from_the_save_var);
    return "err in remove func (maybe no input 🤷‍♂️)".to_string();
    
}

fn add (x: &Vec<&str>) -> String {
    let mut result = String::new();
    for i in &x[1..] {
        result.push_str(&i);
        result.push_str(" ");
    }
    result.pop();

    return result;
}

fn main() {
    let mut save:Vec<String> = vec![];
    loop {
        let input: String = input();
        let parser_out: Vec<&str> = parser(&input);
        let ref_parser_out: &Vec<&str> = &parser_out;

        let command: Result<i8, String> = command_detechtor(&ref_parser_out);

        if command == Ok(1) {
            save.push(add(&parser_out));
        } else if command == Ok(2) {
            remove(&parser_out[1..].join(" "), &mut save);
        }

        dbg!(&command);
        dbg!(&save);
    }

    // match command {
    //     Ok(1) => value={save.push(add()) }
    // };

    // dbg!(&save);
    


    
    // println!("{:?}", parser_out);


}

