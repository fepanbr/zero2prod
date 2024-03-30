use secrecy::{ExposeSecret, Secret};


#[derive(serde::Deserialize)]
pub struct Setting {
  pub database: DatabaseSettings,
  pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
  pub host: String,
pub port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: Secret<String>,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

impl DatabaseSettings {
  pub fn connection_string(&self) -> Secret<String> {
    Secret::new(format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username, self.password.expose_secret(), self.host, self.port, self.database_name
    ))
  }

  pub fn connection_string_without_db(&self) -> Secret<String> {
    Secret::new(format!(
      "postgres://{}:{}@{}:{}",
      self.username, self.password.expose_secret(), self.host, self.port
    ))
  }
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
  let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
  let configuration_diction = base_path.join("configuration");

  let environment: Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to parse APP_ENVIRONMENT.");

  let environment_filename = format!("{}.yaml", environment.as_str());

  let settings = config::Config::builder()
  .add_source(
    config::File::from(configuration_diction.join("base.yaml"))
  )
  .add_source(
    config::File::from(configuration_diction.join(environment_filename))
  )
  .build()?;

  settings.try_deserialize::<Setting>()
}

pub enum Environment {
  Local,
  Production,
  Staging,
  Development,
}

impl Environment {
  pub fn as_str(&self) -> &str {
    match self {
      Environment::Local => "local",
      Environment::Production => "production",
      Environment::Staging => "staging",
      Environment::Development => "development",
    }
  }
}

impl TryFrom<String> for Environment {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.to_lowercase().as_str() {
      "local" => Ok(Environment::Local),
      "production" => Ok(Environment::Production),
      "staging" => Ok(Environment::Staging),
      "development" => Ok(Environment::Development),
      _ => Err(format!("{} is not a valid environment.", value)),
    }
  }
}