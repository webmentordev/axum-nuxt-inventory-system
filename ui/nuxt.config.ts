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
      title: 'KaleemSolarPK Multan | A-Grade Solar Panels, Inverters & Accessories',
      titleTemplate: '%s',
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1, maximum-scale=1',
      htmlAttrs: {
        lang: 'en',
      },
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/kaleem-solar-fav.png' },
        { rel: 'canonical', href: 'https://kaleemsolarpk.com' },
      ],
      meta: [
        { name: 'description', content: 'KaleemSolarPK Multan offers A-Grade solar panels, inverters, batteries & accessories at the best prices. Trusted solar solutions in Multan, Pakistan.' },
        { name: 'keywords', content: 'solar panels Multan, solar inverters Pakistan, solar accessories, A-Grade solar panels, KaleemSolarPK' },
        { name: 'author', content: 'KaleemSolarPK' },
        { name: 'robots', content: 'index, follow' },
        { name: 'theme-color', content: '#ffffff' },
        { property: 'og:type', content: 'website' },
        { property: 'og:title', content: 'KaleemSolarPK Multan | A-Grade Solar Panels, Inverters & Accessories' },
        { property: 'og:description', content: 'A-Grade solar panels, inverters, batteries & accessories at the best prices in Multan, Pakistan.' },
        { property: 'og:url', content: 'https://kaleemsolarpk.com' },
        { property: 'og:site_name', content: 'KaleemSolarPK' },
        { property: 'og:image', content: 'https://kaleemsolarpk.com/og-image.jpg' },
        { property: 'og:locale', content: 'en_PK' },
        { name: 'twitter:card', content: 'summary_large_image' },
        { name: 'twitter:title', content: 'KaleemSolarPK Multan | A-Grade Solar Panels, Inverters & Accessories' },
        { name: 'twitter:description', content: 'A-Grade solar panels, inverters, batteries & accessories at the best prices in Multan, Pakistan.' },
        { name: 'twitter:image', content: 'https://kaleemsolarpk.com/og-image.jpg' },
      ],
    },
  }
})