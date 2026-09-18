use std::{env, fs, io, ops::Range, path::Path};

use quote::ToTokens;
use serde_json::Value;
use syn::{Item, spanned::Spanned};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn compact(value: &impl ToTokens) -> String {
    value
        .to_token_stream()
        .to_string()
        .split_whitespace()
        .collect()
}

fn name(item: &Item) -> Option<String> {
    Some(match item {
        Item::Fn(v) => v.sig.ident.to_string(),
        Item::Struct(v) => v.ident.to_string(),
        Item::Enum(v) => v.ident.to_string(),
        Item::Trait(v) => v.ident.to_string(),
        Item::Type(v) => v.ident.to_string(),
        Item::Const(v) => v.ident.to_string(),
        Item::Static(v) => v.ident.to_string(),
        Item::Mod(v) => v.ident.to_string(),
        Item::Use(v) => format!("use:{}", compact(&v.tree)),
        Item::Impl(v) => match &v.trait_ {
            Some((_, trait_path, _)) => {
                format!("impl:{}:{}", compact(trait_path), compact(&v.self_ty))
            }
            None => format!("impl:{}", compact(&v.self_ty)),
        },
        _ => return None,
    })
}

fn find<'a>(items: &'a [Item], selector: &str) -> Result<(usize, &'a Item)> {
    let matches: Vec<_> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| name(item).as_deref() == Some(selector))
        .collect();
    match matches.as_slice() {
        [found] => Ok(*found),
        [] => Err(format!("항목을 찾을 수 없습니다: {selector}").into()),
        _ => Err(format!("같은 이름의 항목이 여러 개입니다: {selector}").into()),
    }
}

