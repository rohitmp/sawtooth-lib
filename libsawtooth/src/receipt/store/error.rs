/*
 * Copyright 2021 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use std::error::Error;

use crate::error::{
    ConstraintViolationError, InternalError, InvalidStateError, ResourceTemporarilyUnavailableError,
};

#[derive(Debug)]
pub enum ReceiptStoreError {
    ConstraintViolationError(ConstraintViolationError),
    InternalError(InternalError),
    InvalidStateError(InvalidStateError),
    ResourceTemporarilyUnavailableError(ResourceTemporarilyUnavailableError),
}

impl Error for ReceiptStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ConstraintViolationError(err) => Some(err),
            Self::InternalError(err) => Some(err),
            Self::InvalidStateError(err) => Some(err),
            Self::ResourceTemporarilyUnavailableError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for ReceiptStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ConstraintViolationError(err) => err.fmt(f),
            Self::InternalError(err) => err.fmt(f),
            Self::InvalidStateError(err) => err.fmt(f),
            Self::ResourceTemporarilyUnavailableError(err) => err.fmt(f),
        }
    }
}
