use async_trait::async_trait;

use crate::{
    contracts::Registry,
    errors::NetworkError,
    package::{contracts::Version, FullPackage, Package, NpmPackage},
};

#[derive(Debug)]
pub struct GitRegistry {
    http: reqwest::Client,
}

impl GitRegistry {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }
}

impl GitRegistry {
    async fn get_archive(&self, package: &Package) -> Result<FullPackage, NetworkError> {
      todo!()
    }
}

#[async_trait]
impl Registry for GitRegistry {
    async fn fetch(&self, package: &Package) -> Result<NpmPackage, NetworkError> {
        let pkg = self.get_archive(&package).await?;

        for (version, remote_package) in pkg.versions.iter() {
            if package.version.satisfies(&version) {
                return Ok(remote_package.clone());
            }
        }

        println!("Failed to fetch version: {}", package.to_string());

        Err(NetworkError::FailedToFetchVersion(
            package.raw_version.clone(),
        ))
    }
}
