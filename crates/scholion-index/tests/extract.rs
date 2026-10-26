use scholion_index::{Symbol, SymbolKind, extract};

fn qnames(syms: &[Symbol]) -> Vec<&str> {
    let mut v: Vec<&str> = syms.iter().map(|s| s.qname.as_str()).collect();
    v.sort();
    v
}

fn find<'a>(syms: &'a [Symbol], q: &str) -> &'a Symbol {
    syms.iter()
        .find(|s| s.qname == q)
        .unwrap_or_else(|| panic!("no symbol {q} in {:?}", qnames(syms)))
}

const PY: &str = "\
def format_name(n):
    return n.title()

class Greeter:
    def hello(self, name):
        return format_name(name)

    @staticmethod
    def shout(x):
        return x.upper()

def main():
    print(Greeter().hello(\"x\"))
";

#[test]
fn extracts_python_class_method_and_function() {
    let syms = extract("app/g.py", PY.as_bytes()).unwrap();
    assert_eq!(
        qnames(&syms),
        vec![
            "Greeter",
            "Greeter.hello",
            "Greeter.shout",
            "format_name",
            "main"
        ]
    );
    assert_eq!(find(&syms, "Greeter").kind, SymbolKind::Type);
    assert_eq!(find(&syms, "Greeter.hello").kind, SymbolKind::Method);
    assert_eq!(find(&syms, "main").kind, SymbolKind::Function);
    let hello = find(&syms, "Greeter.hello");
    assert_eq!(hello.signature, "def hello(self, name):");
    assert_eq!((hello.start_line, hello.end_line), (5, 6));
    assert_eq!(hello.path, "app/g.py");
    assert!(hello.refs.contains(&"format_name".to_string()));
    assert!(
        !hello.refs.contains(&"hello".to_string()),
        "own name excluded"
    );
    let main = find(&syms, "main");
    assert!(main.refs.contains(&"Greeter".to_string()) && main.refs.contains(&"hello".to_string()));
}

const TS: &str = "\
export interface Shape { area(): number }
export class Circle implements Shape {
  area(): number { return helper(3); }
}
export const sq = (x: number): number => helper(x) * x;
export function helper(n: number): number { return n; }
type Id = string;
enum Color { Red }
";

#[test]
fn extracts_typescript_interface_class_arrow_function() {
    let syms = extract("src/s.ts", TS.as_bytes()).unwrap();
    assert_eq!(
        qnames(&syms),
        vec![
            "Circle",
            "Circle.area",
            "Color",
            "Id",
            "Shape",
            "helper",
            "sq"
        ]
    );
    assert_eq!(find(&syms, "Shape").kind, SymbolKind::Type);
    assert_eq!(find(&syms, "Circle.area").kind, SymbolKind::Method);
    assert_eq!(find(&syms, "sq").kind, SymbolKind::Function);
    assert!(
        find(&syms, "Circle.area")
            .refs
            .contains(&"helper".to_string())
    );
    assert!(find(&syms, "sq").refs.contains(&"helper".to_string()));
    assert!(
        extract("src/c.tsx", b"export const A = () => <div/>;\n")
            .unwrap()
            .iter()
            .any(|s| s.qname == "A")
    );
}

const RS: &str = "\
pub struct Foo { x: i32 }
impl Foo {
    pub fn bar(&self) -> i32 { baz() }
}
fn baz() -> i32 { 1 }
pub trait T { fn t(&self); }
enum E { A }
impl<X> Wrapper<X> { fn w(&self) {} }
";

#[test]
fn extracts_rust_struct_impl_methods_trait() {
    let syms = extract("src/lib.rs", RS.as_bytes()).unwrap();
    assert_eq!(
        qnames(&syms),
        vec!["E", "Foo", "Foo.bar", "T", "T.t", "Wrapper.w", "baz"]
    );
    assert_eq!(find(&syms, "Foo.bar").kind, SymbolKind::Method);
    assert_eq!(find(&syms, "Foo").kind, SymbolKind::Type);
    assert_eq!(find(&syms, "baz").kind, SymbolKind::Function);
    assert!(find(&syms, "Foo.bar").refs.contains(&"baz".to_string()));
    assert_eq!(find(&syms, "baz").signature, "fn baz() -> i32");
}

#[test]
fn unparseable_file_returns_empty_not_error() {
    let junk = ")))))) (((((( ===== class class def def ;;;; }}}} {{{{ 12 @@\n".repeat(20);
    assert!(extract("junk.py", junk.as_bytes()).unwrap().is_empty());
    assert!(extract("junk.rs", junk.as_bytes()).unwrap().is_empty());
    assert!(extract("junk.ts", junk.as_bytes()).unwrap().is_empty());
}

#[test]
fn partially_broken_file_still_yields_good_symbols() {
    let src = format!("{PY}\nx = = = (\n");
    let syms = extract("g.py", src.as_bytes()).unwrap();
    assert!(syms.iter().any(|s| s.qname == "Greeter.hello"));
}

#[test]
fn binary_and_non_utf8_return_empty() {
    assert!(
        extract("x.py", &[0xff, 0xfe, 0x00, 0x01, 0x80])
            .unwrap()
            .is_empty()
    );
    assert!(
        extract("y.py", b"def f():\n    pass\n\0\0\0")
            .unwrap()
            .is_empty(),
        "NUL byte means binary"
    );
}

#[test]
fn file_over_1mb_is_skipped() {
    let mut src = String::from("def top():\n    pass\n");
    while src.len() <= 1024 * 1024 {
        src.push_str("# padding padding padding padding padding padding padding\n");
    }
    assert!(extract("big.py", src.as_bytes()).unwrap().is_empty());
}

#[test]
fn unknown_extension_returns_empty() {
    assert!(extract("README.md", b"# hi\n").unwrap().is_empty());
    assert!(extract("Makefile", b"all:\n").unwrap().is_empty());
}
