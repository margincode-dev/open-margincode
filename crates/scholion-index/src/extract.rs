//! tree-sitter symbol extraction for Python, TypeScript/TSX and Rust.

use std::collections::BTreeSet;

use scholion_core::{AvError, Result};
use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Node, Parser};

/// Files larger than this are not indexed.
pub const MAX_FILE_BYTES: usize = 1024 * 1024;
/// Files where more than this share of bytes sits inside syntax errors are skipped.
const MAX_ERROR_SHARE: f64 = 0.30;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Function,
    Method,
    Type,
}

impl SymbolKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Method => "method",
            Self::Type => "type",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    pub path: String,
    /// `Class.method`, `Type.fn`, or just the name for free functions/types.
    pub qname: String,
    pub kind: SymbolKind,
    pub signature: String,
    pub start_line: u32,
    pub end_line: u32,
    /// Distinct identifiers used inside the symbol (own name excluded).
    pub refs: Vec<String>,
}

#[derive(Clone, Copy)]
enum Lang {
    Python,
    TypeScript,
    Tsx,
    Rust,
}

impl Lang {
    fn for_path(path: &str) -> Option<Lang> {
        match path.rsplit_once('.')?.1 {
            "py" => Some(Lang::Python),
            "ts" => Some(Lang::TypeScript),
            "tsx" => Some(Lang::Tsx),
            "rs" => Some(Lang::Rust),
            _ => None,
        }
    }

    fn language(self) -> Language {
        match self {
            Lang::Python => tree_sitter_python::LANGUAGE.into(),
            Lang::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Lang::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
        }
    }

    fn ref_kinds(self) -> &'static [&'static str] {
        match self {
            Lang::Python => &["identifier"],
            Lang::TypeScript | Lang::Tsx => {
                &["identifier", "property_identifier", "type_identifier"]
            }
            Lang::Rust => &["identifier", "type_identifier", "field_identifier"],
        }
    }
}

/// Extracts symbols from one file. Unknown extensions, binary / non-UTF8 files, files over
/// `MAX_FILE_BYTES` and files that are mostly syntax errors yield `Ok(vec![])`, never an error.
pub fn extract(path: &str, bytes: &[u8]) -> Result<Vec<Symbol>> {
    let Some(lang) = Lang::for_path(path) else {
        return Ok(vec![]);
    };
    if bytes.len() > MAX_FILE_BYTES || bytes.contains(&0) || std::str::from_utf8(bytes).is_err() {
        return Ok(vec![]);
    }
    let mut parser = Parser::new();
    parser
        .set_language(&lang.language())
        .map_err(|e| AvError::Index(e.to_string()))?;
    let Some(tree) = parser.parse(bytes, None) else {
        return Ok(vec![]);
    };
    let root = tree.root_node();
    if error_bytes(root) as f64 > bytes.len() as f64 * MAX_ERROR_SHARE {
        return Ok(vec![]);
    }
    let mut w = Walker {
        src: bytes,
        path,
        lang,
        out: vec![],
    };
    match lang {
        Lang::Python => w.py_block(root, "", false),
        Lang::TypeScript | Lang::Tsx => w.ts_block(root, ""),
        Lang::Rust => w.rs_block(root, ""),
    }
    Ok(w.out)
}

/// Bytes covered by outermost ERROR / MISSING nodes.
fn error_bytes(node: Node) -> usize {
    if node.is_error() || node.is_missing() {
        return node.end_byte() - node.start_byte();
    }
    if !node.has_error() {
        return 0;
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).map(error_bytes).sum()
}

struct Walker<'a> {
    src: &'a [u8],
    path: &'a str,
    lang: Lang,
    out: Vec<Symbol>,
}

