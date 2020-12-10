use std::{error, fs, io, path, process};
use std::io::{Read, Write};

use curl::easy::{Easy, WriteError};
use curl::easy;

pub struct WebContext {
    pub cookie_file_path: String,
    pub cookie_store: String,
    pub lynx_config_path: String,
    pub tmpdir: tempfile::TempDir,
    // TODO implement drop? Should not be necessary
}

impl WebContext {
    pub fn new(cookie_file_path: String, lynx_config_path: String) -> io::Result<WebContext> {
        let mut cookie_store = String::new();
        fs::File::open(&cookie_file_path)?.read_to_string(&mut cookie_store)?;
        return Ok(WebContext {
            cookie_file_path,
            cookie_store,
            lynx_config_path,
            tmpdir: tempfile::Builder::new().suffix("aoc").rand_bytes(6).tempdir()?,
        });
    }

    pub fn close(self) -> io::Result<()> {
        self.tmpdir.close()
    }

    pub fn lynx(&mut self, address: String) -> io::Result<()> {
        {
            fs::File::create(&self.cookie_file_path)?.write(self.cookie_store.as_bytes())?;
        }
        let args = &[
            format!("-cfg={}", self.lynx_config_path),
            format!("-cookie_file={}", self.cookie_file_path),     // TODO cmd-file
            address,
        ];

        process::Command::new("lynx")
            .args(args)
            .status()?;
        
        Ok(())
    }

    pub fn curl_base(&mut self) -> Result<Easy, curl::Error> {
        let mut request = Easy::new();
        request.follow_location(true)?;
        request.cookie_list(&self.cookie_store)?;
        return Ok(request);
    }

    pub fn curl_request<F>(&mut self, url: &str, f: F) -> Result<(), curl::Error>
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + Send + 'static {
        let mut request = self.curl_base()?;
        request.url(url)?;
        request.write_function(f)?;
        request.perform()?;
        return Ok(());
    }

    pub fn curl_post<F>(&mut self, url: &str, data: &[u8], f: F) -> Result<(), curl::Error>
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + Send + 'static {
        let mut request = self.curl_base()?;
        request.url(url)?;
        request.write_function(f)?;
        request.post(true)?;
        request.post_fields_copy(data)?;
        let mut headers = easy::List::new();
        headers.append("Content-Type: application/x-www-form-urlencoded")?;
        request.http_headers(headers)?;
        request.perform()?;
        return Ok(());
    }

    pub fn curl_request_to_file(&mut self, url: &str, mut file: fs::File) -> Result<(), Box<dyn error::Error>> {
        self.curl_request(url, move |data| file.write(data).map_err(|_| easy::WriteError::Pause)).map_err(Into::into)
    }

    pub fn curl_request_to_named_file<P: AsRef<path::Path>>(&mut self, url: &str, path: P) -> Result<(), Box<dyn error::Error>> {
        self.curl_request_to_file(url, fs::File::create(path)?)
    }

    pub fn curl_post_to_lynx(&mut self, url: &str, data: &[u8]) -> Result<(), Box<dyn error::Error>> {
        let mut path = path::PathBuf::from(self.tmpdir.path());
        path.push("result.html");
        {
            let mut file = fs::File::create(&path)?;
            self.curl_post(url, data, move |data: &[u8]| {
                file.write(data).map_err(|_| WriteError::Pause)
            })?;
        }
        self.lynx(path.to_str().unwrap().to_owned())?;
        return Ok(());
    }
}