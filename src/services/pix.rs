use fast_qr::convert::{image::ImageBuilder, Builder, ConvertError, Shape};
use fast_qr::qr::QRBuilder;

pub struct Pix {
    pub name: String,
    pub pix_key: String,
    pub value: String,
    pub city: String,
    pub txt_id: String,
}


impl Pix {
    pub fn generate_pix(pix_data: &mut Pix) -> String{
        let payload_format = format!("000201");
        let merchant_account = format!("0014BR.GOV.BCB.PIX01{:02}{}", pix_data.pix_key.len(), pix_data.pix_key);
        let transaction_amount = format!("54{:02}{}", pix_data.value.len(), pix_data.value);
        let add_data_field = format!("05{:02}{}", pix_data.txt_id.len(), pix_data.txt_id);
        let mercant_name = format!("59{:02}{}", pix_data.name.len(), pix_data.name);
        let merchant_city = format!("60{:02}{}", pix_data.city.len(), pix_data.city);
        let payload = format!("{}26{}{}520400005303986{}5802BR{}{}62{:02}{}6304", 
            payload_format,
            merchant_account.len(),
            merchant_account,
            transaction_amount,
            mercant_name,
            merchant_city,
            add_data_field.len(),
            add_data_field
        );
        let crc = Self::generate_crc(payload.clone());
        let payload_complete = format!("{}{}", payload, crc);
        let _ = Self::generate_qr_code(payload_complete.clone());
        payload_complete
    }
    
    fn generate_crc(payload: String) -> String {
        let mut crc_value = 0xFFFF;

        for byte in payload.as_bytes() {
            let mut x = ((crc_value >> 8) ^ (*byte as u16)) & 0xFF;
            x ^= x >> 4;
            crc_value = (crc_value << 8) ^ (x << 12) ^ (x << 5) ^ x;
            crc_value &= 0xFFFF;
        }
        
        format!("{:04X}", crc_value)
    }
    
    pub fn generate_qr_code(payload: String) -> Result<(), ConvertError> {
        let qrcode = QRBuilder::new(payload)
            .build()
            .unwrap();
    
        let _img = ImageBuilder::default()
                .shape(Shape::RoundedSquare)
                .background_color([255, 255, 255])
                .fit_width(600)
                .to_file(&qrcode, "./temp_files/out.png");
    
        Ok(())
    }
}

#[test]
fn test_generate_pix() {
    let mut pix = Pix {
        name: "fritzhenrique".to_string(),
        pix_key: "+5595991561987".to_string(),
        value: "10.50".to_string(),
        city: "boavista".to_string(),
        txt_id: "huha".to_string()
    };
    let qr = Pix::generate_pix(&mut pix);
    assert_eq!(qr, "00020126360014BR.GOV.BCB.PIX0114+5595991561987520400005303986540510.505802BR5913fritzhenrique6008boavista62080504huha6304BDD4");
}

#[test]
fn test_generate_crc() {
    let crc = Pix::generate_crc("00020126360014BR.GOV.BCB.PIX0114+5595991561987520400005303986540510.505802BR5913fritzhenrique6008boavista62080504huha6304".to_string());
    assert_eq!(crc, "BDD4");
}