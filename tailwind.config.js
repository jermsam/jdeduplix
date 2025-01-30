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
        theme: {
          'bg-base-light': '#FFFFFF',
          'bg-base-dark': '#0F1117',
          'bg-surface-light': '#F1F5F9',
          'bg-surface-dark': '#1E1F2E',
          'bg-elevated-light': '#FFFFFF',
          'bg-elevated-dark': '#252838',
          'border-light': '#E2E8F0',
          'border-dark': '#2D334D',
          'text-primary-light': '#0F172A',
          'text-primary-dark': '#F1F5F9',
          'text-secondary-light': '#334155',
          'text-secondary-dark': '#94A3B8',
          'text-muted-light': '#64748B',
          'text-muted-dark': '#64748B'
        },
        brand: {
          primary: '#6366F1',
          secondary: '#818CF8',
          accent: '#4F46E5',
          success: '#10B981',
          warning: '#F59E0B',
          error: '#EF4444',
          info: '#3B82F6'
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
