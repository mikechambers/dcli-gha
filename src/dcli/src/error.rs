/*
* Copyright 2020 Mike Chambers
* https://github.com/mikechambers/dcli
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy of 
* this software and associated documentation files (the "Software"), to deal in 
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do 
* so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all 
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR 
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//For error handling approach, we are going to start with one error for all
//APIs and individual apps. Given that there is only a general range of what the apps
//do, mostly loading and parsing api data, then we should be able to cover
//error cases without super ballooning the number of error types.
//If it turns out this becomes unwieldy, then we will break it out, into API
//and app specific errors

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    ApiRequest {description:String },
    ApiStatus {description:String },

    //when parameters are malformed in wrong format (i.e. expecting id, getting a name)
    ParameterParseFailure,

    //when id & platform are not correct combination
    InvalidParameters,

    //Api key not set correctly
    ApiKeyMissingFromRequest,

    ApiNotAvailableException,

    PrivacyException,

    ApiParse {description:String },
    IoError {description:String },
    IoErrorDirIsFile {description:String },
    ZipError {description:String },
    Unknown {description:String },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::ApiRequest {description} => write!(f, "Error calling Destiny 2 API. {}", description),
            Error::ApiStatus {description} => write!(f, "Destiny 2 API call returned an error. {}", description),
            Error::ApiParse {description} => write!(f, "Error parsing results from Destiny 2 API call. {}", description),
            Error::IoError {description} => write!(f, "Error working with file system. {}", description),
            Error::ZipError {description} => write!(f, "Error decompressing manifest. {}", description),
            Error::IoErrorDirIsFile {description} => write!(f, "Expected directory but found file. {}", description),
            Error::Unknown {description} => write!(f, "An unknown error occured. {}", description),
            Error::ParameterParseFailure => write!(f, "Could not parse Parameters. (code 7)"),
            Error::InvalidParameters => write!(f, "Invalid input parameters. (code 18)"),
            Error::ApiKeyMissingFromRequest => write!(f, "Missing API Key. Set DESTINY_API_KEY environment variable before compiling."),
            Error::ApiNotAvailableException => write!(f, "The Destiny API is currently not available. (code 5)"),
            Error::PrivacyException => write!(f, "Privacy settings for Bungie account are too restrictive. (code 5)"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ApiParse{description:format!("serde_json::Error : {:?}", err)} //TODO:: impliment this for all error types
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ApiRequest{description:format!("reqwest::Error : {:?}", err)}
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError{description:format!("std::io::Error : {:?}", err)}
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Error {
        Error::ZipError{description:format!("zip::result::ZipError : {:?}", err)}
    }
}