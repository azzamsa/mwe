use regex::Regex;

fn main() {
    let input = r#"# Document Title

first line

```lua
print("Hello World!")
print("Other Hello!")
```

second line
"#;

    let re = Regex::new(r"(?ms)(?P<before>```lua\n)(?P<code>.*)").unwrap();
    let caps = re.captures(input);
    println!("{:#?}", caps);
}
