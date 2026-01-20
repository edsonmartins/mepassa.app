/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#e6f7f5',
          100: '#ccefe9',
          200: '#99dfd4',
          300: '#66cfbe',
          400: '#33bfa9',
          500: '#00af93',  // Main brand color
          600: '#008c76',
          700: '#006958',
          800: '#00463b',
          900: '#00231d',
        },
      },
    },
  },
  plugins: [],
}
