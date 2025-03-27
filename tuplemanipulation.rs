fn salary_hike(emp: (i32, &str, f64)) -> (i32, &str, f64) {
    let new_salary = if emp.2 < 50000.0 { emp.2 * 1.1 } else { emp.2 };
    (emp.0, emp.1, new_salary)
}

fn main() {
    let employee = (101, "Alice", 45000.0);
    let updated_employee = salary_hike(employee);

    println!("Updated Employee Data: {:?}", updated_employee);
}
