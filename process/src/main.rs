use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Lock {
    lock: bool, // commit to git
    name: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    value: Vec<Insd>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Insd {
    filename: String,
    value: String,
}
fn maketext(data: &Data) {
    let mut file = File::create("all.txt").expect("notcreated");
    let mut st = String::new();
    for i in data.value.iter() {
        st.push_str("Ques ");
        st.push_str(&i.filename);
        st.push_str(" >>>>\n");
        st.push_str(&i.value.replace("\n\n", "\n"));
        st.push_str("===============================================================\n");
    }
    file.write_all(&st.as_bytes()).expect("not written");
}

fn get_lock() -> Lock {
    let lk: Lock = serde_json::from_reader(File::open("lock.json").unwrap()).unwrap();

    lk
}
fn get_lock_increment() -> Lock {
    let mut lk: Lock = serde_json::from_reader(File::open("lock.json").unwrap()).unwrap();
    lk.name += 1;
    serde_json::to_writer(File::create("lock.json").unwrap(), &lk).unwrap();

    lk
}
fn get_lock_true() -> Lock {
    let mut lk: Lock = serde_json::from_reader(File::open("lock.json").unwrap()).unwrap();
    lk.lock = true;
    serde_json::to_writer(File::create("lock.json").unwrap(), &lk).unwrap();

    lk
}
fn get_lock_false() -> Lock {
    let mut lk: Lock = serde_json::from_reader(File::open("lock.json").unwrap()).unwrap();
    lk.lock = false;
    serde_json::to_writer(File::create("lock.json").unwrap(), &lk).unwrap();

    lk
}

fn main() {
    let mut lk = get_lock_increment();
    let mut name: String = lk.name.to_string();
    name.push_str(".png");
    let file = lk.name.to_string();
    let mut maimpng = lk.name.to_string();
    maimpng.push_str("/");
    maimpng.push_str(&name);
    let mut ttext = lk.name.to_string();
    ttext.push_str("/");
    ttext.push_str(&file);
    let mkfold = Command::new("mkdir")
        .arg(&file)
        .spawn()
        .expect("asf")
        .wait()
        .unwrap();
    let maimout = Command::new("maim")
        .arg("-s")
        .arg(&maimpng)
        .spawn()
        .expect("maim no crop")
        .wait()
        .unwrap();
    let notifyout = Command::new("notify-send")
        .arg("Taken")
        .arg("Screenshot")
        .spawn()
        .expect("notify-send not")
        .wait()
        .unwrap();
    let tesseractout = Command::new("tesseract")
        .arg(&maimpng)
        .arg(&ttext)
        .spawn()
        .expect("tesseract")
        .wait()
        .unwrap();
    let fina = ttext.clone();
    ttext.push_str(".txt");
    let mut file = File::open("questionlist.json").unwrap();
    let mut foo: Data = serde_json::from_reader(file).unwrap();
    foo.value.push(Insd {
        value: get_text(&ttext),
        filename: fina,
    });
    println!("{:#?}", foo);

    maketext(&foo);
    serde_json::to_writer(File::create("questionlist.json").unwrap(), &foo).unwrap();
    let mut tlock: Lock = serde_json::from_reader(File::open("lock.json").unwrap()).unwrap();
    if tlock.lock == false {
        get_lock_true();
        let commit = Command::new("./commit.sh")
            .spawn()
            .expect("commitfaile")
            .wait()
            .expect("wait fil");
        get_lock_false();
    }
}
fn get_text(path: &str) -> String {
    let mut a: String = std::fs::read_to_string(path).unwrap().parse().unwrap();
    a = a.replace("\n\n", "\n");
    a = a.replace("\n\n", "\n");
    a = a.replace("O000", "");
    a = a.replace("O ", "");
    a = a.replace("Q", "");
    a = a.replace("Q ", "");
    a = a.replace("o ", "");
    a = a.replace("Options.", "");
    a = a.replace("options.", "");
    a = a.replace("Options", "");
    a = a.replace("options", "");
    a
}
