/// This attribute #[route] would be defined by the framework as a procedural macro
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

#[route(GET, "/")]
fn index() {}

fn main() {
    println!("Hello, world!");
}

// An example of function-like macro
let sql = sql!(SELECT * from posts WHERE id=1);
// This could parse it for ensuring its syntactic correctness
// And would be defined like:
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}



