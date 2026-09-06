module.exports = {
  apps: [
    {
      name: 'InventoryApp',
      script: './.output/server/index.mjs',
      exec_mode: 'fork',
      instances: 1,
      node_args: '--env-file=.env',
      env: {
        NODE_ENV: 'production',
        PORT: 3000,
      },
    },
  ],
}