fn main() -> Result<()> {
	let app_start = Instant::now();

	let cliargs = process_cmdline()?;

	asyncgit::register_tracing_logging();
	ensure_valid_path(&cliargs.repo_path)?;

	let key_config = KeyConfig::init()
		.map_err(|e| log_eprintln!("KeyConfig loading error: {e}"))
		.unwrap_or_default();
	let theme = Theme::init(&cliargs.theme);

	setup_terminal()?;
	defer! {
		shutdown_terminal();
	}

	set_panic_handler()?;

	let mut repo_path = cliargs.repo_path;
	let mut terminal = start_terminal(io::stdout(), &repo_path)?;
	let input = Input::new();

	let updater = if cliargs.notify_watcher {
		Updater::NotifyWatcher
	} else {
		Updater::Ticker
	};

	loop {
		let quit_state = run_app(
			app_start,
			repo_path.clone(),
			theme.clone(),
			key_config.clone(),
			&input,
			updater,
			&mut terminal,
		)?;

		match quit_state {
			QuitState::OpenSubmodule(p) => {
				repo_path = p;
			}
			_ => break,
		}
	}

	Ok(())
}
