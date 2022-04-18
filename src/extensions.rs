pub mod octocrab {
    use octocrab::{models, Octocrab, Page, Result};
    use url::Url;

    #[async_trait::async_trait]
    pub(crate) trait UsersExt {
        async fn list_contributors(&self, url: Url) -> Result<Page<models::User>>;
    }

    #[async_trait::async_trait]
    impl UsersExt for Octocrab {
        async fn list_contributors(&self, url: Url) -> Result<Page<models::User>> {
            self.get(url, None::<&()>).await
        }
    }
}
