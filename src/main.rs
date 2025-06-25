// use parser::grammar::ProgramParser;
use parser::visitor::ast_optimizer;
use parser::visitor::ast_printer_visitor::AstPrinterVisitor;
use parser::visitor::semantic_type_checker::SemanticTypeChecker;
use parser::visitor::LLVMGenerator;
use parser::visitor::Visitable;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn _strip_comments(source: &str) -> Result<String, String> {
    let mut result = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_multiline_comment = false;
    let mut multiline_comment_depth = 0;

    while let Some(&c) = chars.peek() {
        if in_multiline_comment {
            // Inside multi-line comment, look for end or nested start
            if c == '*' {
                chars.next();
                if let Some(&next_c) = chars.peek() {
                    if next_c == '/' {
                        chars.next();
                        multiline_comment_depth -= 1;
                        if multiline_comment_depth == 0 {
                            in_multiline_comment = false;
                        }
                        continue;
                    }
                }
            } else if c == '/' {
                chars.next();
                if let Some(&next_c) = chars.peek() {
                    if next_c == '*' {
                        // Nested multi-line comment detected - error
                        return Err("Nested multi-line comments are not allowed".to_string());
                    }
                }
            } else {
                chars.next();
            }
        } else {
            if c == '/' {
                chars.next();
                if let Some(&next_c) = chars.peek() {
                    if next_c == '/' {
                        // Single-line comment: skip until end of line
                        chars.next();
                        while let Some(&line_c) = chars.peek() {
                            chars.next();
                            if line_c == '\n' {
                                result.push('\n');
                                break;
                            }
                        }
                        continue;
                    } else if next_c == '*' {
                        // Start multi-line comment
                        chars.next();
                        in_multiline_comment = true;
                        multiline_comment_depth = 1;
                        continue;
                    } else {
                        // Just a single slash
                        result.push(c);
                        continue;
                    }
                } else {
                    // Slash at end of input
                    result.push(c);
                    break;
                }
            } else {
                // Normal character, add to result
                result.push(c);
                chars.next();
            }
        }
    }

    if in_multiline_comment {
        return Err("Unterminated multi-line comment".to_string());
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <script.hulk>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("No se pudo leer el archivo de entrada");

    let preprocessed = ast_optimizer::preprocess_functions(&source);
    println!(
        "--- Código preprocesado ---\n{}\n---------------------------",
        preprocessed
    );
    match parser::parse_program(&preprocessed) {
        Ok(program) => {
            let mut checker = SemanticTypeChecker::new(preprocessed.clone());
            program.accept(&mut checker);

            if !checker.errors.is_empty() {
                for err in checker.errors {
                    eprintln!("Type error: {}", err);
                }
                std::process::exit(1);
            }

            let mut printer = AstPrinterVisitor::new();
            program.accept(&mut printer);

            let mut llvm_gen = LLVMGenerator::new(checker.symbol_table.clone());
            program.accept(&mut llvm_gen);

            // Escribir LLVM IR en archivo
            let mut file = File::create("hulk/script.ll").unwrap();
            let header = LLVMGenerator::llvm_header();
            let (before_main, after_main) = header.split_at(
                header
                    .iter()
                    .position(|l| l.contains("define i32 @main()"))
                    .unwrap(),
            );
            for line in before_main {
                writeln!(file, "{}", line).unwrap();
            }
            for line in llvm_gen.string_globals {
                writeln!(file, "{}", line).unwrap();
            }
            for line in llvm_gen.functions {
                writeln!(file, "{}", line).unwrap();
            }
            for line in after_main {
                writeln!(file, "{}", line).unwrap();
            }
            for line in llvm_gen.code {
                writeln!(file, "  {}", line).unwrap();
            }
            for line in LLVMGenerator::llvm_footer() {
                writeln!(file, "{}", line).unwrap();
            }
        }
        Err(err) => {
            if let Some(line) = err.line {
                println!("Error en la línea {}: {}", line, err.message);
            } else {
                println!("Error: {}", err.message);
            }
        }
    }
}
