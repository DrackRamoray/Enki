/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ["src-ui/index.html", "src-ui/src/**/*.rs"],
  theme: {
    extend: {
      spacing: {
        53: '13.25rem',
        66: '16.5rem'
      },
      colors: {
        'dark-primary': '#282c34',
        'dark-secondary': '#21252b',
        'dark-tertiary': '#2c313a',
        'dark-quaternary': '#3e4451',
        'dark-quinary': '#5c6370',
        'dark-senary': '#313440',
        'dark-septenary': '#3c4049',
        'light-primary': '#abb2bf',
        'light-secondary': '#cbd5e1',
        'light-success': '#8cbe6e',
        'light-info': '#7492e2',
        'light-highlight': '#bf8c5b'
      }
    }
  },
  plugins: [],
}
