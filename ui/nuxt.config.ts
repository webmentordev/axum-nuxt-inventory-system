// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  modules: ['@nuxt/icon', '@vueuse/nuxt', '@nuxtjs/turnstile', '@nuxt/image'],
  vite: {
    plugins: [
      tailwindcss(),
    ],
  },

  routeRules: {
    '/admin/**': { ssr: false },
    '/checkout': { ssr: false },
    '/api/**': { cors: true },
  },

  icon: {
    clientBundle: {
      scan: true,
      sizeLimitKb: 256,
    },
  },

  turnstile: {
    siteKey: '1x00000000000000000000AA',
    addValidateEndpoint: true
  },
  runtimeConfig: {
    apiUrl: 'https://content.ahmerdev.online',
    public: {
      siteUrl: '',
      currency: '',
      email: '',
      phone: ''
    },
    turnstile: {
      secretKey: '1x0000000000000000000000000000000AA',
    },
  },

  app: {
    head: {
      title: 'Example Mart | Quality Groceries, Household Essentials & Everyday Products',
      titleTemplate: '%s',
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      htmlAttrs: {
        lang: 'en',
      },
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.webp' },
        { rel: 'canonical', href: 'https://examplemart.com' },
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:ital,wght@0,200..800;1,200..800&display=swap',
        },
      ],
      meta: [
        { name: 'description', content: 'Example Mart offers quality groceries, household essentials & everyday products at the best prices. Your trusted local mart in Pakistan.' },
        { name: 'keywords', content: 'online mart, grocery store, household essentials, everyday products, Example Mart' },
        { name: 'author', content: 'Example Mart' },
        { name: 'robots', content: 'index, follow' },
        { name: 'theme-color', content: '#ffffff' },
        { property: 'og:type', content: 'website' },
        { property: 'og:title', content: 'Example Mart | Quality Groceries, Household Essentials & Everyday Products' },
        { property: 'og:description', content: 'Quality groceries, household essentials & everyday products at the best prices in Pakistan.' },
        { property: 'og:url', content: 'https://examplemart.com' },
        { property: 'og:site_name', content: 'Example Mart' },
        { property: 'og:image', content: 'https://examplemart.com/mart-image.webp' },
        { property: 'og:locale', content: 'en_PK' },
        { name: 'twitter:card', content: 'summary_large_image' },
        { name: 'twitter:title', content: 'Example Mart | Quality Groceries, Household Essentials & Everyday Products' },
        { name: 'twitter:description', content: 'Quality groceries, household essentials & everyday products at the best prices in Pakistan.' },
        { name: 'twitter:image', content: 'https://examplemart.com/mart-image.webp' },
      ],
    },
  }
})