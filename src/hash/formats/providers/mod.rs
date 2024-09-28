//! Providers for xml parsing

pub trait XMLProvider {
    fn find_xpath(source: &str, xpath: &str) -> Option<String>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "libxml")] {
        mod libxml;
        pub use libxml::LibXML as Provider;
    } else {
        pub use other::Other as Provider;
    }
}

mod other;