impl<'a> Walker<'a> {
    fn text(&self, n: Node) -> &'a str {
        std::str::from_utf8(&self.src[n.start_byte()..n.end_byte()]).unwrap_or("")
    }

    fn name_of(&self, n: Node) -> Option<&'a str> {
        n.child_by_field_name("name").map(|c| self.text(c))
    }

    fn qualify(prefix: &str, name: &str) -> String {
        if prefix.is_empty() {
            name.to_owned()
        } else {
            format!("{prefix}.{name}")
        }
    }

    /// `header_end`: byte where the signature stops. `refs_end`: refs are collected before this byte.
    fn push(
        &mut self,
        node: Node,
        qname: String,
        kind: SymbolKind,
        header_end: usize,
        refs_end: usize,
    ) {
        let own = qname.rsplit('.').next().unwrap_or(&qname).to_owned();
        let header =
            std::str::from_utf8(&self.src[node.start_byte()..header_end.max(node.start_byte())])
                .unwrap_or("");
        let signature = header
            .trim()
            .trim_end_matches(';')
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let mut refs = BTreeSet::new();
        self.collect_refs(node, refs_end, &mut refs);
        refs.remove(&own);
        self.out.push(Symbol {
            path: self.path.to_owned(),
            qname,
            kind,
            signature,
            start_line: node.start_position().row as u32 + 1,
            end_line: node.end_position().row as u32 + 1,
            refs: refs.into_iter().collect(),
        });
    }

    fn collect_refs(&self, node: Node, until: usize, out: &mut BTreeSet<String>) {
        if node.start_byte() >= until {
            return;
        }
        if self.lang.ref_kinds().contains(&node.kind()) {
            out.insert(self.text(node).to_owned());
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.collect_refs(child, until, out);
        }
    }

    fn body_start(node: Node) -> usize {
        node.child_by_field_name("body")
            .map(|b| b.start_byte())
            .unwrap_or_else(|| node.end_byte())
    }

    /// A function-like node: refs cover the whole node.
    fn push_callable(&mut self, node: Node, prefix: &str, kind: SymbolKind) {
        if let Some(name) = self.name_of(node) {
            let q = Self::qualify(prefix, name);
            self.push(node, q, kind, Self::body_start(node), node.end_byte());
        }
    }

    /// A type whose members are separate symbols: refs cover only its header.
    fn push_container(&mut self, node: Node, prefix: &str) -> Option<String> {
        let name = self.name_of(node)?;
        let q = Self::qualify(prefix, name);
        let header = Self::body_start(node);
        self.push(node, q.clone(), SymbolKind::Type, header, header);
        Some(q)
    }

    /// A type with no member symbols: refs cover the whole node.
    fn push_leaf_type(&mut self, node: Node, prefix: &str) {
        if let Some(name) = self.name_of(node) {
            let q = Self::qualify(prefix, name);
            self.push(
                node,
                q,
                SymbolKind::Type,
                Self::body_start(node),
                node.end_byte(),
            );
        }
    }

    // ---- Python ----
    fn py_block(&mut self, parent: Node, prefix: &str, in_class: bool) {
        let mut cursor = parent.walk();
        let children: Vec<Node> = parent.named_children(&mut cursor).collect();
        for child in children {
            let node = if child.kind() == "decorated_definition" {
                match child.child_by_field_name("definition") {
                    Some(def) => def,
                    None => continue,
                }
            } else {
                child
            };
            self.py_one(node, prefix, in_class);
        }
    }

    fn py_one(&mut self, node: Node, prefix: &str, in_class: bool) {
        match node.kind() {
            "function_definition" => {
                let kind = if in_class {
                    SymbolKind::Method
                } else {
                    SymbolKind::Function
                };
                self.push_callable(node, prefix, kind);
            }
            "class_definition" => {
                if let Some(q) = self.push_container(node, prefix)
                    && let Some(body) = node.child_by_field_name("body")
                {
                    self.py_block(body, &q, true);
                }
            }
            _ => {}
        }
    }

    // ---- TypeScript / TSX ----
    fn ts_block(&mut self, parent: Node, prefix: &str) {
        let mut cursor = parent.walk();
        let children: Vec<Node> = parent.named_children(&mut cursor).collect();
        for child in children {
            match child.kind() {
                "export_statement" => self.ts_block(child, prefix),
                "function_declaration" | "generator_function_declaration" => {
                    self.push_callable(child, prefix, SymbolKind::Function)
                }
                "class_declaration" | "abstract_class_declaration" => {
                    if let Some(q) = self.push_container(child, prefix)
                        && let Some(body) = child.child_by_field_name("body")
                    {
                        let mut c = body.walk();
                        let members: Vec<Node> = body.named_children(&mut c).collect();
                        for m in members
                            .into_iter()
                            .filter(|m| m.kind() == "method_definition")
                        {
                            self.push_callable(m, &q, SymbolKind::Method);
                        }
                    }
                }
                "interface_declaration" | "type_alias_declaration" | "enum_declaration" => {
                    self.push_leaf_type(child, prefix)
                }
                "lexical_declaration" | "variable_declaration" => {
                    let mut c = child.walk();
                    let decls: Vec<Node> = child.named_children(&mut c).collect();
                    for d in decls
                        .into_iter()
                        .filter(|d| d.kind() == "variable_declarator")
                    {
                        let Some(value) = d.child_by_field_name("value") else {
                            continue;
                        };
                        if !matches!(
                            value.kind(),
                            "arrow_function" | "function_expression" | "function"
                        ) {
                            continue;
                        }
                        if let Some(name) = self.name_of(d) {
                            let q = Self::qualify(prefix, name);
                            self.push(
                                child,
                                q,
                                SymbolKind::Function,
                                Self::body_start(value),
                                child.end_byte(),
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // ---- Rust ----
    fn rs_block(&mut self, parent: Node, prefix: &str) {
        let mut cursor = parent.walk();
        let children: Vec<Node> = parent.named_children(&mut cursor).collect();
        for child in children {
            match child.kind() {
                "function_item" => {
                    let kind = if prefix.is_empty() {
                        SymbolKind::Function
                    } else {
                        SymbolKind::Method
                    };
                    self.push_callable(child, prefix, kind);
                }
                "function_signature_item" => self.push_callable(child, prefix, SymbolKind::Method),
                "struct_item" | "enum_item" | "union_item" | "type_item" => {
                    self.push_leaf_type(child, prefix)
                }
                "trait_item" => {
                    if let Some(q) = self.push_container(child, prefix)
                        && let Some(body) = child.child_by_field_name("body")
                    {
                        self.rs_block(body, &q);
                    }
                }
                "impl_item" => {
                    let Some(ty) = child.child_by_field_name("type") else {
                        continue;
                    };
                    let name = self.rs_type_name(ty).to_owned();
                    if let Some(body) = child.child_by_field_name("body") {
                        self.rs_block(body, &name);
                    }
                }
                "mod_item" => {
                    if let Some(body) = child.child_by_field_name("body") {
                        self.rs_block(body, prefix);
                    }
                }
                _ => {}
            }
        }
    }

    fn rs_type_name(&self, mut node: Node<'a>) -> &'a str {
        loop {
            let next = match node.kind() {
                "generic_type" | "reference_type" => node.child_by_field_name("type"),
                "scoped_type_identifier" => node.child_by_field_name("name"),
                _ => None,
            };
            match next {
                Some(n) => node = n,
                None => return self.text(node),
            }
        }
    }
}
