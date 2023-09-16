use color_eyre::Result;
use ui_test::*;

fn main() -> Result<()> {
    exec("dump", Mode::Pass)?;
    exec(
        "step",
        Mode::Fail {
            require_patterns: false,
            rustfix: RustfixMode::Disabled,
        },
    )?;
    Ok(())
}

fn exec(name: &str, mode: Mode) -> Result<()> {
    let mut program = CommandBuilder::cargo();
    program.args = vec!["run".into(), "--bin".into(), name.into(), "--quiet".into()];
    program.input_file_flag = Some("--".into());
    program.out_dir_flag = None;
    let mut config = Config {
        host: None,
        target: None,
        mode,
        program,
        cfgs: CommandBuilder {
            program: "asdfasdfasdfasdf".into(),
            args: vec![],
            out_dir_flag: None,
            input_file_flag: None,
            envs: vec![],
        },
        output_conflict_handling: OutputConflictHandling::Bless,
        dependencies_crate_manifest_path: None,
        dependency_builder: CommandBuilder::cargo(),
        out_dir: std::env::current_dir()?,
        edition: None,
        skip_files: vec![],
        filter_files: vec![],
        threads: None,
        ..Config::rustc(std::env::current_dir()?.join("tests").join(name))
    };

    config.path_stdout_filter(&std::env::current_dir()?, "DIR");

    let args = Args::test()?;
    if let Format::Pretty = args.format {
        println!("Compiler: {}", config.program.display());
    }

    let name = config.root_dir.display().to_string();

    let text = match args.format {
        Format::Terse => status_emitter::Text::quiet(),
        Format::Pretty => status_emitter::Text::verbose(),
    };
    config.with_args(&args, true);

    run_tests_generic(
        vec![config],
        |p, c| p.extension().is_some_and(|ext| ext == "story") && default_any_file_filter(p, c),
        |_c, _p, _f| {},
        (text, status_emitter::Gha::<true> { name }),
    )
}
