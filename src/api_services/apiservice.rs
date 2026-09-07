macro_rules! api_service {
    ($name:ident, $desc:literal) => {
        #[doc=$desc]
        #[derive(Clone, Debug)]
        pub struct $name(crate::OpenLibraryClient);

        impl $name {
            pub(crate) fn new(client: crate::OpenLibraryClient) -> Self {
                Self(client)
            }

            pub fn client(&self) -> crate::OpenLibraryClient {
                self.0.clone()
            }
        }
    };
}

pub(crate) use api_service;
