package config

// acquisition:
//   channels:
//     - id: ai-0
//       type: input
//     - id: ao-0
//       type: output

type ChannelConfig struct {
	ID   string `mapstructure:"id"`
	Type string `mapstructure:"type"`
}

type AcquisitionConfig struct {
	Channels []ChannelConfig `mapstructure:"channels"`
}
