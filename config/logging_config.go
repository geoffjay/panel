package config

// logging:
//   columns:
//     - id: col-0
//       label: AI0
//       type: float
//       format: "%.2f"
//     - id: col-1
//       label: AO0
//       type: float
//       format: "%.2f"

type ColumnConfig struct {
	ID     string `mapstructure:"id"`
	Label  string `mapstructure:"label"`
	Type   string `mapstructure:"type"`
	Format string `mapstructure:"format"`
}

type LoggingConfig struct {
	Columns []ColumnConfig `mapstructure:"columns"`
}
