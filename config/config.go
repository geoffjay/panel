package config

import (
	"errors"
	"path/filepath"
	"regexp"
	"strings"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

type Config struct {
	App AppConfig `mapstructure:"app"`

	Acquisition AcquisitionConfig `mapstructure:"acquisition"`
	Connection  ConnectionConfig  `mapstructure:"connection"`
	Layout      LayoutConfig      `mapstructure:"layout"`
	Logging     LoggingConfig     `mapstructure:"logging"`
}

const envPrefix = "PANEL"

func prepare(filename string) (*viper.Viper, error) {
	config := viper.New()

	var extension string

	regex := regexp.MustCompile("((y(a)?ml)|json|toml)$")
	base := filepath.Base(filename)
	if regex.Match([]byte(base)) {
		// strip the file type for viper
		parts := strings.Split(filepath.Base(filename), ".")
		base = strings.Join(parts[:len(parts)-1], ".")
		extension = parts[len(parts)-1]
	} else {
		return nil, errors.New("configuration does not support that extension type")
	}

	config.SetConfigName(base)
	config.SetConfigType(extension)
	config.SetConfigFile(filename)
	config.AddConfigPath(filepath.Dir(filename))

	return config, nil
}

// LoadConfig reads in a configuration file from a set of locations and
// deserializes it into a Config instance.
func LoadConfig(filename string, c interface{}) error {
	config, err := prepare(filename)
	if err != nil {
		return err
	}

	err = config.ReadInConfig()
	if err != nil {
		return err
	}

	config.SetEnvPrefix(envPrefix)
	config.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	config.AutomaticEnv()

	err = config.Unmarshal(&c)
	if err != nil {
		return err
	}

	return nil
}

// GetConfig returns the application configuration.
func GetConfig(filename string) (*Config, error) {
	var config *Config

	if err := LoadConfig(filename, &config); err != nil {
		return nil, err
	}

	log.Tracef("config: %+v", config)

	return config, nil
}
