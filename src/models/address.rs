use chrono::NaiveDateTime;
//
use crate::schema::addresses;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Address {
    pub id: i64,
    pub created_at: NaiveDateTime, // Local::now().naive_local()
    //
    pub label: String,
    //
    pub pubkey: String,
    pub privkey: String,
    pub seed_phrase: String,
    //
    pub owner_id: i64,
    pub currency_id: i64,
}

#[derive(Insertable)]
#[table_name = "addresses"]
pub struct NewAddress<'a> {
    pub label: &'a str,
    //
    pub pubkey: &'a str,
    pub privkey: &'a str,
    pub seed_phrase: &'a str,
    //
    pub owner_id: i64,
    pub currency_id: i64,
}

/// AddrResp represents an Address as gets returned by the API
#[derive(Serialize)]
pub struct AddrResp {
    pub addr: AddrPub,
}
/// AddrPub is an Address stripped to essential, public fields
#[derive(Serialize)]
pub struct AddrPub {
    pub id: i64,
    pub label: String,
    pub pubkey: String,
    pub currency_id: i64,
}
impl From<Address> for AddrResp {
    fn from(addr: Address) -> Self {
        AddrResp {
            addr: AddrPub {
                id: addr.id,
                label: addr.label,
                pubkey: addr.pubkey,
                currency_id: addr.currency_id,
            },
        }
    }
}
