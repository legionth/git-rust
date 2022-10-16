
pub struct Repository {
    name: String,
    history: Vec<Commit>
}

pub struct Commit {
    id: String,
    message: String
}

impl Commit {
    pub fn new(message: String) -> Commit
    {
        let id = String::from("1d");

        Commit {
            id,
            message
        }
    }
}

pub fn commit(mut repository: Repository, message: String) {
    let commit: Commit = Commit::new(message);

    repository.history.push(commit);
}