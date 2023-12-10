use serde::Serialize;

pub trait Hashable {
    fn hashable_data(&self) -> Vec<u8>;

    fn hash(&self) -> String {
        blake3::hash(&self.hashable_data()).to_string()
    }

    fn refresh_hash(&mut self);
}

pub trait EmailTemplate: Serialize + Sized {
    const TEMPLATE_NAME: &'static str;

    fn data(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize template data")
    }

    fn email_content(self) -> aws_sdk_sesv2::types::EmailContent {
        let template = aws_sdk_sesv2::types::Template::builder()
            .template_name(Self::TEMPLATE_NAME)
            .template_data(self.data())
            .build();

        aws_sdk_sesv2::types::EmailContent::builder()
            .template(template)
            .build()
    }
}
