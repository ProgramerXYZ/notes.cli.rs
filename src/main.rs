
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
    else if &x[0] == &"remove" {
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

fn add (x: &Vec<&str>) -> String {
    let mut result = String::new();
    for i in &x[1..] {
        result.push_str(&i);
    }

    result
}

fn main() {

    let input:String = input();
    let parser_out: Vec<&str> = parser(&input);
    let ref_parser_out: &Vec<&str> = &parser_out;

    let command: Result<bool, String> = command_detechtor(&ref_parser_out);

    let value:String = match command {
        Ok(1) => {
            
        }
    }

    println!("{:?}", command)


    
    // println!("{:?}", parser_out);


}

