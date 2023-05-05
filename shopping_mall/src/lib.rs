pub mod mall;
pub use mall::*;
pub use mall::Mall;
pub use mall::floor::store;
pub use mall::{floor, guard};
pub use mall::floor::store::Store;
pub use mall::floor::store::employee;

// Returns the store with the highest square_meters in the mall
pub fn biggest_store(m: mall::Mall) -> Store {
    let vec_floors: Vec<floor::Floor> = Vec::from(m.floors);
    let mut vec_stores: Vec<store::Store> = Vec::new();

    // Collect all stores from all floors in the mall
    for i in vec_floors {
        for st in i.stores {
            vec_stores.push(st)
        }
    }

    let mut max_store = vec_stores[0].clone();
    // Find the store with the highest square_meters
    for st in vec_stores {
        if st.square_meters > max_store.square_meters {
            max_store = st
        }
    }
    max_store
}

// Returns the list of employees with the highest salary in the mall
pub fn highest_paid_employee(m: mall::Mall) -> Vec<employee::Employee> {
    let vec_floors: Vec<floor::Floor> = Vec::from(m.floors);
    let mut vec_stores: Vec<store::Store> = Vec::new();
    let mut vec_employees: Vec<employee::Employee> = Vec::new();

    // Collect all stores from all floors in the mall
    for i in vec_floors {
        for st in i.stores {
            vec_stores.push(st)
        }
    }

    // Collect all employees from all stores in the mall
    for st in vec_stores {
        for em in st.employees {
            vec_employees.push(em)
        }
    }

    let mut max_employee = vec_employees[0].clone();
    // Find the employee(s) with the highest salary
    for emp in vec_employees.clone() {
        if emp.salary > max_employee.salary {
            max_employee = emp
        }
    }

    let mut res: Vec<employee::Employee> = Vec::new();
    // Collect all employees with the highest salary
    for emp in vec_employees.clone() {
        if emp.salary >= max_employee.salary {
            res.push(emp)
        }
    }
    res
}

// Returns the total number of employees and guards in the mall
pub fn nbr_of_employees(m: mall::Mall) -> usize {
    let vec_floors: Vec<floor::Floor> = Vec::from(m.floors);
    let mut vec_stores: Vec<store::Store> = Vec::new();
    let mut vec_employees: Vec<employee::Employee> = Vec::new();
    let vec_guards: Vec<guard::Guard> = Vec::from(m.guards);

    // Collect all stores from all floors in the mall
    for i in vec_floors {
        for st in i.stores {
            vec_stores.push(st)
        }
    }

    // Collect all employees from all stores in the mall
    for st in vec_stores {
        for em in st.employees {
            vec_employees.push(em)
        }
    }

    // Return the total number of employees and guards
    let res = vec_employees.len() + vec_guards.len();
    res
}

// Calculates the number of guards needed based on the size of the mall, and adds them to the mall's list of guards
pub fn check_for_securities(m: &mut mall::Mall, g: Vec<guard::Guard>) {
    // Calculate the total square meters of all the stores in the mall
    let mut total_sq = 0;
    for st in m.floors.iter_mut() {
        for s in st.stores.iter_mut() {
            total_sq += s.square_meters;
        }
    }
    // Calculate the number of guards needed based on the total square meters
    let guards_needed = (total_sq as f64 / 200.0).round();
    // Add the necessary number of guards to the mall's list of guards
    for i in 0..guards_needed as usize {
        m.guards.push(g[i as usize].clone())
    }
}

pub fn cut_or_raise(m: &mut mall::Mall) {
    // Loop through all the employees in the mall and check their working hours
    for f in m.floors.iter_mut() {
        for s in f.stores.iter_mut() {
            for e in s.employees.iter_mut() {
                // If an employee works more than 10 hours, give them a 10% raise
                if e.working_hours.1 - e.working_hours.0 > 10 {
                    employee::Employee::raise(e, e.salary * 0.10)
                } else {
                // If an employee works less than or equal to 10 hours, cut their salary by 10%
                    employee::Employee::cut(e, e.salary * 0.10)
                }
            }
        }
    }
}