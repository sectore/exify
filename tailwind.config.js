const plugin = require('tailwindcss/plugin')

module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  theme: {
    extend: {}
  },
  variants: {
    extend: {},
  },
  plugins: [plugin(function({ addUtilities }) {
    addUtilities({
      '.text-shadow-light': {
        'text-shadow': '1px 1px 0px white',
      },
      '.text-shadow-dark': {
        'text-shadow': '1px 1px 0px gray',
      },
      '.text-shadow-none': {
        'text-shadow': 'none',
      },
    })
  })],
};
