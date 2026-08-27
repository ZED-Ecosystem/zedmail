use std::fmt;

#[derive(Debug, Clone)]
pub struct MeshNodeInfo {
    pub node_id: String,
    pub is_offgrid_relay: bool,
    pub active_peers: u32,
}

pub struct ZedMailPacket {
    pub sender: String,
    pub recipient: String,
    pub nonce: u64,
    pub encrypted_payload: Vec<u8>,
    pub zk_proof: String,
}

impl ZedMailPacket {
    pub fn create_encrypted_message(sender_raw: &str, recipient_raw: &str, raw_body: &[u8], nonce: u64) -> Result<Self, &'static str> {
        let sender = Self::format_handle(sender_raw);
        let recipient = Self::format_handle(recipient_raw);

        if raw_body.is_empty() {
            return Err("Payload cannot be empty.");
        }

        let obfuscated_payload: Vec<u8> = raw_body.iter().enumerate().map(|(i, &byte)| byte ^ ((nonce as u8).wrapping_add(i as u8))).collect();

        let mock_zk_proof = format!("0xzkp_{:016x}", nonce ^ 0xA5A5A5A5);

        Ok(Self {
            sender,
            recipient,
            nonce,
            encrypted_payload: obfuscated_payload,
            zk_proof: mock_zk_proof,
        })
    }

    fn format_handle(handle: &str) -> String {
        if handle.ends_with("@zedmail") {
            handle.to_string()
        } else {
            format!("{}@zedmail", handle)
        }
    }

    pub fn transmit_over_mesh(&self, relay: &MeshNodeInfo) {
        println!("[MESH TRANSMISSION] Node {} broadcasting packet...", relay.node_id);
        println!(" -> Sender Handle:    {}", self.sender);
        println!(" -> Recipient Handle: {}", self.recipient);
        println!(" -> ZK Proof Hash:    {}", self.zk_proof);
        println!(" -> Encrypted Bytes:  {} bytes", self.encrypted_payload.len());
        println!(" -> Relay Status:     Off-Grid Transport Active ({} peers)", relay.active_peers);
    }
}

impl fmt::Display for ZedMailPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZedMailPacket({} -> {}, ZK: {})", self.sender, self.recipient, self.zk_proof)
    }
}

fn main() {
    println!("=== ZEDMail Encrypted Off-Grid Mesh Protocol ===");
    
    let relay_node = MeshNodeInfo {
        node_id: "MESH-RELAY-LAGOS-01".to_string(),
        is_offgrid_relay: true,
        active_peers: 14,
    };

    let message_body = b"CONFIDENTIAL_L1_VALIDATOR_HANDSHAKE_KEY";
    let packet = ZedMailPacket::create_encrypted_message("sovereign_node", "reserve_vault", message_body, 100492).unwrap();

    packet.transmit_over_mesh(&relay_node);
    println!("Packet Diagnostic Summary: {}", packet);
}
