fn main() -> anyhow::Result<()> {
    MainWindow::new()?.run()?;

    Ok(())
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "Hello, world!";
            color: green;
        }
    }
}