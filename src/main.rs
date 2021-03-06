mod application;

/// アプリケーションのエントリーポイント
fn main() {
	// アプリケーションのインスタンスを初期化します。
	let result = application::Application::new();
	if app.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let application = result.unwrap();

	// アプリケーションを実行します。
	let result = application.run();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
