#[macro_use]
extern crate memoize;
#[macro_use]
extern crate timed;
#[macro_use]
extern crate lazy_static;

use serde::Deserialize;

#[tokio::main]
async fn main() {
    for _ in 0..10 {
        println!("{:?}", get_employee(1).await.data.employee_name);
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Data {
    id: i32,
    employee_name: String,
    employee_salary: i32,
    employee_age: u8,
    profile_image: String,
}
#[derive(Deserialize, Debug, Clone)]
struct Employee {
    status: String,
    data: Data,
    message: String,
}

#[memoize]
#[timed]
async fn get_employee(employee_id: i32) -> Employee {
    let url = format!(
        "http://dummy.restapiexample.com/api/v1/employee/{}",
        employee_id
    );
    println!("Calling {}", url);

    let employee: Employee = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Employee>()
        .await
        .unwrap();

    employee
}
