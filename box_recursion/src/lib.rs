#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Worker>;

#[derive(Debug, PartialEq)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Box<Link>,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: Box::new(self.grade.take()),
        };
        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            self.grade = *worker.next;
            Some(worker.name)
        } else {
            None
        }
    }


    pub fn last_worker(&self) -> Option<(String, String)> {
        if self.grade == None {
           return None;
        }
        self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))

 
    }
}