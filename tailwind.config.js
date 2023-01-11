const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["index.html", "templates/*", "src/*.rs"],
  // theme: {
  //   extend: {
  //     fontFamily: {
  //       sans: ['Inter var', ...defaultTheme.fontFamily.sans],
  //     },
  //   },
  // },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
