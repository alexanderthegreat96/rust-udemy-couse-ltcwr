/// Demonstrates an Enum with three different data patterns:
/// 1. Unit (Cold)
/// 2. Struct-like with named fields (Hot)
/// 3. Tuple-like with positional data (Delicate)
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

/// A standard Struct representing the 'payload' of our network requests.
struct Result {
    status_code: i32,
    message: String,
}

/// A real-world pattern where the Enum acts as a container for data.
/// Each variant carries a 'Result' struct as its inner value.
enum Request {
    Get(Result),
    Put(Result),
    Post(Result),
    Patch(Result),
    Delete(Result),
}

/// Helper function to encapsulate the creation of a Result.
/// Takes a string slice (&str) and converts it to a heap-allocated String.
fn create_result(status_code: i32, message: &str) -> Result {
    Result {
        status_code,
        message: String::from(message),
    }
}

/// Processes different network request types.
/// Note how 'result' is extracted from the Enum variant in every arm.
fn make_request(req: Request) -> () {
    match req {
        // 'result' here is the Result struct instance stored inside the Enum
        Request::Get(result) => {
            println!(
                "Sending: GET and getting {} message {}",
                result.status_code, result.message
            );
        }
        Request::Post(result) => {
            println!(
                "Sending: POST and getting {} message {}",
                result.status_code, result.message
            );
        }
        Request::Put(result) => {
            println!(
                "Sending: PUT and getting {} message {}",
                result.status_code, result.message
            );
        }
        Request::Patch(result) => {
            println!(
                "Sending: PATCH and getting {} message {}",
                result.status_code, result.message
            );
        }
        Request::Delete(result) => {
            println!(
                "Sending: DELETE and getting {} message {}",
                result.status_code, result.message
            );
        }
    }
}

/// Processes laundry cycles using pattern matching.
fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature")
        }
        // DESTRUCTURING: Extracting the named field 'temperature' directly
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with a temperature of {temperature}");
        }
        // POSITIONING: Binding the first value of the tuple to 'fabric_type'
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the laundry with a delicate cycle for {fabric_type}");
        }
    }
}

fn main() {
    // Part 1: Laundry Logic
    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));

    // Part 2: Network Request Logic
    // We create multiple requests, each wrapping a 'Result' struct.
    let get_request = Request::Get(create_result(200, "Downloaded data"));
    let post_request = Request::Post(create_result(201, "Created data"));
    let put_request = Request::Put(create_result(200, "Put some data"));
    let patch_request = Request::Patch(create_result(201, "Patched some data"));
    let delete_request = Request::Delete(create_result(204, "Deleted data"));

    // Dispatch the requests
    make_request(get_request);
    make_request(post_request);
    make_request(put_request);
    make_request(patch_request);
    make_request(delete_request);
}
