// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.
use nfs;
use routing;
use time;
use client;
use nfs::traits::FileWrapper;

pub struct Container {
    client: ::std::sync::Arc<::std::sync::Mutex<client::Client>>,
    directory_listing: nfs::directory_listing::DirectoryListing
}

impl Container {

    /// Authorizes the root directory access and return the Container
    /// Entry point for the Rest API
    pub fn authorize(client: ::std::sync::Arc<::std::sync::Mutex<client::Client>>, dir_id: [u8;64], parent_dir_id: [u8;64]) -> Result<Container, String> {
        let mut directory_helper = nfs::helper::DirectoryHelper::new(client.clone());
        let result = directory_helper.get(::routing::NameType(dir_id), ::routing::NameType(parent_dir_id));
        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        }
        Ok(Container {
            client: client,
            directory_listing: result.unwrap()
        })
    }

    pub fn get_id(&self) -> [u8;64] {
        self.directory_listing.get_id().0
    }

    pub fn get_user_metadata(&self) -> Option<Vec<u8>> {
        self.directory_listing.get_metadata().get_user_metadata()
    }

    pub fn get_name(&self) -> String {
        self.directory_listing.get_metadata().get_name()
    }

    pub fn get_created_time(&self) -> time::Tm {
        self.directory_listing.get_metadata().get_created_time()
    }

    pub fn get_modified_time(&self) -> time::Tm {
        self.directory_listing.get_metadata().get_modified_time()
    }

    pub fn get_blobs(&self) -> Vec<nfs::rest::Blob> {
        self.directory_listing.get_files().iter().map(|x| nfs::rest::Blob::convert_from_file(self.client.clone(), x.clone())).collect()
    }

    pub fn get_containers(&self) -> Vec<String> {
        self.directory_listing.get_sub_directories().iter().map(|x| x.get_metadata().get_name()).collect()
    }

}

impl nfs::traits::DirectoryListingWrapper for Container {

    fn convert_to_directory_listing(&self) -> nfs::directory_listing::DirectoryListing {
        self.directory_listing.clone()
    }

    fn convert_from_directory_listing(client: ::std::sync::Arc<::std::sync::Mutex<client::Client>>, directory_listing: nfs::directory_listing::DirectoryListing) -> Container {
        Container {
            client: client,
            directory_listing: directory_listing
        }
    }

}