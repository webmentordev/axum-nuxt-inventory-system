module.exports = {
  apps: [
    {
      name: 'InventoryApp',
      script: './.output/server/index.mjs',
      exec_mode: 'cluster',
      instances: 1,
      env: {
        NODE_ENV: 'production',
        PORT: 3201,
      },
    },
  ],
}