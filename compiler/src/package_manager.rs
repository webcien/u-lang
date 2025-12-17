// package_manager.rs — U v1.3 Package Manager
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// This module implements the package manager for U Language, including:
// - ul init: Create a new package
// - ul install: Install dependencies
// - ul publish: Publish to registry
// - ul update: Update dependencies

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub package: PackageMetadata,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, Dependency>,
    #[serde(default, rename = "build-dependencies")]
    pub build_dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub lib: Option<LibTarget>,
    #[serde(default)]
    pub bin: Vec<BinTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub edition: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub documentation: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedDependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default = "default_lib_path")]
    pub path: String,
}

fn default_lib_path() -> String {
    "src/lib.ul".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinTarget {
    pub name: String,
    #[serde(default = "default_bin_path")]
    pub path: String,
}

fn default_bin_path() -> String {
    "src/main.ul".to_string()
}

// ============================================================================
// Package Manager
// ============================================================================

pub struct PackageManager {
    registry_url: String,
    cache_dir: PathBuf,
}

impl PackageManager {
    pub fn new() -> Self {
        let cache_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".u")
            .join("cache");
        
        PackageManager {
            registry_url: "https://packages.u-lang.dev/api/v1".to_string(),
            cache_dir,
        }
    }

    // ========================================================================
    // ul init
    // ========================================================================

    pub fn init(&self, name: &str, path: &Path) -> Result<(), String> {
        // Create project directory
        let project_dir = path.join(name);
        fs::create_dir_all(&project_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;

        // Create ul.toml
        let toml_content = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
authors = []
edition = "2025"
description = "A U Language package"
license = "MIT"

[dependencies]

[[bin]]
name = "{}"
path = "src/main.ul"
"#,
            name, name
        );

        fs::write(project_dir.join("ul.toml"), toml_content)
            .map_err(|e| format!("Failed to write ul.toml: {}", e))?;

        // Create src directory
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("Failed to create src directory: {}", e))?;

        // Create main.ul
        let main_content = r#"fn main() {
    unsafe {
        printf("Hello from U Language!\n");
    }
    return 0;
}

