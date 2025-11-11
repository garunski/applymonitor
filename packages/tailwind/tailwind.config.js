export default {
  content: [
    "../web/src/**/*.rs",
    "../ui/src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        brand: {
          50: '#E7F8F9',   // Light cyan from logo
          100: '#D0F1F3',
          200: '#A1E3E7',
          300: '#72D5DB',
          400: '#43C7CF',
          500: '#2FAEB0', // Main brand teal from logo
          600: '#268B8D',
          700: '#1C686A',
          800: '#134547',
          900: '#061F38', // Dark navy from logo
          950: '#030F1C',
        },
      },
    },
  },
  plugins: [],
  darkMode: "class",
};

