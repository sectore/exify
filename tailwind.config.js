const plugin = require('tailwindcss/plugin')

module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  theme: {
    extend: {
      keyframes: {
        gradient: {
          '0%': { 'background-position': '0 50%' },
          '50%': { 'background-position': '100% 50%' },
          '100%': { 'background-position': '0 50%' },
				},
        button: {
          '0%': { 'background-position': '0 50%' },
          '100%': { 'background-position': '100% 50%' },
				},
      },
      animation: {
        ease: '0.2s ease-in-out',
        gradient: 'gradient 12s ease-in-out infinite',
        button: 'button 2s ease-in-out',
      },
    }
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
