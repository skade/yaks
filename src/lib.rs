extern crate glob;
use glob::glob;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process::Command;


#[derive(Debug)]
pub struct Yak {
    pub title: Option<String>,
    pub name: String,
    pub path: PathBuf,
}

impl From<PathBuf> for Yak {
    fn from(p: PathBuf) -> Yak {
        let title = title(&p);
        let name = p.file_stem().unwrap().to_string_lossy().into_owned();
        let path = p.into();

        Yak { title: title, name: name, path: path }
    }
}

pub fn list() -> Vec<Yak> {
    glob("_sources/*md").expect("Failed to read glob pattern")
    .map(Result::unwrap)
    .map(Yak::from)
    .collect()
}

pub fn edit(name: &str) {
    let yak = if let Some(y) = list().into_iter().find(|y| y.name == name) {
        y
    } else {
        let path = format!("_sources/{}.md", name).into();
        Yak { title: None, name: name.into(), path: path }
    };

    Command::new("vim")
            .arg(&yak.path)
            .status()
            .expect("failed to execute editor");

    build();

    Command::new("git")
            .arg("add")
            .arg(".")
            .status()
            .expect("failed to execute git");

    Command::new("git")
            .arg("commit")
            .status()
            .expect("failed to execute git");
}

pub fn remove(name: &str) {
    let yak = list().into_iter().find(|y| y.name == name).expect("couldn't find yak");

    Command::new("git")
            .arg("rm")
            .arg(&yak.path)
            .status()
            .expect("failed to execute git");

    Command::new("git")
            .arg("rm")
            .arg("-rf")
            .arg(&yak.name)
            .status()
            .expect("failed to execute git");
}

pub fn build() {
    Command::new("./build.sh")
            .status()
            .expect("failed to execute build process");
}

pub fn publish() {
    Command::new("git")
            .arg("push")
            .status()
            .expect("failed to execute git");
}

fn title<T: AsRef<Path>>(p: T) -> Option<String> {
    let f = File::open(p.as_ref()).unwrap();
    let reader = BufReader::new(&f);
    reader.lines().nth(0).unwrap().ok()
}
