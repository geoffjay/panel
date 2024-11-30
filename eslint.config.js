import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginReactConfig from "eslint-plugin-react/configs/recommended.js";
import { fixupConfigRules } from "@eslint/compat";

export default [
  {
    env: { browser: true, es6: true, node: true },
    files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"],
  },

  { languageOptions: { parserOptions: { ecmaFeatures: { jsx: true } } } },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...fixupConfigRules(pluginReactConfig),

  {
    rules: {
      "comma-dangle": ["error", "always-multiline"],
      "no-undef": "error",
      "no-unused-vars": "error",
      "react/react-in-jsx-scope": "off",
      "react/jsx-uses-react": "off",
      quotes: ["error", "double"],
      semi: ["error", "always"],
    },
  },

  {
    ignores: ["**/*.d.ts", "**/*.config.cjs", "**/dist/", "**/src-tauri/"],
  },
];
