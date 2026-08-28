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
    apiUrl: '',
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
      title: 'KaleemSolarPK Multan | A-Grade solar panels, inverters & accessories',
      charset: 'utf-16',
      viewport: 'width=device-width, initial-scale=1, maximum-scale=1',
      htmlAttrs: {
        lang: 'en',
      },
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/kaleem-solar-fav.png' },
      ],
    },
  },
})