fn dedent(source: &str) -> String {
    let lines: Vec<_> = source.trim_matches(['\r', '\n']).lines().collect();
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start_matches([' ', '\t']).len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| {
            if line.trim().is_empty() {
                ""
            } else {
                &line[indent..]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn excerpt(source: &str, mode: &str, selector: &str) -> Result<String> {
    let file = syn::parse_file(source)?;
    let range: Range<usize> = match mode {
        "item" => {
            if let Some((first, last)) = selector.split_once("..") {
                let (i, first) = find(&file.items, first)?;
                let (j, last) = find(&file.items, last)?;
                if i > j {
                    return Err("항목 범위의 순서가 뒤집혀 있습니다".into());
                }
                first.span().byte_range().start..last.span().byte_range().end
            } else {
                find(&file.items, selector)?.1.span().byte_range()
            }
        }
        "body" | "statements" => match find(&file.items, selector)?.1 {
            Item::Fn(function) => {
                let start = function.block.brace_token.span.open().byte_range().end;
                let mut end = function.block.brace_token.span.close().byte_range().start;
                if mode == "statements"
                    && let Some(syn::Stmt::Expr(expression, None)) = function.block.stmts.last()
                {
                    end = expression.span().byte_range().start;
                }
                start..end
            }
            _ => return Err(format!("{mode}에는 함수 이름이 필요합니다: {selector}").into()),
        },
        _ => return Err(format!("지원하지 않는 발췌 방식입니다: {mode}").into()),
    };
    let text = source.get(range).ok_or("소스 범위가 올바르지 않습니다")?;
    Ok(if mode == "statements" {
        dedent(text.trim_end())
    } else if mode == "body" {
        dedent(text)
    } else {
        text.to_owned()
    })
}

fn expand(content: &str, directory: &Path) -> Result<String> {
    let mut output = String::new();
    let mut rest = content;
    while let Some(start) = rest.find("{{#rustdoc ") {
        output.push_str(&rest[..start]);
        rest = &rest[start + "{{#rustdoc ".len()..];
        let mut end = rest
            .find("}}")
            .ok_or("rustdoc 지시문의 닫는 괄호가 없습니다")?;
        // use:std::{fmt,io}처럼 선택자의 마지막 중괄호도 보존합니다.
        while rest.as_bytes().get(end + 2) == Some(&b'}') {
            end += 1;
        }
        let args: Vec<_> = rest[..end].split_whitespace().collect();
        if args.len() != 2 {
            return Err("사용법: {{#rustdoc 상대경로 item=이름}} 또는 body=함수이름".into());
        }
        let (mode, selector) = args[1].split_once('=').ok_or("발췌 대상을 지정하세요")?;
        let path = directory.join(args[0]);
        let source =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let code = excerpt(&source, mode, selector)
            .map_err(|error| format!("{} ({mode}={selector}): {error}", path.display()))?;
        output.push_str(&code);
        rest = &rest[end + 2..];
    }
    output.push_str(rest);
    Ok(output)
}

fn chapters(sections: &mut Value, source_root: &Path) -> Result<()> {
    for section in sections.as_array_mut().ok_or("sections 배열이 없습니다")? {
        if let Some(chapter) = section.get_mut("Chapter") {
            // 초안 장에는 소스 파일이 없습니다.
            if let Some(path) = chapter["source_path"].as_str().or(chapter["path"].as_str()) {
                let path = source_root.join(path);
                let content = chapter["content"].as_str().ok_or("장 본문이 없습니다")?;
                chapter["content"] =
                    expand(content, path.parent().ok_or("장 경로가 없습니다")?)?.into();
            }
            chapters(&mut chapter["sub_items"], source_root)?;
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("supports") {
        return Ok(());
    }
    // 소스의 발췌 가능한 이름을 확인하는 보조 명령입니다.
    if args.first().map(String::as_str) == Some("list") && args.len() == 2 {
        let source = fs::read_to_string(&args[1])?;
        let file = syn::parse_file(&source)?;
        let items: Vec<_> = file
            .items
            .iter()
            .filter_map(|item| {
                name(item).map(|name| {
                    serde_json::json!({
                        "name": name, "start": item.span().byte_range().start,
                        "end": item.span().byte_range().end
                    })
                })
            })
            .collect();
        println!("{}", serde_json::to_string(&items)?);
        return Ok(());
    }
    let input: Value = serde_json::from_reader(io::stdin())?;
    let pair = input
        .as_array()
        .filter(|v| v.len() == 2)
        .ok_or("mdBook 입력 형식이 아닙니다")?;
    let context = &pair[0];
    let root = context["root"].as_str().ok_or("교재 경로가 없습니다")?;
    let src = context["config"]["book"]["src"].as_str().unwrap_or("src");
    let mut book = pair[1].clone();
    // mdBook 0.5는 items, 0.4는 sections를 사용합니다.
    let key = if book.get("items").is_some() {
        "items"
    } else {
        "sections"
    };
    chapters(&mut book[key], &Path::new(root).join(src))?;
    serde_json::to_writer(io::stdout(), &book)?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Rust 코드 발췌 실패: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statements_omit_only_the_final_expression() {
        let source = "fn quiz() -> i32 {\n    let answer = match 12 {\n        10..=12 => 12,\n        _ => -1,\n    };\n\n    answer\n}";
        assert_eq!(
            excerpt(source, "statements", "quiz").unwrap(),
            "let answer = match 12 {\n    10..=12 => 12,\n    _ => -1,\n};"
        );
        assert_eq!(
            excerpt(
                "fn f() { if true { return; } println!(\"한글\"); }",
                "statements",
                "f"
            )
            .unwrap(),
            "if true { return; } println!(\"한글\");"
        );
        assert_eq!(
            excerpt("fn f() -> i32 { 42 }", "statements", "f").unwrap(),
            ""
        );
        assert_eq!(excerpt("fn f() {}", "statements", "f").unwrap(), "");
    }

    #[test]
    fn preserves_source_and_handles_rust_syntax() {
        let source = "// 앞 주석\n#[allow(dead_code)]\nfn example<'a>(s: &'a str) -> &'a str {\n    // 주석 {\n    let _ = r#\"문자열 } {\"#;\n    /* { /* } */ } */\n    s\n}\n";
        assert_eq!(
            excerpt(source, "item", "example").unwrap(),
            source.trim_end().strip_prefix("// 앞 주석\n").unwrap()
        );
        assert_eq!(
            excerpt(source, "body", "example").unwrap(),
            "// 주석 {\nlet _ = r#\"문자열 } {\"#;\n/* { /* } */ } */\ns"
        );
        assert_eq!(
            excerpt("fn f() { println!(\"한글\"); }", "body", "f").unwrap(),
            "println!(\"한글\"); "
        );
    }

    #[test]
    fn selects_types_impls_and_ranges() {
        let source = "#[derive(Debug)]\nstruct Thing;\n// 구현 설명\nimpl Thing { fn new() -> Self { Self } }\ntrait Label {}\nimpl Label for Thing {}";
        assert_eq!(
            excerpt(source, "item", "Thing..impl:Thing").unwrap(),
            source.split("\ntrait").next().unwrap()
        );
        assert_eq!(
            excerpt(source, "item", "impl:Label:Thing").unwrap(),
            "impl Label for Thing {}"
        );
        assert!(excerpt(source, "body", "Thing").is_err());
        assert!(excerpt(source, "item", "Label..Thing").is_err());
        assert!(excerpt(source, "item", "absent").is_err());
        assert!(excerpt("fn f() {} fn f() {}", "item", "f").is_err());
        assert!(excerpt("fn broken(", "item", "broken").is_err());
    }

    #[test]
    fn leaves_builtin_includes_untouched() {
        let content = "{{#include example.rs:old}}\n{{#include example.rs:2:8}}\n{{#rustdoc_include example.rs:old}}";
        assert_eq!(expand(content, Path::new(".")).unwrap(), content);
        assert!(expand("{{#rustdoc missing.rs item=f", Path::new(".")).is_err());
        assert!(expand("{{#rustdoc missing.rs}}", Path::new(".")).is_err());
    }
}
