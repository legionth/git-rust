
pub struct Repository {
    name: String,
    history: Vec<Commit>
}

pub struct Commit {
    id: String,
    message: String
}

impl Repository {
    pub fn new(name: String) -> Repository
    {
        Repository {
            name,
            history: vec![]
        }
    }
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

pub fn commit(repository: &mut Repository, message: String) {
    let commit: Commit = Commit::new(message);

    repository.history.push(commit);
}

pub fn init(name: String) {
    let repository = Repository::new(name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let history = Vec::new();
        let mut repository = Repository {
            name: "new-project".to_string(),
            history
        };

        let old_length = repository.history.len();

        commit(&mut repository, "new commit".to_string());
        let new_length = repository.history.len();

        assert_eq!(old_length, 0);
        assert_eq!(new_length, 1);
    }
}