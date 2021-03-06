/* The MIT License (MIT)

Copyright (c) 2014 iron

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */


use iron::Request;
use std::iter::FromIterator;
use std::path::{Component, PathBuf, Path};
use std::convert::AsRef;
use url::percent_encoding::percent_decode;

pub struct RequestedPath {
    pub path: PathBuf,
}

#[inline]
fn decode_percents(string: &&str) -> String {
    percent_decode(string.as_bytes()).decode_utf8().unwrap().into_owned()
}

fn normalize_path(path: &Path) -> PathBuf {
    path.components().fold(PathBuf::new(), |mut result, p| {
        match p {
            Component::Normal(x) => {
                result.push(x);
                result
            }
            Component::ParentDir => {
                result.pop();
                result
            },
            _ => result
        }
    })
}

impl RequestedPath {
    pub fn new<P: AsRef<Path>>(root_path: P, request: &Request) -> RequestedPath {
        let decoded_req_path = PathBuf::from_iter(request.url.path().iter().map(decode_percents));
        let mut result = root_path.as_ref().to_path_buf();
        result.extend(&normalize_path(&decoded_req_path));
        RequestedPath { path: result }
    }

}
