pub mod octocrab {
    use octocrab::{models, Octocrab, Page, Result};
    use url::Url;

    #[async_trait::async_trait]
    pub(crate) trait OrganisationExt {
        async fn list_every_organisation(&self) -> Result<Page<models::orgs::Organization>>;
    }

    #[async_trait::async_trait]
    impl OrganisationExt for Octocrab {
        async fn list_every_organisation(&self) -> Result<Page<models::orgs::Organization>> {
            self.get("/organizations", None::<&()>).await
        }
    }

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
