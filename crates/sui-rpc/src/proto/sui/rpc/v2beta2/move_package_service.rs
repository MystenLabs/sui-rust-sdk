use bytes::Bytes;
use sui_sdk_types::Address;

use super::*;

impl GetPackageRequest {
    pub fn new(package_id: &Address) -> Self {
        Self {
            package_id: Some(package_id.to_string()),
        }
    }
}

impl GetPackageResponse {
    pub fn new(package: Package) -> Self {
        Self {
            package: Some(package),
        }
    }
}

impl GetDatatypeRequest {
    pub fn new(package_id: &Address, module: &str, name: &str) -> Self {
        Self {
            package_id: Some(package_id.to_string()),
            module_name: Some(module.into()),
            name: Some(name.into()),
        }
    }
}

impl GetDatatypeResponse {
    pub fn new(datatype: DatatypeDescriptor) -> Self {
        Self {
            datatype: Some(datatype),
        }
    }
}

impl GetFunctionRequest {
    pub fn new(package_id: &Address, module: &str, name: &str) -> Self {
        Self {
            package_id: Some(package_id.to_string()),
            module_name: Some(module.into()),
            name: Some(name.into()),
        }
    }
}

impl GetFunctionResponse {
    pub fn new(function: FunctionDescriptor) -> Self {
        Self {
            function: Some(function),
        }
    }
}

impl ListPackageVersionsRequest {
    pub fn new(package_id: &Address) -> Self {
        Self {
            package_id: Some(package_id.to_string()),
            page_size: None,
            page_token: None,
        }
    }

    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn with_page_token(mut self, page_token: Bytes) -> Self {
        self.page_token = Some(page_token);
        self
    }
}

impl ListPackageVersionsResponse {
    pub fn new(versions: Vec<PackageVersion>, next_page_token: Option<Bytes>) -> Self {
        Self {
            versions,
            next_page_token,
        }
    }
}

impl PackageVersion {
    pub fn new(package_id: &Address, version: u64) -> Self {
        Self {
            package_id: Some(package_id.to_string()),
            version: Some(version),
        }
    }
}
