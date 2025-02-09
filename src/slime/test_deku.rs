#[cfg(test)]
mod tests {
    use crate::slime::deku::PacketType;
    use deku::DekuContainerWrite;
    use md5::{Digest, Md5};
    use nalgebra::{Quaternion, UnitQuaternion};

    #[test]
    fn handshake() {
        let mut hasher = Md5::new();
        hasher.update(b"This is a joycon serial number");
        let mac: [u8; 6] = hasher.finalize()[0..6].try_into().unwrap();
        let handshake = PacketType::Handshake {
            packet_id: 1,
            board: 2,
            imu: 3,
            mcu_type: 4,
            imu_info: (5, 6, 7),
            build: 8,
            firmware: "test".to_string().into(),
            mac_address: mac,
        };
        let data: Vec<u8> = vec![
            0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0,
            0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 4, 116, 101, 115, 116, 121, 34, 164, 250, 231, 204,
        ];

        assert_eq!(handshake.to_bytes().unwrap(), data);
    }
    #[test]
    fn quat() {
        let quat = UnitQuaternion::new_unchecked(Quaternion::new(1.0f64, 0.0f64, 0.0f64, 0.0f64));
        let rotation = PacketType::Rotation {
            packet_id: 1,
            quat: (*quat.quaternion()).into(),
        };

        let data: Vec<u8> = vec![
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 128, 0, 0,
        ];

        assert_eq!(rotation.to_bytes().unwrap(), data);
    }
}