extern "C" {
    fn printf(format: ptr, ...);
}
"#;

        fs::write(src_dir.join("main.ul"), main_content)
            .map_err(|e| format!("Failed to write main.ul: {}", e))?;

        println!("✓ Created package '{}' at {}", name, project_dir.display());
        Ok(())
    }

    // ========================================================================
    // ul install
    // ========================================================================

    pub fn install(&self, package_name: &str, version: Option<&str>) -> Result<(), String> {
        let version_str = version.unwrap_or("latest");
        
        println!("Installing {} {}...", package_name, version_str);

        // Check if package exists in cache
        let cache_path = self.cache_dir.join(package_name).join(version_str);
        if cache_path.exists() {
            println!("✓ Package already in cache: {}", cache_path.display());
            return Ok(());
        }

        // Download from registry
        let url = format!("{}/packages/{}/{}", self.registry_url, package_name, version_str);
        
        println!("Downloading from {}...", url);
        
        // For now, we'll use a placeholder since the registry doesn't exist yet
        println!("⚠ Registry not yet implemented. Package installation is a placeholder.");
        println!("  In a real implementation, this would:");
        println!("  1. Download package from {}", url);
        println!("  2. Verify package integrity");
        println!("  3. Extract to cache at {}", cache_path.display());
        println!("  4. Update ul.lock file");

        Ok(())
    }

    pub fn install_from_git(&self, git_url: &str, tag: Option<&str>) -> Result<(), String> {
        println!("Installing from Git: {}...", git_url);

        let repo_name = git_url.split('/').last().unwrap_or("package");
        let cache_path = self.cache_dir.join("git").join(repo_name);

        // Clone repository
        let mut cmd = Command::new("git");
        cmd.arg("clone").arg(git_url).arg(&cache_path);

        if let Some(tag_name) = tag {
            cmd.arg("--branch").arg(tag_name);
        }

        let output = cmd.output()
            .map_err(|e| format!("Failed to execute git: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Git clone failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("✓ Cloned {} to {}", git_url, cache_path.display());
        Ok(())
    }

    // ========================================================================
    // ul publish
    // ========================================================================

    pub fn publish(&self, manifest_path: &Path) -> Result<(), String> {
        // Read ul.toml
        let manifest_content = fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read ul.toml: {}", e))?;

        let package: Package = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse ul.toml: {}", e))?;

        println!("Publishing {} {}...", package.package.name, package.package.version);

        // Validate package
        self.validate_package(&package)?;

        // Package source code
        let package_dir = manifest_path.parent().unwrap();
        let archive_path = self.create_package_archive(package_dir, &package)?;

        println!("Created archive: {}", archive_path.display());

        // Upload to registry
        let url = format!(
            "{}/packages/{}/{}",
            self.registry_url,
            package.package.name,
            package.package.version
        );

        println!("Uploading to {}...", url);
        println!("⚠ Registry not yet implemented. Package publishing is a placeholder.");
        println!("  In a real implementation, this would:");
        println!("  1. Authenticate with registry");
        println!("  2. Upload package archive");
        println!("  3. Update package index");

        Ok(())
    }

    fn validate_package(&self, package: &Package) -> Result<(), String> {
        // Check required fields
        if package.package.name.is_empty() {
            return Err("Package name is required".to_string());
        }

        if package.package.version.is_empty() {
            return Err("Package version is required".to_string());
        }

        // Validate version format (semantic versioning)
        if !self.is_valid_semver(&package.package.version) {
            return Err(format!("Invalid version: {}", package.package.version));
        }

        Ok(())
    }

    fn is_valid_semver(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        parts.iter().all(|part| part.parse::<u32>().is_ok())
    }

    fn create_package_archive(&self, package_dir: &Path, package: &Package) -> Result<PathBuf, String> {
        let archive_name = format!("{}-{}.tar.gz", package.package.name, package.package.version);
        let archive_path = package_dir.join(&archive_name);

        // Create tar.gz archive (placeholder)
        println!("Creating archive {}...", archive_name);
        
        // In a real implementation, this would use tar/gzip libraries
        // For now, we'll just create an empty file
        fs::write(&archive_path, b"")
            .map_err(|e| format!("Failed to create archive: {}", e))?;

        Ok(archive_path)
    }

    // ========================================================================
    // ul update
    // ========================================================================

    pub fn update(&self, manifest_path: &Path) -> Result<(), String> {
        // Read ul.toml
        let manifest_content = fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read ul.toml: {}", e))?;

        let package: Package = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse ul.toml: {}", e))?;

        println!("Updating dependencies for {}...", package.package.name);

        for (dep_name, _dep_spec) in &package.dependencies {
            println!("Checking {} for updates...", dep_name);
            // In a real implementation, this would:
            // 1. Query registry for latest compatible version
            // 2. Download if newer version available
            // 3. Update ul.lock file
        }

        println!("⚠ Dependency updates are a placeholder.");
        Ok(())
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    pub fn read_manifest(&self, path: &Path) -> Result<Package, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read ul.toml: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse ul.toml: {}", e))
    }
}

// ============================================================================
// Dependency Resolution
// ============================================================================

pub struct DependencyResolver {
    packages: HashMap<String, Package>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        DependencyResolver {
            packages: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, root_package: &Package) -> Result<Vec<String>, String> {
        let mut resolved = Vec::new();
        let mut visited = std::collections::HashSet::new();

        self.resolve_recursive(&root_package.package.name, &mut resolved, &mut visited)?;

        Ok(resolved)
    }

    fn resolve_recursive(
        &self,
        package_name: &str,
        resolved: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<(), String> {
        if visited.contains(package_name) {
            return Ok(());
        }

        visited.insert(package_name.to_string());

        // Get package dependencies
        if let Some(package) = self.packages.get(package_name) {
            for (dep_name, _dep_spec) in &package.dependencies {
                self.resolve_recursive(dep_name, resolved, visited)?;
            }
        }

        resolved.push(package_name.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_validation() {
        let pm = PackageManager::new();
        
        let mut package = Package {
            package: PackageMetadata {
                name: "test-package".to_string(),
                version: "1.0.0".to_string(),
                authors: vec![],
                edition: "2025".to_string(),
                description: String::new(),
                license: String::new(),
                repository: String::new(),
                homepage: String::new(),
                documentation: String::new(),
                keywords: vec![],
                categories: vec![],
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            features: HashMap::new(),
            lib: None,
            bin: vec![],
        };

        assert!(pm.validate_package(&package).is_ok());

        // Test invalid version
        package.package.version = "invalid".to_string();
        assert!(pm.validate_package(&package).is_err());
    }

    #[test]
    fn test_semver_validation() {
        let pm = PackageManager::new();
        
        assert!(pm.is_valid_semver("1.0.0"));
        assert!(pm.is_valid_semver("0.1.0"));
        assert!(pm.is_valid_semver("10.20.30"));
        
        assert!(!pm.is_valid_semver("1.0"));
        assert!(!pm.is_valid_semver("1.0.0.0"));
        assert!(!pm.is_valid_semver("invalid"));
    }
}
