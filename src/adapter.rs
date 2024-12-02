use crate::result::Result;
use tokio_postgres::NoTls;
use rocket::tokio;

pub enum DbAdapter {
    Pg(tokio_postgres::Client),
    Mock
}

impl DbAdapter {
    pub async fn tokio_postgres(login: &str) -> Result<DbAdapter> {
        let (client, connection) = tokio_postgres::connect(login, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("{e}");
            }
        });

        Ok(DbAdapter::Pg(client))
    }

    pub fn mock() -> DbAdapter {
        DbAdapter::Mock
    }

    pub async fn create_note(&self, name: &str, contents: &str) -> Result<()> {
        match &self {
            DbAdapter::Pg(client) => {
                client.query(r#"
                    insert into notes(name, contents)
                    values ($1::TEXT, $2::TEXT)
                "#, &[&name, &contents]).await?;
            },
            DbAdapter::Mock => {
                return Ok(())
            }
        }

        Ok(())
    }
}
