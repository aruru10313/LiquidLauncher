/*
 * This file is part of LiquidLauncher (https://github.com/CCBlueX/LiquidLauncher)
 *
 * Copyright (c) 2015 - 2025 CCBlueX
 *
 * LiquidLauncher is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * LiquidLauncher is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with LiquidLauncher. If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::BTreeMap;

use crate::auth::ClientAccount;
use crate::minecraft::java::JavaDistribution;
use crate::utils::get_maven_artifact_path;
use crate::HTTP_CLIENT;
use anyhow::{Error, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::{debug, debug_span, error, info, warn};

/// API endpoint url
pub const LAUNCHER_API: [&str; 3] = [
    "https://api.liquidbounce.net",
    "https://api.ccbluex.net",

    // Non-secure connection requires additional confirmation from the user,
    // as they are vulnerable to MITM attacks and data leaks.
    // A VPN or a proxy can be used to secure the connection.
    "http://nossl.api.liquidbounce.net",
];

pub const API_V1: &str = "api/v1";
pub const API_V3: &str = "api/v3";

#[derive(Serialize, Deserialize)]
pub struct Client {
    url: String,
    // To show a warning to the user when using a non-secure connection,
    // we need to pass this information to the frontend.
    is_secure: bool,
    session_token: String
}

impl Client {
    pub fn new(host: &str, session_token: String) -> Self {
        Self {
            url: host.to_string(),
            is_secure: host.starts_with("https://"),
            session_token
        }
    }

    pub async fn lookup(session_token: String) -> Result<Self, String> {
        Ok(Self::new("http://localhost", session_token))
    }

    pub fn is_secure(&self) -> bool {
        true
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    
    pub fn session_token(&self) -> &str {
        &self.session_token
    }

    pub async fn blog_posts(&self, _page: u32) -> Result<PaginatedResponse<BlogPost>> {
        Ok(PaginatedResponse {
            items: vec![],
            pagination: Pagination { current: 1, pages: 1, items: 0 },
        })
    }

    pub async fn branches(&self) -> Result<Branches> {
        Ok(Branches {
            default_branch: "vanilla".to_string(),
            branches: vec!["vanilla".to_string(), "fabric".to_string()],
        })
    }

    fn create_build(id: u32, branch: &str, mc_version: &str, lb_version: &str) -> Build {
        Build {
            build_id: id,
            commit_id: format!("commit-{}", id),
            branch: branch.to_string(),
            subsystem: if branch == "fabric" { "fabric".to_string() } else { "vanilla".to_string() },
            lb_version: lb_version.to_string(),
            mc_version: mc_version.to_string(),
            release: true,
            date: Utc::now(),
            message: format!("{} {}", branch, mc_version),
            url: "".to_string(),
            jre_distribution: JavaDistribution::Zulu,
            jre_version: if mc_version.starts_with("1.8") { 8 } else if mc_version.starts_with("1.12") || mc_version.starts_with("1.16") { 8 } else { 21 },
            subsystem_specific_data: SubsystemSpecificData {
                fabric_api_version: "".to_string(),
                fabric_loader_version: "0.15.7".to_string(),
                kotlin_version: "".to_string(),
                kotlin_mod_version: "".to_string(),
            },
        }
    }

    pub async fn builds_by_branch(&self, branch: &str, _release: bool) -> Result<Vec<Build>> {
        let mut builds = Vec::new();
        if branch == "vanilla" {
            builds.push(Self::create_build(1, "vanilla", "1.21", "1.21"));
            builds.push(Self::create_build(2, "vanilla", "1.20.4", "1.20.4"));
            builds.push(Self::create_build(3, "vanilla", "1.19.4", "1.19.4"));
            builds.push(Self::create_build(4, "vanilla", "1.16.5", "1.16.5"));
            builds.push(Self::create_build(5, "vanilla", "1.12.2", "1.12.2"));
            builds.push(Self::create_build(6, "vanilla", "1.8.9", "1.8.9"));
        } else if branch == "fabric" {
            builds.push(Self::create_build(101, "fabric", "1.21", "1.21"));
            builds.push(Self::create_build(102, "fabric", "1.20.4", "1.20.4"));
            builds.push(Self::create_build(103, "fabric", "1.19.4", "1.19.4"));
        }
        Ok(builds)
    }

    pub async fn fetch_launch_manifest(&self, build_id: u32) -> Result<LaunchManifest> {
        let mut all_builds = self.builds_by_branch("vanilla", true).await?;
        all_builds.extend(self.builds_by_branch("fabric", true).await?);
        
        let build = all_builds.into_iter().find(|b| b.build_id == build_id).unwrap_or_else(|| {
            Self::create_build(1, "vanilla", "1.20.4", "1.20.4")
        });

        let subsystem = if build.branch == "fabric" {
            LoaderSubsystem::Fabric {
                manifest: "https://meta.fabricmc.net/v2/versions/loader/{MINECRAFT_VERSION}/{FABRIC_LOADER_VERSION}/profile/json".to_string(),
                mod_directory: "mods".to_string(),
            }
        } else {
            LoaderSubsystem::Vanilla
        };

        Ok(LaunchManifest {
            build,
            subsystem,
            mods: vec![],
            repositories: std::collections::BTreeMap::new(),
        })
    }

    pub async fn fetch_mods(&self, _mc_version: &str, _subsystem: &str) -> Result<Vec<LoaderMod>> {
        Ok(vec![])
    }

    pub async fn fetch_changelog(&self, build_id: u32) -> Result<Changelog> {
        let mut all_builds = self.builds_by_branch("vanilla", true).await?;
        all_builds.extend(self.builds_by_branch("fabric", true).await?);
        let build = all_builds.into_iter().find(|b| b.build_id == build_id).unwrap_or_else(|| {
            Self::create_build(1, "vanilla", "1.20.4", "1.20.4")
        });
        
        Ok(Changelog {
            build,
            changelog: "Enjoy standard Minecraft!".to_string(),
        })
    }

    pub async fn fetch_user(&self, _client_account: &ClientAccount) -> Result<UserInformation> {
        Ok(UserInformation {
            nickname: "Player".to_string(),
            user_id: "0".to_string(),
            premium: false,
        })
    }

    pub async fn resolve_skip_file(
        &self,
        _client_account: &ClientAccount,
        _pid: &str,
    ) -> Result<SkipFileResolve> {
        Ok(SkipFileResolve {
            error: false,
            msg: "".to_string(),
            target_pid: None,
        })
    }

    pub fn get_direct_download_link(&self, pid: &str) -> String {
        format!("{}/{}/file/{}", self.url, API_V3, pid)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub current: u32,
    pub pages: u32,
    pub items: u32,
}

#[derive(Serialize, Deserialize)]
pub struct BlogPost {
    #[serde(rename(serialize = "postId"))]
    pub post_id: u32,
    #[serde(rename(serialize = "postUid"))]
    pub post_uid: String,
    pub author: String,
    pub title: String,
    pub description: String,
    pub date: NaiveDateTime,
    #[serde(rename(serialize = "bannerText"))]
    pub banner_text: String,
    #[serde(rename(serialize = "bannerImageUrl"))]
    pub banner_image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Branches {
    #[serde(rename = "defaultBranch")]
    pub default_branch: String,
    pub branches: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Changelog {
    pub build: Build,
    pub changelog: String,
}

///
/// JSON struct of Build
///
#[derive(Serialize, Deserialize, Clone)]
pub struct Build {
    #[serde(rename(serialize = "buildId"))]
    pub build_id: u32,
    #[serde(rename(serialize = "commitId"))]
    pub commit_id: String,
    pub branch: String,
    pub subsystem: String,
    #[serde(rename(serialize = "lbVersion"))]
    pub lb_version: String,
    #[serde(rename(serialize = "mcVersion"))]
    pub mc_version: String,
    pub release: bool,
    pub date: DateTime<Utc>,
    pub message: String,
    pub url: String,
    #[serde(rename(serialize = "jreDistribution"), default)]
    pub jre_distribution: JavaDistribution,
    #[serde(rename(serialize = "jreVersion"))]
    pub jre_version: u32,
    #[serde(flatten)]
    pub subsystem_specific_data: SubsystemSpecificData,
}

///
/// Subsystem-specific data
/// This can be used for any subsystem, but for now it is only implemented for Fabric.
/// It has to be turned into an Enum to be able to decide on it's own for specific data, but for now this is not required.
///
#[derive(Serialize, Deserialize, Clone)]
pub struct SubsystemSpecificData {
    // Additional data
    #[serde(rename(serialize = "fabricApiVersion"))]
    pub fabric_api_version: String,
    #[serde(rename(serialize = "fabricLoaderVersion"))]
    pub fabric_loader_version: String,
    #[serde(rename(serialize = "kotlinVersion"))]
    pub kotlin_version: String,
    #[serde(rename(serialize = "kotlinModVersion"))]
    pub kotlin_mod_version: String,
}

///
/// JSON struct of Launch Manifest
///
#[derive(Deserialize)]
pub struct LaunchManifest {
    pub build: Build,
    pub subsystem: LoaderSubsystem,
    pub mods: Vec<LoaderMod>,
    pub repositories: BTreeMap<String, String>,
}

///
/// JSON struct of mod
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoaderMod {
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    #[serde(alias = "default")]
    pub enabled: bool,
    pub name: String,
    pub source: ModSource,
}

///
/// JSON struct of ModSource (the method to be used for downloading the mod)
///
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum ModSource {
    #[serde(rename = "skip")]
    #[serde(rename_all = "camelCase")]
    SkipAd {
        artifact_name: String,
        url: String,
        #[serde(default)]
        extract: bool,
    },
    #[serde(rename = "repository")]
    #[serde(rename_all = "camelCase")]
    Repository {
        repository: String,
        artifact: String,
    },
    #[serde(rename = "local")]
    #[serde(rename_all = "camelCase")]
    Local { file_name: String },
}

impl ModSource {
    pub fn get_path(&self) -> Result<String> {
        Ok(match self {
            ModSource::SkipAd { artifact_name, .. } => format!("{}.jar", artifact_name),
            ModSource::Repository {
                repository: _repository,
                artifact,
            } => get_maven_artifact_path(artifact)?,
            ModSource::Local { file_name } => file_name.clone(),
        })
    }
}

///
/// JSON struct of subsystem
///
#[derive(Deserialize)]
#[serde(tag = "name")]
pub enum LoaderSubsystem {
    #[serde(rename = "vanilla")]
    Vanilla,
    #[serde(rename = "fabric")]
    Fabric {
        manifest: String,
        mod_directory: String,
    },
    #[serde(rename = "forge")]
    Forge {
        manifest: String,
        mod_directory: String,
    },
}

#[derive(Deserialize, Serialize)]
pub struct SkipFileResolve {
    pub error: bool,
    pub msg: String,
    pub target_pid: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserInformation {
    pub nickname: String,
    #[serde(rename = "userId", alias = "user_id")]
    pub user_id: String,
    pub premium: bool,
}
