## Inventory management system In Rust

Start Nuxt project in production
```
$ npm nuxt build
$ pm2 start ecosystem.config.cjs --only InventoryApp
```
```
- First time setup
cargo install sqlx-cli
cargo sqlx database create

- Manage migrations
cargo sqlx migrate add <table-name>
cargo sqlx migrate run
cargo sqlx migrate revert

- Info command
cargo sqlx migrate info
```

Policies reminder
```
├── privacy-policy.vue
├── return-policy.vue
├── shipping-policy.vue
├── terms-of-service.vue
└── warranty-policy.vue
```  
Fix:
```
Use PUT instead to patch to rest values on update!
```  