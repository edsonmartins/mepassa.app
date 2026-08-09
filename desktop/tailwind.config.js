/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Paleta ZapLivre navy + âmbar (documentos/zaplivre-paleta-cores.md)
        primary: {
          50: '#F7F9FC',
          100: '#EEF2F7',
          200: '#DDE5EE',
          300: '#C3CFDD',
          400: '#8A9BB0',
          500: '#FFAA00',  // Âmbar — ação principal
          600: '#F59E0B',
          700: '#D97706',
          800: '#92400E',
          900: '#78350F',
        },
        navy: {
          950: '#03152E',
          900: '#061C3A',
          800: '#0B2A50',
        },
        brand: {
          yellow: '#FFD400',
          amber: '#FFAA00',
          orange: '#FF7900',
        },
        cloud: {
          50: '#F7F9FC',
          100: '#EEF2F7',
        },
        ink: {
          DEFAULT: '#0F172A',
          secondary: '#64748B',
        },
      },
      backgroundImage: {
        'brand-gradient': 'linear-gradient(135deg, #FFD400 0%, #FFAA00 55%, #FF7900 100%)',
      },
    },
  },
  plugins: [],
}
