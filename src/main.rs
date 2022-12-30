use std::fs::File;
use std::io::{ self, Write };
use std::{ fs };

fn main() {
    run().unwrap();
}

fn run() -> Result<(), io::Error> {
    let old_content = fs::read_to_string("pubspec.yaml").expect("could not read file");
    let mut content_split: Vec<&str> = old_content.split("\n").collect();
    let mut version = String::new();
    let mut build = String::new();

    println!("Coloca a versão");

    io::stdin().read_line(&mut version).expect("Deu errado meu parcero");

    println!("Coloca a build");

    io::stdin().read_line(&mut build).expect("Deu errado denovo");

    let mut str_item = format!(
        "version: {}+{}",
        version.replace("\n", ""),
        build.replace("\n", "")
    );

    let mut new_arr = content_split;
    new_arr[5] = &*str_item;
    content_split = new_arr;

    println!("{:?}", content_split);
    let mut file = File::create("pubspec.yaml")?;
    for data in content_split {
        file.write(format!("{}\n", data).as_bytes())?;
    }

    Ok(())
}