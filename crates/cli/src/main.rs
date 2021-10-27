use parser::parse;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        input.clear();

        if parse.errors().len() > 0 {
            println!("Found errors:");
            for error in parse.errors() {
                println!("{}", error);
            }

            continue;
        }

        let syntax = parse.syntax();
        let ast = ast::Root::cast(syntax.clone());

        let errors = ast_validation::validate(&syntax);
        if errors.len() > 0 {
            println!("Found errors:");
            for error in errors {
                println!("{}", error);
            }

            continue;
        }

        let (database, hir) = ast_lowering::lower_root(ast.unwrap());

        let source = codegen_js::generate(hir, database);

        println!("Source: {}", source);
    }
}
