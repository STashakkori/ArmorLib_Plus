fn main() {
    //println!("Hello World");

	println!("{} is a pretty {} {}", "Rust", "Sweet", "Thingy");
}
/*TODO
NOTE  In Salvum, this will be called like so:
     hash sha256 <file>
-or- hash sha256 "string in here"     
-or- hash all    <file>
-or- hash all    "string in here"

NOTE In this tool, this will be called like so:
     ./hash sha256 <file>
     etc...

(Where sha256 is whatever algorithm the user wants)
NOTE Implement message &input by writing to a file, removing trailing newline, and reading it back in and passing it to rhash 
NOTE See readme.txt for example usage
NOTE Only use --bsd with the -a
XXX DESIGN XXX
Things that will be used in all cases that need to be functions:
1. Atomic writing to a file
2. One singular match statement on arg[1] -- get hash to pass to rhash 
3. When rhash executes, capture stdout and print to console

ALGORITHM
1. Sanity check num args ie > 3 bad, < 3 bad, want exactly 3
2. Match on arg[1], if not recognized, return, else set variable or push to vec
3. Look at arg[2], if quote, then remove quotes and pass string to atomic write function with argument for tmp.txt output file, then pass it to rhash.
4. Print standard out to console with something like: String::from_utf8_lossy(&output.stdout)
*/