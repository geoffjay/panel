package config

type AppConfig struct {
	Title    string `mapstructure:"title"`
	Subtitle string `mapstructure:"subtitle"`
}
