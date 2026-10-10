"""Exercise the documented task workflow and validation in all supported targets."""

import importlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib

from jsonschema import Draft202012Validator

WWW = Path(__file__).resolve().parents[1]
REPO = WWW.parent
DOCS = WWW / "src/content/docs"


def blocks(page, language):
    """Read examples directly from Markdown so verification catches stale snippets."""
    source = (DOCS / page).read_text()
    return re.findall(r"^```" + language + r"[^\n]*\n(.*?)^```", source, re.M | re.S)


def run(*args, cwd):
    result = subprocess.run(args, cwd=cwd, text=True, capture_output=True)
    if result.returncode:
        print(result.stdout, result.stderr, file=sys.stderr)
        result.check_returncode()
    return result.stdout


def check_examples(work):
    """Check generation, imports, round trips, and constraint reports used in the guides."""
    cli = str(REPO / "target/debug/sidex")
    tasks = work / "models"
    shutil.copytree(WWW / "public/examples/tasks", tasks)
    assert blocks("getting-started.md", "sidex")[0] == (tasks / "schemas/task.sidex").read_text()

    outcomes = "\n".join(blocks("generation/python.md", "sidex")[:2])
    (tasks / "schemas/outcomes.sidex").write_text(outcomes)

    validation = work / "validation"
    (validation / "schemas").mkdir(parents=True)
    (validation / "sidex.toml").write_text(
        '[bundle]\nname = "validation"\nversion = "0.1.0"\n'
        '[backend.rust]\nplugins = ["builder", "serde", "validate"]\n'
        '[backend.ts]\nplugins = ["validate"]\n'
    )
    (validation / "schemas/users.sidex").write_text("\n".join(blocks("guides/validation.md", "sidex")))

    metadata = work / "metadata"
    (metadata / "schemas").mkdir(parents=True)
    (metadata / "sidex.toml").write_text('[bundle]\nname = "metadata"\nversion = "2.1.0"\n')
    (metadata / "schemas/requests.sidex").write_text(blocks("guides/requests.md", "sidex")[0])
    ui_blocks = blocks("guides/extending-sidex.md", "sidex")
    (metadata / "schemas/ui.sidex").write_text(ui_blocks[1] + "\n" + ui_blocks[0])
    (metadata / "schemas/lifecycle.sidex").write_text(
        "\n".join(blocks("guides/api-lifecycle.md", "sidex"))
    )

    for bundle in (tasks, validation, metadata):
        run(cli, "check", cwd=bundle)
    run(cli, "fmt", cwd=tasks)
    run(cli, "fmt", "--check", cwd=tasks)

    py_out = work / "task_models"
    ts_out = work / "generated"
    schema_out = work / "json-schema"
    run(cli, "generate", "py", str(py_out), cwd=tasks)
    run(cli, "generate", "ts", str(ts_out), cwd=tasks)
    run(cli, "generate", "json-schema", str(schema_out), cwd=tasks)
    run(cli, "generate", "ts", str(ts_out), cwd=validation)
    run(cli, "generate", "rust", str(work / "validation_generated"), cwd=validation)
    run(cli, "generate", "ir", str(work / "ir"), cwd=metadata)

    ir = json.loads((work / "ir/ir.json").read_text())
    request = next(d for d in ir["defs"] if d["name"]["name"] == "CreateTask")
    assert request["typedAttrs"]["request"]["name"] == "tasks.CreateTask"
    profile = next(d for d in ir["defs"] if d["name"]["name"] == "Profile")
    assert profile["typedAttrs"]["ui"]["label"] == "Display Name"

    sys.path.insert(0, str(work))
    importlib.invalidate_caches()
    python_example = {}
    exec(blocks("generation/python.md", "python")[0], python_example)
    task_json = json.loads(python_example["encoded"])
    assert task_json == {"title": "Write a schema", "completed": False}
    for example in blocks("generation/python.md", "python")[1:]:
        exec(example, {})

    validator = Draft202012Validator(
        json.loads((schema_out / "tasks.task.Task.schema.json").read_text())
    )
    validator.validate(task_json)
    assert not validator.is_valid({"title": "Incomplete"})
    assert not validator.is_valid({**task_json, "description": None})
    samples = run(cli, "json", "generate", "tasks::task::Task", "--count", "5", "--seed", "42", cwd=tasks)
    assert samples == run(cli, "json", "generate", "tasks::task::Task", "--count", "5", "--seed", "42", cwd=tasks)
    for line in samples.splitlines():
        validator.validate(json.loads(line))

    runtime = work / "runtime"
    shutil.copytree(REPO / "languages/typescript/packages/types/src", runtime / "types")
    (runtime / "validate").mkdir()
    shutil.copy(REPO / "languages/typescript/packages/validate/src/index.ts", runtime / "validate/index.ts")
    (work / "task.ts").write_text(blocks("generation/typescript.md", "ts")[0])
    ts_validation = blocks("guides/validation.md", "ts")[0]
    (work / "validate.ts").write_text(ts_validation + """
if (report.errors.length !== 1 || report.errors[0].code !== "min_size") {
  throw new Error("Expected the documented username constraint to fail.");
}
if (fieldReport.errors.length !== 1 || !result.ok) {
  throw new Error("Expected a field error and a valid slug.");
}
if (User.validate({username: "😀", age: 151 as U32, nickname: ""}).errors.length !== 2) {
  throw new Error("Expected numeric and optional field constraints to fail.");
}
""")
    (work / "tsconfig.json").write_text(json.dumps({
        "compilerOptions": {
            "target": "ES2022", "module": "commonjs", "moduleResolution": "node",
            "outDir": "js", "baseUrl": ".",
            "paths": {"@sidex/types": ["runtime/types/index.ts"], "@sidex/validate": ["runtime/validate/index.ts"]},
            "skipLibCheck": True, "strictNullChecks": True,
        },
        "include": ["*.ts", "generated/*.ts", "runtime/**/*.ts"],
    }))
    run(str(WWW / "node_modules/.bin/tsc"), "-p", str(work / "tsconfig.json"), cwd=work)
    module_root = work / "js/node_modules/@sidex"
    module_root.mkdir(parents=True)
    (module_root / "types").symlink_to(work / "js/runtime/types", target_is_directory=True)
    (module_root / "validate").symlink_to(work / "js/runtime/validate", target_is_directory=True)
    run("node", str(work / "js/task.js"), cwd=work)
    run("node", str(work / "js/validate.js"), cwd=work)

    (work / "src/bin").mkdir(parents=True)
    (work / "build.rs").write_text(blocks("generation/rust.md", "rust")[0])
    (work / "src/main.rs").write_text(blocks("generation/rust.md", "rust")[1])
    (work / "src/bin/validation.rs").write_text("""
#[path = "../../validation_generated/mod.rs"]
mod validation;
use validation::users::{User, Slug, Order};
use sidex_validate::Validate;
fn main() {
    let user = User::new("".into(), 30);
    let report = user.validate();
    assert_eq!(report.errors().len(), 1);
    assert_eq!(report.errors()[0].code, "min_size");
    assert!(Slug::try_new("release-notes".into()).is_ok());
    assert!(Slug::try_new("".into()).is_err());
    assert!(Order::new("AB-1234".into()).validate().is_ok());
    let report = Order::new("invalid".into()).validate();
    assert_eq!(report.errors()[0].code, "format:order");
}
""")
    manifest = blocks("generation/rust.md", "toml")[1]
    (work / "Cargo.toml").write_text(
        '[package]\nname = "docs-task"\nversion = "0.0.0"\nedition = "2024"\n'
        + manifest
        + '\n[dependencies.sidex-validate]\npath = ' + json.dumps(str(REPO / "crates/sidex-validate"))
        + '\n[patch.crates-io]\n'
        + "\n".join(
            name + " = { path = " + json.dumps(str(REPO / entry["path"])) + " }"
            for name, entry in tomllib.loads((REPO / "Cargo.toml").read_text())["workspace"]["dependencies"].items()
            if isinstance(entry, dict) and "path" in entry
        )
    )
    rust_json = run("cargo", "run", "--quiet", "--bin", "docs-task", cwd=work)
    validator.validate(json.loads(rust_json))
    run("cargo", "run", "--quiet", "--bin", "validation", cwd=work)


if __name__ == "__main__":
    subprocess.run(["cargo", "build", "--bin", "sidex", "--locked"], cwd=REPO, check=True)
    os.environ.setdefault("CARGO_TARGET_DIR", str(REPO / "target"))
    with tempfile.TemporaryDirectory(prefix="sidex-docs-") as directory:
        check_examples(Path(directory))
    print("Documentation examples passed: Rust, TypeScript, Python, JSON Schema, samples, and typed metadata.")
