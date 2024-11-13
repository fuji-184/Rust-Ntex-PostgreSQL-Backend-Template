#![allow(clippy::uninit_vec)]
use std::cell::RefCell;

use ntex::util::{Bytes, BytesMut};
// use smallvec::SmallVec;
// use tokio_postgres::types::ToSql;
use tokio_postgres::{connect, Client, Statement};
use yarte::{Serialize};

use super::utils;

#[derive(Clone, Serialize, Debug)]
pub struct Tes {
    pub nama: String
}

/// Postgres interface
pub struct PgConnection {
    cl: Client,
    tes: Statement,
    buf: RefCell<BytesMut>,
}

impl PgConnection {
    pub async fn connect(db_url: &str) -> PgConnection {
        let (cl, conn) = connect(db_url)
            .await
            .expect("can not connect to postgresql");
        ntex::rt::spawn(async move {
            let _ = conn.await;
        });

        let tes = cl.prepare("SELECT * FROM tes").await.unwrap();

        PgConnection {
            cl,
            tes,
            buf: RefCell::new(BytesMut::with_capacity(10 * 1024 * 1024)),
        }
    }
}

impl PgConnection {
    pub async fn tes_db(&self) -> Bytes {
        let nama = String::from("fuji");

        let row = self.cl.query_one(&self.tes, &[&nama]).await.unwrap();

        let mut body = self.buf.borrow_mut();
        utils::reserve(&mut body, 1024);
        Tes {
            nama: row.get(0)
        }
        .to_bytes_mut(&mut *body);
        body.split().freeze()
    }
}
