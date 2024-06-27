package config

// connection:
//   connections:
//     - producer: ai-0
//       consumer:
//         - col-0
//         - value-0
//     - producer: value-1
//       consumer:
//         - ao-0
//         - col-1

type ConnectionItemConfig struct {
	ID       string   `mapstructure:"id"`
	Producer string   `mapstructure:"producer"`
	Consumer []string `mapstructure:"consumer"`
}

type ConnectionConfig struct {
	Connections []ConnectionItemConfig `mapstructure:"connection"`
}
