use clap::Parser as ClapParser;
use std::fs;
use markdown_to_html_convertor::convert_md_to_html;

#[derive(ClapParser)]
#[command(name = "markdown-to-html_convertor", version = "1.0", about = "Converts Markdown to HTML")]
struct Cli {
    input_file: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let md_content = fs::read_to_string(&cli.input_file)
        .expect("Failed to read input file. — OR — Does not exist.");

    let html_content = convert_md_to_html(&md_content);

    match cli.output {
        Some(output_file) => {
            fs::write(&output_file, html_content)
                .expect("Failed to write to the output file.");
            println!("Successfully converted {} to {}", cli.input_file, output_file);
        }
        None => {
            println!("{}", html_content);
        }
    }
}