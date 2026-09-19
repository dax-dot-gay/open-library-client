macro_rules! api_service {
    ($name:ident, $desc:literal) => {
        #[doc=$desc]
        #[derive(Clone, Debug)]
        pub struct $name(crate::OpenLibraryClient);

        impl $name {
            pub(crate) fn new(client: crate::OpenLibraryClient) -> Self {
                Self(client)
            }

            /// Return the contained API client
            pub fn client(&self) -> crate::OpenLibraryClient {
                self.0.clone()
            }

            /// Return the reqwest client
            pub fn reqwest(&self) -> reqwest::Client {
                self.client().client()
            }
        }
    };
}

pub(crate) use api_service;
