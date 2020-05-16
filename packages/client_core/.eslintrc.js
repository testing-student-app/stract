module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: ['plugin:vue/recommended', '@vue/airbnb', '@vue/prettier'],
  plugins: ['prettier', 'vue'],
  parserOptions: {
    parser: 'babel-eslint',
  },
  rules: {
    'import/prefer-default-export': 'off',
    'no-shadow': 'off',
    'no-param-reassign': 'off',
    'no-console': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'prettier/prettier': ['error'],
    'import/extensions': [0],
  },
};
