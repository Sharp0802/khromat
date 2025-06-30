# Khromat

Khromat is simple management application of Chroma DB.

## Usage

### Tenant Management:

```
tenant new <tenant_name> - Creates a new tenant.
tenant get <tenant_name> - Retrieves an existing tenant.
```

### Database Management:

```
database new <tenant> <db_name> - Creates a new database within a tenant.
database del <tenant> <db_name> - Deletes a database from a tenant.
database get <tenant> <db_name> - Retrieves an existing database.
database ls <tenant>            - Lists all databases within a tenant.
```

### Collection Management:

```
collection new <t> <d> <c> [ef] - Creates a new collection.
collection del <t> <d> <c>      - Deletes a collection.
collection get <t> <d> <c>      - Retrieves a collection and shows its item count.
collection ls <t> <d>           - Lists all collections in a database.
```

- t: tenant_name
- d: database_name
- c: collection_name
- ef: optional embedding function, e.g., 'ollama <model_name>'

### General:

```
help - Shows this help message.
exit - Exits the application.
```
