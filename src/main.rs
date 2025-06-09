use parser::grammar::ProgramParser;
use parser::visitor::ast_printer_visitor::AstPrinterVisitor;
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

    let source = fs::read_to_string(filename)
        .expect("No se pudo leer el archivo de entrada");

    match parser::parse_program(&source) {
        Ok(program) => {
            let mut printer = AstPrinterVisitor::new();
            program.accept(&mut printer);

            let mut llvm_gen = LLVMGenerator::new();
            program.accept(&mut llvm_gen);

            // Escribir LLVM IR en archivo
            let mut file = File::create("build/script.ll").unwrap();
            for line in LLVMGenerator::llvm_header() {
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
            if let Some(pos) = err.position {
                println!("Error en {}–{}: {}", pos.start, pos.end, err.message);
            } else {
                println!("Error: {}", err.message);
            }
        }
    }
}