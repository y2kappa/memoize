## Rust `memoize` proc macro

```rust
#[memoize]
fn add(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    for _ in 0..3 {
        println!("{:?}", add(100, 200));
    }
}
```

```
Calculated "100-200" with value 20000
20000
Loaded "100-200" from cache with value 20000
20000
Loaded "100-200" from cache with value 20000
20000
```

`async` mode as well:

```rust

#[tokio::main]
async fn main() {
    for _ in 0..10 {
        println!("{:?}", get_employee(1).await.data.employee_name);
    }
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

```

Output:

```
Calling http://dummy.restapiexample.com/api/v1/employee/1
function=get_employee duration=1.18539639s
"Tiger Nixon"
function=get_employee duration=5.084µs
"Tiger Nixon"
function=get_employee duration=2.806µs
"Tiger Nixon"
function=get_employee duration=1.658µs
"Tiger Nixon"
function=get_employee duration=1.502µs
"Tiger Nixon"
function=get_employee duration=1.559µs
"Tiger Nixon"
function=get_employee duration=1.529µs
"Tiger Nixon"
function=get_employee duration=1.485µs
"Tiger Nixon"
function=get_employee duration=1.502µs
"Tiger Nixon"
function=get_employee duration=1.496µs
"Tiger Nixon"
```

## TODO
- [ ] allow/test function with no args
- [ ] measure performance
- [ ] ensure noop when function does not have an output
- [ ] TTL cache maybe
- [ ] cache prints with debug mode
- [x] async test
- [x] dynamically rename cache to be custom to that function