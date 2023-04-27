#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None
        }
    }

    pub fn push(&mut self, value: T) {
        let list = self.head.take();
    
        match list {
            Some(node) => {
                let new_node = Node { 
                    value,
                    next: Some(Box::new(node)),
                };
                self.head = Some(new_node);
            }
            None => {
                let new_node = Node { 
                    value,
                    next: None,
                };
                self.head = Some(new_node);
            } 
        }
    }

    pub fn pop(&mut self) {
        let list = self.head.take();

        match list {
            Some(node) => {
                match node.next {
                    Some(next) => self.head = Some(*next),
                    None => self.head = None,
                }
            } 
            None => return
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref().map(|boxed_node| &**boxed_node);
        }
        count
    }
}


// pub fn len(&self) ->  usize where T: Clone  {
//     match self.head {
//         None => {return 0},
//         _ => {}
//     }
//     let mut count: usize = 1;
//     let mut current = self.head.as_ref().unwrap().clone();
//     loop {
//         match current.next {
//             Some(x) => {
//                 count += 1;
//                 current = *x;
//             },
//             None => {break}
//         }
//     }
//     return count

// }