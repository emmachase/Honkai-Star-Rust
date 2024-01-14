const plugin = require('tailwindcss/plugin')

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{ts,tsx}"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('tailwindcss-react-aria-components'),
    plugin(function({ addVariant }) {
      addVariant('hocus', ['&:hover', '&:focus'])
    })
  ],
}
