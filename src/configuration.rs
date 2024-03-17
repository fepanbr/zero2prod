#[derive(serde::Deserialize)]
pub struct Setting {
  pub database: DatabaseSettings,
  pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

impl DatabaseSettings {
  pub fn connection_string(&self) -> String {
    format!(
      "postgres:://{}:{}@{}:{}/{}",
      self.username, self.password, self.host, self.port, self.database_name
    )
  }
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
  // 설정을 읽는다.
  let settings = config::Config::builder()
  .add_source(
    config::File::new("configuration.yaml", config::FileFormat::Yaml)
  )
  .build()?;

  // 읽은 설정값을 Settings 타입으로 변환한다.
  settings.try_deserialize::<Setting>()
}

