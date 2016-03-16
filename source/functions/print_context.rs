/*
Functions are categorized into different types:

- intrinsic (part of standard Dyon environment)
- external (custom Rust functions operating on the Dyon environment)
- loaded (imported and local functions)
*/

fn print_function(f) {
    print(f.name)
    print("(")
    n := len(f.arguments)
    for i := 0; i < n; i += 1 {
        print(f.arguments[i].name)
        if f.arguments[i].lifetime != none() {
            print(": '" + unwrap(f.arguments[i].lifetime))
        }
        if (i + 1) < n {
            print(", ")
        }
    }
    print(")")
    if f.returns {
        print(" ->")
    }
    print(", " + f.type)
    println("")
}

fn main() {
    fs := functions()
    n := len(fs)
    // type := "intrinsic"
    type := "external"
    // type := "loaded"
    for i := 0; i < n; i += 1 {
        if fs[i].type != type { continue }
        print(function: fs[i])
    }
    // println(fs)

    say_hello()
}
