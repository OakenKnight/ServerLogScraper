use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct IcNode{
    pub ic_node: String,
    pub dc : String,
    pub ic_subnet: String,
    pub ipv6: String,
    pub port : String
}
