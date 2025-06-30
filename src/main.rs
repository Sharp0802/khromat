use khroma::models::{CollectionConfiguration, CreateCollectionPayload, EmbeddingFunctionConfiguration, EmbeddingFunctionNewConfiguration, GetRequestPayload, Include, IncludeList};
use khroma::Khroma;
use serde_json::json;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let khroma = Khroma::new("http://localhost:8000", None)?;

    let mut line: String = String::new();
    loop {
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut line)?;

        let term: Vec<&str> = line
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim())
            .collect();

        match term.as_slice() {
            ["tenant", "new", tenant] => {
                let tenant = khroma.create_tenant(tenant).await?;
                println!("{}", tenant.name);
            }
            ["tenant", "get", tenant] => {
                let tenant = khroma.get_tenant(tenant).await?;
                println!("{}", tenant.name);
            }
            ["database", "new", tenant, database] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.create_database(database).await?;
                println!("{}", database.name);
            }
            ["database", "del", tenant, database] => {
                let tenant = khroma.get_tenant(tenant).await?;
                tenant.delete_database(database).await?;
                println!("{}", database);
            }
            ["database", "get", tenant, database] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.get_database(database).await?;
                println!("{}", database.name);
            }
            ["database", "ls", tenant] => {
                let tenant = khroma.get_tenant(tenant).await?;
                for x in tenant.list_databases(Some(i32::MAX), None).await? {
                    println!("{}", x.name);
                }
            }
            ["collection", "new", tenant, database, collection, ..] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.get_database(database).await?;

                let ef = match &term[5..] {
                    ["ollama", model] => Some(EmbeddingFunctionConfiguration::Known {
                        r#type: "known".to_string(),
                        config: EmbeddingFunctionNewConfiguration {
                            name: "ollama".to_string(),
                            config: json!({
                                "url": "http://localhost:8000",
                                "model_name": model,
                                "timeout": 60
                            })
                        },
                    }),
                    [] => None,
                    _ => {
                        eprintln!("unrecognized embedding function");
                        continue;
                    }
                };

                let payload = CreateCollectionPayload {
                    name: collection.to_string(),
                    metadata: None,
                    configuration: Some(CollectionConfiguration{
                        embedding_function: ef,
                        hnsw: None,
                        spann: None,
                    }),
                    get_or_create: None,
                };

                let collection = database.create_collection(&payload).await?;
                println!("{} {}", collection.id, collection.name);
            }
            ["collection", "del", tenant, database, collection] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.get_database(database).await?;
                database.delete_collection(collection).await?;
                println!("{}", collection);
            }
            ["collection", "get", tenant, database, collection_name] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.get_database(database).await?;
                let collection = database.get_collection(collection_name).await?;
                println!("{} {} {}", collection.id, collection.name, collection.count().await?);
            }
            ["collection", "ls", tenant, database] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.get_database(database).await?;
                for x in database.list_collections(Some(i32::MAX), None).await? {
                    println!("{} {} {}", x.id, x.name, x.count().await?);
                };
            }
            ["collection", "read", tenant, database, collection_name] => {
                let tenant = khroma.get_tenant(tenant).await?;
                let database = tenant.get_database(database).await?;
                let collection = database.get_collection(collection_name).await?;

                let payload = GetRequestPayload {
                    where_fields: Default::default(),
                    ids: None,
                    include: Some(vec![ Include::Documents, Include::Metadatas ]),
                    limit: Some(i32::MAX),
                    offset: None,
                };

                let resp = collection.get(&payload).await?;

                let docs = resp.documents.unwrap();
                let meta = resp.metadatas.unwrap();
                for i in 0..resp.ids.len() {
                    let mut doc = match &docs[i] {
                        Some(doc) => doc.to_string(),
                        None => "<null>".into()
                    };
                    if doc.len() > 64 {
                        doc = format!("{}...", doc[..64].to_string());
                    }

                    let doc = serde_json::to_string(&doc)?;

                    let meta = match &meta[i] {
                        Some(meta) => serde_json::to_string(&meta)?,
                        None => "<null>".into()
                    };

                    println!("{} {} {}", resp.ids[i], meta, doc)
                }
            }
            ["help"] => {
                println!(
                    r#"Available commands:

Tenant Management:
  tenant new <tenant_name>        - Creates a new tenant.
  tenant get <tenant_name>        - Retrieves an existing tenant.

Database Management:
  database new <tenant> <db_name>   - Creates a new database within a tenant.
  database del <tenant> <db_name>   - Deletes a database from a tenant.
  database get <tenant> <db_name>   - Retrieves an existing database.
  database ls <tenant>              - Lists all databases within a tenant.

Collection Management:
  collection new <t> <d> <c> [ef]  - Creates a new collection.
    t: tenant_name
    d: database_name
    c: collection_name
    ef: optional embedding function, e.g., 'ollama <model_name>'
  collection del <t> <d> <c>        - Deletes a collection.
  collection get <t> <d> <c>        - Retrieves a collection and shows its item count.
  collection ls <t> <d>             - Lists all collections in a database.
  collection read <t> <d> <c>       - Reads a collection.

General:
  help                            - Shows this help message.
  exit                            - Exits the application.
"#
                );
            }
            ["exit"] => {
                break;
            }
            _  => {
                eprintln!("unrecognized command; use `help` to get help");
                continue;
            }
        }

        line.clear();
    }

    Ok(())
}
