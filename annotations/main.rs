fn main() {
    // Vars can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0;     // Regular annotation
    let an_integer   = 5i32;    // Suffix annotation

    // Or a default gets used if nothing's present
    let default_float   = 3.0; // f64
    let default_integer = 7;   // i32

    // Types can be inferred from context
    // -> Type i64 is inferred from another line
    let mut inferred_type = 12;
    inferred_type = 345983769;

    // Oh yeah, ^ `mut`-able things are mutable
    let mut mutable = 12; // <- i32
    mutable = 21;

    // mutable = true;
    // ^ This doesn't work b/c variable type
    // cannot be changed

    // But vars can be overwritten with shadowing:
    let mutable = true;
}

