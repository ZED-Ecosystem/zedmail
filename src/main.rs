pub struct ZedMailPacket {
    pub sender_handle: String,
    pub recipient_handle: String,
    pub encrypted_payload: Vec<u8>,
    pub zk_proof_hash: String,
}

impl ZedMailPacket {
    pub fn new(sender: &str, recipient: &str, raw_message: &[u8]) -> Self {
        // Enforce handle domain format
        let formatted_sender = if sender.ends_with("@zedmail") {
            sender.to_string()
        } else {
            format!("{}@zedmail", sender)
        };

        let formatted_recipient = if recipient.ends_with("@zedmail") {
            recipient.to_string()
        } else {
            format!("{}@zedmail", recipient)
        };

        // Placeholder for ZK payload obfuscation & mesh transport routing
        let dummy_encrypted = raw_message.iter().map(|b| b ^ 0xFF).collect();

        Self {
            sender_handle: formatted_sender,
            recipient_handle: formatted_recipient,
            encrypted_payload: dummy_encrypted,
            zk_proof_hash: "0xzkp_3f92a10b88c4d9e2".to_string(),
        }
    }

    pub fn transmit_mesh_packet(&self) {
        println!("[MESH TRANSPORT] Sending encrypted payload from {} to {}", self.sender_handle, self.recipient_handle);
        println!(" -> ZK Proof Hash: {}", self.zk_proof_hash);
        println!(" -> Payload Size: {} bytes", self.encrypted_payload.len());
    }
}

fn main() {
    println!("=== ZEDMail Off-Grid Mesh Protocol ===");
    let packet = ZedMailPacket::new("genesis_node", "validator_01", b"PEER_TO_PEER_MESH_ROUTING_TEST");
    packet.transmit_mesh_packet();
}
