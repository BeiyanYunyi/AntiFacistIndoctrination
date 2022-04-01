use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SignReqData {
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignReq {
    data: SignReqData,
    message: String,
    redirect: String,
    status: u8,
}
