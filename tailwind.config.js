/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        blackrock: {
          50: '#e7ecff',
          100: '#d3dcff',
          200: '#b0bcff',
          300: '#8190ff',
          400: '#4f52ff',
          500: '#3828ff',
          600: '#2c04ff',
          700: '#2700ff',
          800: '#2100d3',
          900: '#200ba4',
          950: '#090329',
        },
        aquamarine: {
          50: '#e9fff7',
          100: '#c9ffe9',
          200: '#98ffd8',
          300: '#65ffcd',
          400: '#14f3b2',
          500: '#00db9c',
          600: '#00b381',
          700: '#008f6b',
          800: '#007156',
          900: '#005c49',
          950: '#00342a',
        },
        charcoal: '#232323',
        mineshaft: '#333333',
        dovegray: '#737373',
      },
    },
  },
  plugins: [],
};
