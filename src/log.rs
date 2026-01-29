use std::io::Write;

use colored::Colorize;
use itertools::Itertools;
use log::Level;

pub fn init_env_logger<TNamespaces, TNamespace>(namespaces: TNamespaces)
where
  TNamespaces: IntoIterator<Item = TNamespace>,
  TNamespace: AsRef<str>,
{
  colored::control::set_override(true);

  let namespaces = namespaces
    .into_iter()
    .map(|namespace| namespace.as_ref().to_owned());

  let level = if cfg!(debug_assertions) {
    "trace"
  } else {
    "info"
  };

  let mut filter = namespaces
    .map(|namespace| format!("{}={level}", namespace.replace("-", "_")))
    .join(",");

  if filter.is_empty() {
    filter = level.to_string();
  }

  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter))
    .format(|formatter, record| {
      let timestamp = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S.%3f")
        .to_string();

      let indent = " ".repeat(timestamp.len() + 1);

      let timestamp = match record.level() {
        Level::Error => timestamp.bright_red(),
        Level::Warn => timestamp.bright_yellow(),
        Level::Info => timestamp.bright_green(),
        Level::Debug => timestamp.bright_black(),
        Level::Trace => timestamp.dimmed(),
      };

      let args = record.args().to_string();
      let args = args.replace('\n', &format!("\n{indent}"));

      writeln!(formatter, "{} {}", timestamp, args)
    })
    .write_style(env_logger::WriteStyle::Always)
    .init();
}
