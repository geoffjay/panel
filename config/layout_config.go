package config

// layout:
//   variables:
//     - id: value-0
//       type: float
//     - id: value-1
//       type: float
//       default: 0.0
//   components:
//     - type: stack
//       direction: vertical
//       children:
//         - type: panel
//           title: Measurements
//           subtitle: Acquisition inputs
//           children:
//             - type: stack
//               direction: horizontal
//               children:
//                 - type: stat
//                   title: Input 1
//                   value: ${value-0}
//                   description: Analog measurement
//         - type: panel
//           title: Signals
//           subtitle: Acquisition outputs
//           children:
//             - type: range
//               title: Output 1
//               value: ${value-1}

type VariableConfig struct {
	ID      string `mapstructure:"id"`
	Type    string `mapstructure:"type"`
	Default any    `mapstructure:"default"`
}

type ComponentConfig struct {
	ID       string                 `mapstructure:"id"`
	Type     string                 `mapstructure:"type"`
	Args     map[string]interface{} `mapstructure:",remain"`
	Children []ComponentConfig      `mapstructure:"children"`
}

type LayoutConfig struct {
	Variables  []VariableConfig  `mapstructure:"variables"`
	Components []ComponentConfig `mapstructure:"components"`
}
