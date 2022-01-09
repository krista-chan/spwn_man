use man::prelude::*;

fn main() {
    let out = Manual::new("spwn")
        .author(Author::new("Spu7nix").email("main@spu7nix.net"))
        .flag(Flag::new().long("--console-output").short("-c").help(
            "Makes the script print the created level into the console instead of writing it to your save file",
        ))
        .flag(
            Flag::new()
                .long("--no-level")
                .short("-l")
                .help("Only compiles the script, no level creation at all")
        )
        .flag(
            Flag::new()
                .long("--no-optimize")
                .short("-o")
                .help("Removes post-optimization of triggers, making the output more readable, while also using a lot more objects and groups")
        )
        .flag(
            Flag::new()
                .long("--level-name [name]")
                .short("-n [name]")
                .help("Targets a specific level")   
        )
        .flag(
            Flag::new()
                .long("--live-editor")
                .short("-e")
                .help("Instead of writing the level to the save file, the script will use a live editor library if it's installed (Currently works only for MacOS)")   
        )
        .flag(
            Flag::new()
                .long("--save-file [file]")
                .short("-s [file]")
                .help("Chooses a specific save file to write to")
        )
        .flag(
            Flag::new()
                .long("--include-path [folder]")
                .short("-i [folder]")
                .help("Adds a search path to look for libraries")
        )
        .flag(
            Flag::new()
                .long("--allow [builtin]")
                .help("Allow use of builtin")
        )
        .flag(
            Flag::new()
                .long("--deny [builtin]")
                .help("Deny use of a builtin")
        )
        .option(Opt::new("build [script file]").short("b [script file]").help("Runs/builds a given file"))
        .option(
            Opt::new("doc")
                .help("Generates documentation for a SPWN library, in the form of a markdown file"),
        )
        .option(
            Opt::new("version")
                .short("-v")
                .long("--version")
                .help("Gets the version of spwn"),
        )
        .render();

        println!("{}", out);
}
