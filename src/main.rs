use std::io;

fn input() -> String {
    let mut text: String = String::new();

    io::stdin().read_line(&mut text).unwrap();

    text.trim().to_string()
}
//------------------------------------Parser------------------------------------------

fn parser(x: &str) -> Vec<&str> {
     x.split_whitespace().collect()
}


//-----------------------------------------Command Detechtor-------------------------------------
fn command_detechtor(x: &[&str]) -> Result<i8, String> {
    // let command = x[0];
    if x[0] == "add" {
        Ok(1)
    } else if x[0] == "remove" || x[0] == "rm" {
        Ok(2)
    } else if x[0] == "show" {
        Ok(3)
    } else {
        Err("unknown command".to_string())
    }
}
//-----------------------------------------Remove-------------------------------------
fn remove(
    what_to_remove: &String,
    the_vector_to_remove_from_the_save_var: &mut Vec<String>,
) -> String {
    let length_of_vec = the_vector_to_remove_from_the_save_var.len();

    for i in 0..length_of_vec {
        if &the_vector_to_remove_from_the_save_var[i] == what_to_remove {
            the_vector_to_remove_from_the_save_var.remove(i);
            return format!("{} as been success fully removed",what_to_remove);
        }
    }
    
    dbg!(the_vector_to_remove_from_the_save_var);
    format!("{what_to_remove} not found")
}
//-----------------------------------------Add---------------------------------------
fn add(x: &[&str]) -> String {
    let mut result = String::new();
    if x[0] == ""{panic!("CAN'T ADD NOTHING!");}

    for i in &x[1..] {
        result.push_str(i);
        result.push(' ');
    }
    result.pop();

    result
}

fn main() {
    let mut save: Vec<String> = vec![];
    loop {
        let input: String = input();
        let parser_out: Vec<&str> = parser(&input);
        

        let command: Result<i8, String> = command_detechtor(&parser_out);

//------------------------------------Match-----------------------------------------
        match command {
            Ok(1) => save.push(add(&parser_out)) ,
            Ok(2) => println!("{}",remove(&parser_out[1..].join(" "), &mut save)),
            Ok(3) => {let y: String= save[0..].join(" ");println!("{}", y);},
            _ => {}
        }
    dbg!(&save);

    }



    // dbg!(&save);

    // println!("{:?}", parser_out);
}
