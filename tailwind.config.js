/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,vue}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        'theme-bg-base-light': '#EDF0F5',
        'theme-bg-base-dark': '#161821',
        'theme-bg-surface-light': '#DDE2EA',
        'theme-bg-surface-dark': '#1E2028',
        'theme-bg-elevated-light': '#F5F7FA',
        'theme-bg-elevated-dark': '#272A34',
        'theme-border-light': '#C5CCD7',
        'theme-border-dark': '#383C48',
        'theme-text-primary-light': '#2C3544',
        'theme-text-primary-dark': '#EBEEF2',
        'theme-text-secondary-light': '#4A5468',
        'theme-text-secondary-dark': '#A3ABB9',
        'theme-text-muted-light': '#697285',
        'theme-text-muted-dark': '#787F91',
        brand: {
          primary: '#6366F1',
          secondary: '#818CF8',
          accent: '#4F46E5'
        }
      },
      boxShadow: {
        'surface-light': '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
        'surface-dark': '0 1px 3px 0 rgb(0 0 0 / 0.3), 0 1px 2px -1px rgb(0 0 0 / 0.3)',
        'elevated-light': '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
        'elevated-dark': '0 4px 6px -1px rgb(0 0 0 / 0.3), 0 2px 4px -2px rgb(0 0 0 / 0.3)',
        'glow-light': '0 0 15px rgba(99, 102, 241, 0.5)',
        'glow-dark': '0 0 15px rgba(99, 102, 241, 0.3)'
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))'
      }
    },
  },
  plugins: [],
}
