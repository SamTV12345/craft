use std::collections::HashMap;

use crate::actors::PackageType;
use crate::cache::RegistryKey;
use crate::{contracts::PipeArtifact, package::NpmPackage};
// --------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ResolveArtifacts {
    packages: HashMap<String, ResolvedItem>,
}

#[derive(Debug, Clone)]
pub struct ResolvedItem {
    pub package: NpmPackage,
    pub parent: Option<Vec<RegistryKey>>,
    pub specifier: String,
    pub package_type: PackageType,
}

// --------------------------------------------------------------------------------

impl ResolvedItem {
    pub fn new(
        package: NpmPackage,
        parent: Option<Vec<RegistryKey>>,
        mut specifier: String,
        package_type: PackageType,
    ) -> Self {
        if specifier == "*" {
            specifier = format!("^{}", package.version)
        }

        Self {
            package_type,
            package,
            parent,
            specifier,
        }
    }

    #[cfg(test)]
    pub fn with_no_parent(
        package: NpmPackage,
        specifier: String,
        package_type: PackageType,
    ) -> Self {
        Self::new(package, None, specifier, package_type)
    }
}

// --------------------------------------------------------------------------------

impl ResolveArtifacts {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&ResolvedItem> {
        self.packages.get(key)
    }

    pub fn insert(&mut self, key: String, value: ResolvedItem) {
        self.packages.insert(key, value);
    }
}

// --------------------------------------------------------------------------------

impl PipeArtifact<Vec<ResolvedItem>> for ResolveArtifacts {
    fn get_artifacts(&self) -> Vec<ResolvedItem> {
        self.packages.values().cloned().collect()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_artifacts() {
        let mut resolve_artifacts = ResolveArtifacts::new();

        let package = serde_json::from_str::<NpmPackage>(
            r#"
            {
                "name": "package",
                "version": "1.0.0",
                "dist": {
                    "shasum": "shasum",
                    "tarball": "https://registry.npmjs.org/package/-/package-1.0.0.tgz"
                }
            }
            "#,
        )
        .unwrap();
        resolve_artifacts.insert(
            "package".to_string(),
            ResolvedItem::with_no_parent(
                package,
                "1.2.0".to_string(),
                PackageType::Prod("1.2.0".to_string()),
            ),
        );

        assert_eq!(
            resolve_artifacts.get("package").unwrap().package.version,
            "1.0.0"
        );
    }

    #[test]
    fn test_get_artifacts() {
        let mut resolve_artifacts = ResolveArtifacts::new();

        let package = serde_json::from_str::<NpmPackage>(
            r#"
            {
                "name": "package",
                "version": "1.0.0",
                "dist": {
                    "shasum": "shasum",
                    "tarball": "https://registry.npmjs.org/package/-/package-1.0.0.tgz"
                }
            }
            "#,
        )
        .unwrap();
        resolve_artifacts.insert(
            "package".to_string(),
            ResolvedItem::with_no_parent(
                package,
                "1.0.0".to_string(),
                PackageType::Prod("1.2.0".to_string()),
            ),
        );

        assert_eq!(resolve_artifacts.get_artifacts().len(), 1);
    }
}
