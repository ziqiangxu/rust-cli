extern crate clap;

use clap::ArgMatches;
use clap::{App, Arg};
use git2::BranchType;
use git2::Repository;
use std::fs::File;
use std::io::Write;
use json::object;
//use ascii::

const REPO_VERSION: &str = "repo_version";
const TYPESCRIPT: &str = "typescript";
const JSON: &str = "json";

fn get_newst_tag_test(repo_path: &str) -> (Option<String>, String) {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let head = repo.head();
    let commit;
    match head {
        Ok(head) => {
            println!(
                "kind of head: {:?}, name: {}",
                head.kind().unwrap(),
                head.name().unwrap()
            );
            commit = head.peel_to_commit().unwrap();
            println!("the commit: {:?}", commit);
            println!("tree id: {:?}", commit.tree_id());
            println!("message_raw: {}", commit.message_raw().unwrap());
            // repo.find_tag(commit.id()).unwrap();
            let tag_ref = repo.find_reference("refs/tags/rust-cli-tag").unwrap();
            println!("tag_ref: {:?}", tag_ref.name());
            let tag = tag_ref.peel_to_tag().unwrap();
            println!("target id: {:?}", tag.target_id());

            if tag_ref.is_tag() {
                println!("A tag");
            } else {
                println!("Not a tag");
            }

            let tag = head.peel_to_tag();
            match tag {
                Ok(tag) => {
                    println!("commit-id: {:?}, tag got: {:?}, ", commit.id(), tag);
                }
                Err(_e) => {
                    println!(
                        "commit-id: {:?}, failed to get tag: {:?}",
                        commit.id(),
                        commit.summary().unwrap()
                    );
                    let msg = commit.message().unwrap();

                    return (Some(String::from(msg)), String::from("id"));
                }
            };
        }
        Err(e) => {
            panic!("Failed to get head of the repo:{}", e);
        }
    };

    let master = repo.find_branch("master", BranchType::Local);
    match master {
        Ok(_master) => {}
        Err(e) => panic!("Failed to get master branch: {}", e),
    };

    let tags = match repo.tag_names(None) {
        Ok(tags) => tags,
        Err(e) => panic!("Failed to get tags: {}", e),
    };

    //*
    for tag in tags.iter() {
        //let tag: Option<&str> = None;
        match tag {
            Some(t) => {
                println!("{}", t)
            }
            None => (),
        };
    }
    // */
    if tags.is_empty() {
        return (None, String::from("xxx"));
    }
    let size = tags.len();

    // 对于Git，tag名不可能为空
    let t = tags.get(size - 1).unwrap();

    println!("tag: {}, commit: {:?}", &String::from(t), commit);
    return (Some(String::from(t)), String::from("commit id"));
}

fn get_head_tag(repo_path: &str) -> (Option<String>, String) {
    // (tag_name, commit_id)

    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();
    let tag_names = repo.tag_names(Some("v*")).unwrap();
    //    let tag_names = repo.tag_names(None).unwrap();
    for name in tag_names.iter() {
        println!("Check tag: {}", name.unwrap());
        let tag_ref_name = format!("refs/tags/{}", name.unwrap());
        let tag_ref = repo.find_reference(&tag_ref_name).unwrap();

        let commit_tag = tag_ref.peel_to_commit().unwrap();
        if commit_tag.id() == commit.id() {
            println!(
                "Version tag of the head got: {}, commit_id: {:?}",
                name.unwrap(),
                commit.id()
            );
            return (Some(String::from(name.unwrap())), commit.id().to_string());
        }

        /*
        let tag = match tag_ref.peel_to_tag() {
        Ok(tag) => tag,
        Err(e) => {
                    // TODO 为什么peel_to_tag会失败？
            println!("Failed to peel to a tag: {} \n{}", tag_ref_name, e);
            continue;
        }
        };
        if tag.target_id() == commit.id() {
        println!("latest tag got: {}, commit_id: {:?}", name.unwrap(), commit.id());
        return (Some(String::from(name.unwrap())),
            String::from("commit-id"))
        }
        */
    }

    // println!("head: {:?}, commit: {:?}", commit.id(), commit);
    (None, commit.id().to_string())
}

fn parse_arguments() -> ArgMatches {
    let repo_version = App::new(REPO_VERSION)
        .about(
            "自动生成目的语言代码\n
             如生成typescript代码: export const VERSION = '0.0.1-beta@xxx'\n
             使用示例：rust-cli repo_version /path/to/git/repo version.json json",
        )
        .arg(
            Arg::new("repo_url")
                .index(1)
                .about("代码仓库地址： 1. path/to/git/repo"),
        )
        .arg(Arg::new("out_file").index(2).about("要生成的文件"))
        .arg(
            Arg::new("language")
                .index(3)
                .about("目的编程语言，目前支持json、typescript")
                .default_value(JSON), // default typescript
        );
    let matches = App::new("rust-cli")
        .version("0.1.0")
        .author("Daryl.Xu <ziqiang_xu@qq.com>")
        .about("Some useful command for development")
        .subcommand(repo_version)
        //   .subcommand(Subcommand::with_name("test")
        //               .about("controls testing features")
        //               .version("1.3")
        //               .author("Someone E. <someone_else@other.com>")
        //               .arg_from_usage("-d, --debug 'Print debug information'"))
        .get_matches();
    matches
}

fn main() {
    let app_matches = parse_arguments();

    match app_matches.subcommand() {
        Some((name, matches)) => {
            println!("Subcommand calling: {}", name);
            // TODO 把各种if替换成match
            if name == REPO_VERSION {
                // 执行 REPO_VERSION相关动作
                let repo_url = matches.value_of("repo_url").unwrap();
                let out_file = matches.value_of("out_file").unwrap();
                let language = matches.value_of("language").unwrap();

                // Get the version
                let (version_tag, commit_id) = get_head_tag(repo_url);
                let version;
                match &version_tag {
                    Some(tag) => {
                        println!("tag name: {} \ncommit-id:{}", tag, commit_id);
                        version = format!("{}@{}", tag, commit_id);
                    },
                    None => {
                        println!("have no tag, {}", commit_id);
                        version = format!("{}", commit_id)
                    },
                };

                // Generate the code
                let code;
                match language {
                    TYPESCRIPT => {
                        code = format!("export const VERSION = '{}'\n", version);
                    },
                    JSON => {
                        code = object! {
                            tag: version_tag.unwrap_or(String::from("None")),
                            commit_id: commit_id,
                            version: version
                        }.dump();
                    },
                    _ => {
                        code = format!("version=version");
                    }
                }

                // write the code to file
                let mut file = match File::create(&out_file) {
                    Ok(file) => file,
                    Err(e) => {
                        panic!("Failed to create file: {}. \n error:{}", out_file, e);
                    }
                };
                file.write_all(code.as_bytes()).unwrap();
            } else {
                panic!("Subcommand '{}' is not support yet!", name);
            }

        }
        None => println!("Nothing to do!"),
    }
}
