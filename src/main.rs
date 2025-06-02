use parser::grammar::ProgramParser;
use parser::visitor::AstPrinterVisitor::AstPrinterVisitor;
use parser::visitor::Visitable;

fn strip_comments(source: &str) -> Result<String, String> {
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
    let expr = ProgramParser::new()
        .parse("let x=\"hola\" in x;
    print(x);
{
    print(y);
    let z= 5 in z;
    };")
        .unwrap();
    let mut printer = AstPrinterVisitor::new();
    expr.accept(&mut printer);
}