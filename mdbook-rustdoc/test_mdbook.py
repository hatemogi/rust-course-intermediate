"""실제 mdBook에서 기존 include와 새 발췌 문법을 함께 검사합니다."""
import html
import json
from pathlib import Path
import re
import subprocess
import tempfile
import unittest

TOOL = Path(__file__).resolve().parent


class MdBookCompatibility(unittest.TestCase):
    def test_builtins_and_named_excerpts(self):
        with tempfile.TemporaryDirectory(prefix="mdbook excerpts ") as temporary:
            root = Path(temporary)
            (root / "src/nested").mkdir(parents=True)
            (root / "examples").mkdir()
            (root / "book.toml").write_text(
                '[book]\ntitle = "발췌 호환성"\nsrc = "src"\n'
                '[preprocessor.rustdoc-excerpt]\ncommand = '
                + json.dumps(f'sh "{TOOL / "run.sh"}"')
                + '\nbefore = ["links"]\n', encoding="utf-8"
            )
            (root / "src/SUMMARY.md").write_text(
                '# Summary\n\n- [검사](nested/check.md)\n', encoding="utf-8"
            )
            (root / "examples/legacy.rs").write_text(
                'fn main() {\n// ANCHOR: old\n    let old = "기존 방식";\n'
                '// ANCHOR_END: old\n}\n', encoding="utf-8"
            )
            (root / "examples/new.rs").write_text(
                'use std::{fmt, io};\n\nfn sample() {\n    // 원본 주석\n'
                '    let text = r#"새 방식 { 한글 }"#;\n}\n', encoding="utf-8"
            )
            chapter = root / "src/nested/check.md"
            chapter.write_text(
                '# 검사\n\n```rust\n{{#include ../../examples/legacy.rs:old}}\n```\n\n'
                '```rust\n{{#include ../../examples/legacy.rs:3:3}}\n```\n\n'
                '```rust\n{{#rustdoc_include ../../examples/legacy.rs:old}}\n```\n\n'
                '```rust\n{{#rustdoc ../../examples/new.rs item=sample}}\n```\n\n'
                '```rust\n{{#rustdoc ../../examples/new.rs body=sample}}\n```\n\n'
                '```rust\n{{#rustdoc ../../examples/new.rs item=use:std::{fmt,io}}}\n```\n',
                encoding="utf-8"
            )
            result = subprocess.run(["mdbook", "build", str(root)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            rendered = (root / "book/nested/check.html").read_text(encoding="utf-8")
            blocks = [html.unescape(re.sub(r'<[^>]*>', '', re.sub(r'<span class="boring">.*?</span>', '', block, flags=re.S))) for block in
                      re.findall(r'<pre[^>]*><code[^>]*>(.*?)</code></pre>', rendered, re.S)]
            self.assertEqual(blocks[0].strip(), 'let old = "기존 방식";')
            self.assertEqual(blocks[1].strip(), 'let old = "기존 방식";')
            self.assertIn('let old = "기존 방식";', blocks[2])
            self.assertIn('fn sample() {', blocks[3])
            self.assertEqual(blocks[4].strip(), '// 원본 주석\nlet text = r#"새 방식 { 한글 }"#;')
            self.assertEqual(blocks[5].strip(), 'use std::{fmt, io};')
            self.assertNotIn('ANCHOR', '\n'.join(blocks))
            self.assertNotIn('{{#rustdoc', rendered)

            chapter.write_text('# 검사\n{{#rustdoc ../../examples/new.rs item=missing}}\n', encoding="utf-8")
            result = subprocess.run(["mdbook", "build", str(root)], capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('missing', result.stderr)


if __name__ == '__main__':
    unittest.main()
