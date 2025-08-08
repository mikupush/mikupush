# MikuPush

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Development

Before all you should install the dependencies.

```sh
npm install
```

Then you can run the development mode.

```sh
npm run tauri dev
```

### Build

Build for production.

```sh
npm run tauri build
```

Or run the debug build.

```sh
npm run tauri build -- --debug
```

### Setting up the ORM

Install Sea ORM CLI:

```sh
cargo install sea-orm-cli
```

Create the temporary database file:

> ℹ️**NOTE**
>
> This sqlite file is not the real database, is temporary for
generate migrations and entities with `sea-orm-cli`.
>
> The application will execute the migrations on the client-created database file.

```sh
touch mikupush.tmp.db
```

#### Create a new entity

First, we need to create a migration.

```sh
npm run make:migration -- create_entity_table
```

Once the migration is created and edited with the table definition, we need to
execute the migration to the database.

```sh
npm run migrate
```

Then, now we can generate the entities from the migrated schema.

> ℹ️**NOTE**
>
> This command will generate the entity source code from the migrated
database schema.

```sh
npm run make:entity
```
