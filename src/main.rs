use git2::Repository;
use git2::BranchType;
//use ascii::

fn get_newst_tag_test(repo_path: &str) -> (Option<String>, String){
    let repo = match Repository::open(repo_path) {
	Ok(repo) => repo,
	Err(e) => panic!("Failed to open: {}", e),
    };


    let head = repo.head();
    let commit;
    match head {
	Ok(head) => {
            println!("kind of head: {:?}, name: {}", head.kind().unwrap(), head.name().unwrap());
	    commit =  head.peel_to_commit().unwrap();	    
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
		},
		Err(_e) => {
		    println!("commit-id: {:?}, failed to get tag: {:?}", commit.id(), commit.summary().unwrap());
		    let msg = commit.message().unwrap();
		    
		    return (Some(String::from(msg)), String::from("id"))
		}
	    };
	},
	Err(e) => {
	    panic!("Failed to get head of the repo:{}", e);
	}
    };

    let master = repo.find_branch("master", BranchType::Local);
    match master {
	Ok(_master) => {
	    
	},
	Err(e) => panic!("Failed to get master branch: {}", e)
    };
    
    let tags = match repo.tag_names(None) {
	Ok(tags) => tags,
	Err(e) => panic!("Failed to get tags: {}", e)
    };

    //*
    for tag in tags.iter() {
	//let tag: Option<&str> = None;
        match tag {
	   Some(t) => {println!("{}", t)},
	   None => (),
	};
    };
    // */

    if tags.is_empty() {
	return (None, String::from("xxx"))
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
//    let tag_names = repo.tag_names(Some("v*")).unwrap();
    let tag_names = repo.tag_names(None).unwrap();
    for name in tag_names.iter() {
	println!("{}", name.unwrap());
	{
	    let tag_ref_name = format!("refs/tags/{}", name.unwrap());
	    let tag_ref = repo.find_reference(&tag_ref_name).unwrap();
	    let tag = match tag_ref.peel_to_tag() {
		Ok(tag) => tag,
		Err(e) => {
		    println!("Failed to peel to a tag: {}, {}", tag_ref_name, e);
		    continue;
		}
	    };
	    if tag.target_id() == commit.id() {
		println!("latest tag got: {}, commit_id: {:?}", name.unwrap(), commit.id());
		return (Some(String::from(name.unwrap())),
			String::from("commit-id"))
	    }
	}
    }

    println!("head: {:?}, commit: {:?}", commit.id(), commit);
    (None, String::from("commit_id"))
}


fn main() {
    let (tag_name, id) = get_head_tag("/home/daryl/git/words-picker");
    match tag_name {
	Some(tag) => println!("tag name: {}\ncommit-id:{}", tag, id),
	None => println!("have no tag, {}", id),
    };
}
